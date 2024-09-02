## ⚠️ Deprecated ⚠️
Use https://github.com/SnowflakePowered/spirv-cross2 instead.

---

<details>
<summary>Old README</summary>
  
# librashader-spirv-cross

(Hopefully) temporary fork of [spirv_cross](https://github.com/grovesNL/spirv_cross) while some changes get upstreamed.

**Please do not use this.**
---

<h1 align="center">
  spirv_cross
</h1>
<div align="center">
  Safe wrapper around <a href="https://github.com/KhronosGroup/SPIRV-Cross">SPIR-V Cross</a>
</div>
<br />
<div align="center">
  <a href="https://crates.io/crates/spirv_cross"><img src="https://img.shields.io/crates/v/spirv_cross.svg?label=spirv_cross" alt="Crate"></a> <a href="https://travis-ci.org/grovesNL/spirv_cross"><img src="https://travis-ci.org/grovesNL/spirv_cross.svg?branch=master" alt="Travis Build Status" /></a> <a href="https://ci.appveyor.com/project/grovesNL/spirv-cross/branch/master"><img src="https://ci.appveyor.com/api/projects/status/ja22j0ueje51sd76/branch/master?svg=true" alt="Appveyor Build Status" /></a>
</div>

## Example

`spirv_cross` provides a safe wrapper around [SPIRV-Cross](https://github.com/KhronosGroup/SPIRV-Cross) for use with Rust. For example, here is a simple function to parse a SPIR-V module and compile it to HLSL and MSL:

```rust
extern crate spirv_cross;
use spirv_cross::{spirv, hlsl, msl, ErrorCode};

fn example(module: spirv::Module) -> Result<(), ErrorCode> {
    // Compile to HLSL
    let ast = spirv::Ast::<hlsl::Target>::parse(&module)?;
    println!("{}", ast.compile()?);

    // Compile to MSL
    let ast = spirv::Ast::<msl::Target>::parse(&module)?;
    println!("{}", ast.compile()?);

    Ok(())
}
```

## License

This project is licensed under either of [Apache License, Version
2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache 2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).

</details>
