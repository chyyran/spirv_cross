use crate::bindings as br;
use crate::{compiler, spirv, ErrorCode};
use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr;

pub use crate::bindings::root::ScHlslRootConstant as RootConstant;

#[derive(Debug, Copy, Clone)]
pub struct HlslResourceBindingSpaceRegister {
    pub register_space: u32,
    pub register_binding: u32,
}

#[derive(Debug, Clone)]
pub struct HlslResourceBinding {
    pub stage: spirv::ExecutionModel,
    pub desc_set: u32,
    pub binding: u32,
    pub cbv: HlslResourceBindingSpaceRegister,
    pub uav: HlslResourceBindingSpaceRegister,
    pub srv: HlslResourceBindingSpaceRegister,
    pub sampler: HlslResourceBindingSpaceRegister,
}

#[derive(Debug, Clone)]
pub struct HlslVertexAttributeRemap {
    pub location: u32,
    pub semantic: String
}

/// A HLSL target.
#[derive(Debug, Clone)]
pub enum Target {}

impl spirv::Target for Target {
    type Data = ();
}

/// A HLSL shader model version.
#[allow(non_snake_case, non_camel_case_types)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[non_exhaustive]
pub enum ShaderModel {
    V3_0,
    V4_0,
    V4_0L9_0,
    V4_0L9_1,
    V4_0L9_3,
    V4_1,
    V5_0,
    V5_1,
    V6_0,
}

