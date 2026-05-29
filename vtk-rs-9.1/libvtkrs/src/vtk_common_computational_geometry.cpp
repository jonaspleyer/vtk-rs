// Include header file
#include<vtk_common_computational_geometry.h>

// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkBilinearQuadIntersection.h>
#include<vtkCardinalSpline.h>
#include<vtkKochanekSpline.h>
#include<vtkParametricBohemianDome.h>
#include<vtkParametricBour.h>
#include<vtkParametricBoy.h>
#include<vtkParametricCatalanMinimal.h>
#include<vtkParametricConicSpiral.h>
#include<vtkParametricCrossCap.h>
#include<vtkParametricDini.h>
#include<vtkParametricEllipsoid.h>
#include<vtkParametricEnneper.h>
#include<vtkParametricFigure8Klein.h>
#include<vtkParametricFunction.h>
#include<vtkParametricHenneberg.h>
#include<vtkParametricKlein.h>
#include<vtkParametricKuen.h>
#include<vtkParametricMobius.h>
#include<vtkParametricPluckerConoid.h>
#include<vtkParametricPseudosphere.h>
#include<vtkParametricRandomHills.h>
#include<vtkParametricRoman.h>
#include<vtkParametricSpline.h>
#include<vtkParametricSuperEllipsoid.h>
#include<vtkParametricSuperToroid.h>
#include<vtkParametricTorus.h>

