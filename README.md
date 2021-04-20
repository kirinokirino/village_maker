# village_maker
Game where you build your village from tiles

Models in the assets folder are licensed under CC0 and provided by kenney.nl

Don't forget to use dynamic feature to improve iteration times:
`cargo run --features bevy/dynamic`

Clippy sometimes weirds out, use this magic line to fix it:
`find -name "*.rs" -not -path "./target/*" -exec touch "{}" +`

Since wgpu now has opengl support, I can use bevy on my old laptop! 
