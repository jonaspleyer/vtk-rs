// Include header file
#include <vtk_common_computational_geometry.h>

// Default include in all modules
#include <vtkNew.h>
#include <vtkObjectBase.h>

// Include objects of this module
#include <vtkBilinearQuadIntersection.h>
#include <vtkCardinalSpline.h>
#include <vtkKochanekSpline.h>
#include <vtkParametricBohemianDome.h>
#include <vtkParametricBour.h>
#include <vtkParametricBoy.h>
#include <vtkParametricCatalanMinimal.h>
#include <vtkParametricConicSpiral.h>
#include <vtkParametricCrossCap.h>
#include <vtkParametricDini.h>
#include <vtkParametricEllipsoid.h>
#include <vtkParametricEnneper.h>
#include <vtkParametricFigure8Klein.h>
#include <vtkParametricFunction.h>
#include <vtkParametricHenneberg.h>
#include <vtkParametricKlein.h>
#include <vtkParametricKuen.h>
#include <vtkParametricMobius.h>
#include <vtkParametricPluckerConoid.h>
#include <vtkParametricPseudosphere.h>
#include <vtkParametricRandomHills.h>
#include <vtkParametricRoman.h>
#include <vtkParametricSpline.h>
#include <vtkParametricSuperEllipsoid.h>
#include <vtkParametricSuperToroid.h>
#include <vtkParametricTorus.h>

