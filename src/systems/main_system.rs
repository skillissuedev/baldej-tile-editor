use super::System;
use crate::{
    assets::{model_asset::ModelAsset, shader_asset::ShaderAsset, texture_asset::{TextureAsset, TextureAssetError}}, framework::{self, get_delta_time, get_resolution, set_global_system_value}, managers::{
        input::{self, is_mouse_locked, set_mouse_locked, InputEventType}, networking::Message, physics::{BodyColliderType, BodyType}, render::{get_camera_front, get_camera_position, get_camera_right, get_camera_rotation, set_camera_position, set_camera_rotation, set_light_direction}, systems::{CallList, SystemValue}
    }, objects::{master_instanced_model_onbject::CurrentAnimationSettings, model_object::ModelObject, ray::Ray, Object}
};
use egui_glium::egui_winit::egui::{Color32, ComboBox, Image, Pos2, TextureId, Window};
use glam::Vec3;
use rapier3d::parry::utils::Array1;

#[derive(Debug)]
struct Prop {
    name: String,
    model_path: String,
    texture_path: String
}

pub struct MainSystem {
    pub is_destroyed: bool,
    pub objects: Vec<Box<dyn Object>>,
    tile_path: String,
    texture_path: String,
    new_prop_path: String,
    new_prop_texture: String,
    new_prop_name: String,
    current_prop: String,
    props_list: Vec<Prop>,
    last_added_objects_names: Vec<String>,
    prop_count: usize
}

impl MainSystem {
    pub fn new() -> Self {
        MainSystem {
            is_destroyed: false,
            objects: vec![],
            tile_path: String::new(),
            texture_path: String::new(),
            new_prop_path: String::new(),
            new_prop_texture: String::new(),
            new_prop_name: String::new(),
            current_prop: String::new(),
            props_list: Vec::new(),
            last_added_objects_names: Vec::new(),
            prop_count: 0
        }
    }
}

impl System for MainSystem {
    fn ui_render(&mut self, ctx: &egui_glium::egui_winit::egui::Context) {
        Window::new("editor").show(ctx, |ui| {
            ui.label("use the RMB/Return to place the selected prop, Z to undo");
            ui.separator();
            ui.label("tile path:");
            ui.text_edit_singleline(&mut self.tile_path);
            ui.label("tile texture path:");
            ui.text_edit_singleline(&mut self.texture_path);

            if ui.button("load & create").clicked() {
                self.delete_object("tile");
                let asset = ModelAsset::from_gltf(&self.tile_path);
                match asset {
                    Ok(asset) => {
                        let texture = TextureAsset::from_file(&self.texture_path);
                        let shader_asset = ShaderAsset::load_default_shader().unwrap();
                        let mut tile;
                        match texture {
                            Ok(texture) => {
                                tile = ModelObject::new("tile", asset.clone(), Some(texture), shader_asset);
                            },
                            Err(_) => {
                                tile = ModelObject::new("tile", asset.clone(), None, shader_asset);
                            },
                        }
                        tile.set_position(Vec3::new(0.0, -100.0, 0.0), true);
                        tile.build_object_rigid_body(
                            Some(BodyType::Fixed(Some(BodyColliderType::TriangleMesh(asset)))),
                            None, 1.0, None, None
                        );
                        self.add_object(Box::new(tile));
                    },
                    Err(_) => (),
                }
            }
            //self.props_list
            ui.separator();
            ui.heading("new prop:");
            ui.label("name:");
            ui.text_edit_singleline(&mut self.new_prop_name);
            ui.label("model path:");
            ui.text_edit_singleline(&mut self.new_prop_path);
            ui.label("texture path:");
            ui.text_edit_singleline(&mut self.new_prop_texture);
            if ui.button("add to the props list").clicked() {
                self.props_list.push(Prop {
                    name: self.new_prop_name.clone(),
                    model_path: self.new_prop_path.clone(),
                    texture_path: self.new_prop_texture.clone(),
                });
            }

            if ui.button("remove from the props list").clicked() {
                let mut i: Option<usize> = None;
                for (idx, prop) in self.props_list.iter().enumerate() {
                    if prop.name == self.new_prop_name {
                        i = Some(idx);
                    }
                }

                if let Some(i) = i {
                    self.props_list.remove(i);
                }
            }

            ComboBox::from_label("current prop").selected_text(&self.current_prop).show_ui(ui, |ui| {
                for prop in &self.props_list {
                    if ui.selectable_label(false, &prop.name).clicked() {
                        self.current_prop = prop.name.clone().into();
                    }
                }
            });
        });
        let screen_center = get_resolution() / 2.0;
        ctx.debug_painter().circle(Pos2::new(screen_center.x, screen_center.y), 3.0, Color32::WHITE, (1.0, Color32::WHITE));
    }

