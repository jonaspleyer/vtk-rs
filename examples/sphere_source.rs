use vtk_rs as vtk;
use vtk_rs::prelude::*;

fn main() {
    let _colors = vtk::NamedColors::new();

    // Create a sphere
    let mut sphere_source = vtk::SphereSource::new();
    sphere_source.set_center(0., 0., 0.);
    sphere_source.set_radius(5.0);

    // Make the surface smooth
    sphere_source.set_phi_resolution(100);
    sphere_source.set_theta_resolution(100);

    println!("sphere source: {sphere_source:?}")

    /* let mut mapper = vtk::PolyDataMapper::new();
    mapper.set_input_connection(sphere_source.get_output_port());
    let mut mapper = vtkPolyDataMapper::New();
    mapper.SetInputConnection(sphere_source.GetOutputPort());

    let mut actor = vtkActor::New();
    actor.SetMapper(&mapper);
    actor.GetProperty().SetColor(colors.GetColor3d("Cornsilk"));

    let mut renderer = vtkRenderer::New();
    let mut render_window = vtkRenderWindow::New();
    render_window.SetWindowName("Sphere");
    render_window.AddRenderer(&renderer);
    let render_window_interactor = vtkRenderWindowInteractor::New();
    render_window_interactor.SetRenderWindow(&render_window);

    renderer.AddActor(&actor);
    renderer.SetBackground(colors.GetColor3d("DarkGreen"));

    render_window.Render();
    render_window_interactor.Start();*/
}
