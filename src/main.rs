use vtk_rs::*;

fn main() {
    let colors = vtkNamedColors::New();

    // Create a sphere
    let mut sphere_source = vtkSphereSource::New();
    sphere_source.SetCenter([0.; 3]);
    sphere_source.SetRadius(5.0);

    // Make the surface smooth
    /* sphere_source.SetPhiResolution(100.);
    sphere_source.SetThetaResolution(100.);

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