// Implement declared functions
extern "C" vtkCardinalSpline * vtkCardinalSpline_new () {return vtkCardinalSpline :: New () ;}
extern "C" void vtkCardinalSpline_destructor (vtkCardinalSpline * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_cardinal_spline_compute(vtkCardinalSpline* sself) { sself->Compute(); }
extern "C" double vtk_cardinal_spline_evaluate(vtkCardinalSpline* sself, double t) { return sself->Evaluate(t); }
extern "C" vtkKochanekSpline * vtkKochanekSpline_new () {return vtkKochanekSpline :: New () ;}
extern "C" void vtkKochanekSpline_destructor (vtkKochanekSpline * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_kochanek_spline_compute(vtkKochanekSpline* sself) { sself->Compute(); }
extern "C" double vtk_kochanek_spline_evaluate(vtkKochanekSpline* sself, double t) { return sself->Evaluate(t); }
extern "C" void vtk_kochanek_spline_set_default_bias(vtkKochanekSpline* sself, double _arg) { sself->SetDefaultBias(_arg); }
extern "C" double vtk_kochanek_spline_get_default_bias(vtkKochanekSpline* sself) { return sself->GetDefaultBias(); }
extern "C" void vtk_kochanek_spline_set_default_tension(vtkKochanekSpline* sself, double _arg) { sself->SetDefaultTension(_arg); }
extern "C" double vtk_kochanek_spline_get_default_tension(vtkKochanekSpline* sself) { return sself->GetDefaultTension(); }
extern "C" void vtk_kochanek_spline_set_default_continuity(vtkKochanekSpline* sself, double _arg) { sself->SetDefaultContinuity(_arg); }
extern "C" double vtk_kochanek_spline_get_default_continuity(vtkKochanekSpline* sself) { return sself->GetDefaultContinuity(); }
extern "C" vtkParametricBohemianDome * vtkParametricBohemianDome_new () {return vtkParametricBohemianDome :: New () ;}
extern "C" void vtkParametricBohemianDome_destructor (vtkParametricBohemianDome * sself) {sself -> Delete () ; return ;}
extern "C" double vtk_parametric_bohemian_dome_get_a(vtkParametricBohemianDome* sself) { return sself->GetA(); }
extern "C" void vtk_parametric_bohemian_dome_set_a(vtkParametricBohemianDome* sself, double _arg) { sself->SetA(_arg); }
extern "C" double vtk_parametric_bohemian_dome_get_b(vtkParametricBohemianDome* sself) { return sself->GetB(); }
extern "C" void vtk_parametric_bohemian_dome_set_b(vtkParametricBohemianDome* sself, double _arg) { sself->SetB(_arg); }
extern "C" double vtk_parametric_bohemian_dome_get_c(vtkParametricBohemianDome* sself) { return sself->GetC(); }
extern "C" void vtk_parametric_bohemian_dome_set_c(vtkParametricBohemianDome* sself, double _arg) { sself->SetC(_arg); }
extern "C" int vtk_parametric_bohemian_dome_get_dimension(vtkParametricBohemianDome* sself) { return sself->GetDimension(); }
extern "C" vtkParametricBour * vtkParametricBour_new () {return vtkParametricBour :: New () ;}
extern "C" void vtkParametricBour_destructor (vtkParametricBour * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_bour_get_dimension(vtkParametricBour* sself) { return sself->GetDimension(); }
extern "C" vtkParametricBoy * vtkParametricBoy_new () {return vtkParametricBoy :: New () ;}
extern "C" void vtkParametricBoy_destructor (vtkParametricBoy * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_boy_get_dimension(vtkParametricBoy* sself) { return sself->GetDimension(); }
extern "C" void vtk_parametric_boy_set_z_scale(vtkParametricBoy* sself, double _arg) { sself->SetZScale(_arg); }
extern "C" double vtk_parametric_boy_get_z_scale(vtkParametricBoy* sself) { return sself->GetZScale(); }
extern "C" vtkParametricCatalanMinimal * vtkParametricCatalanMinimal_new () {return vtkParametricCatalanMinimal :: New () ;}
extern "C" void vtkParametricCatalanMinimal_destructor (vtkParametricCatalanMinimal * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_catalan_minimal_get_dimension(vtkParametricCatalanMinimal* sself) { return sself->GetDimension(); }
extern "C" vtkParametricConicSpiral * vtkParametricConicSpiral_new () {return vtkParametricConicSpiral :: New () ;}
extern "C" void vtkParametricConicSpiral_destructor (vtkParametricConicSpiral * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_conic_spiral_get_dimension(vtkParametricConicSpiral* sself) { return sself->GetDimension(); }
extern "C" void vtk_parametric_conic_spiral_set_a(vtkParametricConicSpiral* sself, double _arg) { sself->SetA(_arg); }
extern "C" double vtk_parametric_conic_spiral_get_a(vtkParametricConicSpiral* sself) { return sself->GetA(); }
extern "C" void vtk_parametric_conic_spiral_set_b(vtkParametricConicSpiral* sself, double _arg) { sself->SetB(_arg); }
extern "C" double vtk_parametric_conic_spiral_get_b(vtkParametricConicSpiral* sself) { return sself->GetB(); }
extern "C" void vtk_parametric_conic_spiral_set_c(vtkParametricConicSpiral* sself, double _arg) { sself->SetC(_arg); }
extern "C" double vtk_parametric_conic_spiral_get_c(vtkParametricConicSpiral* sself) { return sself->GetC(); }
extern "C" void vtk_parametric_conic_spiral_set_n(vtkParametricConicSpiral* sself, double _arg) { sself->SetN(_arg); }
extern "C" double vtk_parametric_conic_spiral_get_n(vtkParametricConicSpiral* sself) { return sself->GetN(); }
extern "C" vtkParametricCrossCap * vtkParametricCrossCap_new () {return vtkParametricCrossCap :: New () ;}
extern "C" void vtkParametricCrossCap_destructor (vtkParametricCrossCap * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_cross_cap_get_dimension(vtkParametricCrossCap* sself) { return sself->GetDimension(); }
extern "C" vtkParametricDini * vtkParametricDini_new () {return vtkParametricDini :: New () ;}
extern "C" void vtkParametricDini_destructor (vtkParametricDini * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_dini_get_dimension(vtkParametricDini* sself) { return sself->GetDimension(); }
extern "C" void vtk_parametric_dini_set_a(vtkParametricDini* sself, double _arg) { sself->SetA(_arg); }
extern "C" double vtk_parametric_dini_get_a(vtkParametricDini* sself) { return sself->GetA(); }
extern "C" void vtk_parametric_dini_set_b(vtkParametricDini* sself, double _arg) { sself->SetB(_arg); }
extern "C" double vtk_parametric_dini_get_b(vtkParametricDini* sself) { return sself->GetB(); }
extern "C" vtkParametricEllipsoid * vtkParametricEllipsoid_new () {return vtkParametricEllipsoid :: New () ;}
extern "C" void vtkParametricEllipsoid_destructor (vtkParametricEllipsoid * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_ellipsoid_get_dimension(vtkParametricEllipsoid* sself) { return sself->GetDimension(); }
extern "C" void vtk_parametric_ellipsoid_set_x_radius(vtkParametricEllipsoid* sself, double _arg) { sself->SetXRadius(_arg); }
extern "C" double vtk_parametric_ellipsoid_get_x_radius(vtkParametricEllipsoid* sself) { return sself->GetXRadius(); }
extern "C" void vtk_parametric_ellipsoid_set_y_radius(vtkParametricEllipsoid* sself, double _arg) { sself->SetYRadius(_arg); }
extern "C" double vtk_parametric_ellipsoid_get_y_radius(vtkParametricEllipsoid* sself) { return sself->GetYRadius(); }
extern "C" void vtk_parametric_ellipsoid_set_z_radius(vtkParametricEllipsoid* sself, double _arg) { sself->SetZRadius(_arg); }
extern "C" double vtk_parametric_ellipsoid_get_z_radius(vtkParametricEllipsoid* sself) { return sself->GetZRadius(); }
extern "C" vtkParametricEnneper * vtkParametricEnneper_new () {return vtkParametricEnneper :: New () ;}
extern "C" void vtkParametricEnneper_destructor (vtkParametricEnneper * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_enneper_get_dimension(vtkParametricEnneper* sself) { return sself->GetDimension(); }
extern "C" vtkParametricFigure8Klein * vtkParametricFigure8Klein_new () {return vtkParametricFigure8Klein :: New () ;}
extern "C" void vtkParametricFigure8Klein_destructor (vtkParametricFigure8Klein * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_parametric_figure_8_klein_set_radius(vtkParametricFigure8Klein* sself, double _arg) { sself->SetRadius(_arg); }
extern "C" double vtk_parametric_figure_8_klein_get_radius(vtkParametricFigure8Klein* sself) { return sself->GetRadius(); }
extern "C" int vtk_parametric_figure_8_klein_get_dimension(vtkParametricFigure8Klein* sself) { return sself->GetDimension(); }
extern "C" vtkParametricHenneberg * vtkParametricHenneberg_new () {return vtkParametricHenneberg :: New () ;}
extern "C" void vtkParametricHenneberg_destructor (vtkParametricHenneberg * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_henneberg_get_dimension(vtkParametricHenneberg* sself) { return sself->GetDimension(); }
extern "C" vtkParametricKlein * vtkParametricKlein_new () {return vtkParametricKlein :: New () ;}
extern "C" void vtkParametricKlein_destructor (vtkParametricKlein * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_klein_get_dimension(vtkParametricKlein* sself) { return sself->GetDimension(); }
extern "C" vtkParametricKuen * vtkParametricKuen_new () {return vtkParametricKuen :: New () ;}
extern "C" void vtkParametricKuen_destructor (vtkParametricKuen * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_kuen_get_dimension(vtkParametricKuen* sself) { return sself->GetDimension(); }
extern "C" void vtk_parametric_kuen_set_delta_v_0(vtkParametricKuen* sself, double _arg) { sself->SetDeltaV0(_arg); }
extern "C" double vtk_parametric_kuen_get_delta_v_0(vtkParametricKuen* sself) { return sself->GetDeltaV0(); }
extern "C" vtkParametricMobius * vtkParametricMobius_new () {return vtkParametricMobius :: New () ;}
extern "C" void vtkParametricMobius_destructor (vtkParametricMobius * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_parametric_mobius_set_radius(vtkParametricMobius* sself, double _arg) { sself->SetRadius(_arg); }
extern "C" double vtk_parametric_mobius_get_radius(vtkParametricMobius* sself) { return sself->GetRadius(); }
extern "C" int vtk_parametric_mobius_get_dimension(vtkParametricMobius* sself) { return sself->GetDimension(); }
extern "C" vtkParametricPluckerConoid * vtkParametricPluckerConoid_new () {return vtkParametricPluckerConoid :: New () ;}
extern "C" void vtkParametricPluckerConoid_destructor (vtkParametricPluckerConoid * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_plucker_conoid_get_n(vtkParametricPluckerConoid* sself) { return sself->GetN(); }
extern "C" void vtk_parametric_plucker_conoid_set_n(vtkParametricPluckerConoid* sself, int _arg) { sself->SetN(_arg); }
extern "C" int vtk_parametric_plucker_conoid_get_dimension(vtkParametricPluckerConoid* sself) { return sself->GetDimension(); }
extern "C" vtkParametricPseudosphere * vtkParametricPseudosphere_new () {return vtkParametricPseudosphere :: New () ;}
extern "C" void vtkParametricPseudosphere_destructor (vtkParametricPseudosphere * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_pseudosphere_get_dimension(vtkParametricPseudosphere* sself) { return sself->GetDimension(); }
extern "C" vtkParametricRandomHills * vtkParametricRandomHills_new () {return vtkParametricRandomHills :: New () ;}
extern "C" void vtkParametricRandomHills_destructor (vtkParametricRandomHills * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_random_hills_get_dimension(vtkParametricRandomHills* sself) { return sself->GetDimension(); }
extern "C" void vtk_parametric_random_hills_set_number_of_hills(vtkParametricRandomHills* sself, int _arg) { sself->SetNumberOfHills(_arg); }
extern "C" int vtk_parametric_random_hills_get_number_of_hills(vtkParametricRandomHills* sself) { return sself->GetNumberOfHills(); }
extern "C" void vtk_parametric_random_hills_set_hill_x_variance(vtkParametricRandomHills* sself, double _arg) { sself->SetHillXVariance(_arg); }
extern "C" double vtk_parametric_random_hills_get_hill_x_variance(vtkParametricRandomHills* sself) { return sself->GetHillXVariance(); }
extern "C" void vtk_parametric_random_hills_set_hill_y_variance(vtkParametricRandomHills* sself, double _arg) { sself->SetHillYVariance(_arg); }
extern "C" double vtk_parametric_random_hills_get_hill_y_variance(vtkParametricRandomHills* sself) { return sself->GetHillYVariance(); }
extern "C" void vtk_parametric_random_hills_set_hill_amplitude(vtkParametricRandomHills* sself, double _arg) { sself->SetHillAmplitude(_arg); }
extern "C" double vtk_parametric_random_hills_get_hill_amplitude(vtkParametricRandomHills* sself) { return sself->GetHillAmplitude(); }
extern "C" void vtk_parametric_random_hills_set_random_seed(vtkParametricRandomHills* sself, int _arg) { sself->SetRandomSeed(_arg); }
extern "C" int vtk_parametric_random_hills_get_random_seed(vtkParametricRandomHills* sself) { return sself->GetRandomSeed(); }
extern "C" void vtk_parametric_random_hills_set_allow_random_generation(vtkParametricRandomHills* sself, int _arg) { sself->SetAllowRandomGeneration(_arg); }
extern "C" int vtk_parametric_random_hills_get_allow_random_generation_min_value(vtkParametricRandomHills* sself) { return sself->GetAllowRandomGenerationMinValue(); }
extern "C" int vtk_parametric_random_hills_get_allow_random_generation_max_value(vtkParametricRandomHills* sself) { return sself->GetAllowRandomGenerationMaxValue(); }
extern "C" int vtk_parametric_random_hills_get_allow_random_generation(vtkParametricRandomHills* sself) { return sself->GetAllowRandomGeneration(); }
extern "C" void vtk_parametric_random_hills_allow_random_generation_on(vtkParametricRandomHills* sself) { sself->AllowRandomGenerationOn(); }
extern "C" void vtk_parametric_random_hills_allow_random_generation_off(vtkParametricRandomHills* sself) { sself->AllowRandomGenerationOff(); }
extern "C" void vtk_parametric_random_hills_set_x_variance_scale_factor(vtkParametricRandomHills* sself, double _arg) { sself->SetXVarianceScaleFactor(_arg); }
extern "C" double vtk_parametric_random_hills_get_x_variance_scale_factor(vtkParametricRandomHills* sself) { return sself->GetXVarianceScaleFactor(); }
extern "C" void vtk_parametric_random_hills_set_y_variance_scale_factor(vtkParametricRandomHills* sself, double _arg) { sself->SetYVarianceScaleFactor(_arg); }
extern "C" double vtk_parametric_random_hills_get_y_variance_scale_factor(vtkParametricRandomHills* sself) { return sself->GetYVarianceScaleFactor(); }
extern "C" void vtk_parametric_random_hills_set_amplitude_scale_factor(vtkParametricRandomHills* sself, double _arg) { sself->SetAmplitudeScaleFactor(_arg); }
extern "C" double vtk_parametric_random_hills_get_amplitude_scale_factor(vtkParametricRandomHills* sself) { return sself->GetAmplitudeScaleFactor(); }
extern "C" vtkParametricRoman * vtkParametricRoman_new () {return vtkParametricRoman :: New () ;}
extern "C" void vtkParametricRoman_destructor (vtkParametricRoman * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_roman_get_dimension(vtkParametricRoman* sself) { return sself->GetDimension(); }
extern "C" void vtk_parametric_roman_set_radius(vtkParametricRoman* sself, double _arg) { sself->SetRadius(_arg); }
extern "C" double vtk_parametric_roman_get_radius(vtkParametricRoman* sself) { return sself->GetRadius(); }
extern "C" vtkParametricSpline * vtkParametricSpline_new () {return vtkParametricSpline :: New () ;}
extern "C" void vtkParametricSpline_destructor (vtkParametricSpline * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_spline_get_dimension(vtkParametricSpline* sself) { return sself->GetDimension(); }
extern "C" void vtk_parametric_spline_set_number_of_points(vtkParametricSpline* sself, long long numPts) { sself->SetNumberOfPoints(numPts); }
extern "C" void vtk_parametric_spline_set_point(vtkParametricSpline* sself, long long index, double x, double y, double z) { sself->SetPoint(index, x, y, z); }
extern "C" void vtk_parametric_spline_set_closed(vtkParametricSpline* sself, int _arg) { sself->SetClosed(_arg); }
extern "C" int vtk_parametric_spline_get_closed(vtkParametricSpline* sself) { return sself->GetClosed(); }
extern "C" void vtk_parametric_spline_closed_on(vtkParametricSpline* sself) { sself->ClosedOn(); }
extern "C" void vtk_parametric_spline_closed_off(vtkParametricSpline* sself) { sself->ClosedOff(); }
extern "C" void vtk_parametric_spline_set_parameterize_by_length(vtkParametricSpline* sself, int _arg) { sself->SetParameterizeByLength(_arg); }
extern "C" int vtk_parametric_spline_get_parameterize_by_length(vtkParametricSpline* sself) { return sself->GetParameterizeByLength(); }
extern "C" void vtk_parametric_spline_parameterize_by_length_on(vtkParametricSpline* sself) { sself->ParameterizeByLengthOn(); }
extern "C" void vtk_parametric_spline_parameterize_by_length_off(vtkParametricSpline* sself) { sself->ParameterizeByLengthOff(); }
extern "C" void vtk_parametric_spline_set_left_constraint(vtkParametricSpline* sself, int _arg) { sself->SetLeftConstraint(_arg); }
extern "C" int vtk_parametric_spline_get_left_constraint_min_value(vtkParametricSpline* sself) { return sself->GetLeftConstraintMinValue(); }
extern "C" int vtk_parametric_spline_get_left_constraint_max_value(vtkParametricSpline* sself) { return sself->GetLeftConstraintMaxValue(); }
extern "C" int vtk_parametric_spline_get_left_constraint(vtkParametricSpline* sself) { return sself->GetLeftConstraint(); }
extern "C" void vtk_parametric_spline_set_right_constraint(vtkParametricSpline* sself, int _arg) { sself->SetRightConstraint(_arg); }
extern "C" int vtk_parametric_spline_get_right_constraint_min_value(vtkParametricSpline* sself) { return sself->GetRightConstraintMinValue(); }
extern "C" int vtk_parametric_spline_get_right_constraint_max_value(vtkParametricSpline* sself) { return sself->GetRightConstraintMaxValue(); }
extern "C" int vtk_parametric_spline_get_right_constraint(vtkParametricSpline* sself) { return sself->GetRightConstraint(); }
extern "C" void vtk_parametric_spline_set_left_value(vtkParametricSpline* sself, double _arg) { sself->SetLeftValue(_arg); }
extern "C" double vtk_parametric_spline_get_left_value(vtkParametricSpline* sself) { return sself->GetLeftValue(); }
extern "C" void vtk_parametric_spline_set_right_value(vtkParametricSpline* sself, double _arg) { sself->SetRightValue(_arg); }
extern "C" double vtk_parametric_spline_get_right_value(vtkParametricSpline* sself) { return sself->GetRightValue(); }
extern "C" vtkParametricSuperEllipsoid * vtkParametricSuperEllipsoid_new () {return vtkParametricSuperEllipsoid :: New () ;}
extern "C" void vtkParametricSuperEllipsoid_destructor (vtkParametricSuperEllipsoid * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_super_ellipsoid_get_dimension(vtkParametricSuperEllipsoid* sself) { return sself->GetDimension(); }
extern "C" void vtk_parametric_super_ellipsoid_set_x_radius(vtkParametricSuperEllipsoid* sself, double _arg) { sself->SetXRadius(_arg); }
extern "C" double vtk_parametric_super_ellipsoid_get_x_radius(vtkParametricSuperEllipsoid* sself) { return sself->GetXRadius(); }
extern "C" void vtk_parametric_super_ellipsoid_set_y_radius(vtkParametricSuperEllipsoid* sself, double _arg) { sself->SetYRadius(_arg); }
extern "C" double vtk_parametric_super_ellipsoid_get_y_radius(vtkParametricSuperEllipsoid* sself) { return sself->GetYRadius(); }
extern "C" void vtk_parametric_super_ellipsoid_set_z_radius(vtkParametricSuperEllipsoid* sself, double _arg) { sself->SetZRadius(_arg); }
extern "C" double vtk_parametric_super_ellipsoid_get_z_radius(vtkParametricSuperEllipsoid* sself) { return sself->GetZRadius(); }
extern "C" void vtk_parametric_super_ellipsoid_set_n_1(vtkParametricSuperEllipsoid* sself, double _arg) { sself->SetN1(_arg); }
extern "C" double vtk_parametric_super_ellipsoid_get_n_1(vtkParametricSuperEllipsoid* sself) { return sself->GetN1(); }
extern "C" void vtk_parametric_super_ellipsoid_set_n_2(vtkParametricSuperEllipsoid* sself, double _arg) { sself->SetN2(_arg); }
extern "C" double vtk_parametric_super_ellipsoid_get_n_2(vtkParametricSuperEllipsoid* sself) { return sself->GetN2(); }
extern "C" vtkParametricSuperToroid * vtkParametricSuperToroid_new () {return vtkParametricSuperToroid :: New () ;}
extern "C" void vtkParametricSuperToroid_destructor (vtkParametricSuperToroid * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_parametric_super_toroid_get_dimension(vtkParametricSuperToroid* sself) { return sself->GetDimension(); }
extern "C" void vtk_parametric_super_toroid_set_ring_radius(vtkParametricSuperToroid* sself, double _arg) { sself->SetRingRadius(_arg); }
extern "C" double vtk_parametric_super_toroid_get_ring_radius(vtkParametricSuperToroid* sself) { return sself->GetRingRadius(); }
extern "C" void vtk_parametric_super_toroid_set_cross_section_radius(vtkParametricSuperToroid* sself, double _arg) { sself->SetCrossSectionRadius(_arg); }
extern "C" double vtk_parametric_super_toroid_get_cross_section_radius(vtkParametricSuperToroid* sself) { return sself->GetCrossSectionRadius(); }
extern "C" void vtk_parametric_super_toroid_set_x_radius(vtkParametricSuperToroid* sself, double _arg) { sself->SetXRadius(_arg); }
extern "C" double vtk_parametric_super_toroid_get_x_radius(vtkParametricSuperToroid* sself) { return sself->GetXRadius(); }
extern "C" void vtk_parametric_super_toroid_set_y_radius(vtkParametricSuperToroid* sself, double _arg) { sself->SetYRadius(_arg); }
extern "C" double vtk_parametric_super_toroid_get_y_radius(vtkParametricSuperToroid* sself) { return sself->GetYRadius(); }
extern "C" void vtk_parametric_super_toroid_set_z_radius(vtkParametricSuperToroid* sself, double _arg) { sself->SetZRadius(_arg); }
extern "C" double vtk_parametric_super_toroid_get_z_radius(vtkParametricSuperToroid* sself) { return sself->GetZRadius(); }
extern "C" void vtk_parametric_super_toroid_set_n_1(vtkParametricSuperToroid* sself, double _arg) { sself->SetN1(_arg); }
extern "C" double vtk_parametric_super_toroid_get_n_1(vtkParametricSuperToroid* sself) { return sself->GetN1(); }
extern "C" void vtk_parametric_super_toroid_set_n_2(vtkParametricSuperToroid* sself, double _arg) { sself->SetN2(_arg); }
extern "C" double vtk_parametric_super_toroid_get_n_2(vtkParametricSuperToroid* sself) { return sself->GetN2(); }
extern "C" vtkParametricTorus * vtkParametricTorus_new () {return vtkParametricTorus :: New () ;}
extern "C" void vtkParametricTorus_destructor (vtkParametricTorus * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_parametric_torus_set_ring_radius(vtkParametricTorus* sself, double _arg) { sself->SetRingRadius(_arg); }
extern "C" double vtk_parametric_torus_get_ring_radius(vtkParametricTorus* sself) { return sself->GetRingRadius(); }
extern "C" void vtk_parametric_torus_set_cross_section_radius(vtkParametricTorus* sself, double _arg) { sself->SetCrossSectionRadius(_arg); }
extern "C" double vtk_parametric_torus_get_cross_section_radius(vtkParametricTorus* sself) { return sself->GetCrossSectionRadius(); }
extern "C" int vtk_parametric_torus_get_dimension(vtkParametricTorus* sself) { return sself->GetDimension(); }
