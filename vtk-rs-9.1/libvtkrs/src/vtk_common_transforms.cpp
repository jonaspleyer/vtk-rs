// Include header file
#include<vtk_common_transforms.h>

// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkAbstractTransform.h>
#include<vtkCylindricalTransform.h>
#include<vtkGeneralTransform.h>
#include<vtkHomogeneousTransform.h>
#include<vtkIdentityTransform.h>
#include<vtkLandmarkTransform.h>
#include<vtkLinearTransform.h>
#include<vtkMatrixToHomogeneousTransform.h>
#include<vtkMatrixToLinearTransform.h>
#include<vtkPerspectiveTransform.h>
#include<vtkSphericalTransform.h>
#include<vtkThinPlateSplineTransform.h>
#include<vtkTransform.h>
#include<vtkTransform2D.h>
#include<vtkTransformCollection.h>
#include<vtkAbstractTransform.h>
#include<vtkAbstractTransform.h>
#include<vtkAbstractTransform.h>
#include<vtkWarpTransform.h>

// Implement declared functions
extern "C" vtkNew < vtkCylindricalTransform > vtkCylindricalTransform_new () {return vtkNew < vtkCylindricalTransform > () ;}
extern "C" void vtkCylindricalTransform_destructor (vtkNew < vtkCylindricalTransform > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCylindricalTransform_get_ptr (vtkNew < vtkCylindricalTransform > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkGeneralTransform > vtkGeneralTransform_new () {return vtkNew < vtkGeneralTransform > () ;}
extern "C" void vtkGeneralTransform_destructor (vtkNew < vtkGeneralTransform > sself) {sself . Reset () ; return ;}
extern "C" void * vtkGeneralTransform_get_ptr (vtkNew < vtkGeneralTransform > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkIdentityTransform > vtkIdentityTransform_new () {return vtkNew < vtkIdentityTransform > () ;}
extern "C" void vtkIdentityTransform_destructor (vtkNew < vtkIdentityTransform > sself) {sself . Reset () ; return ;}
extern "C" void * vtkIdentityTransform_get_ptr (vtkNew < vtkIdentityTransform > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkLandmarkTransform > vtkLandmarkTransform_new () {return vtkNew < vtkLandmarkTransform > () ;}
extern "C" void vtkLandmarkTransform_destructor (vtkNew < vtkLandmarkTransform > sself) {sself . Reset () ; return ;}
extern "C" void * vtkLandmarkTransform_get_ptr (vtkNew < vtkLandmarkTransform > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMatrixToHomogeneousTransform > vtkMatrixToHomogeneousTransform_new () {return vtkNew < vtkMatrixToHomogeneousTransform > () ;}
extern "C" void vtkMatrixToHomogeneousTransform_destructor (vtkNew < vtkMatrixToHomogeneousTransform > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMatrixToHomogeneousTransform_get_ptr (vtkNew < vtkMatrixToHomogeneousTransform > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMatrixToLinearTransform > vtkMatrixToLinearTransform_new () {return vtkNew < vtkMatrixToLinearTransform > () ;}
extern "C" void vtkMatrixToLinearTransform_destructor (vtkNew < vtkMatrixToLinearTransform > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMatrixToLinearTransform_get_ptr (vtkNew < vtkMatrixToLinearTransform > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPerspectiveTransform > vtkPerspectiveTransform_new () {return vtkNew < vtkPerspectiveTransform > () ;}
extern "C" void vtkPerspectiveTransform_destructor (vtkNew < vtkPerspectiveTransform > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPerspectiveTransform_get_ptr (vtkNew < vtkPerspectiveTransform > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSphericalTransform > vtkSphericalTransform_new () {return vtkNew < vtkSphericalTransform > () ;}
extern "C" void vtkSphericalTransform_destructor (vtkNew < vtkSphericalTransform > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSphericalTransform_get_ptr (vtkNew < vtkSphericalTransform > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkThinPlateSplineTransform > vtkThinPlateSplineTransform_new () {return vtkNew < vtkThinPlateSplineTransform > () ;}
extern "C" void vtkThinPlateSplineTransform_destructor (vtkNew < vtkThinPlateSplineTransform > sself) {sself . Reset () ; return ;}
extern "C" void * vtkThinPlateSplineTransform_get_ptr (vtkNew < vtkThinPlateSplineTransform > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTransform > vtkTransform_new () {return vtkNew < vtkTransform > () ;}
extern "C" void vtkTransform_destructor (vtkNew < vtkTransform > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTransform_get_ptr (vtkNew < vtkTransform > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTransform2D > vtkTransform2D_new () {return vtkNew < vtkTransform2D > () ;}
extern "C" void vtkTransform2D_destructor (vtkNew < vtkTransform2D > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTransform2D_get_ptr (vtkNew < vtkTransform2D > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTransformCollection > vtkTransformCollection_new () {return vtkNew < vtkTransformCollection > () ;}
extern "C" void vtkTransformCollection_destructor (vtkNew < vtkTransformCollection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTransformCollection_get_ptr (vtkNew < vtkTransformCollection > sself) {return sself . GetPointer () ;}
