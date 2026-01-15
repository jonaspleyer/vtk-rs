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
extern "C" vtkNew < vtkCardinalSpline > vtkCardinalSpline_new () ;
extern "C" void vtkCardinalSpline_destructor (vtkNew < vtkCardinalSpline > sself) ;
extern "C" void * vtkCardinalSpline_get_ptr (vtkNew < vtkCardinalSpline > sself) ;
extern "C" vtkNew < vtkKochanekSpline > vtkKochanekSpline_new () ;
extern "C" void vtkKochanekSpline_destructor (vtkNew < vtkKochanekSpline > sself) ;
extern "C" void * vtkKochanekSpline_get_ptr (vtkNew < vtkKochanekSpline > sself) ;
extern "C" vtkNew < vtkParametricBohemianDome > vtkParametricBohemianDome_new () ;
extern "C" void vtkParametricBohemianDome_destructor (vtkNew < vtkParametricBohemianDome > sself) ;
extern "C" void * vtkParametricBohemianDome_get_ptr (vtkNew < vtkParametricBohemianDome > sself) ;
extern "C" vtkNew < vtkParametricBour > vtkParametricBour_new () ;
extern "C" void vtkParametricBour_destructor (vtkNew < vtkParametricBour > sself) ;
extern "C" void * vtkParametricBour_get_ptr (vtkNew < vtkParametricBour > sself) ;
extern "C" vtkNew < vtkParametricBoy > vtkParametricBoy_new () ;
extern "C" void vtkParametricBoy_destructor (vtkNew < vtkParametricBoy > sself) ;
extern "C" void * vtkParametricBoy_get_ptr (vtkNew < vtkParametricBoy > sself) ;
extern "C" vtkNew < vtkParametricCatalanMinimal > vtkParametricCatalanMinimal_new () ;
extern "C" void vtkParametricCatalanMinimal_destructor (vtkNew < vtkParametricCatalanMinimal > sself) ;
extern "C" void * vtkParametricCatalanMinimal_get_ptr (vtkNew < vtkParametricCatalanMinimal > sself) ;
extern "C" vtkNew < vtkParametricConicSpiral > vtkParametricConicSpiral_new () ;
extern "C" void vtkParametricConicSpiral_destructor (vtkNew < vtkParametricConicSpiral > sself) ;
extern "C" void * vtkParametricConicSpiral_get_ptr (vtkNew < vtkParametricConicSpiral > sself) ;
extern "C" vtkNew < vtkParametricCrossCap > vtkParametricCrossCap_new () ;
extern "C" void vtkParametricCrossCap_destructor (vtkNew < vtkParametricCrossCap > sself) ;
extern "C" void * vtkParametricCrossCap_get_ptr (vtkNew < vtkParametricCrossCap > sself) ;
extern "C" vtkNew < vtkParametricDini > vtkParametricDini_new () ;
extern "C" void vtkParametricDini_destructor (vtkNew < vtkParametricDini > sself) ;
extern "C" void * vtkParametricDini_get_ptr (vtkNew < vtkParametricDini > sself) ;
extern "C" vtkNew < vtkParametricEllipsoid > vtkParametricEllipsoid_new () ;
extern "C" void vtkParametricEllipsoid_destructor (vtkNew < vtkParametricEllipsoid > sself) ;
extern "C" void * vtkParametricEllipsoid_get_ptr (vtkNew < vtkParametricEllipsoid > sself) ;
extern "C" vtkNew < vtkParametricEnneper > vtkParametricEnneper_new () ;
extern "C" void vtkParametricEnneper_destructor (vtkNew < vtkParametricEnneper > sself) ;
extern "C" void * vtkParametricEnneper_get_ptr (vtkNew < vtkParametricEnneper > sself) ;
extern "C" vtkNew < vtkParametricFigure8Klein > vtkParametricFigure8Klein_new () ;
extern "C" void vtkParametricFigure8Klein_destructor (vtkNew < vtkParametricFigure8Klein > sself) ;
extern "C" void * vtkParametricFigure8Klein_get_ptr (vtkNew < vtkParametricFigure8Klein > sself) ;
extern "C" vtkNew < vtkParametricHenneberg > vtkParametricHenneberg_new () ;
extern "C" void vtkParametricHenneberg_destructor (vtkNew < vtkParametricHenneberg > sself) ;
extern "C" void * vtkParametricHenneberg_get_ptr (vtkNew < vtkParametricHenneberg > sself) ;
extern "C" vtkNew < vtkParametricKlein > vtkParametricKlein_new () ;
extern "C" void vtkParametricKlein_destructor (vtkNew < vtkParametricKlein > sself) ;
extern "C" void * vtkParametricKlein_get_ptr (vtkNew < vtkParametricKlein > sself) ;
extern "C" vtkNew < vtkParametricKuen > vtkParametricKuen_new () ;
extern "C" void vtkParametricKuen_destructor (vtkNew < vtkParametricKuen > sself) ;
extern "C" void * vtkParametricKuen_get_ptr (vtkNew < vtkParametricKuen > sself) ;
extern "C" vtkNew < vtkParametricMobius > vtkParametricMobius_new () ;
extern "C" void vtkParametricMobius_destructor (vtkNew < vtkParametricMobius > sself) ;
extern "C" void * vtkParametricMobius_get_ptr (vtkNew < vtkParametricMobius > sself) ;
extern "C" vtkNew < vtkParametricPluckerConoid > vtkParametricPluckerConoid_new () ;
extern "C" void vtkParametricPluckerConoid_destructor (vtkNew < vtkParametricPluckerConoid > sself) ;
extern "C" void * vtkParametricPluckerConoid_get_ptr (vtkNew < vtkParametricPluckerConoid > sself) ;
extern "C" vtkNew < vtkParametricPseudosphere > vtkParametricPseudosphere_new () ;
extern "C" void vtkParametricPseudosphere_destructor (vtkNew < vtkParametricPseudosphere > sself) ;
extern "C" void * vtkParametricPseudosphere_get_ptr (vtkNew < vtkParametricPseudosphere > sself) ;
extern "C" vtkNew < vtkParametricRandomHills > vtkParametricRandomHills_new () ;
extern "C" void vtkParametricRandomHills_destructor (vtkNew < vtkParametricRandomHills > sself) ;
extern "C" void * vtkParametricRandomHills_get_ptr (vtkNew < vtkParametricRandomHills > sself) ;
extern "C" vtkNew < vtkParametricRoman > vtkParametricRoman_new () ;
extern "C" void vtkParametricRoman_destructor (vtkNew < vtkParametricRoman > sself) ;
extern "C" void * vtkParametricRoman_get_ptr (vtkNew < vtkParametricRoman > sself) ;
extern "C" vtkNew < vtkParametricSpline > vtkParametricSpline_new () ;
extern "C" void vtkParametricSpline_destructor (vtkNew < vtkParametricSpline > sself) ;
extern "C" void * vtkParametricSpline_get_ptr (vtkNew < vtkParametricSpline > sself) ;
extern "C" vtkNew < vtkParametricSuperEllipsoid > vtkParametricSuperEllipsoid_new () ;
extern "C" void vtkParametricSuperEllipsoid_destructor (vtkNew < vtkParametricSuperEllipsoid > sself) ;
extern "C" void * vtkParametricSuperEllipsoid_get_ptr (vtkNew < vtkParametricSuperEllipsoid > sself) ;
extern "C" vtkNew < vtkParametricSuperToroid > vtkParametricSuperToroid_new () ;
extern "C" void vtkParametricSuperToroid_destructor (vtkNew < vtkParametricSuperToroid > sself) ;
extern "C" void * vtkParametricSuperToroid_get_ptr (vtkNew < vtkParametricSuperToroid > sself) ;
extern "C" vtkNew < vtkParametricTorus > vtkParametricTorus_new () ;
extern "C" void vtkParametricTorus_destructor (vtkNew < vtkParametricTorus > sself) ;
extern "C" void * vtkParametricTorus_get_ptr (vtkNew < vtkParametricTorus > sself) ;
