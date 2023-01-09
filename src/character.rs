fn draw_the_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(SpriteBundle {
        assets: materials.add(asset_server.load("Doctor_Covid.png").into()),
        ..SpriteBundle::default()
    });
}