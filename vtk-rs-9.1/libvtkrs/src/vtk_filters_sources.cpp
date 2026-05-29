// Include header file
#include<vtk_filters_sources.h>

// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkArcSource.h>
#include<vtkArrowSource.h>
#include<vtkButtonSource.h>
#include<vtkCapsuleSource.h>
#include<vtkCellTypeSource.h>
#include<vtkConeSource.h>
#include<vtkCubeSource.h>
#include<vtkCylinderSource.h>
#include<vtkDiagonalMatrixSource.h>
#include<vtkDiskSource.h>
#include<vtkEllipseArcSource.h>
#include<vtkEllipticalButtonSource.h>
#include<vtkFrustumSource.h>
#include<vtkGlyphSource2D.h>
#include<vtkGraphToPolyData.h>
#include<vtkHandleSource.h>
#include<vtkHyperTreeGridSource.h>
#include<vtkLineSource.h>
#include<vtkOutlineCornerFilter.h>
#include<vtkOutlineCornerSource.h>
#include<vtkOutlineSource.h>
#include<vtkParametricFunctionSource.h>
#include<vtkPartitionedDataSetCollectionSource.h>
#include<vtkPartitionedDataSetSource.h>
#include<vtkPlaneSource.h>
#include<vtkPlatonicSolidSource.h>
#include<vtkPointHandleSource.h>
#include<vtkPointSource.h>
#include<vtkPolyLineSource.h>
#include<vtkPolyPointSource.h>
#include<vtkProgrammableDataObjectSource.h>
#include<vtkProgrammableSource.h>
#include<vtkRandomHyperTreeGridSource.h>
#include<vtkRectangularButtonSource.h>
#include<vtkRegularPolygonSource.h>
#include<vtkSelectionSource.h>
#include<vtkSphereSource.h>
#include<vtkSuperquadricSource.h>
#include<vtkTessellatedBoxSource.h>
#include<vtkTextSource.h>
#include<vtkTexturedSphereSource.h>
#include<vtkUniformHyperTreeGridSource.h>