    fn client_start(&mut self) {
        let ray = Ray::new("ray", Vec3::new(0.0, 0.0, 900.0), None);
        self.add_object(Box::new(ray));

        let cube_model_asset = ModelAsset::from_gltf("models/cube.gltf").unwrap();
        let cube = ModelObject::new("cube", cube_model_asset, None, ShaderAsset::load_default_shader().unwrap());
        self.add_object(Box::new(cube));

        set_camera_position(Vec3::new(0.0, 0.0, 0.0));
        input::new_bind(
            "forward",
            vec![InputEventType::Key(glium::glutin::event::VirtualKeyCode::W)],
        );
        input::new_bind(
            "left",
            vec![InputEventType::Key(glium::glutin::event::VirtualKeyCode::A)],
        );
        input::new_bind(
            "backwards",
            vec![InputEventType::Key(glium::glutin::event::VirtualKeyCode::S)],
        );
        input::new_bind(
            "right",
            vec![InputEventType::Key(glium::glutin::event::VirtualKeyCode::D)],
        );
        input::new_bind(
            "cam_up",
            vec![InputEventType::Key(glium::glutin::event::VirtualKeyCode::Q)],
        );
        input::new_bind(
            "cam_down",
            vec![InputEventType::Key(glium::glutin::event::VirtualKeyCode::E)],
        );
        input::new_bind(
            "lock_mouse",
            vec![InputEventType::Key(glium::glutin::event::VirtualKeyCode::L)],
        );
        input::new_bind(
            "undo",
            vec![InputEventType::Key(glium::glutin::event::VirtualKeyCode::Z)],
        );
        input::new_bind(
            "place_prop",
            vec![
                InputEventType::Key(glium::glutin::event::VirtualKeyCode::Return),
                InputEventType::Mouse(glium::glutin::event::MouseButton::Right)
            ],
        );
    }

    fn server_start(&mut self) {}
    fn server_render(&mut self) {}

    fn client_update(&mut self) {
        set_light_direction(Vec3::new(-0.2, 0.0, 0.0));

        //locking mouse
        if input::is_bind_pressed("lock_mouse") {
            set_mouse_locked(!is_mouse_locked());
        }

        // movement
        let delta_time = get_delta_time().as_secs_f32();
        let delta = input::mouse_delta();
        let camera_rotation = get_camera_rotation();

        set_camera_rotation(Vec3::new(camera_rotation.x - delta.y * 50.0 * delta_time, camera_rotation.y + delta.x * 50.0 * delta_time, camera_rotation.z));

        let speed = 420.0 * delta_time;

        let camera_front = get_camera_front();
        let camera_right = get_camera_right();
        let camera_position = get_camera_position();

        if input::is_bind_down("cam_up") {
            set_camera_position(Vec3::new(
                camera_position.x,
                camera_position.y + speed,
                camera_position.z,
            ));
        }

        if input::is_bind_down("cam_down") {
            set_camera_position(Vec3::new(
                camera_position.x,
                camera_position.y - speed,
                camera_position.z,
            ));
        }

        if input::is_bind_down("forward") {
            set_camera_position(camera_position + camera_front * speed);
        }

        if input::is_bind_down("backwards") {
            set_camera_position(camera_position - camera_front * speed);
        }

        if input::is_bind_down("left") {
            set_camera_position(camera_position - camera_right * speed);
        }

        if input::is_bind_down("right") {
            set_camera_position(camera_position + camera_right * speed);
        }
        if get_camera_rotation().x > 89.0 {
            let rot = get_camera_rotation();
            set_camera_rotation(Vec3::new(89.0, rot.y, rot.z));
        } else if get_camera_rotation().x < -89.0 {
            let rot = get_camera_rotation();
            set_camera_rotation(Vec3::new(-89.0, rot.y, rot.z));
        }

        // moving the ray to the camera
        let ray: &mut Ray = self.find_object_mut("ray").unwrap().downcast_mut().unwrap();
        ray.set_position(camera_position, true);
        ray.set_direction(camera_front * 900.0);
        if let Some(pos) = ray.intersection_position() {
            self.find_object_mut("cube").unwrap().set_position(pos, true);
            // placing props
            if input::is_bind_pressed("place_prop") {
                let mut current_prop = None;
                for prop in &self.props_list {
                    if prop.name == self.current_prop {
                        current_prop = Some(prop)
                    }
                }
                if let Some(current_prop) = current_prop {
                    let model_asset = ModelAsset::from_gltf(&current_prop.model_path);
                    if let Ok(model_asset) = model_asset {
                        let texture_asset = TextureAsset::from_file(&current_prop.texture_path);
                        self.prop_count += 1;
                        let mut prop;
                        match texture_asset {
                            Ok(texture_asset) =>
                                prop = ModelObject::new(&format!("prop{}", self.prop_count), model_asset, Some(texture_asset), ShaderAsset::load_default_shader().unwrap()),
                            Err(_) => prop = ModelObject::new(&format!("prop{}", self.prop_count), model_asset, None, ShaderAsset::load_default_shader().unwrap()),
                        }
                        self.last_added_objects_names.push(prop.name().into());
                        prop.set_position(pos, true);
                        self.find_object_mut("tile").unwrap().add_child(Box::new(prop));
                    }
                }
            }
        }
        if input::is_bind_pressed("undo") {
            if self.last_added_objects_names.len() > 0 {
                dbg!(&self.last_added_objects_names);
                let idx = self.last_added_objects_names.len() - 1;
                self.delete_object(self.last_added_objects_names[idx].clone().as_str());
                self.last_added_objects_names.remove(idx);
            }
        }
    }

    fn server_update(&mut self) {}

    fn client_render(&mut self) {}

    fn call(&self, _: &str) {}

    fn call_mut(&mut self, _: &str) {}

    fn objects_list(&self) -> &Vec<Box<dyn Object>> {
        &self.objects
    }

    fn objects_list_mut(&mut self) -> &mut Vec<Box<dyn Object>> {
        &mut self.objects
    }

    fn call_list(&self) -> CallList {
        CallList { immut_call: Vec::new(), mut_call: Vec::new() }
    }

    fn system_id(&self) -> &str {
        "MainSystem"
    }

    fn is_destroyed(&self) -> bool {
        self.is_destroyed
    }

    fn set_destroyed(&mut self, is_destroyed: bool) {
        self.is_destroyed = is_destroyed
    }

    fn reg_message(&mut self, _message: Message) {
    }

    fn get_value(&mut self, _value_name: String) -> Option<SystemValue> {
        None
    }
}