// Implement declared functions
extern "C" vtkNew<vtkCardinalSpline> vtkCardinalSpline_new() {
    return vtkNew<vtkCardinalSpline>();
}
extern "C" void vtkCardinalSpline_destructor(vtkNew<vtkCardinalSpline> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCardinalSpline_get_ptr(vtkNew<vtkCardinalSpline> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_cardinal_spline_compute(vtkNew<vtkCardinalSpline> self) {
    return self->Compute();
}
extern "C" double vtk_cardinal_spline_evaluate(vtkNew<vtkCardinalSpline> self, double t) {
    return self->Evaluate(t);
}
extern "C" vtkNew<vtkKochanekSpline> vtkKochanekSpline_new() {
    return vtkNew<vtkKochanekSpline>();
}
extern "C" void vtkKochanekSpline_destructor(vtkNew<vtkKochanekSpline> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkKochanekSpline_get_ptr(vtkNew<vtkKochanekSpline> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_kochanek_spline_compute(vtkNew<vtkKochanekSpline> self) {
    return self->Compute();
}
extern "C" double vtk_kochanek_spline_evaluate(vtkNew<vtkKochanekSpline> self, double t) {
    return self->Evaluate(t);
}
extern "C" void vtk_kochanek_spline_set_default_bias(vtkNew<vtkKochanekSpline> self, double _arg) {
    return self->SetDefaultBias(_arg);
}
extern "C" double vtk_kochanek_spline_get_default_bias(vtkNew<vtkKochanekSpline> self) {
    return self->GetDefaultBias();
}
extern "C" void vtk_kochanek_spline_set_default_tension(
    vtkNew<vtkKochanekSpline> self, double _arg
) {
    return self->SetDefaultTension(_arg);
}
extern "C" double vtk_kochanek_spline_get_default_tension(vtkNew<vtkKochanekSpline> self) {
    return self->GetDefaultTension();
}
extern "C" void vtk_kochanek_spline_set_default_continuity(
    vtkNew<vtkKochanekSpline> self, double _arg
) {
    return self->SetDefaultContinuity(_arg);
}
extern "C" double vtk_kochanek_spline_get_default_continuity(vtkNew<vtkKochanekSpline> self) {
    return self->GetDefaultContinuity();
}
extern "C" vtkNew<vtkParametricBohemianDome> vtkParametricBohemianDome_new() {
    return vtkNew<vtkParametricBohemianDome>();
}
extern "C" void vtkParametricBohemianDome_destructor(vtkNew<vtkParametricBohemianDome> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricBohemianDome_get_ptr(vtkNew<vtkParametricBohemianDome> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_parametric_bohemian_dome_get_a(vtkNew<vtkParametricBohemianDome> self) {
    return self->GetA();
}
extern "C" void vtk_parametric_bohemian_dome_set_a(
    vtkNew<vtkParametricBohemianDome> self, double _arg
) {
    return self->SetA(_arg);
}
extern "C" double vtk_parametric_bohemian_dome_get_b(vtkNew<vtkParametricBohemianDome> self) {
    return self->GetB();
}
extern "C" void vtk_parametric_bohemian_dome_set_b(
    vtkNew<vtkParametricBohemianDome> self, double _arg
) {
    return self->SetB(_arg);
}
extern "C" double vtk_parametric_bohemian_dome_get_c(vtkNew<vtkParametricBohemianDome> self) {
    return self->GetC();
}
extern "C" void vtk_parametric_bohemian_dome_set_c(
    vtkNew<vtkParametricBohemianDome> self, double _arg
) {
    return self->SetC(_arg);
}
extern "C" int vtk_parametric_bohemian_dome_get_dimension(vtkNew<vtkParametricBohemianDome> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_bohemian_dome_evaluate(
    vtkNew<vtkParametricBohemianDome> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_bohemian_dome_evaluate_scalar(
    vtkNew<vtkParametricBohemianDome> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricBour> vtkParametricBour_new() {
    return vtkNew<vtkParametricBour>();
}
extern "C" void vtkParametricBour_destructor(vtkNew<vtkParametricBour> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricBour_get_ptr(vtkNew<vtkParametricBour> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_bour_get_dimension(vtkNew<vtkParametricBour> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_bour_evaluate(
    vtkNew<vtkParametricBour> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_bour_evaluate_scalar(
    vtkNew<vtkParametricBour> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricBoy> vtkParametricBoy_new() {
    return vtkNew<vtkParametricBoy>();
}
extern "C" void vtkParametricBoy_destructor(vtkNew<vtkParametricBoy> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricBoy_get_ptr(vtkNew<vtkParametricBoy> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_boy_get_dimension(vtkNew<vtkParametricBoy> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_boy_set_z_scale(vtkNew<vtkParametricBoy> self, double _arg) {
    return self->SetZScale(_arg);
}
extern "C" double vtk_parametric_boy_get_z_scale(vtkNew<vtkParametricBoy> self) {
    return self->GetZScale();
}
extern "C" void vtk_parametric_boy_evaluate(
    vtkNew<vtkParametricBoy> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_boy_evaluate_scalar(
    vtkNew<vtkParametricBoy> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricCatalanMinimal> vtkParametricCatalanMinimal_new() {
    return vtkNew<vtkParametricCatalanMinimal>();
}
extern "C" void vtkParametricCatalanMinimal_destructor(vtkNew<vtkParametricCatalanMinimal> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricCatalanMinimal_get_ptr(vtkNew<vtkParametricCatalanMinimal> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_catalan_minimal_get_dimension(
    vtkNew<vtkParametricCatalanMinimal> self
) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_catalan_minimal_evaluate(
    vtkNew<vtkParametricCatalanMinimal> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_catalan_minimal_evaluate_scalar(
    vtkNew<vtkParametricCatalanMinimal> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricConicSpiral> vtkParametricConicSpiral_new() {
    return vtkNew<vtkParametricConicSpiral>();
}
extern "C" void vtkParametricConicSpiral_destructor(vtkNew<vtkParametricConicSpiral> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricConicSpiral_get_ptr(vtkNew<vtkParametricConicSpiral> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_conic_spiral_get_dimension(vtkNew<vtkParametricConicSpiral> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_conic_spiral_set_a(
    vtkNew<vtkParametricConicSpiral> self, double _arg
) {
    return self->SetA(_arg);
}
extern "C" double vtk_parametric_conic_spiral_get_a(vtkNew<vtkParametricConicSpiral> self) {
    return self->GetA();
}
extern "C" void vtk_parametric_conic_spiral_set_b(
    vtkNew<vtkParametricConicSpiral> self, double _arg
) {
    return self->SetB(_arg);
}
extern "C" double vtk_parametric_conic_spiral_get_b(vtkNew<vtkParametricConicSpiral> self) {
    return self->GetB();
}
extern "C" void vtk_parametric_conic_spiral_set_c(
    vtkNew<vtkParametricConicSpiral> self, double _arg
) {
    return self->SetC(_arg);
}
extern "C" double vtk_parametric_conic_spiral_get_c(vtkNew<vtkParametricConicSpiral> self) {
    return self->GetC();
}
extern "C" void vtk_parametric_conic_spiral_set_n(
    vtkNew<vtkParametricConicSpiral> self, double _arg
) {
    return self->SetN(_arg);
}
extern "C" double vtk_parametric_conic_spiral_get_n(vtkNew<vtkParametricConicSpiral> self) {
    return self->GetN();
}
extern "C" void vtk_parametric_conic_spiral_evaluate(
    vtkNew<vtkParametricConicSpiral> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_conic_spiral_evaluate_scalar(
    vtkNew<vtkParametricConicSpiral> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricCrossCap> vtkParametricCrossCap_new() {
    return vtkNew<vtkParametricCrossCap>();
}
extern "C" void vtkParametricCrossCap_destructor(vtkNew<vtkParametricCrossCap> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricCrossCap_get_ptr(vtkNew<vtkParametricCrossCap> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_cross_cap_get_dimension(vtkNew<vtkParametricCrossCap> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_cross_cap_evaluate(
    vtkNew<vtkParametricCrossCap> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_cross_cap_evaluate_scalar(
    vtkNew<vtkParametricCrossCap> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricDini> vtkParametricDini_new() {
    return vtkNew<vtkParametricDini>();
}
extern "C" void vtkParametricDini_destructor(vtkNew<vtkParametricDini> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricDini_get_ptr(vtkNew<vtkParametricDini> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_dini_get_dimension(vtkNew<vtkParametricDini> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_dini_set_a(vtkNew<vtkParametricDini> self, double _arg) {
    return self->SetA(_arg);
}
extern "C" double vtk_parametric_dini_get_a(vtkNew<vtkParametricDini> self) {
    return self->GetA();
}
extern "C" void vtk_parametric_dini_set_b(vtkNew<vtkParametricDini> self, double _arg) {
    return self->SetB(_arg);
}
extern "C" double vtk_parametric_dini_get_b(vtkNew<vtkParametricDini> self) {
    return self->GetB();
}
extern "C" void vtk_parametric_dini_evaluate(
    vtkNew<vtkParametricDini> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_dini_evaluate_scalar(
    vtkNew<vtkParametricDini> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricEllipsoid> vtkParametricEllipsoid_new() {
    return vtkNew<vtkParametricEllipsoid>();
}
extern "C" void vtkParametricEllipsoid_destructor(vtkNew<vtkParametricEllipsoid> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricEllipsoid_get_ptr(vtkNew<vtkParametricEllipsoid> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_ellipsoid_get_dimension(vtkNew<vtkParametricEllipsoid> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_ellipsoid_set_x_radius(
    vtkNew<vtkParametricEllipsoid> self, double _arg
) {
    return self->SetXRadius(_arg);
}
extern "C" double vtk_parametric_ellipsoid_get_x_radius(vtkNew<vtkParametricEllipsoid> self) {
    return self->GetXRadius();
}
extern "C" void vtk_parametric_ellipsoid_set_y_radius(
    vtkNew<vtkParametricEllipsoid> self, double _arg
) {
    return self->SetYRadius(_arg);
}
extern "C" double vtk_parametric_ellipsoid_get_y_radius(vtkNew<vtkParametricEllipsoid> self) {
    return self->GetYRadius();
}
extern "C" void vtk_parametric_ellipsoid_set_z_radius(
    vtkNew<vtkParametricEllipsoid> self, double _arg
) {
    return self->SetZRadius(_arg);
}
extern "C" double vtk_parametric_ellipsoid_get_z_radius(vtkNew<vtkParametricEllipsoid> self) {
    return self->GetZRadius();
}
extern "C" void vtk_parametric_ellipsoid_evaluate(
    vtkNew<vtkParametricEllipsoid> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_ellipsoid_evaluate_scalar(
    vtkNew<vtkParametricEllipsoid> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricEnneper> vtkParametricEnneper_new() {
    return vtkNew<vtkParametricEnneper>();
}
extern "C" void vtkParametricEnneper_destructor(vtkNew<vtkParametricEnneper> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricEnneper_get_ptr(vtkNew<vtkParametricEnneper> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_enneper_get_dimension(vtkNew<vtkParametricEnneper> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_enneper_evaluate(
    vtkNew<vtkParametricEnneper> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_enneper_evaluate_scalar(
    vtkNew<vtkParametricEnneper> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricFigure8Klein> vtkParametricFigure8Klein_new() {
    return vtkNew<vtkParametricFigure8Klein>();
}
extern "C" void vtkParametricFigure8Klein_destructor(vtkNew<vtkParametricFigure8Klein> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricFigure8Klein_get_ptr(vtkNew<vtkParametricFigure8Klein> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_parametric_figure_8_klein_set_radius(
    vtkNew<vtkParametricFigure8Klein> self, double _arg
) {
    return self->SetRadius(_arg);
}
extern "C" double vtk_parametric_figure_8_klein_get_radius(vtkNew<vtkParametricFigure8Klein> self) {
    return self->GetRadius();
}
extern "C" int vtk_parametric_figure_8_klein_get_dimension(vtkNew<vtkParametricFigure8Klein> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_figure_8_klein_evaluate(
    vtkNew<vtkParametricFigure8Klein> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_figure_8_klein_evaluate_scalar(
    vtkNew<vtkParametricFigure8Klein> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricHenneberg> vtkParametricHenneberg_new() {
    return vtkNew<vtkParametricHenneberg>();
}
extern "C" void vtkParametricHenneberg_destructor(vtkNew<vtkParametricHenneberg> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricHenneberg_get_ptr(vtkNew<vtkParametricHenneberg> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_henneberg_get_dimension(vtkNew<vtkParametricHenneberg> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_henneberg_evaluate(
    vtkNew<vtkParametricHenneberg> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_henneberg_evaluate_scalar(
    vtkNew<vtkParametricHenneberg> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricKlein> vtkParametricKlein_new() {
    return vtkNew<vtkParametricKlein>();
}
extern "C" void vtkParametricKlein_destructor(vtkNew<vtkParametricKlein> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricKlein_get_ptr(vtkNew<vtkParametricKlein> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_klein_get_dimension(vtkNew<vtkParametricKlein> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_klein_evaluate(
    vtkNew<vtkParametricKlein> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_klein_evaluate_scalar(
    vtkNew<vtkParametricKlein> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricKuen> vtkParametricKuen_new() {
    return vtkNew<vtkParametricKuen>();
}
extern "C" void vtkParametricKuen_destructor(vtkNew<vtkParametricKuen> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricKuen_get_ptr(vtkNew<vtkParametricKuen> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_kuen_get_dimension(vtkNew<vtkParametricKuen> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_kuen_set_delta_v_0(vtkNew<vtkParametricKuen> self, double _arg) {
    return self->SetDeltaV0(_arg);
}
extern "C" double vtk_parametric_kuen_get_delta_v_0(vtkNew<vtkParametricKuen> self) {
    return self->GetDeltaV0();
}
extern "C" void vtk_parametric_kuen_evaluate(
    vtkNew<vtkParametricKuen> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_kuen_evaluate_scalar(
    vtkNew<vtkParametricKuen> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricMobius> vtkParametricMobius_new() {
    return vtkNew<vtkParametricMobius>();
}
extern "C" void vtkParametricMobius_destructor(vtkNew<vtkParametricMobius> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricMobius_get_ptr(vtkNew<vtkParametricMobius> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_parametric_mobius_set_radius(vtkNew<vtkParametricMobius> self, double _arg) {
    return self->SetRadius(_arg);
}
extern "C" double vtk_parametric_mobius_get_radius(vtkNew<vtkParametricMobius> self) {
    return self->GetRadius();
}
extern "C" int vtk_parametric_mobius_get_dimension(vtkNew<vtkParametricMobius> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_mobius_evaluate(
    vtkNew<vtkParametricMobius> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_mobius_evaluate_scalar(
    vtkNew<vtkParametricMobius> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricPluckerConoid> vtkParametricPluckerConoid_new() {
    return vtkNew<vtkParametricPluckerConoid>();
}
extern "C" void vtkParametricPluckerConoid_destructor(vtkNew<vtkParametricPluckerConoid> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricPluckerConoid_get_ptr(vtkNew<vtkParametricPluckerConoid> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_plucker_conoid_get_n(vtkNew<vtkParametricPluckerConoid> self) {
    return self->GetN();
}
extern "C" void vtk_parametric_plucker_conoid_set_n(
    vtkNew<vtkParametricPluckerConoid> self, int _arg
) {
    return self->SetN(_arg);
}
extern "C" int vtk_parametric_plucker_conoid_get_dimension(
    vtkNew<vtkParametricPluckerConoid> self
) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_plucker_conoid_evaluate(
    vtkNew<vtkParametricPluckerConoid> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_plucker_conoid_evaluate_scalar(
    vtkNew<vtkParametricPluckerConoid> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricPseudosphere> vtkParametricPseudosphere_new() {
    return vtkNew<vtkParametricPseudosphere>();
}
extern "C" void vtkParametricPseudosphere_destructor(vtkNew<vtkParametricPseudosphere> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricPseudosphere_get_ptr(vtkNew<vtkParametricPseudosphere> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_pseudosphere_get_dimension(vtkNew<vtkParametricPseudosphere> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_pseudosphere_evaluate(
    vtkNew<vtkParametricPseudosphere> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_pseudosphere_evaluate_scalar(
    vtkNew<vtkParametricPseudosphere> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricRandomHills> vtkParametricRandomHills_new() {
    return vtkNew<vtkParametricRandomHills>();
}
extern "C" void vtkParametricRandomHills_destructor(vtkNew<vtkParametricRandomHills> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricRandomHills_get_ptr(vtkNew<vtkParametricRandomHills> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_random_hills_get_dimension(vtkNew<vtkParametricRandomHills> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_random_hills_set_number_of_hills(
    vtkNew<vtkParametricRandomHills> self, int _arg
) {
    return self->SetNumberOfHills(_arg);
}
extern "C" int vtk_parametric_random_hills_get_number_of_hills(
    vtkNew<vtkParametricRandomHills> self
) {
    return self->GetNumberOfHills();
}
extern "C" void vtk_parametric_random_hills_set_hill_x_variance(
    vtkNew<vtkParametricRandomHills> self, double _arg
) {
    return self->SetHillXVariance(_arg);
}
extern "C" double vtk_parametric_random_hills_get_hill_x_variance(
    vtkNew<vtkParametricRandomHills> self
) {
    return self->GetHillXVariance();
}
extern "C" void vtk_parametric_random_hills_set_hill_y_variance(
    vtkNew<vtkParametricRandomHills> self, double _arg
) {
    return self->SetHillYVariance(_arg);
}
extern "C" double vtk_parametric_random_hills_get_hill_y_variance(
    vtkNew<vtkParametricRandomHills> self
) {
    return self->GetHillYVariance();
}
extern "C" void vtk_parametric_random_hills_set_hill_amplitude(
    vtkNew<vtkParametricRandomHills> self, double _arg
) {
    return self->SetHillAmplitude(_arg);
}
extern "C" double vtk_parametric_random_hills_get_hill_amplitude(
    vtkNew<vtkParametricRandomHills> self
) {
    return self->GetHillAmplitude();
}
extern "C" void vtk_parametric_random_hills_set_random_seed(
    vtkNew<vtkParametricRandomHills> self, int _arg
) {
    return self->SetRandomSeed(_arg);
}
extern "C" int vtk_parametric_random_hills_get_random_seed(vtkNew<vtkParametricRandomHills> self) {
    return self->GetRandomSeed();
}
extern "C" void vtk_parametric_random_hills_set_allow_random_generation(
    vtkNew<vtkParametricRandomHills> self, int _arg
) {
    return self->SetAllowRandomGeneration(_arg);
}
extern "C" int vtk_parametric_random_hills_get_allow_random_generation_min_value(
    vtkNew<vtkParametricRandomHills> self
) {
    return self->GetAllowRandomGenerationMinValue();
}
extern "C" int vtk_parametric_random_hills_get_allow_random_generation_max_value(
    vtkNew<vtkParametricRandomHills> self
) {
    return self->GetAllowRandomGenerationMaxValue();
}
extern "C" int vtk_parametric_random_hills_get_allow_random_generation(
    vtkNew<vtkParametricRandomHills> self
) {
    return self->GetAllowRandomGeneration();
}
extern "C" void vtk_parametric_random_hills_allow_random_generation_on(
    vtkNew<vtkParametricRandomHills> self
) {
    return self->AllowRandomGenerationOn();
}
extern "C" void vtk_parametric_random_hills_allow_random_generation_off(
    vtkNew<vtkParametricRandomHills> self
) {
    return self->AllowRandomGenerationOff();
}
extern "C" void vtk_parametric_random_hills_set_x_variance_scale_factor(
    vtkNew<vtkParametricRandomHills> self, double _arg
) {
    return self->SetXVarianceScaleFactor(_arg);
}
extern "C" double vtk_parametric_random_hills_get_x_variance_scale_factor(
    vtkNew<vtkParametricRandomHills> self
) {
    return self->GetXVarianceScaleFactor();
}
extern "C" void vtk_parametric_random_hills_set_y_variance_scale_factor(
    vtkNew<vtkParametricRandomHills> self, double _arg
) {
    return self->SetYVarianceScaleFactor(_arg);
}
extern "C" double vtk_parametric_random_hills_get_y_variance_scale_factor(
    vtkNew<vtkParametricRandomHills> self
) {
    return self->GetYVarianceScaleFactor();
}
extern "C" void vtk_parametric_random_hills_set_amplitude_scale_factor(
    vtkNew<vtkParametricRandomHills> self, double _arg
) {
    return self->SetAmplitudeScaleFactor(_arg);
}
extern "C" double vtk_parametric_random_hills_get_amplitude_scale_factor(
    vtkNew<vtkParametricRandomHills> self
) {
    return self->GetAmplitudeScaleFactor();
}
extern "C" void vtk_parametric_random_hills_evaluate(
    vtkNew<vtkParametricRandomHills> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_random_hills_evaluate_scalar(
    vtkNew<vtkParametricRandomHills> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricRoman> vtkParametricRoman_new() {
    return vtkNew<vtkParametricRoman>();
}
extern "C" void vtkParametricRoman_destructor(vtkNew<vtkParametricRoman> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricRoman_get_ptr(vtkNew<vtkParametricRoman> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_roman_get_dimension(vtkNew<vtkParametricRoman> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_roman_set_radius(vtkNew<vtkParametricRoman> self, double _arg) {
    return self->SetRadius(_arg);
}
extern "C" double vtk_parametric_roman_get_radius(vtkNew<vtkParametricRoman> self) {
    return self->GetRadius();
}
extern "C" void vtk_parametric_roman_evaluate(
    vtkNew<vtkParametricRoman> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_roman_evaluate_scalar(
    vtkNew<vtkParametricRoman> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricSpline> vtkParametricSpline_new() {
    return vtkNew<vtkParametricSpline>();
}
extern "C" void vtkParametricSpline_destructor(vtkNew<vtkParametricSpline> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricSpline_get_ptr(vtkNew<vtkParametricSpline> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_spline_get_dimension(vtkNew<vtkParametricSpline> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_spline_evaluate(
    vtkNew<vtkParametricSpline> self, double* u, double* Pt, double* Du
) {
    return self->Evaluate(u, Pt, Du);
}
extern "C" double vtk_parametric_spline_evaluate_scalar(
    vtkNew<vtkParametricSpline> self, double* u, double* Pt, double* Du
) {
    return self->EvaluateScalar(u, Pt, Du);
}
extern "C" void vtk_parametric_spline_set_number_of_points(
    vtkNew<vtkParametricSpline> self, long long numPts
) {
    return self->SetNumberOfPoints(numPts);
}
extern "C" void vtk_parametric_spline_set_point(
    vtkNew<vtkParametricSpline> self, long long index, double x, double y, double z
) {
    return self->SetPoint(index, x, y, z);
}
extern "C" void vtk_parametric_spline_set_closed(vtkNew<vtkParametricSpline> self, int _arg) {
    return self->SetClosed(_arg);
}
extern "C" int vtk_parametric_spline_get_closed(vtkNew<vtkParametricSpline> self) {
    return self->GetClosed();
}
extern "C" void vtk_parametric_spline_closed_on(vtkNew<vtkParametricSpline> self) {
    return self->ClosedOn();
}
extern "C" void vtk_parametric_spline_closed_off(vtkNew<vtkParametricSpline> self) {
    return self->ClosedOff();
}
extern "C" void vtk_parametric_spline_set_parameterize_by_length(
    vtkNew<vtkParametricSpline> self, int _arg
) {
    return self->SetParameterizeByLength(_arg);
}
extern "C" int vtk_parametric_spline_get_parameterize_by_length(vtkNew<vtkParametricSpline> self) {
    return self->GetParameterizeByLength();
}
extern "C" void vtk_parametric_spline_parameterize_by_length_on(vtkNew<vtkParametricSpline> self) {
    return self->ParameterizeByLengthOn();
}
extern "C" void vtk_parametric_spline_parameterize_by_length_off(vtkNew<vtkParametricSpline> self) {
    return self->ParameterizeByLengthOff();
}
extern "C" void vtk_parametric_spline_set_left_constraint(
    vtkNew<vtkParametricSpline> self, int _arg
) {
    return self->SetLeftConstraint(_arg);
}
extern "C" int vtk_parametric_spline_get_left_constraint_min_value(
    vtkNew<vtkParametricSpline> self
) {
    return self->GetLeftConstraintMinValue();
}
extern "C" int vtk_parametric_spline_get_left_constraint_max_value(
    vtkNew<vtkParametricSpline> self
) {
    return self->GetLeftConstraintMaxValue();
}
extern "C" int vtk_parametric_spline_get_left_constraint(vtkNew<vtkParametricSpline> self) {
    return self->GetLeftConstraint();
}
extern "C" void vtk_parametric_spline_set_right_constraint(
    vtkNew<vtkParametricSpline> self, int _arg
) {
    return self->SetRightConstraint(_arg);
}
extern "C" int vtk_parametric_spline_get_right_constraint_min_value(
    vtkNew<vtkParametricSpline> self
) {
    return self->GetRightConstraintMinValue();
}
extern "C" int vtk_parametric_spline_get_right_constraint_max_value(
    vtkNew<vtkParametricSpline> self
) {
    return self->GetRightConstraintMaxValue();
}
extern "C" int vtk_parametric_spline_get_right_constraint(vtkNew<vtkParametricSpline> self) {
    return self->GetRightConstraint();
}
extern "C" void vtk_parametric_spline_set_left_value(
    vtkNew<vtkParametricSpline> self, double _arg
) {
    return self->SetLeftValue(_arg);
}
extern "C" double vtk_parametric_spline_get_left_value(vtkNew<vtkParametricSpline> self) {
    return self->GetLeftValue();
}
extern "C" void vtk_parametric_spline_set_right_value(
    vtkNew<vtkParametricSpline> self, double _arg
) {
    return self->SetRightValue(_arg);
}
extern "C" double vtk_parametric_spline_get_right_value(vtkNew<vtkParametricSpline> self) {
    return self->GetRightValue();
}
extern "C" vtkNew<vtkParametricSuperEllipsoid> vtkParametricSuperEllipsoid_new() {
    return vtkNew<vtkParametricSuperEllipsoid>();
}
extern "C" void vtkParametricSuperEllipsoid_destructor(vtkNew<vtkParametricSuperEllipsoid> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricSuperEllipsoid_get_ptr(vtkNew<vtkParametricSuperEllipsoid> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_super_ellipsoid_get_dimension(
    vtkNew<vtkParametricSuperEllipsoid> self
) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_super_ellipsoid_set_x_radius(
    vtkNew<vtkParametricSuperEllipsoid> self, double _arg
) {
    return self->SetXRadius(_arg);
}
extern "C" double vtk_parametric_super_ellipsoid_get_x_radius(
    vtkNew<vtkParametricSuperEllipsoid> self
) {
    return self->GetXRadius();
}
extern "C" void vtk_parametric_super_ellipsoid_set_y_radius(
    vtkNew<vtkParametricSuperEllipsoid> self, double _arg
) {
    return self->SetYRadius(_arg);
}
extern "C" double vtk_parametric_super_ellipsoid_get_y_radius(
    vtkNew<vtkParametricSuperEllipsoid> self
) {
    return self->GetYRadius();
}
extern "C" void vtk_parametric_super_ellipsoid_set_z_radius(
    vtkNew<vtkParametricSuperEllipsoid> self, double _arg
) {
    return self->SetZRadius(_arg);
}
extern "C" double vtk_parametric_super_ellipsoid_get_z_radius(
    vtkNew<vtkParametricSuperEllipsoid> self
) {
    return self->GetZRadius();
}
extern "C" void vtk_parametric_super_ellipsoid_set_n_1(
    vtkNew<vtkParametricSuperEllipsoid> self, double _arg
) {
    return self->SetN1(_arg);
}
extern "C" double vtk_parametric_super_ellipsoid_get_n_1(vtkNew<vtkParametricSuperEllipsoid> self) {
    return self->GetN1();
}
extern "C" void vtk_parametric_super_ellipsoid_set_n_2(
    vtkNew<vtkParametricSuperEllipsoid> self, double _arg
) {
    return self->SetN2(_arg);
}
extern "C" double vtk_parametric_super_ellipsoid_get_n_2(vtkNew<vtkParametricSuperEllipsoid> self) {
    return self->GetN2();
}
extern "C" void vtk_parametric_super_ellipsoid_evaluate(
    vtkNew<vtkParametricSuperEllipsoid> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_super_ellipsoid_evaluate_scalar(
    vtkNew<vtkParametricSuperEllipsoid> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricSuperToroid> vtkParametricSuperToroid_new() {
    return vtkNew<vtkParametricSuperToroid>();
}
extern "C" void vtkParametricSuperToroid_destructor(vtkNew<vtkParametricSuperToroid> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricSuperToroid_get_ptr(vtkNew<vtkParametricSuperToroid> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_parametric_super_toroid_get_dimension(vtkNew<vtkParametricSuperToroid> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_super_toroid_set_ring_radius(
    vtkNew<vtkParametricSuperToroid> self, double _arg
) {
    return self->SetRingRadius(_arg);
}
extern "C" double vtk_parametric_super_toroid_get_ring_radius(
    vtkNew<vtkParametricSuperToroid> self
) {
    return self->GetRingRadius();
}
extern "C" void vtk_parametric_super_toroid_set_cross_section_radius(
    vtkNew<vtkParametricSuperToroid> self, double _arg
) {
    return self->SetCrossSectionRadius(_arg);
}
extern "C" double vtk_parametric_super_toroid_get_cross_section_radius(
    vtkNew<vtkParametricSuperToroid> self
) {
    return self->GetCrossSectionRadius();
}
extern "C" void vtk_parametric_super_toroid_set_x_radius(
    vtkNew<vtkParametricSuperToroid> self, double _arg
) {
    return self->SetXRadius(_arg);
}
extern "C" double vtk_parametric_super_toroid_get_x_radius(vtkNew<vtkParametricSuperToroid> self) {
    return self->GetXRadius();
}
extern "C" void vtk_parametric_super_toroid_set_y_radius(
    vtkNew<vtkParametricSuperToroid> self, double _arg
) {
    return self->SetYRadius(_arg);
}
extern "C" double vtk_parametric_super_toroid_get_y_radius(vtkNew<vtkParametricSuperToroid> self) {
    return self->GetYRadius();
}
extern "C" void vtk_parametric_super_toroid_set_z_radius(
    vtkNew<vtkParametricSuperToroid> self, double _arg
) {
    return self->SetZRadius(_arg);
}
extern "C" double vtk_parametric_super_toroid_get_z_radius(vtkNew<vtkParametricSuperToroid> self) {
    return self->GetZRadius();
}
extern "C" void vtk_parametric_super_toroid_set_n_1(
    vtkNew<vtkParametricSuperToroid> self, double _arg
) {
    return self->SetN1(_arg);
}
extern "C" double vtk_parametric_super_toroid_get_n_1(vtkNew<vtkParametricSuperToroid> self) {
    return self->GetN1();
}
extern "C" void vtk_parametric_super_toroid_set_n_2(
    vtkNew<vtkParametricSuperToroid> self, double _arg
) {
    return self->SetN2(_arg);
}
extern "C" double vtk_parametric_super_toroid_get_n_2(vtkNew<vtkParametricSuperToroid> self) {
    return self->GetN2();
}
extern "C" void vtk_parametric_super_toroid_evaluate(
    vtkNew<vtkParametricSuperToroid> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_super_toroid_evaluate_scalar(
    vtkNew<vtkParametricSuperToroid> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
extern "C" vtkNew<vtkParametricTorus> vtkParametricTorus_new() {
    return vtkNew<vtkParametricTorus>();
}
extern "C" void vtkParametricTorus_destructor(vtkNew<vtkParametricTorus> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkParametricTorus_get_ptr(vtkNew<vtkParametricTorus> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_parametric_torus_set_ring_radius(vtkNew<vtkParametricTorus> self, double _arg) {
    return self->SetRingRadius(_arg);
}
extern "C" double vtk_parametric_torus_get_ring_radius(vtkNew<vtkParametricTorus> self) {
    return self->GetRingRadius();
}
extern "C" void vtk_parametric_torus_set_cross_section_radius(
    vtkNew<vtkParametricTorus> self, double _arg
) {
    return self->SetCrossSectionRadius(_arg);
}
extern "C" double vtk_parametric_torus_get_cross_section_radius(vtkNew<vtkParametricTorus> self) {
    return self->GetCrossSectionRadius();
}
extern "C" int vtk_parametric_torus_get_dimension(vtkNew<vtkParametricTorus> self) {
    return self->GetDimension();
}
extern "C" void vtk_parametric_torus_evaluate(
    vtkNew<vtkParametricTorus> self, double* uvw, double* Pt, double* Duvw
) {
    return self->Evaluate(uvw, Pt, Duvw);
}
extern "C" double vtk_parametric_torus_evaluate_scalar(
    vtkNew<vtkParametricTorus> self, double* uvw, double* Pt, double* Duvw
) {
    return self->EvaluateScalar(uvw, Pt, Duvw);
}
