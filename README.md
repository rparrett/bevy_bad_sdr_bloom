# bevy_bad_sdr_bloom

[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

This is probably a bad idea, but I took Bevy 0.9's bloom and

- replaced all the 16-bit float textures with Rgba8UNorm
- used a bunch of textures instead of mips

So that it would work with WebGL2.

This isn't quite bloom anymore, but you can use it to make stuff sort of glowy if you want.

## Compatibility

|bevy|bevy_bad_sdr_bloom|
|-|-|
|0.9|0.1|
