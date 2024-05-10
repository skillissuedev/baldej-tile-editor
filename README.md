# Baldej Tile Editor
So i wrote this small program for designing tiles in my game. Yes, that's it
# How to use
WASD to move, RMB/Return to place the selected prop, Z to undo.
# Current To Do:
- [x] camera movement
- [x] loading tiles, props
- [x] placing props
- [x] randomly place instanced details like grass/small stones
- [ ] generating code to use inside the game
# Output(pseudocode):
```
const PROP_NAME_TRANSFORMS = vec![
    Transform { position: .., rotation: .., scale: .. },
    Transform { position: .., rotation: .., scale: .. },
    ...
];

fn spawn_tile_server(&mut self, position: Vec3) {
    let tile_model_asset = ModelAsset::new("path_here");
    let tile = EmptyObject::new(/*using our assets here*/);
    tile.set_position(position);
    tile.build_object_body(/*build the trimesh static thing here*/);


    // USING TILE AS A PARENT!
    for prop_transform in PROP_NAME_TRANSFORMS {
        new_prop_name_server(tile, prop_transform);
    }
    // and do the similar thing for all of the props
    self.add_object(tile);
}

fn spawn_tile_client(&mut self, position: Vec3) {
    let tile_model_asset = ModelAsset::new("path_here");
    let tile_texture_asset = TextureAsset::new("path_here");
    let tile = Box::new(ModelObject::new(/*using our assets here*/));
    tile.set_position(position);
    // maybe build a body here?


    let prop_name_model = ModelAsset::new("path");
    let prop_name_texture = ModelAsset::new("path");
    // USING TILE AS A PARENT!
    for prop_transform in PROP_NAME_TRANSFORMS {
        new_prop_name_client(tile, prop_transform,
            prop_name_model.clone, prop_name_texture
        );
    }
    // and do the similar thing for all of the props
}

fn new_prop_name_server(tile: &mut Box<Object>, transform: Transform) {
    // so spawn the object here, i guess?
    tile.add_child(Box::new(prop));
}

fn new_prop_name_client(tile: &mut Box<Object>, transform: Transform, model: ModelAsset, texture: TextureAsset) {
    let prop = ModelObject::new(model, texture);
    prop.set_transform(transform);
    // so spawn the object here, i guess?
    tile.add_child(Box::new(prop));
}
```
