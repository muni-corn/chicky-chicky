use textures;

const base_dir: &'static str = "./assets/photos/blocks/"

// TODO These should be specific to their block modules
let (
    dirt_texture = textures::new_lazy_texture("dirt.png")
    grass_texture = textures::new_lazy_texture("grass.png")
    stone_texture = textures::new_lazy_texture("stone.png")
    sand_texture = textures::new_lazy_texture("sand.png")
)