// Implement declared functions
extern "C" vtkArcSource * vtkArcSource_new () {return vtkArcSource :: New () ;}
extern "C" void vtkArcSource_destructor (vtkArcSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_arc_source_set_point_1(vtkArcSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetPoint1(_arg1, _arg2, _arg3); }
extern "C" void vtk_arc_source_set_point_2(vtkArcSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetPoint2(_arg1, _arg2, _arg3); }
extern "C" void vtk_arc_source_set_center(vtkArcSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetCenter(_arg1, _arg2, _arg3); }
extern "C" void vtk_arc_source_set_normal(vtkArcSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetNormal(_arg1, _arg2, _arg3); }
extern "C" void vtk_arc_source_set_polar_vector(vtkArcSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetPolarVector(_arg1, _arg2, _arg3); }
extern "C" void vtk_arc_source_set_angle(vtkArcSource* sself, double _arg) { sself->SetAngle(_arg); }
extern "C" double vtk_arc_source_get_angle_min_value(vtkArcSource* sself) { return sself->GetAngleMinValue(); }
extern "C" double vtk_arc_source_get_angle_max_value(vtkArcSource* sself) { return sself->GetAngleMaxValue(); }
extern "C" double vtk_arc_source_get_angle(vtkArcSource* sself) { return sself->GetAngle(); }
extern "C" void vtk_arc_source_set_resolution(vtkArcSource* sself, int _arg) { sself->SetResolution(_arg); }
extern "C" int vtk_arc_source_get_resolution_min_value(vtkArcSource* sself) { return sself->GetResolutionMinValue(); }
extern "C" int vtk_arc_source_get_resolution_max_value(vtkArcSource* sself) { return sself->GetResolutionMaxValue(); }
extern "C" int vtk_arc_source_get_resolution(vtkArcSource* sself) { return sself->GetResolution(); }
extern "C" void vtk_arc_source_set_negative(vtkArcSource* sself, bool _arg) { sself->SetNegative(_arg); }
extern "C" bool vtk_arc_source_get_negative(vtkArcSource* sself) { return sself->GetNegative(); }
extern "C" void vtk_arc_source_negative_on(vtkArcSource* sself) { sself->NegativeOn(); }
extern "C" void vtk_arc_source_negative_off(vtkArcSource* sself) { sself->NegativeOff(); }
extern "C" void vtk_arc_source_set_use_normal_and_angle(vtkArcSource* sself, bool _arg) { sself->SetUseNormalAndAngle(_arg); }
extern "C" bool vtk_arc_source_get_use_normal_and_angle(vtkArcSource* sself) { return sself->GetUseNormalAndAngle(); }
extern "C" void vtk_arc_source_use_normal_and_angle_on(vtkArcSource* sself) { sself->UseNormalAndAngleOn(); }
extern "C" void vtk_arc_source_use_normal_and_angle_off(vtkArcSource* sself) { sself->UseNormalAndAngleOff(); }
extern "C" void vtk_arc_source_set_output_points_precision(vtkArcSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_arc_source_get_output_points_precision(vtkArcSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkArrowSource * vtkArrowSource_new () {return vtkArrowSource :: New () ;}
extern "C" void vtkArrowSource_destructor (vtkArrowSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_arrow_source_set_tip_length(vtkArrowSource* sself, double _arg) { sself->SetTipLength(_arg); }
extern "C" double vtk_arrow_source_get_tip_length_min_value(vtkArrowSource* sself) { return sself->GetTipLengthMinValue(); }
extern "C" double vtk_arrow_source_get_tip_length_max_value(vtkArrowSource* sself) { return sself->GetTipLengthMaxValue(); }
extern "C" double vtk_arrow_source_get_tip_length(vtkArrowSource* sself) { return sself->GetTipLength(); }
extern "C" void vtk_arrow_source_set_tip_radius(vtkArrowSource* sself, double _arg) { sself->SetTipRadius(_arg); }
extern "C" double vtk_arrow_source_get_tip_radius_min_value(vtkArrowSource* sself) { return sself->GetTipRadiusMinValue(); }
extern "C" double vtk_arrow_source_get_tip_radius_max_value(vtkArrowSource* sself) { return sself->GetTipRadiusMaxValue(); }
extern "C" double vtk_arrow_source_get_tip_radius(vtkArrowSource* sself) { return sself->GetTipRadius(); }
extern "C" void vtk_arrow_source_set_tip_resolution(vtkArrowSource* sself, int _arg) { sself->SetTipResolution(_arg); }
extern "C" int vtk_arrow_source_get_tip_resolution_min_value(vtkArrowSource* sself) { return sself->GetTipResolutionMinValue(); }
extern "C" int vtk_arrow_source_get_tip_resolution_max_value(vtkArrowSource* sself) { return sself->GetTipResolutionMaxValue(); }
extern "C" int vtk_arrow_source_get_tip_resolution(vtkArrowSource* sself) { return sself->GetTipResolution(); }
extern "C" void vtk_arrow_source_set_shaft_radius(vtkArrowSource* sself, double _arg) { sself->SetShaftRadius(_arg); }
extern "C" double vtk_arrow_source_get_shaft_radius_min_value(vtkArrowSource* sself) { return sself->GetShaftRadiusMinValue(); }
extern "C" double vtk_arrow_source_get_shaft_radius_max_value(vtkArrowSource* sself) { return sself->GetShaftRadiusMaxValue(); }
extern "C" double vtk_arrow_source_get_shaft_radius(vtkArrowSource* sself) { return sself->GetShaftRadius(); }
extern "C" void vtk_arrow_source_set_shaft_resolution(vtkArrowSource* sself, int _arg) { sself->SetShaftResolution(_arg); }
extern "C" int vtk_arrow_source_get_shaft_resolution_min_value(vtkArrowSource* sself) { return sself->GetShaftResolutionMinValue(); }
extern "C" int vtk_arrow_source_get_shaft_resolution_max_value(vtkArrowSource* sself) { return sself->GetShaftResolutionMaxValue(); }
extern "C" int vtk_arrow_source_get_shaft_resolution(vtkArrowSource* sself) { return sself->GetShaftResolution(); }
extern "C" void vtk_arrow_source_invert_on(vtkArrowSource* sself) { sself->InvertOn(); }
extern "C" void vtk_arrow_source_invert_off(vtkArrowSource* sself) { sself->InvertOff(); }
extern "C" void vtk_arrow_source_set_invert(vtkArrowSource* sself, bool _arg) { sself->SetInvert(_arg); }
extern "C" bool vtk_arrow_source_get_invert(vtkArrowSource* sself) { return sself->GetInvert(); }
extern "C" void vtk_arrow_source_set_arrow_origin_to_default(vtkArrowSource* sself) { sself->SetArrowOriginToDefault(); }
extern "C" void vtk_arrow_source_set_arrow_origin_to_center(vtkArrowSource* sself) { sself->SetArrowOriginToCenter(); }
extern "C" vtkCapsuleSource * vtkCapsuleSource_new () {return vtkCapsuleSource :: New () ;}
extern "C" void vtkCapsuleSource_destructor (vtkCapsuleSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_capsule_source_set_radius(vtkCapsuleSource* sself, double _arg) { sself->SetRadius(_arg); }
extern "C" double vtk_capsule_source_get_radius_min_value(vtkCapsuleSource* sself) { return sself->GetRadiusMinValue(); }
extern "C" double vtk_capsule_source_get_radius_max_value(vtkCapsuleSource* sself) { return sself->GetRadiusMaxValue(); }
extern "C" double vtk_capsule_source_get_radius(vtkCapsuleSource* sself) { return sself->GetRadius(); }
extern "C" void vtk_capsule_source_set_center(vtkCapsuleSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetCenter(_arg1, _arg2, _arg3); }
extern "C" void vtk_capsule_source_set_cylinder_length(vtkCapsuleSource* sself, double _arg) { sself->SetCylinderLength(_arg); }
extern "C" double vtk_capsule_source_get_cylinder_length_min_value(vtkCapsuleSource* sself) { return sself->GetCylinderLengthMinValue(); }
extern "C" double vtk_capsule_source_get_cylinder_length_max_value(vtkCapsuleSource* sself) { return sself->GetCylinderLengthMaxValue(); }
extern "C" double vtk_capsule_source_get_cylinder_length(vtkCapsuleSource* sself) { return sself->GetCylinderLength(); }
extern "C" void vtk_capsule_source_set_theta_resolution(vtkCapsuleSource* sself, int _arg) { sself->SetThetaResolution(_arg); }
extern "C" int vtk_capsule_source_get_theta_resolution_min_value(vtkCapsuleSource* sself) { return sself->GetThetaResolutionMinValue(); }
extern "C" int vtk_capsule_source_get_theta_resolution_max_value(vtkCapsuleSource* sself) { return sself->GetThetaResolutionMaxValue(); }
extern "C" int vtk_capsule_source_get_theta_resolution(vtkCapsuleSource* sself) { return sself->GetThetaResolution(); }
extern "C" void vtk_capsule_source_set_phi_resolution(vtkCapsuleSource* sself, int _arg) { sself->SetPhiResolution(_arg); }
extern "C" int vtk_capsule_source_get_phi_resolution_min_value(vtkCapsuleSource* sself) { return sself->GetPhiResolutionMinValue(); }
extern "C" int vtk_capsule_source_get_phi_resolution_max_value(vtkCapsuleSource* sself) { return sself->GetPhiResolutionMaxValue(); }
extern "C" int vtk_capsule_source_get_phi_resolution(vtkCapsuleSource* sself) { return sself->GetPhiResolution(); }
extern "C" void vtk_capsule_source_set_lat_long_tessellation(vtkCapsuleSource* sself, int _arg) { sself->SetLatLongTessellation(_arg); }
extern "C" int vtk_capsule_source_get_lat_long_tessellation(vtkCapsuleSource* sself) { return sself->GetLatLongTessellation(); }
extern "C" void vtk_capsule_source_lat_long_tessellation_on(vtkCapsuleSource* sself) { sself->LatLongTessellationOn(); }
extern "C" void vtk_capsule_source_lat_long_tessellation_off(vtkCapsuleSource* sself) { sself->LatLongTessellationOff(); }
extern "C" void vtk_capsule_source_set_output_points_precision(vtkCapsuleSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_capsule_source_get_output_points_precision(vtkCapsuleSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkCellTypeSource * vtkCellTypeSource_new () {return vtkCellTypeSource :: New () ;}
extern "C" void vtkCellTypeSource_destructor (vtkCellTypeSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_cell_type_source_set_cell_type(vtkCellTypeSource* sself, int cellType) { sself->SetCellType(cellType); }
extern "C" int vtk_cell_type_source_get_cell_type(vtkCellTypeSource* sself) { return sself->GetCellType(); }
extern "C" void vtk_cell_type_source_set_cell_order(vtkCellTypeSource* sself, int _arg) { sself->SetCellOrder(_arg); }
extern "C" int vtk_cell_type_source_get_cell_order(vtkCellTypeSource* sself) { return sself->GetCellOrder(); }
extern "C" void vtk_cell_type_source_set_complete_quadratic_simplicial_elements(vtkCellTypeSource* sself, bool _arg) { sself->SetCompleteQuadraticSimplicialElements(_arg); }
extern "C" bool vtk_cell_type_source_get_complete_quadratic_simplicial_elements(vtkCellTypeSource* sself) { return sself->GetCompleteQuadraticSimplicialElements(); }
extern "C" void vtk_cell_type_source_complete_quadratic_simplicial_elements_on(vtkCellTypeSource* sself) { sself->CompleteQuadraticSimplicialElementsOn(); }
extern "C" void vtk_cell_type_source_complete_quadratic_simplicial_elements_off(vtkCellTypeSource* sself) { sself->CompleteQuadraticSimplicialElementsOff(); }
extern "C" void vtk_cell_type_source_set_polynomial_field_order(vtkCellTypeSource* sself, int _arg) { sself->SetPolynomialFieldOrder(_arg); }
extern "C" int vtk_cell_type_source_get_polynomial_field_order_min_value(vtkCellTypeSource* sself) { return sself->GetPolynomialFieldOrderMinValue(); }
extern "C" int vtk_cell_type_source_get_polynomial_field_order_max_value(vtkCellTypeSource* sself) { return sself->GetPolynomialFieldOrderMaxValue(); }
extern "C" int vtk_cell_type_source_get_polynomial_field_order(vtkCellTypeSource* sself) { return sself->GetPolynomialFieldOrder(); }
extern "C" int vtk_cell_type_source_get_cell_dimension(vtkCellTypeSource* sself) { return sself->GetCellDimension(); }
extern "C" void vtk_cell_type_source_set_output_precision(vtkCellTypeSource* sself, int _arg) { sself->SetOutputPrecision(_arg); }
extern "C" int vtk_cell_type_source_get_output_precision_min_value(vtkCellTypeSource* sself) { return sself->GetOutputPrecisionMinValue(); }
extern "C" int vtk_cell_type_source_get_output_precision_max_value(vtkCellTypeSource* sself) { return sself->GetOutputPrecisionMaxValue(); }
extern "C" int vtk_cell_type_source_get_output_precision(vtkCellTypeSource* sself) { return sself->GetOutputPrecision(); }
extern "C" vtkConeSource * vtkConeSource_new () {return vtkConeSource :: New () ;}
extern "C" void vtkConeSource_destructor (vtkConeSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_cone_source_set_height(vtkConeSource* sself, double _arg) { sself->SetHeight(_arg); }
extern "C" double vtk_cone_source_get_height_min_value(vtkConeSource* sself) { return sself->GetHeightMinValue(); }
extern "C" double vtk_cone_source_get_height_max_value(vtkConeSource* sself) { return sself->GetHeightMaxValue(); }
extern "C" double vtk_cone_source_get_height(vtkConeSource* sself) { return sself->GetHeight(); }
extern "C" void vtk_cone_source_set_radius(vtkConeSource* sself, double _arg) { sself->SetRadius(_arg); }
extern "C" double vtk_cone_source_get_radius_min_value(vtkConeSource* sself) { return sself->GetRadiusMinValue(); }
extern "C" double vtk_cone_source_get_radius_max_value(vtkConeSource* sself) { return sself->GetRadiusMaxValue(); }
extern "C" double vtk_cone_source_get_radius(vtkConeSource* sself) { return sself->GetRadius(); }
extern "C" void vtk_cone_source_set_resolution(vtkConeSource* sself, int _arg) { sself->SetResolution(_arg); }
extern "C" int vtk_cone_source_get_resolution_min_value(vtkConeSource* sself) { return sself->GetResolutionMinValue(); }
extern "C" int vtk_cone_source_get_resolution_max_value(vtkConeSource* sself) { return sself->GetResolutionMaxValue(); }
extern "C" int vtk_cone_source_get_resolution(vtkConeSource* sself) { return sself->GetResolution(); }
extern "C" void vtk_cone_source_set_center(vtkConeSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetCenter(_arg1, _arg2, _arg3); }
extern "C" void vtk_cone_source_set_direction(vtkConeSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetDirection(_arg1, _arg2, _arg3); }
extern "C" void vtk_cone_source_set_angle(vtkConeSource* sself, double angle) { sself->SetAngle(angle); }
extern "C" double vtk_cone_source_get_angle(vtkConeSource* sself) { return sself->GetAngle(); }
extern "C" void vtk_cone_source_set_capping(vtkConeSource* sself, int _arg) { sself->SetCapping(_arg); }
extern "C" int vtk_cone_source_get_capping(vtkConeSource* sself) { return sself->GetCapping(); }
extern "C" void vtk_cone_source_capping_on(vtkConeSource* sself) { sself->CappingOn(); }
extern "C" void vtk_cone_source_capping_off(vtkConeSource* sself) { sself->CappingOff(); }
extern "C" void vtk_cone_source_set_output_points_precision(vtkConeSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_cone_source_get_output_points_precision(vtkConeSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkCubeSource * vtkCubeSource_new () {return vtkCubeSource :: New () ;}
extern "C" void vtkCubeSource_destructor (vtkCubeSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_cube_source_set_x_length(vtkCubeSource* sself, double _arg) { sself->SetXLength(_arg); }
extern "C" double vtk_cube_source_get_x_length_min_value(vtkCubeSource* sself) { return sself->GetXLengthMinValue(); }
extern "C" double vtk_cube_source_get_x_length_max_value(vtkCubeSource* sself) { return sself->GetXLengthMaxValue(); }
extern "C" double vtk_cube_source_get_x_length(vtkCubeSource* sself) { return sself->GetXLength(); }
extern "C" void vtk_cube_source_set_y_length(vtkCubeSource* sself, double _arg) { sself->SetYLength(_arg); }
extern "C" double vtk_cube_source_get_y_length_min_value(vtkCubeSource* sself) { return sself->GetYLengthMinValue(); }
extern "C" double vtk_cube_source_get_y_length_max_value(vtkCubeSource* sself) { return sself->GetYLengthMaxValue(); }
extern "C" double vtk_cube_source_get_y_length(vtkCubeSource* sself) { return sself->GetYLength(); }
extern "C" void vtk_cube_source_set_z_length(vtkCubeSource* sself, double _arg) { sself->SetZLength(_arg); }
extern "C" double vtk_cube_source_get_z_length_min_value(vtkCubeSource* sself) { return sself->GetZLengthMinValue(); }
extern "C" double vtk_cube_source_get_z_length_max_value(vtkCubeSource* sself) { return sself->GetZLengthMaxValue(); }
extern "C" double vtk_cube_source_get_z_length(vtkCubeSource* sself) { return sself->GetZLength(); }
extern "C" void vtk_cube_source_set_center(vtkCubeSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetCenter(_arg1, _arg2, _arg3); }
extern "C" void vtk_cube_source_set_bounds(vtkCubeSource* sself, double xMin, double xMax, double yMin, double yMax, double zMin, double zMax) { sself->SetBounds(xMin, xMax, yMin, yMax, zMin, zMax); }
extern "C" void vtk_cube_source_set_output_points_precision(vtkCubeSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_cube_source_get_output_points_precision(vtkCubeSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkCylinderSource * vtkCylinderSource_new () {return vtkCylinderSource :: New () ;}
extern "C" void vtkCylinderSource_destructor (vtkCylinderSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_cylinder_source_set_height(vtkCylinderSource* sself, double _arg) { sself->SetHeight(_arg); }
extern "C" double vtk_cylinder_source_get_height_min_value(vtkCylinderSource* sself) { return sself->GetHeightMinValue(); }
extern "C" double vtk_cylinder_source_get_height_max_value(vtkCylinderSource* sself) { return sself->GetHeightMaxValue(); }
extern "C" double vtk_cylinder_source_get_height(vtkCylinderSource* sself) { return sself->GetHeight(); }
extern "C" void vtk_cylinder_source_set_radius(vtkCylinderSource* sself, double _arg) { sself->SetRadius(_arg); }
extern "C" double vtk_cylinder_source_get_radius_min_value(vtkCylinderSource* sself) { return sself->GetRadiusMinValue(); }
extern "C" double vtk_cylinder_source_get_radius_max_value(vtkCylinderSource* sself) { return sself->GetRadiusMaxValue(); }
extern "C" double vtk_cylinder_source_get_radius(vtkCylinderSource* sself) { return sself->GetRadius(); }
extern "C" void vtk_cylinder_source_set_center(vtkCylinderSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetCenter(_arg1, _arg2, _arg3); }
extern "C" void vtk_cylinder_source_set_resolution(vtkCylinderSource* sself, int _arg) { sself->SetResolution(_arg); }
extern "C" int vtk_cylinder_source_get_resolution_min_value(vtkCylinderSource* sself) { return sself->GetResolutionMinValue(); }
extern "C" int vtk_cylinder_source_get_resolution_max_value(vtkCylinderSource* sself) { return sself->GetResolutionMaxValue(); }
extern "C" int vtk_cylinder_source_get_resolution(vtkCylinderSource* sself) { return sself->GetResolution(); }
extern "C" void vtk_cylinder_source_set_capping(vtkCylinderSource* sself, int _arg) { sself->SetCapping(_arg); }
extern "C" int vtk_cylinder_source_get_capping(vtkCylinderSource* sself) { return sself->GetCapping(); }
extern "C" void vtk_cylinder_source_capping_on(vtkCylinderSource* sself) { sself->CappingOn(); }
extern "C" void vtk_cylinder_source_capping_off(vtkCylinderSource* sself) { sself->CappingOff(); }
extern "C" void vtk_cylinder_source_set_output_points_precision(vtkCylinderSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_cylinder_source_get_output_points_precision(vtkCylinderSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkDiagonalMatrixSource * vtkDiagonalMatrixSource_new () {return vtkDiagonalMatrixSource :: New () ;}
extern "C" void vtkDiagonalMatrixSource_destructor (vtkDiagonalMatrixSource * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_diagonal_matrix_source_get_array_type(vtkDiagonalMatrixSource* sself) { return sself->GetArrayType(); }
extern "C" void vtk_diagonal_matrix_source_set_array_type(vtkDiagonalMatrixSource* sself, int _arg) { sself->SetArrayType(_arg); }
extern "C" long long vtk_diagonal_matrix_source_get_extents(vtkDiagonalMatrixSource* sself) { return sself->GetExtents(); }
extern "C" void vtk_diagonal_matrix_source_set_extents(vtkDiagonalMatrixSource* sself, long long _arg) { sself->SetExtents(_arg); }
extern "C" double vtk_diagonal_matrix_source_get_diagonal(vtkDiagonalMatrixSource* sself) { return sself->GetDiagonal(); }
extern "C" void vtk_diagonal_matrix_source_set_diagonal(vtkDiagonalMatrixSource* sself, double _arg) { sself->SetDiagonal(_arg); }
extern "C" double vtk_diagonal_matrix_source_get_super_diagonal(vtkDiagonalMatrixSource* sself) { return sself->GetSuperDiagonal(); }
extern "C" void vtk_diagonal_matrix_source_set_super_diagonal(vtkDiagonalMatrixSource* sself, double _arg) { sself->SetSuperDiagonal(_arg); }
extern "C" double vtk_diagonal_matrix_source_get_sub_diagonal(vtkDiagonalMatrixSource* sself) { return sself->GetSubDiagonal(); }
extern "C" void vtk_diagonal_matrix_source_set_sub_diagonal(vtkDiagonalMatrixSource* sself, double _arg) { sself->SetSubDiagonal(_arg); }
extern "C" void vtk_diagonal_matrix_source_set_row_label(vtkDiagonalMatrixSource* sself, const char* _arg) { sself->SetRowLabel(_arg); }
extern "C" void vtk_diagonal_matrix_source_set_column_label(vtkDiagonalMatrixSource* sself, const char* _arg) { sself->SetColumnLabel(_arg); }
extern "C" vtkDiskSource * vtkDiskSource_new () {return vtkDiskSource :: New () ;}
extern "C" void vtkDiskSource_destructor (vtkDiskSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_disk_source_set_inner_radius(vtkDiskSource* sself, double _arg) { sself->SetInnerRadius(_arg); }
extern "C" double vtk_disk_source_get_inner_radius_min_value(vtkDiskSource* sself) { return sself->GetInnerRadiusMinValue(); }
extern "C" double vtk_disk_source_get_inner_radius_max_value(vtkDiskSource* sself) { return sself->GetInnerRadiusMaxValue(); }
extern "C" double vtk_disk_source_get_inner_radius(vtkDiskSource* sself) { return sself->GetInnerRadius(); }
extern "C" void vtk_disk_source_set_outer_radius(vtkDiskSource* sself, double _arg) { sself->SetOuterRadius(_arg); }
extern "C" double vtk_disk_source_get_outer_radius_min_value(vtkDiskSource* sself) { return sself->GetOuterRadiusMinValue(); }
extern "C" double vtk_disk_source_get_outer_radius_max_value(vtkDiskSource* sself) { return sself->GetOuterRadiusMaxValue(); }
extern "C" double vtk_disk_source_get_outer_radius(vtkDiskSource* sself) { return sself->GetOuterRadius(); }
extern "C" void vtk_disk_source_set_radial_resolution(vtkDiskSource* sself, int _arg) { sself->SetRadialResolution(_arg); }
extern "C" int vtk_disk_source_get_radial_resolution_min_value(vtkDiskSource* sself) { return sself->GetRadialResolutionMinValue(); }
extern "C" int vtk_disk_source_get_radial_resolution_max_value(vtkDiskSource* sself) { return sself->GetRadialResolutionMaxValue(); }
extern "C" int vtk_disk_source_get_radial_resolution(vtkDiskSource* sself) { return sself->GetRadialResolution(); }
extern "C" void vtk_disk_source_set_circumferential_resolution(vtkDiskSource* sself, int _arg) { sself->SetCircumferentialResolution(_arg); }
extern "C" int vtk_disk_source_get_circumferential_resolution_min_value(vtkDiskSource* sself) { return sself->GetCircumferentialResolutionMinValue(); }
extern "C" int vtk_disk_source_get_circumferential_resolution_max_value(vtkDiskSource* sself) { return sself->GetCircumferentialResolutionMaxValue(); }
extern "C" int vtk_disk_source_get_circumferential_resolution(vtkDiskSource* sself) { return sself->GetCircumferentialResolution(); }
extern "C" void vtk_disk_source_set_output_points_precision(vtkDiskSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_disk_source_get_output_points_precision(vtkDiskSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkEllipseArcSource * vtkEllipseArcSource_new () {return vtkEllipseArcSource :: New () ;}
extern "C" void vtkEllipseArcSource_destructor (vtkEllipseArcSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_ellipse_arc_source_set_center(vtkEllipseArcSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetCenter(_arg1, _arg2, _arg3); }
extern "C" void vtk_ellipse_arc_source_set_normal(vtkEllipseArcSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetNormal(_arg1, _arg2, _arg3); }
extern "C" void vtk_ellipse_arc_source_set_major_radius_vector(vtkEllipseArcSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetMajorRadiusVector(_arg1, _arg2, _arg3); }
extern "C" void vtk_ellipse_arc_source_set_start_angle(vtkEllipseArcSource* sself, double _arg) { sself->SetStartAngle(_arg); }
extern "C" double vtk_ellipse_arc_source_get_start_angle_min_value(vtkEllipseArcSource* sself) { return sself->GetStartAngleMinValue(); }
extern "C" double vtk_ellipse_arc_source_get_start_angle_max_value(vtkEllipseArcSource* sself) { return sself->GetStartAngleMaxValue(); }
extern "C" double vtk_ellipse_arc_source_get_start_angle(vtkEllipseArcSource* sself) { return sself->GetStartAngle(); }
extern "C" void vtk_ellipse_arc_source_set_segment_angle(vtkEllipseArcSource* sself, double _arg) { sself->SetSegmentAngle(_arg); }
extern "C" double vtk_ellipse_arc_source_get_segment_angle_min_value(vtkEllipseArcSource* sself) { return sself->GetSegmentAngleMinValue(); }
extern "C" double vtk_ellipse_arc_source_get_segment_angle_max_value(vtkEllipseArcSource* sself) { return sself->GetSegmentAngleMaxValue(); }
extern "C" double vtk_ellipse_arc_source_get_segment_angle(vtkEllipseArcSource* sself) { return sself->GetSegmentAngle(); }
extern "C" void vtk_ellipse_arc_source_set_resolution(vtkEllipseArcSource* sself, int _arg) { sself->SetResolution(_arg); }
extern "C" int vtk_ellipse_arc_source_get_resolution_min_value(vtkEllipseArcSource* sself) { return sself->GetResolutionMinValue(); }
extern "C" int vtk_ellipse_arc_source_get_resolution_max_value(vtkEllipseArcSource* sself) { return sself->GetResolutionMaxValue(); }
extern "C" int vtk_ellipse_arc_source_get_resolution(vtkEllipseArcSource* sself) { return sself->GetResolution(); }
extern "C" void vtk_ellipse_arc_source_set_close(vtkEllipseArcSource* sself, bool _arg) { sself->SetClose(_arg); }
extern "C" bool vtk_ellipse_arc_source_get_close(vtkEllipseArcSource* sself) { return sself->GetClose(); }
extern "C" void vtk_ellipse_arc_source_close_on(vtkEllipseArcSource* sself) { sself->CloseOn(); }
extern "C" void vtk_ellipse_arc_source_close_off(vtkEllipseArcSource* sself) { sself->CloseOff(); }
extern "C" void vtk_ellipse_arc_source_set_output_points_precision(vtkEllipseArcSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_ellipse_arc_source_get_output_points_precision(vtkEllipseArcSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" void vtk_ellipse_arc_source_set_ratio(vtkEllipseArcSource* sself, double _arg) { sself->SetRatio(_arg); }
extern "C" double vtk_ellipse_arc_source_get_ratio_min_value(vtkEllipseArcSource* sself) { return sself->GetRatioMinValue(); }
extern "C" double vtk_ellipse_arc_source_get_ratio_max_value(vtkEllipseArcSource* sself) { return sself->GetRatioMaxValue(); }
extern "C" double vtk_ellipse_arc_source_get_ratio(vtkEllipseArcSource* sself) { return sself->GetRatio(); }
extern "C" vtkEllipticalButtonSource * vtkEllipticalButtonSource_new () {return vtkEllipticalButtonSource :: New () ;}
extern "C" void vtkEllipticalButtonSource_destructor (vtkEllipticalButtonSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_elliptical_button_source_set_width(vtkEllipticalButtonSource* sself, double _arg) { sself->SetWidth(_arg); }
extern "C" double vtk_elliptical_button_source_get_width_min_value(vtkEllipticalButtonSource* sself) { return sself->GetWidthMinValue(); }
extern "C" double vtk_elliptical_button_source_get_width_max_value(vtkEllipticalButtonSource* sself) { return sself->GetWidthMaxValue(); }
extern "C" double vtk_elliptical_button_source_get_width(vtkEllipticalButtonSource* sself) { return sself->GetWidth(); }
extern "C" void vtk_elliptical_button_source_set_height(vtkEllipticalButtonSource* sself, double _arg) { sself->SetHeight(_arg); }
extern "C" double vtk_elliptical_button_source_get_height_min_value(vtkEllipticalButtonSource* sself) { return sself->GetHeightMinValue(); }
extern "C" double vtk_elliptical_button_source_get_height_max_value(vtkEllipticalButtonSource* sself) { return sself->GetHeightMaxValue(); }
extern "C" double vtk_elliptical_button_source_get_height(vtkEllipticalButtonSource* sself) { return sself->GetHeight(); }
extern "C" void vtk_elliptical_button_source_set_depth(vtkEllipticalButtonSource* sself, double _arg) { sself->SetDepth(_arg); }
extern "C" double vtk_elliptical_button_source_get_depth_min_value(vtkEllipticalButtonSource* sself) { return sself->GetDepthMinValue(); }
extern "C" double vtk_elliptical_button_source_get_depth_max_value(vtkEllipticalButtonSource* sself) { return sself->GetDepthMaxValue(); }
extern "C" double vtk_elliptical_button_source_get_depth(vtkEllipticalButtonSource* sself) { return sself->GetDepth(); }
extern "C" void vtk_elliptical_button_source_set_circumferential_resolution(vtkEllipticalButtonSource* sself, int _arg) { sself->SetCircumferentialResolution(_arg); }
extern "C" int vtk_elliptical_button_source_get_circumferential_resolution_min_value(vtkEllipticalButtonSource* sself) { return sself->GetCircumferentialResolutionMinValue(); }
extern "C" int vtk_elliptical_button_source_get_circumferential_resolution_max_value(vtkEllipticalButtonSource* sself) { return sself->GetCircumferentialResolutionMaxValue(); }
extern "C" int vtk_elliptical_button_source_get_circumferential_resolution(vtkEllipticalButtonSource* sself) { return sself->GetCircumferentialResolution(); }
extern "C" void vtk_elliptical_button_source_set_texture_resolution(vtkEllipticalButtonSource* sself, int _arg) { sself->SetTextureResolution(_arg); }
extern "C" int vtk_elliptical_button_source_get_texture_resolution_min_value(vtkEllipticalButtonSource* sself) { return sself->GetTextureResolutionMinValue(); }
extern "C" int vtk_elliptical_button_source_get_texture_resolution_max_value(vtkEllipticalButtonSource* sself) { return sself->GetTextureResolutionMaxValue(); }
extern "C" int vtk_elliptical_button_source_get_texture_resolution(vtkEllipticalButtonSource* sself) { return sself->GetTextureResolution(); }
extern "C" void vtk_elliptical_button_source_set_shoulder_resolution(vtkEllipticalButtonSource* sself, int _arg) { sself->SetShoulderResolution(_arg); }
extern "C" int vtk_elliptical_button_source_get_shoulder_resolution_min_value(vtkEllipticalButtonSource* sself) { return sself->GetShoulderResolutionMinValue(); }
extern "C" int vtk_elliptical_button_source_get_shoulder_resolution_max_value(vtkEllipticalButtonSource* sself) { return sself->GetShoulderResolutionMaxValue(); }
extern "C" int vtk_elliptical_button_source_get_shoulder_resolution(vtkEllipticalButtonSource* sself) { return sself->GetShoulderResolution(); }
extern "C" void vtk_elliptical_button_source_set_radial_ratio(vtkEllipticalButtonSource* sself, double _arg) { sself->SetRadialRatio(_arg); }
extern "C" double vtk_elliptical_button_source_get_radial_ratio_min_value(vtkEllipticalButtonSource* sself) { return sself->GetRadialRatioMinValue(); }
extern "C" double vtk_elliptical_button_source_get_radial_ratio_max_value(vtkEllipticalButtonSource* sself) { return sself->GetRadialRatioMaxValue(); }
extern "C" double vtk_elliptical_button_source_get_radial_ratio(vtkEllipticalButtonSource* sself) { return sself->GetRadialRatio(); }
extern "C" void vtk_elliptical_button_source_set_output_points_precision(vtkEllipticalButtonSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_elliptical_button_source_get_output_points_precision(vtkEllipticalButtonSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkFrustumSource * vtkFrustumSource_new () {return vtkFrustumSource :: New () ;}
extern "C" void vtkFrustumSource_destructor (vtkFrustumSource * sself) {sself -> Delete () ; return ;}
extern "C" bool vtk_frustum_source_get_show_lines(vtkFrustumSource* sself) { return sself->GetShowLines(); }
extern "C" void vtk_frustum_source_set_show_lines(vtkFrustumSource* sself, bool _arg) { sself->SetShowLines(_arg); }
extern "C" void vtk_frustum_source_show_lines_on(vtkFrustumSource* sself) { sself->ShowLinesOn(); }
extern "C" void vtk_frustum_source_show_lines_off(vtkFrustumSource* sself) { sself->ShowLinesOff(); }
extern "C" double vtk_frustum_source_get_lines_length(vtkFrustumSource* sself) { return sself->GetLinesLength(); }
extern "C" void vtk_frustum_source_set_lines_length(vtkFrustumSource* sself, double _arg) { sself->SetLinesLength(_arg); }
extern "C" unsigned long vtk_frustum_source_get_m_time(vtkFrustumSource* sself) { return sself->GetMTime(); }
extern "C" void vtk_frustum_source_set_output_points_precision(vtkFrustumSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_frustum_source_get_output_points_precision(vtkFrustumSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkGlyphSource2D * vtkGlyphSource2D_new () {return vtkGlyphSource2D :: New () ;}
extern "C" void vtkGlyphSource2D_destructor (vtkGlyphSource2D * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_glyph_source_2_d_set_center(vtkGlyphSource2D* sself, double _arg1, double _arg2, double _arg3) { sself->SetCenter(_arg1, _arg2, _arg3); }
extern "C" void vtk_glyph_source_2_d_set_scale(vtkGlyphSource2D* sself, double _arg) { sself->SetScale(_arg); }
extern "C" double vtk_glyph_source_2_d_get_scale_min_value(vtkGlyphSource2D* sself) { return sself->GetScaleMinValue(); }
extern "C" double vtk_glyph_source_2_d_get_scale_max_value(vtkGlyphSource2D* sself) { return sself->GetScaleMaxValue(); }
extern "C" double vtk_glyph_source_2_d_get_scale(vtkGlyphSource2D* sself) { return sself->GetScale(); }
extern "C" void vtk_glyph_source_2_d_set_scale_2(vtkGlyphSource2D* sself, double _arg) { sself->SetScale2(_arg); }
extern "C" double vtk_glyph_source_2_d_get_scale_2_min_value(vtkGlyphSource2D* sself) { return sself->GetScale2MinValue(); }
extern "C" double vtk_glyph_source_2_d_get_scale_2_max_value(vtkGlyphSource2D* sself) { return sself->GetScale2MaxValue(); }
extern "C" double vtk_glyph_source_2_d_get_scale_2(vtkGlyphSource2D* sself) { return sself->GetScale2(); }
extern "C" void vtk_glyph_source_2_d_set_color(vtkGlyphSource2D* sself, double _arg1, double _arg2, double _arg3) { sself->SetColor(_arg1, _arg2, _arg3); }
extern "C" void vtk_glyph_source_2_d_set_filled(vtkGlyphSource2D* sself, int _arg) { sself->SetFilled(_arg); }
extern "C" int vtk_glyph_source_2_d_get_filled(vtkGlyphSource2D* sself) { return sself->GetFilled(); }
extern "C" void vtk_glyph_source_2_d_filled_on(vtkGlyphSource2D* sself) { sself->FilledOn(); }
extern "C" void vtk_glyph_source_2_d_filled_off(vtkGlyphSource2D* sself) { sself->FilledOff(); }
extern "C" void vtk_glyph_source_2_d_set_dash(vtkGlyphSource2D* sself, int _arg) { sself->SetDash(_arg); }
extern "C" int vtk_glyph_source_2_d_get_dash(vtkGlyphSource2D* sself) { return sself->GetDash(); }
extern "C" void vtk_glyph_source_2_d_dash_on(vtkGlyphSource2D* sself) { sself->DashOn(); }
extern "C" void vtk_glyph_source_2_d_dash_off(vtkGlyphSource2D* sself) { sself->DashOff(); }
extern "C" void vtk_glyph_source_2_d_set_cross(vtkGlyphSource2D* sself, int _arg) { sself->SetCross(_arg); }
extern "C" int vtk_glyph_source_2_d_get_cross(vtkGlyphSource2D* sself) { return sself->GetCross(); }
extern "C" void vtk_glyph_source_2_d_cross_on(vtkGlyphSource2D* sself) { sself->CrossOn(); }
extern "C" void vtk_glyph_source_2_d_cross_off(vtkGlyphSource2D* sself) { sself->CrossOff(); }
extern "C" void vtk_glyph_source_2_d_set_rotation_angle(vtkGlyphSource2D* sself, double _arg) { sself->SetRotationAngle(_arg); }
extern "C" double vtk_glyph_source_2_d_get_rotation_angle(vtkGlyphSource2D* sself) { return sself->GetRotationAngle(); }
extern "C" void vtk_glyph_source_2_d_set_resolution(vtkGlyphSource2D* sself, int _arg) { sself->SetResolution(_arg); }
extern "C" int vtk_glyph_source_2_d_get_resolution_min_value(vtkGlyphSource2D* sself) { return sself->GetResolutionMinValue(); }
extern "C" int vtk_glyph_source_2_d_get_resolution_max_value(vtkGlyphSource2D* sself) { return sself->GetResolutionMaxValue(); }
extern "C" int vtk_glyph_source_2_d_get_resolution(vtkGlyphSource2D* sself) { return sself->GetResolution(); }
extern "C" void vtk_glyph_source_2_d_set_glyph_type(vtkGlyphSource2D* sself, int _arg) { sself->SetGlyphType(_arg); }
extern "C" int vtk_glyph_source_2_d_get_glyph_type_min_value(vtkGlyphSource2D* sself) { return sself->GetGlyphTypeMinValue(); }
extern "C" int vtk_glyph_source_2_d_get_glyph_type_max_value(vtkGlyphSource2D* sself) { return sself->GetGlyphTypeMaxValue(); }
extern "C" int vtk_glyph_source_2_d_get_glyph_type(vtkGlyphSource2D* sself) { return sself->GetGlyphType(); }
extern "C" void vtk_glyph_source_2_d_set_glyph_type_to_none(vtkGlyphSource2D* sself) { sself->SetGlyphTypeToNone(); }
extern "C" void vtk_glyph_source_2_d_set_glyph_type_to_vertex(vtkGlyphSource2D* sself) { sself->SetGlyphTypeToVertex(); }
extern "C" void vtk_glyph_source_2_d_set_glyph_type_to_dash(vtkGlyphSource2D* sself) { sself->SetGlyphTypeToDash(); }
extern "C" void vtk_glyph_source_2_d_set_glyph_type_to_cross(vtkGlyphSource2D* sself) { sself->SetGlyphTypeToCross(); }
extern "C" void vtk_glyph_source_2_d_set_glyph_type_to_thick_cross(vtkGlyphSource2D* sself) { sself->SetGlyphTypeToThickCross(); }
extern "C" void vtk_glyph_source_2_d_set_glyph_type_to_triangle(vtkGlyphSource2D* sself) { sself->SetGlyphTypeToTriangle(); }
extern "C" void vtk_glyph_source_2_d_set_glyph_type_to_square(vtkGlyphSource2D* sself) { sself->SetGlyphTypeToSquare(); }
extern "C" void vtk_glyph_source_2_d_set_glyph_type_to_circle(vtkGlyphSource2D* sself) { sself->SetGlyphTypeToCircle(); }
extern "C" void vtk_glyph_source_2_d_set_glyph_type_to_diamond(vtkGlyphSource2D* sself) { sself->SetGlyphTypeToDiamond(); }
extern "C" void vtk_glyph_source_2_d_set_glyph_type_to_arrow(vtkGlyphSource2D* sself) { sself->SetGlyphTypeToArrow(); }
extern "C" void vtk_glyph_source_2_d_set_glyph_type_to_thick_arrow(vtkGlyphSource2D* sself) { sself->SetGlyphTypeToThickArrow(); }
extern "C" void vtk_glyph_source_2_d_set_glyph_type_to_hooked_arrow(vtkGlyphSource2D* sself) { sself->SetGlyphTypeToHookedArrow(); }
extern "C" void vtk_glyph_source_2_d_set_glyph_type_to_edge_arrow(vtkGlyphSource2D* sself) { sself->SetGlyphTypeToEdgeArrow(); }
extern "C" void vtk_glyph_source_2_d_set_output_points_precision(vtkGlyphSource2D* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_glyph_source_2_d_get_output_points_precision(vtkGlyphSource2D* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkGraphToPolyData * vtkGraphToPolyData_new () {return vtkGraphToPolyData :: New () ;}
extern "C" void vtkGraphToPolyData_destructor (vtkGraphToPolyData * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_graph_to_poly_data_set_edge_glyph_output(vtkGraphToPolyData* sself, bool _arg) { sself->SetEdgeGlyphOutput(_arg); }
extern "C" bool vtk_graph_to_poly_data_get_edge_glyph_output(vtkGraphToPolyData* sself) { return sself->GetEdgeGlyphOutput(); }
extern "C" void vtk_graph_to_poly_data_edge_glyph_output_on(vtkGraphToPolyData* sself) { sself->EdgeGlyphOutputOn(); }
extern "C" void vtk_graph_to_poly_data_edge_glyph_output_off(vtkGraphToPolyData* sself) { sself->EdgeGlyphOutputOff(); }
extern "C" void vtk_graph_to_poly_data_set_edge_glyph_position(vtkGraphToPolyData* sself, double _arg) { sself->SetEdgeGlyphPosition(_arg); }
extern "C" double vtk_graph_to_poly_data_get_edge_glyph_position(vtkGraphToPolyData* sself) { return sself->GetEdgeGlyphPosition(); }
extern "C" vtkHyperTreeGridSource * vtkHyperTreeGridSource_new () {return vtkHyperTreeGridSource :: New () ;}
extern "C" void vtkHyperTreeGridSource_destructor (vtkHyperTreeGridSource * sself) {sself -> Delete () ; return ;}
extern "C" unsigned int vtk_hyper_tree_grid_source_get_maximum_level(vtkHyperTreeGridSource* sself) { return sself->GetMaximumLevel(); }
extern "C" void vtk_hyper_tree_grid_source_set_maximum_level(vtkHyperTreeGridSource* sself, unsigned int levels) { sself->SetMaximumLevel(levels); }
extern "C" unsigned int vtk_hyper_tree_grid_source_get_max_depth(vtkHyperTreeGridSource* sself) { return sself->GetMaxDepth(); }
extern "C" void vtk_hyper_tree_grid_source_set_max_depth(vtkHyperTreeGridSource* sself, unsigned int levels) { sself->SetMaxDepth(levels); }
extern "C" void vtk_hyper_tree_grid_source_set_origin(vtkHyperTreeGridSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetOrigin(_arg1, _arg2, _arg3); }
extern "C" void vtk_hyper_tree_grid_source_set_grid_scale(vtkHyperTreeGridSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetGridScale(_arg1, _arg2, _arg3); }
extern "C" void vtk_hyper_tree_grid_source_set_transposed_root_indexing(vtkHyperTreeGridSource* sself, bool _arg) { sself->SetTransposedRootIndexing(_arg); }
extern "C" bool vtk_hyper_tree_grid_source_get_transposed_root_indexing(vtkHyperTreeGridSource* sself) { return sself->GetTransposedRootIndexing(); }
extern "C" void vtk_hyper_tree_grid_source_set_indexing_mode_to_kji(vtkHyperTreeGridSource* sself) { sself->SetIndexingModeToKJI(); }
extern "C" void vtk_hyper_tree_grid_source_set_indexing_mode_to_ijk(vtkHyperTreeGridSource* sself) { sself->SetIndexingModeToIJK(); }
extern "C" unsigned int vtk_hyper_tree_grid_source_get_orientation(vtkHyperTreeGridSource* sself) { return sself->GetOrientation(); }
extern "C" void vtk_hyper_tree_grid_source_set_branch_factor(vtkHyperTreeGridSource* sself, unsigned int _arg) { sself->SetBranchFactor(_arg); }
extern "C" unsigned int vtk_hyper_tree_grid_source_get_branch_factor_min_value(vtkHyperTreeGridSource* sself) { return sself->GetBranchFactorMinValue(); }
extern "C" unsigned int vtk_hyper_tree_grid_source_get_branch_factor_max_value(vtkHyperTreeGridSource* sself) { return sself->GetBranchFactorMaxValue(); }
extern "C" unsigned int vtk_hyper_tree_grid_source_get_branch_factor(vtkHyperTreeGridSource* sself) { return sself->GetBranchFactor(); }
extern "C" void vtk_hyper_tree_grid_source_set_use_descriptor(vtkHyperTreeGridSource* sself, bool _arg) { sself->SetUseDescriptor(_arg); }
extern "C" bool vtk_hyper_tree_grid_source_get_use_descriptor(vtkHyperTreeGridSource* sself) { return sself->GetUseDescriptor(); }
extern "C" void vtk_hyper_tree_grid_source_use_descriptor_on(vtkHyperTreeGridSource* sself) { sself->UseDescriptorOn(); }
extern "C" void vtk_hyper_tree_grid_source_use_descriptor_off(vtkHyperTreeGridSource* sself) { sself->UseDescriptorOff(); }
extern "C" void vtk_hyper_tree_grid_source_set_use_mask(vtkHyperTreeGridSource* sself, bool _arg) { sself->SetUseMask(_arg); }
extern "C" bool vtk_hyper_tree_grid_source_get_use_mask(vtkHyperTreeGridSource* sself) { return sself->GetUseMask(); }
extern "C" void vtk_hyper_tree_grid_source_use_mask_on(vtkHyperTreeGridSource* sself) { sself->UseMaskOn(); }
extern "C" void vtk_hyper_tree_grid_source_use_mask_off(vtkHyperTreeGridSource* sself) { sself->UseMaskOff(); }
extern "C" void vtk_hyper_tree_grid_source_set_generate_interface_fields(vtkHyperTreeGridSource* sself, bool _arg) { sself->SetGenerateInterfaceFields(_arg); }
extern "C" bool vtk_hyper_tree_grid_source_get_generate_interface_fields(vtkHyperTreeGridSource* sself) { return sself->GetGenerateInterfaceFields(); }
extern "C" void vtk_hyper_tree_grid_source_generate_interface_fields_on(vtkHyperTreeGridSource* sself) { sself->GenerateInterfaceFieldsOn(); }
extern "C" void vtk_hyper_tree_grid_source_generate_interface_fields_off(vtkHyperTreeGridSource* sself) { sself->GenerateInterfaceFieldsOff(); }
extern "C" void vtk_hyper_tree_grid_source_set_descriptor(vtkHyperTreeGridSource* sself, const char* _arg) { sself->SetDescriptor(_arg); }
extern "C" void vtk_hyper_tree_grid_source_set_mask(vtkHyperTreeGridSource* sself, const char* _arg) { sself->SetMask(_arg); }
extern "C" unsigned long vtk_hyper_tree_grid_source_get_m_time(vtkHyperTreeGridSource* sself) { return sself->GetMTime(); }
extern "C" vtkLineSource * vtkLineSource_new () {return vtkLineSource :: New () ;}
extern "C" void vtkLineSource_destructor (vtkLineSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_line_source_set_point_1(vtkLineSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetPoint1(_arg1, _arg2, _arg3); }
extern "C" void vtk_line_source_set_point_2(vtkLineSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetPoint2(_arg1, _arg2, _arg3); }
extern "C" void vtk_line_source_set_use_regular_refinement(vtkLineSource* sself, bool _arg) { sself->SetUseRegularRefinement(_arg); }
extern "C" bool vtk_line_source_get_use_regular_refinement(vtkLineSource* sself) { return sself->GetUseRegularRefinement(); }
extern "C" void vtk_line_source_use_regular_refinement_on(vtkLineSource* sself) { sself->UseRegularRefinementOn(); }
extern "C" void vtk_line_source_use_regular_refinement_off(vtkLineSource* sself) { sself->UseRegularRefinementOff(); }
extern "C" void vtk_line_source_set_resolution(vtkLineSource* sself, int _arg) { sself->SetResolution(_arg); }
extern "C" int vtk_line_source_get_resolution_min_value(vtkLineSource* sself) { return sself->GetResolutionMinValue(); }
extern "C" int vtk_line_source_get_resolution_max_value(vtkLineSource* sself) { return sself->GetResolutionMaxValue(); }
extern "C" int vtk_line_source_get_resolution(vtkLineSource* sself) { return sself->GetResolution(); }
extern "C" void vtk_line_source_set_number_of_refinement_ratios(vtkLineSource* sself, int p0) { sself->SetNumberOfRefinementRatios(p0); }
extern "C" void vtk_line_source_set_refinement_ratio(vtkLineSource* sself, int index, double value) { sself->SetRefinementRatio(index, value); }
extern "C" int vtk_line_source_get_number_of_refinement_ratios(vtkLineSource* sself) { return sself->GetNumberOfRefinementRatios(); }
extern "C" double vtk_line_source_get_refinement_ratio(vtkLineSource* sself, int index) { return sself->GetRefinementRatio(index); }
extern "C" void vtk_line_source_set_output_points_precision(vtkLineSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_line_source_get_output_points_precision(vtkLineSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkOutlineCornerFilter * vtkOutlineCornerFilter_new () {return vtkOutlineCornerFilter :: New () ;}
extern "C" void vtkOutlineCornerFilter_destructor (vtkOutlineCornerFilter * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_outline_corner_filter_set_corner_factor(vtkOutlineCornerFilter* sself, double _arg) { sself->SetCornerFactor(_arg); }
extern "C" double vtk_outline_corner_filter_get_corner_factor_min_value(vtkOutlineCornerFilter* sself) { return sself->GetCornerFactorMinValue(); }
extern "C" double vtk_outline_corner_filter_get_corner_factor_max_value(vtkOutlineCornerFilter* sself) { return sself->GetCornerFactorMaxValue(); }
extern "C" double vtk_outline_corner_filter_get_corner_factor(vtkOutlineCornerFilter* sself) { return sself->GetCornerFactor(); }
extern "C" vtkOutlineCornerSource * vtkOutlineCornerSource_new () {return vtkOutlineCornerSource :: New () ;}
extern "C" void vtkOutlineCornerSource_destructor (vtkOutlineCornerSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_outline_corner_source_set_corner_factor(vtkOutlineCornerSource* sself, double _arg) { sself->SetCornerFactor(_arg); }
extern "C" double vtk_outline_corner_source_get_corner_factor_min_value(vtkOutlineCornerSource* sself) { return sself->GetCornerFactorMinValue(); }
extern "C" double vtk_outline_corner_source_get_corner_factor_max_value(vtkOutlineCornerSource* sself) { return sself->GetCornerFactorMaxValue(); }
extern "C" double vtk_outline_corner_source_get_corner_factor(vtkOutlineCornerSource* sself) { return sself->GetCornerFactor(); }
extern "C" vtkOutlineSource * vtkOutlineSource_new () {return vtkOutlineSource :: New () ;}
extern "C" void vtkOutlineSource_destructor (vtkOutlineSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_outline_source_set_box_type(vtkOutlineSource* sself, int _arg) { sself->SetBoxType(_arg); }
extern "C" int vtk_outline_source_get_box_type(vtkOutlineSource* sself) { return sself->GetBoxType(); }
extern "C" void vtk_outline_source_set_box_type_to_axis_aligned(vtkOutlineSource* sself) { sself->SetBoxTypeToAxisAligned(); }
extern "C" void vtk_outline_source_set_box_type_to_oriented(vtkOutlineSource* sself) { sself->SetBoxTypeToOriented(); }
extern "C" void vtk_outline_source_set_bounds(vtkOutlineSource* sself, double _arg1, double _arg2, double _arg3, double _arg4, double _arg5, double _arg6) { sself->SetBounds(_arg1, _arg2, _arg3, _arg4, _arg5, _arg6); }
extern "C" void vtk_outline_source_set_generate_faces(vtkOutlineSource* sself, int _arg) { sself->SetGenerateFaces(_arg); }
extern "C" void vtk_outline_source_generate_faces_on(vtkOutlineSource* sself) { sself->GenerateFacesOn(); }
extern "C" void vtk_outline_source_generate_faces_off(vtkOutlineSource* sself) { sself->GenerateFacesOff(); }
extern "C" int vtk_outline_source_get_generate_faces(vtkOutlineSource* sself) { return sself->GetGenerateFaces(); }
extern "C" void vtk_outline_source_set_output_points_precision(vtkOutlineSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_outline_source_get_output_points_precision(vtkOutlineSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkParametricFunctionSource * vtkParametricFunctionSource_new () {return vtkParametricFunctionSource :: New () ;}
extern "C" void vtkParametricFunctionSource_destructor (vtkParametricFunctionSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_parametric_function_source_set_u_resolution(vtkParametricFunctionSource* sself, int _arg) { sself->SetUResolution(_arg); }
extern "C" int vtk_parametric_function_source_get_u_resolution_min_value(vtkParametricFunctionSource* sself) { return sself->GetUResolutionMinValue(); }
extern "C" int vtk_parametric_function_source_get_u_resolution_max_value(vtkParametricFunctionSource* sself) { return sself->GetUResolutionMaxValue(); }
extern "C" int vtk_parametric_function_source_get_u_resolution(vtkParametricFunctionSource* sself) { return sself->GetUResolution(); }
extern "C" void vtk_parametric_function_source_set_v_resolution(vtkParametricFunctionSource* sself, int _arg) { sself->SetVResolution(_arg); }
extern "C" int vtk_parametric_function_source_get_v_resolution_min_value(vtkParametricFunctionSource* sself) { return sself->GetVResolutionMinValue(); }
extern "C" int vtk_parametric_function_source_get_v_resolution_max_value(vtkParametricFunctionSource* sself) { return sself->GetVResolutionMaxValue(); }
extern "C" int vtk_parametric_function_source_get_v_resolution(vtkParametricFunctionSource* sself) { return sself->GetVResolution(); }
extern "C" void vtk_parametric_function_source_set_w_resolution(vtkParametricFunctionSource* sself, int _arg) { sself->SetWResolution(_arg); }
extern "C" int vtk_parametric_function_source_get_w_resolution_min_value(vtkParametricFunctionSource* sself) { return sself->GetWResolutionMinValue(); }
extern "C" int vtk_parametric_function_source_get_w_resolution_max_value(vtkParametricFunctionSource* sself) { return sself->GetWResolutionMaxValue(); }
extern "C" int vtk_parametric_function_source_get_w_resolution(vtkParametricFunctionSource* sself) { return sself->GetWResolution(); }
extern "C" void vtk_parametric_function_source_generate_texture_coordinates_on(vtkParametricFunctionSource* sself) { sself->GenerateTextureCoordinatesOn(); }
extern "C" void vtk_parametric_function_source_generate_texture_coordinates_off(vtkParametricFunctionSource* sself) { sself->GenerateTextureCoordinatesOff(); }
extern "C" void vtk_parametric_function_source_set_generate_texture_coordinates(vtkParametricFunctionSource* sself, int _arg) { sself->SetGenerateTextureCoordinates(_arg); }
extern "C" int vtk_parametric_function_source_get_generate_texture_coordinates_min_value(vtkParametricFunctionSource* sself) { return sself->GetGenerateTextureCoordinatesMinValue(); }
extern "C" int vtk_parametric_function_source_get_generate_texture_coordinates_max_value(vtkParametricFunctionSource* sself) { return sself->GetGenerateTextureCoordinatesMaxValue(); }
extern "C" int vtk_parametric_function_source_get_generate_texture_coordinates(vtkParametricFunctionSource* sself) { return sself->GetGenerateTextureCoordinates(); }
extern "C" void vtk_parametric_function_source_generate_normals_on(vtkParametricFunctionSource* sself) { sself->GenerateNormalsOn(); }
extern "C" void vtk_parametric_function_source_generate_normals_off(vtkParametricFunctionSource* sself) { sself->GenerateNormalsOff(); }
extern "C" void vtk_parametric_function_source_set_generate_normals(vtkParametricFunctionSource* sself, int _arg) { sself->SetGenerateNormals(_arg); }
extern "C" int vtk_parametric_function_source_get_generate_normals_min_value(vtkParametricFunctionSource* sself) { return sself->GetGenerateNormalsMinValue(); }
extern "C" int vtk_parametric_function_source_get_generate_normals_max_value(vtkParametricFunctionSource* sself) { return sself->GetGenerateNormalsMaxValue(); }
extern "C" int vtk_parametric_function_source_get_generate_normals(vtkParametricFunctionSource* sself) { return sself->GetGenerateNormals(); }
extern "C" void vtk_parametric_function_source_set_scalar_mode(vtkParametricFunctionSource* sself, int _arg) { sself->SetScalarMode(_arg); }
extern "C" int vtk_parametric_function_source_get_scalar_mode_min_value(vtkParametricFunctionSource* sself) { return sself->GetScalarModeMinValue(); }
extern "C" int vtk_parametric_function_source_get_scalar_mode_max_value(vtkParametricFunctionSource* sself) { return sself->GetScalarModeMaxValue(); }
extern "C" int vtk_parametric_function_source_get_scalar_mode(vtkParametricFunctionSource* sself) { return sself->GetScalarMode(); }
extern "C" void vtk_parametric_function_source_set_scalar_mode_to_none(vtkParametricFunctionSource* sself) { sself->SetScalarModeToNone(); }
extern "C" void vtk_parametric_function_source_set_scalar_mode_to_u(vtkParametricFunctionSource* sself) { sself->SetScalarModeToU(); }
extern "C" void vtk_parametric_function_source_set_scalar_mode_to_v(vtkParametricFunctionSource* sself) { sself->SetScalarModeToV(); }
extern "C" void vtk_parametric_function_source_set_scalar_mode_to_u_0(vtkParametricFunctionSource* sself) { sself->SetScalarModeToU0(); }
extern "C" void vtk_parametric_function_source_set_scalar_mode_to_v_0(vtkParametricFunctionSource* sself) { sself->SetScalarModeToV0(); }
extern "C" void vtk_parametric_function_source_set_scalar_mode_to_u_0_v_0(vtkParametricFunctionSource* sself) { sself->SetScalarModeToU0V0(); }
extern "C" void vtk_parametric_function_source_set_scalar_mode_to_modulus(vtkParametricFunctionSource* sself) { sself->SetScalarModeToModulus(); }
extern "C" void vtk_parametric_function_source_set_scalar_mode_to_phase(vtkParametricFunctionSource* sself) { sself->SetScalarModeToPhase(); }
extern "C" void vtk_parametric_function_source_set_scalar_mode_to_quadrant(vtkParametricFunctionSource* sself) { sself->SetScalarModeToQuadrant(); }
extern "C" void vtk_parametric_function_source_set_scalar_mode_to_x(vtkParametricFunctionSource* sself) { sself->SetScalarModeToX(); }
extern "C" void vtk_parametric_function_source_set_scalar_mode_to_y(vtkParametricFunctionSource* sself) { sself->SetScalarModeToY(); }
extern "C" void vtk_parametric_function_source_set_scalar_mode_to_z(vtkParametricFunctionSource* sself) { sself->SetScalarModeToZ(); }
extern "C" void vtk_parametric_function_source_set_scalar_mode_to_distance(vtkParametricFunctionSource* sself) { sself->SetScalarModeToDistance(); }
extern "C" void vtk_parametric_function_source_set_scalar_mode_to_function_defined(vtkParametricFunctionSource* sself) { sself->SetScalarModeToFunctionDefined(); }
extern "C" unsigned long vtk_parametric_function_source_get_m_time(vtkParametricFunctionSource* sself) { return sself->GetMTime(); }
extern "C" void vtk_parametric_function_source_set_output_points_precision(vtkParametricFunctionSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_parametric_function_source_get_output_points_precision(vtkParametricFunctionSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkPartitionedDataSetCollectionSource * vtkPartitionedDataSetCollectionSource_new () {return vtkPartitionedDataSetCollectionSource :: New () ;}
extern "C" void vtkPartitionedDataSetCollectionSource_destructor (vtkPartitionedDataSetCollectionSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_partitioned_data_set_collection_source_set_number_of_shapes(vtkPartitionedDataSetCollectionSource* sself, int _arg) { sself->SetNumberOfShapes(_arg); }
extern "C" int vtk_partitioned_data_set_collection_source_get_number_of_shapes_min_value(vtkPartitionedDataSetCollectionSource* sself) { return sself->GetNumberOfShapesMinValue(); }
extern "C" int vtk_partitioned_data_set_collection_source_get_number_of_shapes_max_value(vtkPartitionedDataSetCollectionSource* sself) { return sself->GetNumberOfShapesMaxValue(); }
extern "C" int vtk_partitioned_data_set_collection_source_get_number_of_shapes(vtkPartitionedDataSetCollectionSource* sself) { return sself->GetNumberOfShapes(); }
extern "C" vtkPartitionedDataSetSource * vtkPartitionedDataSetSource_new () {return vtkPartitionedDataSetSource :: New () ;}
extern "C" void vtkPartitionedDataSetSource_destructor (vtkPartitionedDataSetSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_partitioned_data_set_source_enable_rank(vtkPartitionedDataSetSource* sself, int rank) { sself->EnableRank(rank); }
extern "C" void vtk_partitioned_data_set_source_enable_all_ranks(vtkPartitionedDataSetSource* sself) { sself->EnableAllRanks(); }
extern "C" void vtk_partitioned_data_set_source_disable_rank(vtkPartitionedDataSetSource* sself, int rank) { sself->DisableRank(rank); }
extern "C" void vtk_partitioned_data_set_source_disable_all_ranks(vtkPartitionedDataSetSource* sself) { sself->DisableAllRanks(); }
extern "C" bool vtk_partitioned_data_set_source_is_enabled_rank(vtkPartitionedDataSetSource* sself, int rank) { return sself->IsEnabledRank(rank); }
extern "C" void vtk_partitioned_data_set_source_set_number_of_partitions(vtkPartitionedDataSetSource* sself, int _arg) { sself->SetNumberOfPartitions(_arg); }
extern "C" int vtk_partitioned_data_set_source_get_number_of_partitions_min_value(vtkPartitionedDataSetSource* sself) { return sself->GetNumberOfPartitionsMinValue(); }
extern "C" int vtk_partitioned_data_set_source_get_number_of_partitions_max_value(vtkPartitionedDataSetSource* sself) { return sself->GetNumberOfPartitionsMaxValue(); }
extern "C" int vtk_partitioned_data_set_source_get_number_of_partitions(vtkPartitionedDataSetSource* sself) { return sself->GetNumberOfPartitions(); }
extern "C" vtkPlaneSource * vtkPlaneSource_new () {return vtkPlaneSource :: New () ;}
extern "C" void vtkPlaneSource_destructor (vtkPlaneSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_plane_source_set_x_resolution(vtkPlaneSource* sself, int _arg) { sself->SetXResolution(_arg); }
extern "C" int vtk_plane_source_get_x_resolution(vtkPlaneSource* sself) { return sself->GetXResolution(); }
extern "C" void vtk_plane_source_set_y_resolution(vtkPlaneSource* sself, int _arg) { sself->SetYResolution(_arg); }
extern "C" int vtk_plane_source_get_y_resolution(vtkPlaneSource* sself) { return sself->GetYResolution(); }
extern "C" void vtk_plane_source_set_resolution(vtkPlaneSource* sself, const int xR, const int yR) { sself->SetResolution(xR, yR); }
extern "C" void vtk_plane_source_get_resolution(vtkPlaneSource* sself, int& xR, int& yR) { sself->GetResolution(xR, yR); }
extern "C" void vtk_plane_source_set_origin(vtkPlaneSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetOrigin(_arg1, _arg2, _arg3); }
extern "C" void vtk_plane_source_set_point_1(vtkPlaneSource* sself, double x, double y, double z) { sself->SetPoint1(x, y, z); }
extern "C" void vtk_plane_source_set_point_2(vtkPlaneSource* sself, double x, double y, double z) { sself->SetPoint2(x, y, z); }
extern "C" void vtk_plane_source_set_center(vtkPlaneSource* sself, double x, double y, double z) { sself->SetCenter(x, y, z); }
extern "C" void vtk_plane_source_set_normal(vtkPlaneSource* sself, double nx, double ny, double nz) { sself->SetNormal(nx, ny, nz); }
extern "C" void vtk_plane_source_push(vtkPlaneSource* sself, double distance) { sself->Push(distance); }
extern "C" void vtk_plane_source_set_output_points_precision(vtkPlaneSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_plane_source_get_output_points_precision(vtkPlaneSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkPlatonicSolidSource * vtkPlatonicSolidSource_new () {return vtkPlatonicSolidSource :: New () ;}
extern "C" void vtkPlatonicSolidSource_destructor (vtkPlatonicSolidSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_platonic_solid_source_set_solid_type(vtkPlatonicSolidSource* sself, int _arg) { sself->SetSolidType(_arg); }
extern "C" int vtk_platonic_solid_source_get_solid_type_min_value(vtkPlatonicSolidSource* sself) { return sself->GetSolidTypeMinValue(); }
extern "C" int vtk_platonic_solid_source_get_solid_type_max_value(vtkPlatonicSolidSource* sself) { return sself->GetSolidTypeMaxValue(); }
extern "C" int vtk_platonic_solid_source_get_solid_type(vtkPlatonicSolidSource* sself) { return sself->GetSolidType(); }
extern "C" void vtk_platonic_solid_source_set_solid_type_to_tetrahedron(vtkPlatonicSolidSource* sself) { sself->SetSolidTypeToTetrahedron(); }
extern "C" void vtk_platonic_solid_source_set_solid_type_to_cube(vtkPlatonicSolidSource* sself) { sself->SetSolidTypeToCube(); }
extern "C" void vtk_platonic_solid_source_set_solid_type_to_octahedron(vtkPlatonicSolidSource* sself) { sself->SetSolidTypeToOctahedron(); }
extern "C" void vtk_platonic_solid_source_set_solid_type_to_icosahedron(vtkPlatonicSolidSource* sself) { sself->SetSolidTypeToIcosahedron(); }
extern "C" void vtk_platonic_solid_source_set_solid_type_to_dodecahedron(vtkPlatonicSolidSource* sself) { sself->SetSolidTypeToDodecahedron(); }
extern "C" void vtk_platonic_solid_source_set_output_points_precision(vtkPlatonicSolidSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_platonic_solid_source_get_output_points_precision(vtkPlatonicSolidSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkPointHandleSource * vtkPointHandleSource_new () {return vtkPointHandleSource :: New () ;}
extern "C" void vtkPointHandleSource_destructor (vtkPointHandleSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_point_handle_source_set_position(vtkPointHandleSource* sself, double xPos, double yPos, double zPos) { sself->SetPosition(xPos, yPos, zPos); }
extern "C" void vtk_point_handle_source_set_direction(vtkPointHandleSource* sself, double xDir, double yDir, double zDir) { sself->SetDirection(xDir, yDir, zDir); }
extern "C" vtkPointSource * vtkPointSource_new () {return vtkPointSource :: New () ;}
extern "C" void vtkPointSource_destructor (vtkPointSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_point_source_set_number_of_points(vtkPointSource* sself, long long _arg) { sself->SetNumberOfPoints(_arg); }
extern "C" long long vtk_point_source_get_number_of_points_min_value(vtkPointSource* sself) { return sself->GetNumberOfPointsMinValue(); }
extern "C" long long vtk_point_source_get_number_of_points_max_value(vtkPointSource* sself) { return sself->GetNumberOfPointsMaxValue(); }
extern "C" long long vtk_point_source_get_number_of_points(vtkPointSource* sself) { return sself->GetNumberOfPoints(); }
extern "C" void vtk_point_source_set_center(vtkPointSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetCenter(_arg1, _arg2, _arg3); }
extern "C" void vtk_point_source_set_radius(vtkPointSource* sself, double _arg) { sself->SetRadius(_arg); }
extern "C" double vtk_point_source_get_radius_min_value(vtkPointSource* sself) { return sself->GetRadiusMinValue(); }
extern "C" double vtk_point_source_get_radius_max_value(vtkPointSource* sself) { return sself->GetRadiusMaxValue(); }
extern "C" double vtk_point_source_get_radius(vtkPointSource* sself) { return sself->GetRadius(); }
extern "C" void vtk_point_source_set_distribution(vtkPointSource* sself, int _arg) { sself->SetDistribution(_arg); }
extern "C" void vtk_point_source_set_distribution_to_uniform(vtkPointSource* sself) { sself->SetDistributionToUniform(); }
extern "C" void vtk_point_source_set_distribution_to_shell(vtkPointSource* sself) { sself->SetDistributionToShell(); }
extern "C" int vtk_point_source_get_distribution(vtkPointSource* sself) { return sself->GetDistribution(); }
extern "C" void vtk_point_source_set_output_points_precision(vtkPointSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_point_source_get_output_points_precision(vtkPointSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkPolyLineSource * vtkPolyLineSource_new () {return vtkPolyLineSource :: New () ;}
extern "C" void vtkPolyLineSource_destructor (vtkPolyLineSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_poly_line_source_set_closed(vtkPolyLineSource* sself, int _arg) { sself->SetClosed(_arg); }
extern "C" int vtk_poly_line_source_get_closed(vtkPolyLineSource* sself) { return sself->GetClosed(); }
extern "C" void vtk_poly_line_source_closed_on(vtkPolyLineSource* sself) { sself->ClosedOn(); }
extern "C" void vtk_poly_line_source_closed_off(vtkPolyLineSource* sself) { sself->ClosedOff(); }
extern "C" vtkPolyPointSource * vtkPolyPointSource_new () {return vtkPolyPointSource :: New () ;}
extern "C" void vtkPolyPointSource_destructor (vtkPolyPointSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_poly_point_source_set_number_of_points(vtkPolyPointSource* sself, long long numPoints) { sself->SetNumberOfPoints(numPoints); }
extern "C" long long vtk_poly_point_source_get_number_of_points(vtkPolyPointSource* sself) { return sself->GetNumberOfPoints(); }
extern "C" void vtk_poly_point_source_resize(vtkPolyPointSource* sself, long long numPoints) { sself->Resize(numPoints); }
extern "C" void vtk_poly_point_source_set_point(vtkPolyPointSource* sself, long long id, double x, double y, double z) { sself->SetPoint(id, x, y, z); }
extern "C" unsigned long vtk_poly_point_source_get_m_time(vtkPolyPointSource* sself) { return sself->GetMTime(); }
extern "C" vtkProgrammableDataObjectSource * vtkProgrammableDataObjectSource_new () {return vtkProgrammableDataObjectSource :: New () ;}
extern "C" void vtkProgrammableDataObjectSource_destructor (vtkProgrammableDataObjectSource * sself) {sself -> Delete () ; return ;}
extern "C" vtkProgrammableSource * vtkProgrammableSource_new () {return vtkProgrammableSource :: New () ;}
extern "C" void vtkProgrammableSource_destructor (vtkProgrammableSource * sself) {sself -> Delete () ; return ;}
extern "C" vtkRandomHyperTreeGridSource * vtkRandomHyperTreeGridSource_new () {return vtkRandomHyperTreeGridSource :: New () ;}
extern "C" void vtkRandomHyperTreeGridSource_destructor (vtkRandomHyperTreeGridSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_random_hyper_tree_grid_source_set_dimensions(vtkRandomHyperTreeGridSource* sself, unsigned int _arg1, unsigned int _arg2, unsigned int _arg3) { sself->SetDimensions(_arg1, _arg2, _arg3); }
extern "C" void vtk_random_hyper_tree_grid_source_set_output_bounds(vtkRandomHyperTreeGridSource* sself, double _arg1, double _arg2, double _arg3, double _arg4, double _arg5, double _arg6) { sself->SetOutputBounds(_arg1, _arg2, _arg3, _arg4, _arg5, _arg6); }
extern "C" unsigned int vtk_random_hyper_tree_grid_source_get_seed(vtkRandomHyperTreeGridSource* sself) { return sself->GetSeed(); }
extern "C" void vtk_random_hyper_tree_grid_source_set_seed(vtkRandomHyperTreeGridSource* sself, unsigned int _arg) { sself->SetSeed(_arg); }
extern "C" long long vtk_random_hyper_tree_grid_source_get_max_depth(vtkRandomHyperTreeGridSource* sself) { return sself->GetMaxDepth(); }
extern "C" void vtk_random_hyper_tree_grid_source_set_max_depth(vtkRandomHyperTreeGridSource* sself, long long _arg) { sself->SetMaxDepth(_arg); }
extern "C" long long vtk_random_hyper_tree_grid_source_get_max_depth_min_value(vtkRandomHyperTreeGridSource* sself) { return sself->GetMaxDepthMinValue(); }
extern "C" long long vtk_random_hyper_tree_grid_source_get_max_depth_max_value(vtkRandomHyperTreeGridSource* sself) { return sself->GetMaxDepthMaxValue(); }
extern "C" double vtk_random_hyper_tree_grid_source_get_split_fraction(vtkRandomHyperTreeGridSource* sself) { return sself->GetSplitFraction(); }
extern "C" void vtk_random_hyper_tree_grid_source_set_split_fraction(vtkRandomHyperTreeGridSource* sself, double _arg) { sself->SetSplitFraction(_arg); }
extern "C" double vtk_random_hyper_tree_grid_source_get_split_fraction_min_value(vtkRandomHyperTreeGridSource* sself) { return sself->GetSplitFractionMinValue(); }
extern "C" double vtk_random_hyper_tree_grid_source_get_split_fraction_max_value(vtkRandomHyperTreeGridSource* sself) { return sself->GetSplitFractionMaxValue(); }
extern "C" vtkRectangularButtonSource * vtkRectangularButtonSource_new () {return vtkRectangularButtonSource :: New () ;}
extern "C" void vtkRectangularButtonSource_destructor (vtkRectangularButtonSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_rectangular_button_source_set_width(vtkRectangularButtonSource* sself, double _arg) { sself->SetWidth(_arg); }
extern "C" double vtk_rectangular_button_source_get_width_min_value(vtkRectangularButtonSource* sself) { return sself->GetWidthMinValue(); }
extern "C" double vtk_rectangular_button_source_get_width_max_value(vtkRectangularButtonSource* sself) { return sself->GetWidthMaxValue(); }
extern "C" double vtk_rectangular_button_source_get_width(vtkRectangularButtonSource* sself) { return sself->GetWidth(); }
extern "C" void vtk_rectangular_button_source_set_height(vtkRectangularButtonSource* sself, double _arg) { sself->SetHeight(_arg); }
extern "C" double vtk_rectangular_button_source_get_height_min_value(vtkRectangularButtonSource* sself) { return sself->GetHeightMinValue(); }
extern "C" double vtk_rectangular_button_source_get_height_max_value(vtkRectangularButtonSource* sself) { return sself->GetHeightMaxValue(); }
extern "C" double vtk_rectangular_button_source_get_height(vtkRectangularButtonSource* sself) { return sself->GetHeight(); }
extern "C" void vtk_rectangular_button_source_set_depth(vtkRectangularButtonSource* sself, double _arg) { sself->SetDepth(_arg); }
extern "C" double vtk_rectangular_button_source_get_depth_min_value(vtkRectangularButtonSource* sself) { return sself->GetDepthMinValue(); }
extern "C" double vtk_rectangular_button_source_get_depth_max_value(vtkRectangularButtonSource* sself) { return sself->GetDepthMaxValue(); }
extern "C" double vtk_rectangular_button_source_get_depth(vtkRectangularButtonSource* sself) { return sself->GetDepth(); }
extern "C" void vtk_rectangular_button_source_set_box_ratio(vtkRectangularButtonSource* sself, double _arg) { sself->SetBoxRatio(_arg); }
extern "C" double vtk_rectangular_button_source_get_box_ratio_min_value(vtkRectangularButtonSource* sself) { return sself->GetBoxRatioMinValue(); }
extern "C" double vtk_rectangular_button_source_get_box_ratio_max_value(vtkRectangularButtonSource* sself) { return sself->GetBoxRatioMaxValue(); }
extern "C" double vtk_rectangular_button_source_get_box_ratio(vtkRectangularButtonSource* sself) { return sself->GetBoxRatio(); }
extern "C" void vtk_rectangular_button_source_set_texture_ratio(vtkRectangularButtonSource* sself, double _arg) { sself->SetTextureRatio(_arg); }
extern "C" double vtk_rectangular_button_source_get_texture_ratio_min_value(vtkRectangularButtonSource* sself) { return sself->GetTextureRatioMinValue(); }
extern "C" double vtk_rectangular_button_source_get_texture_ratio_max_value(vtkRectangularButtonSource* sself) { return sself->GetTextureRatioMaxValue(); }
extern "C" double vtk_rectangular_button_source_get_texture_ratio(vtkRectangularButtonSource* sself) { return sself->GetTextureRatio(); }
extern "C" void vtk_rectangular_button_source_set_texture_height_ratio(vtkRectangularButtonSource* sself, double _arg) { sself->SetTextureHeightRatio(_arg); }
extern "C" double vtk_rectangular_button_source_get_texture_height_ratio_min_value(vtkRectangularButtonSource* sself) { return sself->GetTextureHeightRatioMinValue(); }
extern "C" double vtk_rectangular_button_source_get_texture_height_ratio_max_value(vtkRectangularButtonSource* sself) { return sself->GetTextureHeightRatioMaxValue(); }
extern "C" double vtk_rectangular_button_source_get_texture_height_ratio(vtkRectangularButtonSource* sself) { return sself->GetTextureHeightRatio(); }
extern "C" void vtk_rectangular_button_source_set_output_points_precision(vtkRectangularButtonSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_rectangular_button_source_get_output_points_precision(vtkRectangularButtonSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkRegularPolygonSource * vtkRegularPolygonSource_new () {return vtkRegularPolygonSource :: New () ;}
extern "C" void vtkRegularPolygonSource_destructor (vtkRegularPolygonSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_regular_polygon_source_set_number_of_sides(vtkRegularPolygonSource* sself, int _arg) { sself->SetNumberOfSides(_arg); }
extern "C" int vtk_regular_polygon_source_get_number_of_sides_min_value(vtkRegularPolygonSource* sself) { return sself->GetNumberOfSidesMinValue(); }
extern "C" int vtk_regular_polygon_source_get_number_of_sides_max_value(vtkRegularPolygonSource* sself) { return sself->GetNumberOfSidesMaxValue(); }
extern "C" int vtk_regular_polygon_source_get_number_of_sides(vtkRegularPolygonSource* sself) { return sself->GetNumberOfSides(); }
extern "C" void vtk_regular_polygon_source_set_center(vtkRegularPolygonSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetCenter(_arg1, _arg2, _arg3); }
extern "C" void vtk_regular_polygon_source_set_normal(vtkRegularPolygonSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetNormal(_arg1, _arg2, _arg3); }
extern "C" void vtk_regular_polygon_source_set_radius(vtkRegularPolygonSource* sself, double _arg) { sself->SetRadius(_arg); }
extern "C" double vtk_regular_polygon_source_get_radius(vtkRegularPolygonSource* sself) { return sself->GetRadius(); }
extern "C" void vtk_regular_polygon_source_set_generate_polygon(vtkRegularPolygonSource* sself, int _arg) { sself->SetGeneratePolygon(_arg); }
extern "C" int vtk_regular_polygon_source_get_generate_polygon(vtkRegularPolygonSource* sself) { return sself->GetGeneratePolygon(); }
extern "C" void vtk_regular_polygon_source_generate_polygon_on(vtkRegularPolygonSource* sself) { sself->GeneratePolygonOn(); }
extern "C" void vtk_regular_polygon_source_generate_polygon_off(vtkRegularPolygonSource* sself) { sself->GeneratePolygonOff(); }
extern "C" void vtk_regular_polygon_source_set_generate_polyline(vtkRegularPolygonSource* sself, int _arg) { sself->SetGeneratePolyline(_arg); }
extern "C" int vtk_regular_polygon_source_get_generate_polyline(vtkRegularPolygonSource* sself) { return sself->GetGeneratePolyline(); }
extern "C" void vtk_regular_polygon_source_generate_polyline_on(vtkRegularPolygonSource* sself) { sself->GeneratePolylineOn(); }
extern "C" void vtk_regular_polygon_source_generate_polyline_off(vtkRegularPolygonSource* sself) { sself->GeneratePolylineOff(); }
extern "C" void vtk_regular_polygon_source_set_output_points_precision(vtkRegularPolygonSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_regular_polygon_source_get_output_points_precision(vtkRegularPolygonSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkSelectionSource * vtkSelectionSource_new () {return vtkSelectionSource :: New () ;}
extern "C" void vtkSelectionSource_destructor (vtkSelectionSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_selection_source_add_id(vtkSelectionSource* sself, long long piece, long long id) { sself->AddID(piece, id); }
extern "C" void vtk_selection_source_add_string_id(vtkSelectionSource* sself, long long piece, const char* id) { sself->AddStringID(piece, id); }
extern "C" void vtk_selection_source_add_location(vtkSelectionSource* sself, double x, double y, double z) { sself->AddLocation(x, y, z); }
extern "C" void vtk_selection_source_add_threshold(vtkSelectionSource* sself, double min, double max) { sself->AddThreshold(min, max); }
extern "C" void vtk_selection_source_add_block(vtkSelectionSource* sself, long long blockno) { sself->AddBlock(blockno); }
extern "C" void vtk_selection_source_add_block_selector(vtkSelectionSource* sself, const char* selector) { sself->AddBlockSelector(selector); }
extern "C" void vtk_selection_source_remove_all_block_selectors(vtkSelectionSource* sself) { sself->RemoveAllBlockSelectors(); }
extern "C" void vtk_selection_source_remove_all_i_ds(vtkSelectionSource* sself) { sself->RemoveAllIDs(); }
extern "C" void vtk_selection_source_remove_all_string_i_ds(vtkSelectionSource* sself) { sself->RemoveAllStringIDs(); }
extern "C" void vtk_selection_source_remove_all_thresholds(vtkSelectionSource* sself) { sself->RemoveAllThresholds(); }
extern "C" void vtk_selection_source_remove_all_locations(vtkSelectionSource* sself) { sself->RemoveAllLocations(); }
extern "C" void vtk_selection_source_remove_all_blocks(vtkSelectionSource* sself) { sself->RemoveAllBlocks(); }
extern "C" void vtk_selection_source_set_content_type(vtkSelectionSource* sself, int _arg) { sself->SetContentType(_arg); }
extern "C" int vtk_selection_source_get_content_type(vtkSelectionSource* sself) { return sself->GetContentType(); }
extern "C" void vtk_selection_source_set_field_type(vtkSelectionSource* sself, int _arg) { sself->SetFieldType(_arg); }
extern "C" int vtk_selection_source_get_field_type(vtkSelectionSource* sself) { return sself->GetFieldType(); }
extern "C" void vtk_selection_source_set_containing_cells(vtkSelectionSource* sself, int _arg) { sself->SetContainingCells(_arg); }
extern "C" int vtk_selection_source_get_containing_cells(vtkSelectionSource* sself) { return sself->GetContainingCells(); }
extern "C" void vtk_selection_source_set_number_of_layers(vtkSelectionSource* sself, int _arg) { sself->SetNumberOfLayers(_arg); }
extern "C" int vtk_selection_source_get_number_of_layers_min_value(vtkSelectionSource* sself) { return sself->GetNumberOfLayersMinValue(); }
extern "C" int vtk_selection_source_get_number_of_layers_max_value(vtkSelectionSource* sself) { return sself->GetNumberOfLayersMaxValue(); }
extern "C" int vtk_selection_source_get_number_of_layers(vtkSelectionSource* sself) { return sself->GetNumberOfLayers(); }
extern "C" void vtk_selection_source_set_inverse(vtkSelectionSource* sself, int _arg) { sself->SetInverse(_arg); }
extern "C" int vtk_selection_source_get_inverse(vtkSelectionSource* sself) { return sself->GetInverse(); }
extern "C" void vtk_selection_source_set_array_name(vtkSelectionSource* sself, const char* _arg) { sself->SetArrayName(_arg); }
extern "C" void vtk_selection_source_set_array_component(vtkSelectionSource* sself, int _arg) { sself->SetArrayComponent(_arg); }
extern "C" int vtk_selection_source_get_array_component(vtkSelectionSource* sself) { return sself->GetArrayComponent(); }
extern "C" void vtk_selection_source_set_composite_index(vtkSelectionSource* sself, int _arg) { sself->SetCompositeIndex(_arg); }
extern "C" int vtk_selection_source_get_composite_index(vtkSelectionSource* sself) { return sself->GetCompositeIndex(); }
extern "C" void vtk_selection_source_set_hierarchical_level(vtkSelectionSource* sself, int _arg) { sself->SetHierarchicalLevel(_arg); }
extern "C" int vtk_selection_source_get_hierarchical_level(vtkSelectionSource* sself) { return sself->GetHierarchicalLevel(); }
extern "C" void vtk_selection_source_set_hierarchical_index(vtkSelectionSource* sself, int _arg) { sself->SetHierarchicalIndex(_arg); }
extern "C" int vtk_selection_source_get_hierarchical_index(vtkSelectionSource* sself) { return sself->GetHierarchicalIndex(); }
extern "C" void vtk_selection_source_set_assembly_name(vtkSelectionSource* sself, const char* _arg) { sself->SetAssemblyName(_arg); }
extern "C" void vtk_selection_source_add_selector(vtkSelectionSource* sself, const char* selector) { sself->AddSelector(selector); }
extern "C" void vtk_selection_source_remove_all_selectors(vtkSelectionSource* sself) { sself->RemoveAllSelectors(); }
extern "C" void vtk_selection_source_set_query_string(vtkSelectionSource* sself, const char* _arg) { sself->SetQueryString(_arg); }
extern "C" vtkSphereSource * vtkSphereSource_new () {return vtkSphereSource :: New () ;}
extern "C" void vtkSphereSource_destructor (vtkSphereSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_sphere_source_set_radius(vtkSphereSource* sself, double _arg) { sself->SetRadius(_arg); }
extern "C" double vtk_sphere_source_get_radius_min_value(vtkSphereSource* sself) { return sself->GetRadiusMinValue(); }
extern "C" double vtk_sphere_source_get_radius_max_value(vtkSphereSource* sself) { return sself->GetRadiusMaxValue(); }
extern "C" double vtk_sphere_source_get_radius(vtkSphereSource* sself) { return sself->GetRadius(); }
extern "C" void vtk_sphere_source_set_center(vtkSphereSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetCenter(_arg1, _arg2, _arg3); }
extern "C" void vtk_sphere_source_set_theta_resolution(vtkSphereSource* sself, int _arg) { sself->SetThetaResolution(_arg); }
extern "C" int vtk_sphere_source_get_theta_resolution_min_value(vtkSphereSource* sself) { return sself->GetThetaResolutionMinValue(); }
extern "C" int vtk_sphere_source_get_theta_resolution_max_value(vtkSphereSource* sself) { return sself->GetThetaResolutionMaxValue(); }
extern "C" int vtk_sphere_source_get_theta_resolution(vtkSphereSource* sself) { return sself->GetThetaResolution(); }
extern "C" void vtk_sphere_source_set_phi_resolution(vtkSphereSource* sself, int _arg) { sself->SetPhiResolution(_arg); }
extern "C" int vtk_sphere_source_get_phi_resolution_min_value(vtkSphereSource* sself) { return sself->GetPhiResolutionMinValue(); }
extern "C" int vtk_sphere_source_get_phi_resolution_max_value(vtkSphereSource* sself) { return sself->GetPhiResolutionMaxValue(); }
extern "C" int vtk_sphere_source_get_phi_resolution(vtkSphereSource* sself) { return sself->GetPhiResolution(); }
extern "C" void vtk_sphere_source_set_start_theta(vtkSphereSource* sself, double _arg) { sself->SetStartTheta(_arg); }
extern "C" double vtk_sphere_source_get_start_theta_min_value(vtkSphereSource* sself) { return sself->GetStartThetaMinValue(); }
extern "C" double vtk_sphere_source_get_start_theta_max_value(vtkSphereSource* sself) { return sself->GetStartThetaMaxValue(); }
extern "C" double vtk_sphere_source_get_start_theta(vtkSphereSource* sself) { return sself->GetStartTheta(); }
extern "C" void vtk_sphere_source_set_end_theta(vtkSphereSource* sself, double _arg) { sself->SetEndTheta(_arg); }
extern "C" double vtk_sphere_source_get_end_theta_min_value(vtkSphereSource* sself) { return sself->GetEndThetaMinValue(); }
extern "C" double vtk_sphere_source_get_end_theta_max_value(vtkSphereSource* sself) { return sself->GetEndThetaMaxValue(); }
extern "C" double vtk_sphere_source_get_end_theta(vtkSphereSource* sself) { return sself->GetEndTheta(); }
extern "C" void vtk_sphere_source_set_start_phi(vtkSphereSource* sself, double _arg) { sself->SetStartPhi(_arg); }
extern "C" double vtk_sphere_source_get_start_phi_min_value(vtkSphereSource* sself) { return sself->GetStartPhiMinValue(); }
extern "C" double vtk_sphere_source_get_start_phi_max_value(vtkSphereSource* sself) { return sself->GetStartPhiMaxValue(); }
extern "C" double vtk_sphere_source_get_start_phi(vtkSphereSource* sself) { return sself->GetStartPhi(); }
extern "C" void vtk_sphere_source_set_end_phi(vtkSphereSource* sself, double _arg) { sself->SetEndPhi(_arg); }
extern "C" double vtk_sphere_source_get_end_phi_min_value(vtkSphereSource* sself) { return sself->GetEndPhiMinValue(); }
extern "C" double vtk_sphere_source_get_end_phi_max_value(vtkSphereSource* sself) { return sself->GetEndPhiMaxValue(); }
extern "C" double vtk_sphere_source_get_end_phi(vtkSphereSource* sself) { return sself->GetEndPhi(); }
extern "C" void vtk_sphere_source_set_lat_long_tessellation(vtkSphereSource* sself, int _arg) { sself->SetLatLongTessellation(_arg); }
extern "C" int vtk_sphere_source_get_lat_long_tessellation(vtkSphereSource* sself) { return sself->GetLatLongTessellation(); }
extern "C" void vtk_sphere_source_lat_long_tessellation_on(vtkSphereSource* sself) { sself->LatLongTessellationOn(); }
extern "C" void vtk_sphere_source_lat_long_tessellation_off(vtkSphereSource* sself) { sself->LatLongTessellationOff(); }
extern "C" void vtk_sphere_source_set_output_points_precision(vtkSphereSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_sphere_source_get_output_points_precision(vtkSphereSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" void vtk_sphere_source_set_generate_normals(vtkSphereSource* sself, int _arg) { sself->SetGenerateNormals(_arg); }
extern "C" int vtk_sphere_source_get_generate_normals(vtkSphereSource* sself) { return sself->GetGenerateNormals(); }
extern "C" void vtk_sphere_source_generate_normals_on(vtkSphereSource* sself) { sself->GenerateNormalsOn(); }
extern "C" void vtk_sphere_source_generate_normals_off(vtkSphereSource* sself) { sself->GenerateNormalsOff(); }
extern "C" vtkSuperquadricSource * vtkSuperquadricSource_new () {return vtkSuperquadricSource :: New () ;}
extern "C" void vtkSuperquadricSource_destructor (vtkSuperquadricSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_superquadric_source_set_center(vtkSuperquadricSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetCenter(_arg1, _arg2, _arg3); }
extern "C" void vtk_superquadric_source_set_scale(vtkSuperquadricSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetScale(_arg1, _arg2, _arg3); }
extern "C" int vtk_superquadric_source_get_theta_resolution(vtkSuperquadricSource* sself) { return sself->GetThetaResolution(); }
extern "C" void vtk_superquadric_source_set_theta_resolution(vtkSuperquadricSource* sself, int i) { sself->SetThetaResolution(i); }
extern "C" int vtk_superquadric_source_get_phi_resolution(vtkSuperquadricSource* sself) { return sself->GetPhiResolution(); }
extern "C" void vtk_superquadric_source_set_phi_resolution(vtkSuperquadricSource* sself, int i) { sself->SetPhiResolution(i); }
extern "C" double vtk_superquadric_source_get_thickness(vtkSuperquadricSource* sself) { return sself->GetThickness(); }
extern "C" void vtk_superquadric_source_set_thickness(vtkSuperquadricSource* sself, double _arg) { sself->SetThickness(_arg); }
extern "C" double vtk_superquadric_source_get_thickness_min_value(vtkSuperquadricSource* sself) { return sself->GetThicknessMinValue(); }
extern "C" double vtk_superquadric_source_get_thickness_max_value(vtkSuperquadricSource* sself) { return sself->GetThicknessMaxValue(); }
extern "C" double vtk_superquadric_source_get_phi_roundness(vtkSuperquadricSource* sself) { return sself->GetPhiRoundness(); }
extern "C" void vtk_superquadric_source_set_phi_roundness(vtkSuperquadricSource* sself, double e) { sself->SetPhiRoundness(e); }
extern "C" double vtk_superquadric_source_get_theta_roundness(vtkSuperquadricSource* sself) { return sself->GetThetaRoundness(); }
extern "C" void vtk_superquadric_source_set_theta_roundness(vtkSuperquadricSource* sself, double e) { sself->SetThetaRoundness(e); }
extern "C" void vtk_superquadric_source_set_size(vtkSuperquadricSource* sself, double _arg) { sself->SetSize(_arg); }
extern "C" double vtk_superquadric_source_get_size(vtkSuperquadricSource* sself) { return sself->GetSize(); }
extern "C" void vtk_superquadric_source_set_axis_of_symmetry(vtkSuperquadricSource* sself, int _arg) { sself->SetAxisOfSymmetry(_arg); }
extern "C" int vtk_superquadric_source_get_axis_of_symmetry(vtkSuperquadricSource* sself) { return sself->GetAxisOfSymmetry(); }
extern "C" void vtk_superquadric_source_set_x_axis_of_symmetry(vtkSuperquadricSource* sself) { sself->SetXAxisOfSymmetry(); }
extern "C" void vtk_superquadric_source_set_y_axis_of_symmetry(vtkSuperquadricSource* sself) { sself->SetYAxisOfSymmetry(); }
extern "C" void vtk_superquadric_source_set_z_axis_of_symmetry(vtkSuperquadricSource* sself) { sself->SetZAxisOfSymmetry(); }
extern "C" void vtk_superquadric_source_toroidal_on(vtkSuperquadricSource* sself) { sself->ToroidalOn(); }
extern "C" void vtk_superquadric_source_toroidal_off(vtkSuperquadricSource* sself) { sself->ToroidalOff(); }
extern "C" int vtk_superquadric_source_get_toroidal(vtkSuperquadricSource* sself) { return sself->GetToroidal(); }
extern "C" void vtk_superquadric_source_set_toroidal(vtkSuperquadricSource* sself, int _arg) { sself->SetToroidal(_arg); }
extern "C" void vtk_superquadric_source_set_output_points_precision(vtkSuperquadricSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_superquadric_source_get_output_points_precision(vtkSuperquadricSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkTessellatedBoxSource * vtkTessellatedBoxSource_new () {return vtkTessellatedBoxSource :: New () ;}
extern "C" void vtkTessellatedBoxSource_destructor (vtkTessellatedBoxSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_tessellated_box_source_set_bounds(vtkTessellatedBoxSource* sself, double _arg1, double _arg2, double _arg3, double _arg4, double _arg5, double _arg6) { sself->SetBounds(_arg1, _arg2, _arg3, _arg4, _arg5, _arg6); }
extern "C" void vtk_tessellated_box_source_set_level(vtkTessellatedBoxSource* sself, int _arg) { sself->SetLevel(_arg); }
extern "C" int vtk_tessellated_box_source_get_level(vtkTessellatedBoxSource* sself) { return sself->GetLevel(); }
extern "C" void vtk_tessellated_box_source_set_duplicate_shared_points(vtkTessellatedBoxSource* sself, int _arg) { sself->SetDuplicateSharedPoints(_arg); }
extern "C" int vtk_tessellated_box_source_get_duplicate_shared_points(vtkTessellatedBoxSource* sself) { return sself->GetDuplicateSharedPoints(); }
extern "C" void vtk_tessellated_box_source_duplicate_shared_points_on(vtkTessellatedBoxSource* sself) { sself->DuplicateSharedPointsOn(); }
extern "C" void vtk_tessellated_box_source_duplicate_shared_points_off(vtkTessellatedBoxSource* sself) { sself->DuplicateSharedPointsOff(); }
extern "C" void vtk_tessellated_box_source_set_quads(vtkTessellatedBoxSource* sself, int _arg) { sself->SetQuads(_arg); }
extern "C" int vtk_tessellated_box_source_get_quads(vtkTessellatedBoxSource* sself) { return sself->GetQuads(); }
extern "C" void vtk_tessellated_box_source_quads_on(vtkTessellatedBoxSource* sself) { sself->QuadsOn(); }
extern "C" void vtk_tessellated_box_source_quads_off(vtkTessellatedBoxSource* sself) { sself->QuadsOff(); }
extern "C" void vtk_tessellated_box_source_set_output_points_precision(vtkTessellatedBoxSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_tessellated_box_source_get_output_points_precision(vtkTessellatedBoxSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkTextSource * vtkTextSource_new () {return vtkTextSource :: New () ;}
extern "C" void vtkTextSource_destructor (vtkTextSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_text_source_set_text(vtkTextSource* sself, const char* _arg) { sself->SetText(_arg); }
extern "C" void vtk_text_source_set_backing(vtkTextSource* sself, int _arg) { sself->SetBacking(_arg); }
extern "C" int vtk_text_source_get_backing(vtkTextSource* sself) { return sself->GetBacking(); }
extern "C" void vtk_text_source_backing_on(vtkTextSource* sself) { sself->BackingOn(); }
extern "C" void vtk_text_source_backing_off(vtkTextSource* sself) { sself->BackingOff(); }
extern "C" void vtk_text_source_set_foreground_color(vtkTextSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetForegroundColor(_arg1, _arg2, _arg3); }
extern "C" void vtk_text_source_set_background_color(vtkTextSource* sself, double _arg1, double _arg2, double _arg3) { sself->SetBackgroundColor(_arg1, _arg2, _arg3); }
extern "C" void vtk_text_source_set_output_points_precision(vtkTextSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_text_source_get_output_points_precision(vtkTextSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkTexturedSphereSource * vtkTexturedSphereSource_new () {return vtkTexturedSphereSource :: New () ;}
extern "C" void vtkTexturedSphereSource_destructor (vtkTexturedSphereSource * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_textured_sphere_source_set_radius(vtkTexturedSphereSource* sself, double _arg) { sself->SetRadius(_arg); }
extern "C" double vtk_textured_sphere_source_get_radius_min_value(vtkTexturedSphereSource* sself) { return sself->GetRadiusMinValue(); }
extern "C" double vtk_textured_sphere_source_get_radius_max_value(vtkTexturedSphereSource* sself) { return sself->GetRadiusMaxValue(); }
extern "C" double vtk_textured_sphere_source_get_radius(vtkTexturedSphereSource* sself) { return sself->GetRadius(); }
extern "C" void vtk_textured_sphere_source_set_theta_resolution(vtkTexturedSphereSource* sself, int _arg) { sself->SetThetaResolution(_arg); }
extern "C" int vtk_textured_sphere_source_get_theta_resolution_min_value(vtkTexturedSphereSource* sself) { return sself->GetThetaResolutionMinValue(); }
extern "C" int vtk_textured_sphere_source_get_theta_resolution_max_value(vtkTexturedSphereSource* sself) { return sself->GetThetaResolutionMaxValue(); }
extern "C" int vtk_textured_sphere_source_get_theta_resolution(vtkTexturedSphereSource* sself) { return sself->GetThetaResolution(); }
extern "C" void vtk_textured_sphere_source_set_phi_resolution(vtkTexturedSphereSource* sself, int _arg) { sself->SetPhiResolution(_arg); }
extern "C" int vtk_textured_sphere_source_get_phi_resolution_min_value(vtkTexturedSphereSource* sself) { return sself->GetPhiResolutionMinValue(); }
extern "C" int vtk_textured_sphere_source_get_phi_resolution_max_value(vtkTexturedSphereSource* sself) { return sself->GetPhiResolutionMaxValue(); }
extern "C" int vtk_textured_sphere_source_get_phi_resolution(vtkTexturedSphereSource* sself) { return sself->GetPhiResolution(); }
extern "C" void vtk_textured_sphere_source_set_theta(vtkTexturedSphereSource* sself, double _arg) { sself->SetTheta(_arg); }
extern "C" double vtk_textured_sphere_source_get_theta_min_value(vtkTexturedSphereSource* sself) { return sself->GetThetaMinValue(); }
extern "C" double vtk_textured_sphere_source_get_theta_max_value(vtkTexturedSphereSource* sself) { return sself->GetThetaMaxValue(); }
extern "C" double vtk_textured_sphere_source_get_theta(vtkTexturedSphereSource* sself) { return sself->GetTheta(); }
extern "C" void vtk_textured_sphere_source_set_phi(vtkTexturedSphereSource* sself, double _arg) { sself->SetPhi(_arg); }
extern "C" double vtk_textured_sphere_source_get_phi_min_value(vtkTexturedSphereSource* sself) { return sself->GetPhiMinValue(); }
extern "C" double vtk_textured_sphere_source_get_phi_max_value(vtkTexturedSphereSource* sself) { return sself->GetPhiMaxValue(); }
extern "C" double vtk_textured_sphere_source_get_phi(vtkTexturedSphereSource* sself) { return sself->GetPhi(); }
extern "C" void vtk_textured_sphere_source_set_output_points_precision(vtkTexturedSphereSource* sself, int _arg) { sself->SetOutputPointsPrecision(_arg); }
extern "C" int vtk_textured_sphere_source_get_output_points_precision(vtkTexturedSphereSource* sself) { return sself->GetOutputPointsPrecision(); }
extern "C" vtkUniformHyperTreeGridSource * vtkUniformHyperTreeGridSource_new () {return vtkUniformHyperTreeGridSource :: New () ;}
extern "C" void vtkUniformHyperTreeGridSource_destructor (vtkUniformHyperTreeGridSource * sself) {sself -> Delete () ; return ;}
