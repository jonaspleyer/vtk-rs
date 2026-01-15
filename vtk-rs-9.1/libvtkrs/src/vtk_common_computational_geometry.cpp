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
extern "C" vtkNew < vtkCardinalSpline > vtkCardinalSpline_new () {return vtkNew < vtkCardinalSpline > () ;}
extern "C" void vtkCardinalSpline_destructor (vtkNew < vtkCardinalSpline > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCardinalSpline_get_ptr (vtkNew < vtkCardinalSpline > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkKochanekSpline > vtkKochanekSpline_new () {return vtkNew < vtkKochanekSpline > () ;}
extern "C" void vtkKochanekSpline_destructor (vtkNew < vtkKochanekSpline > sself) {sself . Reset () ; return ;}
extern "C" void * vtkKochanekSpline_get_ptr (vtkNew < vtkKochanekSpline > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricBohemianDome > vtkParametricBohemianDome_new () {return vtkNew < vtkParametricBohemianDome > () ;}
extern "C" void vtkParametricBohemianDome_destructor (vtkNew < vtkParametricBohemianDome > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricBohemianDome_get_ptr (vtkNew < vtkParametricBohemianDome > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricBour > vtkParametricBour_new () {return vtkNew < vtkParametricBour > () ;}
extern "C" void vtkParametricBour_destructor (vtkNew < vtkParametricBour > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricBour_get_ptr (vtkNew < vtkParametricBour > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricBoy > vtkParametricBoy_new () {return vtkNew < vtkParametricBoy > () ;}
extern "C" void vtkParametricBoy_destructor (vtkNew < vtkParametricBoy > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricBoy_get_ptr (vtkNew < vtkParametricBoy > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricCatalanMinimal > vtkParametricCatalanMinimal_new () {return vtkNew < vtkParametricCatalanMinimal > () ;}
extern "C" void vtkParametricCatalanMinimal_destructor (vtkNew < vtkParametricCatalanMinimal > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricCatalanMinimal_get_ptr (vtkNew < vtkParametricCatalanMinimal > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricConicSpiral > vtkParametricConicSpiral_new () {return vtkNew < vtkParametricConicSpiral > () ;}
extern "C" void vtkParametricConicSpiral_destructor (vtkNew < vtkParametricConicSpiral > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricConicSpiral_get_ptr (vtkNew < vtkParametricConicSpiral > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricCrossCap > vtkParametricCrossCap_new () {return vtkNew < vtkParametricCrossCap > () ;}
extern "C" void vtkParametricCrossCap_destructor (vtkNew < vtkParametricCrossCap > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricCrossCap_get_ptr (vtkNew < vtkParametricCrossCap > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricDini > vtkParametricDini_new () {return vtkNew < vtkParametricDini > () ;}
extern "C" void vtkParametricDini_destructor (vtkNew < vtkParametricDini > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricDini_get_ptr (vtkNew < vtkParametricDini > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricEllipsoid > vtkParametricEllipsoid_new () {return vtkNew < vtkParametricEllipsoid > () ;}
extern "C" void vtkParametricEllipsoid_destructor (vtkNew < vtkParametricEllipsoid > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricEllipsoid_get_ptr (vtkNew < vtkParametricEllipsoid > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricEnneper > vtkParametricEnneper_new () {return vtkNew < vtkParametricEnneper > () ;}
extern "C" void vtkParametricEnneper_destructor (vtkNew < vtkParametricEnneper > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricEnneper_get_ptr (vtkNew < vtkParametricEnneper > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricFigure8Klein > vtkParametricFigure8Klein_new () {return vtkNew < vtkParametricFigure8Klein > () ;}
extern "C" void vtkParametricFigure8Klein_destructor (vtkNew < vtkParametricFigure8Klein > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricFigure8Klein_get_ptr (vtkNew < vtkParametricFigure8Klein > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricHenneberg > vtkParametricHenneberg_new () {return vtkNew < vtkParametricHenneberg > () ;}
extern "C" void vtkParametricHenneberg_destructor (vtkNew < vtkParametricHenneberg > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricHenneberg_get_ptr (vtkNew < vtkParametricHenneberg > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricKlein > vtkParametricKlein_new () {return vtkNew < vtkParametricKlein > () ;}
extern "C" void vtkParametricKlein_destructor (vtkNew < vtkParametricKlein > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricKlein_get_ptr (vtkNew < vtkParametricKlein > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricKuen > vtkParametricKuen_new () {return vtkNew < vtkParametricKuen > () ;}
extern "C" void vtkParametricKuen_destructor (vtkNew < vtkParametricKuen > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricKuen_get_ptr (vtkNew < vtkParametricKuen > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricMobius > vtkParametricMobius_new () {return vtkNew < vtkParametricMobius > () ;}
extern "C" void vtkParametricMobius_destructor (vtkNew < vtkParametricMobius > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricMobius_get_ptr (vtkNew < vtkParametricMobius > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricPluckerConoid > vtkParametricPluckerConoid_new () {return vtkNew < vtkParametricPluckerConoid > () ;}
extern "C" void vtkParametricPluckerConoid_destructor (vtkNew < vtkParametricPluckerConoid > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricPluckerConoid_get_ptr (vtkNew < vtkParametricPluckerConoid > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricPseudosphere > vtkParametricPseudosphere_new () {return vtkNew < vtkParametricPseudosphere > () ;}
extern "C" void vtkParametricPseudosphere_destructor (vtkNew < vtkParametricPseudosphere > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricPseudosphere_get_ptr (vtkNew < vtkParametricPseudosphere > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricRandomHills > vtkParametricRandomHills_new () {return vtkNew < vtkParametricRandomHills > () ;}
extern "C" void vtkParametricRandomHills_destructor (vtkNew < vtkParametricRandomHills > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricRandomHills_get_ptr (vtkNew < vtkParametricRandomHills > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricRoman > vtkParametricRoman_new () {return vtkNew < vtkParametricRoman > () ;}
extern "C" void vtkParametricRoman_destructor (vtkNew < vtkParametricRoman > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricRoman_get_ptr (vtkNew < vtkParametricRoman > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricSpline > vtkParametricSpline_new () {return vtkNew < vtkParametricSpline > () ;}
extern "C" void vtkParametricSpline_destructor (vtkNew < vtkParametricSpline > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricSpline_get_ptr (vtkNew < vtkParametricSpline > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricSuperEllipsoid > vtkParametricSuperEllipsoid_new () {return vtkNew < vtkParametricSuperEllipsoid > () ;}
extern "C" void vtkParametricSuperEllipsoid_destructor (vtkNew < vtkParametricSuperEllipsoid > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricSuperEllipsoid_get_ptr (vtkNew < vtkParametricSuperEllipsoid > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricSuperToroid > vtkParametricSuperToroid_new () {return vtkNew < vtkParametricSuperToroid > () ;}
extern "C" void vtkParametricSuperToroid_destructor (vtkNew < vtkParametricSuperToroid > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricSuperToroid_get_ptr (vtkNew < vtkParametricSuperToroid > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkParametricTorus > vtkParametricTorus_new () {return vtkNew < vtkParametricTorus > () ;}
extern "C" void vtkParametricTorus_destructor (vtkNew < vtkParametricTorus > sself) {sself . Reset () ; return ;}
extern "C" void * vtkParametricTorus_get_ptr (vtkNew < vtkParametricTorus > sself) {return sself . GetPointer () ;}