#[allow(non_snake_case, non_camel_case_types)]
impl ShaderModel {
    fn as_raw(self) -> i32 {
        use self::ShaderModel::*;
        match self {
            V3_0 => 30,
            V4_0 => 40,
            V4_0L9_0 => 40,
            V4_0L9_1 => 40,
            V4_0L9_3 => 40,
            V4_1 => 41,
            V5_0 => 50,
            V5_1 => 51,
            V6_0 => 60,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompilerVertexOptions {
    pub invert_y: bool,
    pub transform_clip_space: bool,
}

impl Default for CompilerVertexOptions {
    fn default() -> CompilerVertexOptions {
        CompilerVertexOptions {
            invert_y: false,
            transform_clip_space: false,
        }
    }
}

/// HLSL compiler options.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct CompilerOptions {
    pub shader_model: ShaderModel,
    /// Support point size builtin but ignore the value.
    pub point_size_compat: bool,
    /// Support point coordinate builtin but ignore the value.
    pub point_coord_compat: bool,
    pub vertex: CompilerVertexOptions,
    pub force_storage_buffer_as_uav: bool,
    pub nonwritable_uav_texture_as_srv: bool,
    /// Whether to force all uninitialized variables to be initialized to zero.
    pub force_zero_initialized_variables: bool,
    /// If matrices are used as IO variables, flatten the attribute declaration to use
    /// TEXCOORD{N,N+1,N+2,...} rather than TEXCOORDN_{0,1,2,3}.
    /// If add_vertex_attribute_remap is used and this feature is used,
    /// the semantic name will be queried once per active location.
    pub flatten_matrix_vertex_input_semantics: bool,
    /// The name and execution model of the entry point to use. If no entry
    /// point is specified, then the first entry point found will be used.
    pub entry_point: Option<(String, spirv::ExecutionModel)>,
}

impl Default for CompilerOptions {
    fn default() -> CompilerOptions {
        CompilerOptions {
            shader_model: ShaderModel::V3_0,
            point_size_compat: false,
            point_coord_compat: false,
            vertex: CompilerVertexOptions::default(),
            force_storage_buffer_as_uav: false,
            nonwritable_uav_texture_as_srv: false,
            force_zero_initialized_variables: false,
            flatten_matrix_vertex_input_semantics: false,
            entry_point: None,
        }
    }
}

impl spirv::Parse<Target> for spirv::Ast<Target> {
    fn parse(module: &spirv::Module) -> Result<Self, ErrorCode> {
        let compiler = {
            let mut compiler = ptr::null_mut();
            unsafe {
                check!(br::sc_internal_compiler_hlsl_new(
                    &mut compiler,
                    module.words.as_ptr() as *const u32,
                    module.words.len() as usize,
                ));
            }

            compiler::Compiler {
                sc_compiler: compiler,
                target_data: (),
                has_been_compiled: false,
            }
        };

        Ok(spirv::Ast {
            compiler,
            target_type: PhantomData,
        })
    }
}

impl spirv::Compile<Target> for spirv::Ast<Target> {
    type CompilerOptions = CompilerOptions;

    /// Set HLSL compiler specific compilation settings.
    fn set_compiler_options(&mut self, options: &CompilerOptions) -> Result<(), ErrorCode> {
        if let Some((name, model)) = &options.entry_point {
            let name_raw = CString::new(name.as_str()).map_err(|_| ErrorCode::Unhandled)?;
            let model = model.as_raw();
            unsafe {
                check!(br::sc_internal_compiler_set_entry_point(
                    self.compiler.sc_compiler,
                    name_raw.as_ptr(),
                    model,
                ));
            }
        };
        let raw_options = br::ScHlslCompilerOptions {
            shader_model: options.shader_model.as_raw(),
            point_size_compat: options.point_size_compat,
            point_coord_compat: options.point_coord_compat,
            vertex_invert_y: options.vertex.invert_y,
            vertex_transform_clip_space: options.vertex.transform_clip_space,
            force_storage_buffer_as_uav: options.force_storage_buffer_as_uav,
            nonwritable_uav_texture_as_srv: options.nonwritable_uav_texture_as_srv,
            flatten_matrix_vertex_input_semantics: options.flatten_matrix_vertex_input_semantics,
            force_zero_initialized_variables: options.force_zero_initialized_variables,
        };
        unsafe {
            check!(br::sc_internal_compiler_hlsl_set_options(
                self.compiler.sc_compiler,
                &raw_options,
            ));
        }

        Ok(())
    }

    /// Generate HLSL shader from the AST.
    fn compile(&mut self) -> Result<String, ErrorCode> {
        self.compiler.compile()
    }
}

impl spirv::Ast<Target> {
    ///
    pub fn set_root_constant_layout(&mut self, layout: Vec<RootConstant>) -> Result<(), ErrorCode> {
        unsafe {
            check!(br::sc_internal_compiler_hlsl_set_root_constant_layout(
                self.compiler.sc_compiler,
                layout.as_ptr(),
                layout.len() as _,
            ));
        }

        Ok(())
    }

    ///
    pub fn add_vertex_attribute_remap(&mut self, remap: &HlslVertexAttributeRemap) -> Result<(), ErrorCode> {
        let semantic = CString::new(remap.semantic.as_str()).map_err(|_| ErrorCode::Unhandled)?;

        let r = crate::bindings::root::ScHlslVertexAttributeRemap {
            location: remap.location,
            semantic: semantic.as_ptr() as *mut _,
        };

        unsafe {
            check!(br::sc_internal_compiler_hlsl_add_vertex_attribute_remap(self.compiler.sc_compiler, r));
        }

        Ok(())
    }

    ///
    pub fn add_resource_binding(&mut self, resource_binding: &HlslResourceBinding) -> Result<(), ErrorCode> {
        fn convert_space_register(space_register: HlslResourceBindingSpaceRegister) -> crate::bindings::root::ScHlslResourceBindingSpaceRegister {
            crate::bindings::root::ScHlslResourceBindingSpaceRegister { register_space: space_register.register_space, register_binding: space_register.register_binding }
        }

        let resource_binding = crate::bindings::root::ScHlslResourceBinding {
            stage: resource_binding.stage.as_raw(),
            desc_set: resource_binding.desc_set,
            binding: resource_binding.binding,
            cbv: convert_space_register(resource_binding.cbv),
            uav: convert_space_register(resource_binding.uav),
            srv: convert_space_register(resource_binding.srv),
            sampler: convert_space_register(resource_binding.sampler),
        };

        unsafe {
            check!(br::sc_internal_compiler_hlsl_add_resource_binding(
                self.compiler.sc_compiler,
                resource_binding
            ));
        }

        Ok(())
    }
}
