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

// Declare exported functions
extern "C" vtkCardinalSpline * vtkCardinalSpline_new () ;
extern "C" void vtkCardinalSpline_destructor (vtkCardinalSpline * sself) ;
extern "C" void vtk_cardinal_spline_compute(vtkCardinalSpline* sself);
extern "C" double vtk_cardinal_spline_evaluate(vtkCardinalSpline* sself, double t);
extern "C" vtkKochanekSpline * vtkKochanekSpline_new () ;
extern "C" void vtkKochanekSpline_destructor (vtkKochanekSpline * sself) ;
extern "C" void vtk_kochanek_spline_compute(vtkKochanekSpline* sself);
extern "C" double vtk_kochanek_spline_evaluate(vtkKochanekSpline* sself, double t);
extern "C" void vtk_kochanek_spline_set_default_bias(vtkKochanekSpline* sself, double _arg);
extern "C" double vtk_kochanek_spline_get_default_bias(vtkKochanekSpline* sself);
extern "C" void vtk_kochanek_spline_set_default_tension(vtkKochanekSpline* sself, double _arg);
extern "C" double vtk_kochanek_spline_get_default_tension(vtkKochanekSpline* sself);
extern "C" void vtk_kochanek_spline_set_default_continuity(vtkKochanekSpline* sself, double _arg);
extern "C" double vtk_kochanek_spline_get_default_continuity(vtkKochanekSpline* sself);
extern "C" vtkParametricBohemianDome * vtkParametricBohemianDome_new () ;
extern "C" void vtkParametricBohemianDome_destructor (vtkParametricBohemianDome * sself) ;
extern "C" double vtk_parametric_bohemian_dome_get_a(vtkParametricBohemianDome* sself);
extern "C" void vtk_parametric_bohemian_dome_set_a(vtkParametricBohemianDome* sself, double _arg);
extern "C" double vtk_parametric_bohemian_dome_get_b(vtkParametricBohemianDome* sself);
extern "C" void vtk_parametric_bohemian_dome_set_b(vtkParametricBohemianDome* sself, double _arg);
extern "C" double vtk_parametric_bohemian_dome_get_c(vtkParametricBohemianDome* sself);
extern "C" void vtk_parametric_bohemian_dome_set_c(vtkParametricBohemianDome* sself, double _arg);
extern "C" int vtk_parametric_bohemian_dome_get_dimension(vtkParametricBohemianDome* sself);
extern "C" vtkParametricBour * vtkParametricBour_new () ;
extern "C" void vtkParametricBour_destructor (vtkParametricBour * sself) ;
extern "C" int vtk_parametric_bour_get_dimension(vtkParametricBour* sself);
extern "C" vtkParametricBoy * vtkParametricBoy_new () ;
extern "C" void vtkParametricBoy_destructor (vtkParametricBoy * sself) ;
extern "C" int vtk_parametric_boy_get_dimension(vtkParametricBoy* sself);
extern "C" void vtk_parametric_boy_set_z_scale(vtkParametricBoy* sself, double _arg);
extern "C" double vtk_parametric_boy_get_z_scale(vtkParametricBoy* sself);
extern "C" vtkParametricCatalanMinimal * vtkParametricCatalanMinimal_new () ;
extern "C" void vtkParametricCatalanMinimal_destructor (vtkParametricCatalanMinimal * sself) ;
extern "C" int vtk_parametric_catalan_minimal_get_dimension(vtkParametricCatalanMinimal* sself);
extern "C" vtkParametricConicSpiral * vtkParametricConicSpiral_new () ;
extern "C" void vtkParametricConicSpiral_destructor (vtkParametricConicSpiral * sself) ;
extern "C" int vtk_parametric_conic_spiral_get_dimension(vtkParametricConicSpiral* sself);
extern "C" void vtk_parametric_conic_spiral_set_a(vtkParametricConicSpiral* sself, double _arg);
extern "C" double vtk_parametric_conic_spiral_get_a(vtkParametricConicSpiral* sself);
extern "C" void vtk_parametric_conic_spiral_set_b(vtkParametricConicSpiral* sself, double _arg);
extern "C" double vtk_parametric_conic_spiral_get_b(vtkParametricConicSpiral* sself);
extern "C" void vtk_parametric_conic_spiral_set_c(vtkParametricConicSpiral* sself, double _arg);
extern "C" double vtk_parametric_conic_spiral_get_c(vtkParametricConicSpiral* sself);
extern "C" void vtk_parametric_conic_spiral_set_n(vtkParametricConicSpiral* sself, double _arg);
extern "C" double vtk_parametric_conic_spiral_get_n(vtkParametricConicSpiral* sself);
extern "C" vtkParametricCrossCap * vtkParametricCrossCap_new () ;
extern "C" void vtkParametricCrossCap_destructor (vtkParametricCrossCap * sself) ;
extern "C" int vtk_parametric_cross_cap_get_dimension(vtkParametricCrossCap* sself);
extern "C" vtkParametricDini * vtkParametricDini_new () ;
extern "C" void vtkParametricDini_destructor (vtkParametricDini * sself) ;
extern "C" int vtk_parametric_dini_get_dimension(vtkParametricDini* sself);
extern "C" void vtk_parametric_dini_set_a(vtkParametricDini* sself, double _arg);
extern "C" double vtk_parametric_dini_get_a(vtkParametricDini* sself);
extern "C" void vtk_parametric_dini_set_b(vtkParametricDini* sself, double _arg);
extern "C" double vtk_parametric_dini_get_b(vtkParametricDini* sself);
extern "C" vtkParametricEllipsoid * vtkParametricEllipsoid_new () ;
extern "C" void vtkParametricEllipsoid_destructor (vtkParametricEllipsoid * sself) ;
extern "C" int vtk_parametric_ellipsoid_get_dimension(vtkParametricEllipsoid* sself);
extern "C" void vtk_parametric_ellipsoid_set_x_radius(vtkParametricEllipsoid* sself, double _arg);
extern "C" double vtk_parametric_ellipsoid_get_x_radius(vtkParametricEllipsoid* sself);
extern "C" void vtk_parametric_ellipsoid_set_y_radius(vtkParametricEllipsoid* sself, double _arg);
extern "C" double vtk_parametric_ellipsoid_get_y_radius(vtkParametricEllipsoid* sself);
extern "C" void vtk_parametric_ellipsoid_set_z_radius(vtkParametricEllipsoid* sself, double _arg);
extern "C" double vtk_parametric_ellipsoid_get_z_radius(vtkParametricEllipsoid* sself);
extern "C" vtkParametricEnneper * vtkParametricEnneper_new () ;
extern "C" void vtkParametricEnneper_destructor (vtkParametricEnneper * sself) ;
extern "C" int vtk_parametric_enneper_get_dimension(vtkParametricEnneper* sself);
extern "C" vtkParametricFigure8Klein * vtkParametricFigure8Klein_new () ;
extern "C" void vtkParametricFigure8Klein_destructor (vtkParametricFigure8Klein * sself) ;
extern "C" void vtk_parametric_figure_8_klein_set_radius(vtkParametricFigure8Klein* sself, double _arg);
extern "C" double vtk_parametric_figure_8_klein_get_radius(vtkParametricFigure8Klein* sself);
extern "C" int vtk_parametric_figure_8_klein_get_dimension(vtkParametricFigure8Klein* sself);
extern "C" vtkParametricHenneberg * vtkParametricHenneberg_new () ;
extern "C" void vtkParametricHenneberg_destructor (vtkParametricHenneberg * sself) ;
extern "C" int vtk_parametric_henneberg_get_dimension(vtkParametricHenneberg* sself);
extern "C" vtkParametricKlein * vtkParametricKlein_new () ;
extern "C" void vtkParametricKlein_destructor (vtkParametricKlein * sself) ;
extern "C" int vtk_parametric_klein_get_dimension(vtkParametricKlein* sself);
extern "C" vtkParametricKuen * vtkParametricKuen_new () ;
extern "C" void vtkParametricKuen_destructor (vtkParametricKuen * sself) ;
extern "C" int vtk_parametric_kuen_get_dimension(vtkParametricKuen* sself);
extern "C" void vtk_parametric_kuen_set_delta_v_0(vtkParametricKuen* sself, double _arg);
extern "C" double vtk_parametric_kuen_get_delta_v_0(vtkParametricKuen* sself);
extern "C" vtkParametricMobius * vtkParametricMobius_new () ;
extern "C" void vtkParametricMobius_destructor (vtkParametricMobius * sself) ;
extern "C" void vtk_parametric_mobius_set_radius(vtkParametricMobius* sself, double _arg);
extern "C" double vtk_parametric_mobius_get_radius(vtkParametricMobius* sself);
extern "C" int vtk_parametric_mobius_get_dimension(vtkParametricMobius* sself);
extern "C" vtkParametricPluckerConoid * vtkParametricPluckerConoid_new () ;
extern "C" void vtkParametricPluckerConoid_destructor (vtkParametricPluckerConoid * sself) ;
extern "C" int vtk_parametric_plucker_conoid_get_n(vtkParametricPluckerConoid* sself);
extern "C" void vtk_parametric_plucker_conoid_set_n(vtkParametricPluckerConoid* sself, int _arg);
extern "C" int vtk_parametric_plucker_conoid_get_dimension(vtkParametricPluckerConoid* sself);
extern "C" vtkParametricPseudosphere * vtkParametricPseudosphere_new () ;
extern "C" void vtkParametricPseudosphere_destructor (vtkParametricPseudosphere * sself) ;
extern "C" int vtk_parametric_pseudosphere_get_dimension(vtkParametricPseudosphere* sself);
extern "C" vtkParametricRandomHills * vtkParametricRandomHills_new () ;
extern "C" void vtkParametricRandomHills_destructor (vtkParametricRandomHills * sself) ;
extern "C" int vtk_parametric_random_hills_get_dimension(vtkParametricRandomHills* sself);
extern "C" void vtk_parametric_random_hills_set_number_of_hills(vtkParametricRandomHills* sself, int _arg);
extern "C" int vtk_parametric_random_hills_get_number_of_hills(vtkParametricRandomHills* sself);
extern "C" void vtk_parametric_random_hills_set_hill_x_variance(vtkParametricRandomHills* sself, double _arg);
extern "C" double vtk_parametric_random_hills_get_hill_x_variance(vtkParametricRandomHills* sself);
extern "C" void vtk_parametric_random_hills_set_hill_y_variance(vtkParametricRandomHills* sself, double _arg);
extern "C" double vtk_parametric_random_hills_get_hill_y_variance(vtkParametricRandomHills* sself);
extern "C" void vtk_parametric_random_hills_set_hill_amplitude(vtkParametricRandomHills* sself, double _arg);
extern "C" double vtk_parametric_random_hills_get_hill_amplitude(vtkParametricRandomHills* sself);
extern "C" void vtk_parametric_random_hills_set_random_seed(vtkParametricRandomHills* sself, int _arg);
extern "C" int vtk_parametric_random_hills_get_random_seed(vtkParametricRandomHills* sself);
extern "C" void vtk_parametric_random_hills_set_allow_random_generation(vtkParametricRandomHills* sself, int _arg);
extern "C" int vtk_parametric_random_hills_get_allow_random_generation_min_value(vtkParametricRandomHills* sself);
extern "C" int vtk_parametric_random_hills_get_allow_random_generation_max_value(vtkParametricRandomHills* sself);
extern "C" int vtk_parametric_random_hills_get_allow_random_generation(vtkParametricRandomHills* sself);
extern "C" void vtk_parametric_random_hills_allow_random_generation_on(vtkParametricRandomHills* sself);
extern "C" void vtk_parametric_random_hills_allow_random_generation_off(vtkParametricRandomHills* sself);
extern "C" void vtk_parametric_random_hills_set_x_variance_scale_factor(vtkParametricRandomHills* sself, double _arg);
extern "C" double vtk_parametric_random_hills_get_x_variance_scale_factor(vtkParametricRandomHills* sself);
extern "C" void vtk_parametric_random_hills_set_y_variance_scale_factor(vtkParametricRandomHills* sself, double _arg);
extern "C" double vtk_parametric_random_hills_get_y_variance_scale_factor(vtkParametricRandomHills* sself);
extern "C" void vtk_parametric_random_hills_set_amplitude_scale_factor(vtkParametricRandomHills* sself, double _arg);
extern "C" double vtk_parametric_random_hills_get_amplitude_scale_factor(vtkParametricRandomHills* sself);
extern "C" vtkParametricRoman * vtkParametricRoman_new () ;
extern "C" void vtkParametricRoman_destructor (vtkParametricRoman * sself) ;
extern "C" int vtk_parametric_roman_get_dimension(vtkParametricRoman* sself);
extern "C" void vtk_parametric_roman_set_radius(vtkParametricRoman* sself, double _arg);
extern "C" double vtk_parametric_roman_get_radius(vtkParametricRoman* sself);
extern "C" vtkParametricSpline * vtkParametricSpline_new () ;
extern "C" void vtkParametricSpline_destructor (vtkParametricSpline * sself) ;
extern "C" int vtk_parametric_spline_get_dimension(vtkParametricSpline* sself);
extern "C" void vtk_parametric_spline_set_number_of_points(vtkParametricSpline* sself, long long numPts);
extern "C" void vtk_parametric_spline_set_point(vtkParametricSpline* sself, long long index, double x, double y, double z);
extern "C" void vtk_parametric_spline_set_closed(vtkParametricSpline* sself, int _arg);
extern "C" int vtk_parametric_spline_get_closed(vtkParametricSpline* sself);
extern "C" void vtk_parametric_spline_closed_on(vtkParametricSpline* sself);
extern "C" void vtk_parametric_spline_closed_off(vtkParametricSpline* sself);
extern "C" void vtk_parametric_spline_set_parameterize_by_length(vtkParametricSpline* sself, int _arg);
extern "C" int vtk_parametric_spline_get_parameterize_by_length(vtkParametricSpline* sself);
extern "C" void vtk_parametric_spline_parameterize_by_length_on(vtkParametricSpline* sself);
extern "C" void vtk_parametric_spline_parameterize_by_length_off(vtkParametricSpline* sself);
extern "C" void vtk_parametric_spline_set_left_constraint(vtkParametricSpline* sself, int _arg);
extern "C" int vtk_parametric_spline_get_left_constraint_min_value(vtkParametricSpline* sself);
extern "C" int vtk_parametric_spline_get_left_constraint_max_value(vtkParametricSpline* sself);
extern "C" int vtk_parametric_spline_get_left_constraint(vtkParametricSpline* sself);
extern "C" void vtk_parametric_spline_set_right_constraint(vtkParametricSpline* sself, int _arg);
extern "C" int vtk_parametric_spline_get_right_constraint_min_value(vtkParametricSpline* sself);
extern "C" int vtk_parametric_spline_get_right_constraint_max_value(vtkParametricSpline* sself);
extern "C" int vtk_parametric_spline_get_right_constraint(vtkParametricSpline* sself);
extern "C" void vtk_parametric_spline_set_left_value(vtkParametricSpline* sself, double _arg);
extern "C" double vtk_parametric_spline_get_left_value(vtkParametricSpline* sself);
extern "C" void vtk_parametric_spline_set_right_value(vtkParametricSpline* sself, double _arg);
extern "C" double vtk_parametric_spline_get_right_value(vtkParametricSpline* sself);
extern "C" vtkParametricSuperEllipsoid * vtkParametricSuperEllipsoid_new () ;
extern "C" void vtkParametricSuperEllipsoid_destructor (vtkParametricSuperEllipsoid * sself) ;
extern "C" int vtk_parametric_super_ellipsoid_get_dimension(vtkParametricSuperEllipsoid* sself);
extern "C" void vtk_parametric_super_ellipsoid_set_x_radius(vtkParametricSuperEllipsoid* sself, double _arg);
extern "C" double vtk_parametric_super_ellipsoid_get_x_radius(vtkParametricSuperEllipsoid* sself);
extern "C" void vtk_parametric_super_ellipsoid_set_y_radius(vtkParametricSuperEllipsoid* sself, double _arg);
extern "C" double vtk_parametric_super_ellipsoid_get_y_radius(vtkParametricSuperEllipsoid* sself);
extern "C" void vtk_parametric_super_ellipsoid_set_z_radius(vtkParametricSuperEllipsoid* sself, double _arg);
extern "C" double vtk_parametric_super_ellipsoid_get_z_radius(vtkParametricSuperEllipsoid* sself);
extern "C" void vtk_parametric_super_ellipsoid_set_n_1(vtkParametricSuperEllipsoid* sself, double _arg);
extern "C" double vtk_parametric_super_ellipsoid_get_n_1(vtkParametricSuperEllipsoid* sself);
extern "C" void vtk_parametric_super_ellipsoid_set_n_2(vtkParametricSuperEllipsoid* sself, double _arg);
extern "C" double vtk_parametric_super_ellipsoid_get_n_2(vtkParametricSuperEllipsoid* sself);
extern "C" vtkParametricSuperToroid * vtkParametricSuperToroid_new () ;
extern "C" void vtkParametricSuperToroid_destructor (vtkParametricSuperToroid * sself) ;
extern "C" int vtk_parametric_super_toroid_get_dimension(vtkParametricSuperToroid* sself);
extern "C" void vtk_parametric_super_toroid_set_ring_radius(vtkParametricSuperToroid* sself, double _arg);
extern "C" double vtk_parametric_super_toroid_get_ring_radius(vtkParametricSuperToroid* sself);
extern "C" void vtk_parametric_super_toroid_set_cross_section_radius(vtkParametricSuperToroid* sself, double _arg);
extern "C" double vtk_parametric_super_toroid_get_cross_section_radius(vtkParametricSuperToroid* sself);
extern "C" void vtk_parametric_super_toroid_set_x_radius(vtkParametricSuperToroid* sself, double _arg);
extern "C" double vtk_parametric_super_toroid_get_x_radius(vtkParametricSuperToroid* sself);
extern "C" void vtk_parametric_super_toroid_set_y_radius(vtkParametricSuperToroid* sself, double _arg);
extern "C" double vtk_parametric_super_toroid_get_y_radius(vtkParametricSuperToroid* sself);
extern "C" void vtk_parametric_super_toroid_set_z_radius(vtkParametricSuperToroid* sself, double _arg);
extern "C" double vtk_parametric_super_toroid_get_z_radius(vtkParametricSuperToroid* sself);
extern "C" void vtk_parametric_super_toroid_set_n_1(vtkParametricSuperToroid* sself, double _arg);
extern "C" double vtk_parametric_super_toroid_get_n_1(vtkParametricSuperToroid* sself);
extern "C" void vtk_parametric_super_toroid_set_n_2(vtkParametricSuperToroid* sself, double _arg);
extern "C" double vtk_parametric_super_toroid_get_n_2(vtkParametricSuperToroid* sself);
extern "C" vtkParametricTorus * vtkParametricTorus_new () ;
extern "C" void vtkParametricTorus_destructor (vtkParametricTorus * sself) ;
extern "C" void vtk_parametric_torus_set_ring_radius(vtkParametricTorus* sself, double _arg);
extern "C" double vtk_parametric_torus_get_ring_radius(vtkParametricTorus* sself);
extern "C" void vtk_parametric_torus_set_cross_section_radius(vtkParametricTorus* sself, double _arg);
extern "C" double vtk_parametric_torus_get_cross_section_radius(vtkParametricTorus* sself);
extern "C" int vtk_parametric_torus_get_dimension(vtkParametricTorus* sself);
