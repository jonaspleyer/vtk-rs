// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkAMRBox.h>
#include<vtkAMRDataInternals.h>
#include<vtkAMRInformation.h>
#include<vtkAMRUtilities.h>
#include<vtkAbstractCellLinks.h>
#include<vtkAbstractCellLocator.h>
#include<vtkAbstractElectronicData.h>
#include<vtkAbstractPointLocator.h>
#include<vtkAdjacentVertexIterator.h>
#include<vtkAngularPeriodicDataArray.h>
#include<vtkAnimationScene.h>
#include<vtkAnnotation.h>
#include<vtkAnnotationLayers.h>
#include<vtkArrayData.h>
#include<vtkAtom.h>
#include<vtkAttributesErrorMetric.h>
#include<vtkBSPCuts.h>
#include<vtkBSPIntersections.h>
#include<vtkBezierCurve.h>
#include<vtkBezierHexahedron.h>
#include<vtkBezierInterpolation.h>
#include<vtkBezierQuadrilateral.h>
#include<vtkBezierTetra.h>
#include<vtkBezierTriangle.h>
#include<vtkBezierWedge.h>
#include<vtkBiQuadraticQuad.h>
#include<vtkBiQuadraticQuadraticHexahedron.h>
#include<vtkBiQuadraticQuadraticWedge.h>
#include<vtkBiQuadraticTriangle.h>
#include<vtkBond.h>
#include<vtkBoundingBox.h>
#include<vtkBox.h>
#include<vtkCell.h>
#include<vtkCell3D.h>
#include<vtkCellArray.h>
#include<vtkCellArrayIterator.h>
#include<vtkCellData.h>
#include<vtkCellIterator.h>
#include<vtkCellLinks.h>
#include<vtkCellLocator.h>
#include<vtkCellLocatorStrategy.h>
#include<vtkCellTypes.h>
#include<vtkClosestNPointsStrategy.h>
#include<vtkClosestPointStrategy.h>
#include<vtkColor.h>
#include<vtkColor.h>
#include<vtkColor.h>
#include<vtkColor.h>
#include<vtkColor.h>
#include<vtkColor.h>
#include<vtkColor.h>
#include<vtkColor.h>
#include<vtkCompositeDataIterator.h>
#include<vtkCompositeDataSet.h>
#include<vtkCone.h>
#include<vtkConvexPointSet.h>
#include<vtkCubicLine.h>
#include<vtkCylinder.h>
#include<vtkDataAssembly.h>
#include<vtkDataAssemblyUtilities.h>
#include<vtkDataAssemblyVisitor.h>
#include<vtkDataObject.h>
#include<vtkDataObjectCollection.h>
#include<vtkDataObjectTree.h>
#include<vtkDataObjectTreeInternals.h>
#include<vtkDataObjectTreeInternals.h>
#include<vtkDataObjectTreeIterator.h>
#include<vtkDataObjectTypes.h>
#include<vtkDataSet.h>
#include<vtkDataSetAttributes.h>
#include<vtkDataSetAttributesFieldList.h>
#include<vtkDataSetCellIterator.h>
#include<vtkDataSetCollection.h>
#include<vtkDirectedAcyclicGraph.h>
#include<vtkDirectedGraph.h>
#include<vtkDistributedGraphHelper.h>
#include<vtkEdgeListIterator.h>
#include<vtkEdgeTable.h>
#include<vtkEmptyCell.h>
#include<vtkExplicitStructuredGrid.h>
#include<vtkExtractStructuredGridHelper.h>
#include<vtkFieldData.h>
#include<vtkFindCellStrategy.h>
#include<vtkGenericAdaptorCell.h>
#include<vtkGenericAttribute.h>
#include<vtkGenericAttributeCollection.h>
#include<vtkGenericCell.h>
#include<vtkGenericCellIterator.h>
#include<vtkGenericCellTessellator.h>
#include<vtkGenericDataSet.h>
#include<vtkGenericEdgeTable.h>
#include<vtkGenericInterpolatedVelocityField.h>
#include<vtkGenericPointIterator.h>
#include<vtkGenericSubdivisionErrorMetric.h>
#include<vtkGeometricErrorMetric.h>
#include<vtkGraph.h>
#include<vtkGraphEdge.h>
#include<vtkGraphInternals.h>
#include<vtkHexagonalPrism.h>
#include<vtkHexahedron.h>
#include<vtkHierarchicalBoxDataIterator.h>
#include<vtkHierarchicalBoxDataSet.h>
#include<vtkHigherOrderCurve.h>
#include<vtkHigherOrderHexahedron.h>
#include<vtkHigherOrderInterpolation.h>
#include<vtkHigherOrderQuadrilateral.h>
#include<vtkHigherOrderTetra.h>
#include<vtkHigherOrderTriangle.h>
#include<vtkHigherOrderWedge.h>
#include<vtkHyperTree.h>
#include<vtkHyperTreeCursor.h>
#include<vtkHyperTreeGrid.h>
#include<vtkHyperTreeGridNonOrientedCursor.h>
#include<vtkHyperTreeGridNonOrientedGeometryCursor.h>
#include<vtkHyperTreeGridNonOrientedMooreSuperCursor.h>
#include<vtkHyperTreeGridNonOrientedMooreSuperCursorLight.h>
#include<vtkHyperTreeGridNonOrientedSuperCursor.h>
#include<vtkHyperTreeGridNonOrientedSuperCursorLight.h>
#include<vtkHyperTreeGridNonOrientedVonNeumannSuperCursor.h>
#include<vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight.h>
#include<vtkHyperTreeGridOrientedCursor.h>
#include<vtkHyperTreeGridOrientedGeometryCursor.h>
#include<vtkHyperTreeGridScales.h>
#include<vtkImageData.h>
#include<vtkImageIterator.h>
#include<vtkImageTransform.h>
#include<vtkImplicitBoolean.h>
#include<vtkImplicitDataSet.h>
#include<vtkImplicitFunction.h>
#include<vtkImplicitFunctionCollection.h>
#include<vtkImplicitHalo.h>
#include<vtkImplicitSelectionLoop.h>
#include<vtkImplicitSum.h>
#include<vtkImplicitVolume.h>
#include<vtkImplicitWindowFunction.h>
#include<vtkInEdgeIterator.h>
#include<vtkIncrementalOctreeNode.h>
#include<vtkIncrementalOctreePointLocator.h>
#include<vtkIncrementalPointLocator.h>
#include<vtkInformationQuadratureSchemeDefinitionVectorKey.h>
#include<vtkIntersectionCounter.h>
#include<vtkIterativeClosestPointTransform.h>
#include<vtkKdNode.h>
#include<vtkKdTree.h>
#include<vtkKdTreePointLocator.h>
#include<vtkLagrangeCurve.h>
#include<vtkLagrangeHexahedron.h>
#include<vtkLagrangeInterpolation.h>
#include<vtkLagrangeQuadrilateral.h>
#include<vtkLagrangeTetra.h>
#include<vtkLagrangeTriangle.h>
#include<vtkLagrangeWedge.h>
#include<vtkLine.h>
#include<vtkLocator.h>
#include<vtkMappedUnstructuredGrid.h>
#include<vtkMappedUnstructuredGridCellIterator.h>
#include<vtkMeanValueCoordinatesInterpolator.h>
#include<vtkMergePoints.h>
#include<vtkMolecule.h>
#include<vtkMultiBlockDataSet.h>
#include<vtkMultiPieceDataSet.h>
#include<vtkMutableDirectedGraph.h>
#include<vtkMutableUndirectedGraph.h>
#include<vtkNonLinearCell.h>
#include<vtkNonMergingPointLocator.h>
#include<vtkNonOverlappingAMR.h>
#include<vtkOctreePointLocator.h>
#include<vtkOctreePointLocatorNode.h>
#include<vtkOrderedTriangulator.h>
#include<vtkOutEdgeIterator.h>
#include<vtkOverlappingAMR.h>
#include<vtkPartitionedDataSet.h>
#include<vtkPartitionedDataSetCollection.h>
#include<vtkPath.h>
#include<vtkPentagonalPrism.h>
#include<vtkPeriodicDataArray.h>
#include<vtkPerlinNoise.h>
#include<vtkPiecewiseFunction.h>
#include<vtkPixel.h>
#include<vtkPixelExtent.h>
#include<vtkPixelTransfer.h>
#include<vtkPlane.h>
#include<vtkPlaneCollection.h>
#include<vtkPlanes.h>
#include<vtkPlanesIntersection.h>
#include<vtkPointData.h>
#include<vtkPointLocator.h>
#include<vtkPointSet.h>
#include<vtkPointSetCellIterator.h>
#include<vtkPointsProjectedHull.h>
#include<vtkPolyData.h>
#include<vtkPolyDataCollection.h>
#include<vtkPolyLine.h>
#include<vtkPolyPlane.h>
#include<vtkPolyVertex.h>
#include<vtkPolygon.h>
#include<vtkPolyhedron.h>
#include<vtkPyramid.h>
#include<vtkQuad.h>
#include<vtkQuadraticEdge.h>
#include<vtkQuadraticHexahedron.h>
#include<vtkQuadraticLinearQuad.h>
#include<vtkQuadraticLinearWedge.h>
#include<vtkQuadraticPolygon.h>
#include<vtkQuadraticPyramid.h>
#include<vtkQuadraticQuad.h>
#include<vtkQuadraticTetra.h>
#include<vtkQuadraticTriangle.h>
#include<vtkQuadraticWedge.h>
#include<vtkQuadratureSchemeDefinition.h>
#include<vtkQuadric.h>
#include<vtkRect.h>
#include<vtkRect.h>
#include<vtkRect.h>
#include<vtkRect.h>
#include<vtkRectilinearGrid.h>
#include<vtkReebGraph.h>
#include<vtkReebGraphSimplificationMetric.h>
#include<vtkSelection.h>
#include<vtkSelectionNode.h>
#include<vtkSimpleCellTessellator.h>
#include<vtkSmoothErrorMetric.h>
#include<vtkSortFieldData.h>
#include<vtkSphere.h>
#include<vtkSpheres.h>
#include<vtkSpline.h>
#include<vtkStaticCellLinks.h>
#include<vtkStaticCellLinksTemplate.h>
#include<vtkStaticCellLocator.h>
#include<vtkStaticEdgeLocatorTemplate.h>
#include<vtkStaticPointLocator.h>
#include<vtkStaticPointLocator2D.h>
#include<vtkStructuredData.h>
#include<vtkStructuredExtent.h>
#include<vtkStructuredGrid.h>
#include<vtkStructuredPoints.h>
#include<vtkStructuredPointsCollection.h>
#include<vtkSuperquadric.h>
#include<vtkTable.h>
#include<vtkTetra.h>
#include<vtkTree.h>
#include<vtkTreeBFSIterator.h>
#include<vtkTreeDFSIterator.h>
#include<vtkTreeIterator.h>
#include<vtkTriQuadraticHexahedron.h>
#include<vtkTriQuadraticPyramid.h>
#include<vtkTriangle.h>
#include<vtkTriangleStrip.h>
#include<vtkUndirectedGraph.h>
#include<vtkUniformGrid.h>
#include<vtkUniformGridAMR.h>
#include<vtkUniformGridAMRDataIterator.h>
#include<vtkUniformHyperTreeGrid.h>
#include<vtkUnstructuredGrid.h>
#include<vtkUnstructuredGridBase.h>
#include<vtkUnstructuredGridCellIterator.h>
#include<vtkVector.h>
#include<vtkVector.h>
#include<vtkVector.h>
#include<vtkVector.h>
#include<vtkVector.h>
#include<vtkVector.h>
#include<vtkVector.h>
#include<vtkVector.h>
#include<vtkVector.h>
#include<vtkVector.h>
#include<vtkVector.h>
#include<vtkVector.h>
#include<vtkVertex.h>
#include<vtkGraphInternals.h>
#include<vtkVertexListIterator.h>
#include<vtkVoxel.h>
#include<vtkWedge.h>
#include<vtkXMLDataElement.h>

// Declare exported functions
extern "C" vtkAMRDataInternals * vtkAMRDataInternals_new () ;
extern "C" void vtkAMRDataInternals_destructor (vtkAMRDataInternals * sself) ;
extern "C" void vtk_amr_data_internals_initialize(vtkAMRDataInternals* sself);
extern "C" bool vtk_amr_data_internals_empty(vtkAMRDataInternals* sself);
extern "C" unsigned int vtk_amr_data_internals_get_number_of_blocks(vtkAMRDataInternals* sself);
extern "C" vtkAdjacentVertexIterator * vtkAdjacentVertexIterator_new () ;
extern "C" void vtkAdjacentVertexIterator_destructor (vtkAdjacentVertexIterator * sself) ;
extern "C" long long vtk_adjacent_vertex_iterator_get_vertex(vtkAdjacentVertexIterator* sself);
extern "C" long long vtk_adjacent_vertex_iterator_next(vtkAdjacentVertexIterator* sself);
extern "C" bool vtk_adjacent_vertex_iterator_has_next(vtkAdjacentVertexIterator* sself);
extern "C" vtkAnimationScene * vtkAnimationScene_new () ;
extern "C" void vtkAnimationScene_destructor (vtkAnimationScene * sself) ;
extern "C" void vtk_animation_scene_set_play_mode(vtkAnimationScene* sself, int _arg);
extern "C" void vtk_animation_scene_set_mode_to_sequence(vtkAnimationScene* sself);
extern "C" void vtk_animation_scene_set_mode_to_real_time(vtkAnimationScene* sself);
extern "C" int vtk_animation_scene_get_play_mode(vtkAnimationScene* sself);
extern "C" void vtk_animation_scene_set_frame_rate(vtkAnimationScene* sself, double _arg);
extern "C" double vtk_animation_scene_get_frame_rate(vtkAnimationScene* sself);
extern "C" void vtk_animation_scene_remove_all_cues(vtkAnimationScene* sself);
extern "C" int vtk_animation_scene_get_number_of_cues(vtkAnimationScene* sself);
extern "C" void vtk_animation_scene_play(vtkAnimationScene* sself);
extern "C" void vtk_animation_scene_stop(vtkAnimationScene* sself);
extern "C" void vtk_animation_scene_set_loop(vtkAnimationScene* sself, int _arg);
extern "C" int vtk_animation_scene_get_loop(vtkAnimationScene* sself);
extern "C" void vtk_animation_scene_set_animation_time(vtkAnimationScene* sself, double time);
extern "C" void vtk_animation_scene_set_time_mode(vtkAnimationScene* sself, int mode);
extern "C" int vtk_animation_scene_is_in_play(vtkAnimationScene* sself);
extern "C" vtkAnnotation * vtkAnnotation_new () ;
extern "C" void vtkAnnotation_destructor (vtkAnnotation * sself) ;
extern "C" int vtk_annotation_get_data_object_type(vtkAnnotation* sself);
extern "C" void vtk_annotation_initialize(vtkAnnotation* sself);
extern "C" unsigned long vtk_annotation_get_m_time(vtkAnnotation* sself);
extern "C" vtkAnnotationLayers * vtkAnnotationLayers_new () ;
extern "C" void vtkAnnotationLayers_destructor (vtkAnnotationLayers * sself) ;
extern "C" int vtk_annotation_layers_get_data_object_type(vtkAnnotationLayers* sself);
extern "C" unsigned int vtk_annotation_layers_get_number_of_annotations(vtkAnnotationLayers* sself);
extern "C" void vtk_annotation_layers_initialize(vtkAnnotationLayers* sself);
extern "C" unsigned long vtk_annotation_layers_get_m_time(vtkAnnotationLayers* sself);
extern "C" vtkArrayData * vtkArrayData_new () ;
extern "C" void vtkArrayData_destructor (vtkArrayData * sself) ;
extern "C" void vtk_array_data_clear_arrays(vtkArrayData* sself);
extern "C" long long vtk_array_data_get_number_of_arrays(vtkArrayData* sself);
extern "C" int vtk_array_data_get_data_object_type(vtkArrayData* sself);
extern "C" vtkAttributesErrorMetric * vtkAttributesErrorMetric_new () ;
extern "C" void vtkAttributesErrorMetric_destructor (vtkAttributesErrorMetric * sself) ;
extern "C" double vtk_attributes_error_metric_get_absolute_attribute_tolerance(vtkAttributesErrorMetric* sself);
extern "C" void vtk_attributes_error_metric_set_absolute_attribute_tolerance(vtkAttributesErrorMetric* sself, double value);
extern "C" double vtk_attributes_error_metric_get_attribute_tolerance(vtkAttributesErrorMetric* sself);
extern "C" void vtk_attributes_error_metric_set_attribute_tolerance(vtkAttributesErrorMetric* sself, double value);
extern "C" vtkBSPCuts * vtkBSPCuts_new () ;
extern "C" void vtkBSPCuts_destructor (vtkBSPCuts * sself) ;
extern "C" int vtk_bsp_cuts_get_data_object_type(vtkBSPCuts* sself);
extern "C" int vtk_bsp_cuts_get_number_of_cuts(vtkBSPCuts* sself);
extern "C" void vtk_bsp_cuts_print_tree(vtkBSPCuts* sself);
extern "C" void vtk_bsp_cuts_print_arrays(vtkBSPCuts* sself);
extern "C" vtkBSPIntersections * vtkBSPIntersections_new () ;
extern "C" void vtkBSPIntersections_destructor (vtkBSPIntersections * sself) ;
extern "C" int vtk_bsp_intersections_get_number_of_regions(vtkBSPIntersections* sself);
extern "C" int vtk_bsp_intersections_intersects_sphere_2(vtkBSPIntersections* sself, int regionId, double x, double y, double z, double rSquared);
extern "C" int vtk_bsp_intersections_get_compute_intersections_using_data_bounds(vtkBSPIntersections* sself);
extern "C" void vtk_bsp_intersections_set_compute_intersections_using_data_bounds(vtkBSPIntersections* sself, int c);
extern "C" void vtk_bsp_intersections_compute_intersections_using_data_bounds_on(vtkBSPIntersections* sself);
extern "C" void vtk_bsp_intersections_compute_intersections_using_data_bounds_off(vtkBSPIntersections* sself);
extern "C" vtkBezierCurve * vtkBezierCurve_new () ;
extern "C" void vtkBezierCurve_destructor (vtkBezierCurve * sself) ;
extern "C" int vtk_bezier_curve_get_cell_type(vtkBezierCurve* sself);
extern "C" vtkBezierHexahedron * vtkBezierHexahedron_new () ;
extern "C" void vtkBezierHexahedron_destructor (vtkBezierHexahedron * sself) ;
extern "C" int vtk_bezier_hexahedron_get_cell_type(vtkBezierHexahedron* sself);
extern "C" vtkBezierInterpolation * vtkBezierInterpolation_new () ;
extern "C" void vtkBezierInterpolation_destructor (vtkBezierInterpolation * sself) ;
extern "C" vtkBezierQuadrilateral * vtkBezierQuadrilateral_new () ;
extern "C" void vtkBezierQuadrilateral_destructor (vtkBezierQuadrilateral * sself) ;
extern "C" int vtk_bezier_quadrilateral_get_cell_type(vtkBezierQuadrilateral* sself);
extern "C" vtkBezierTetra * vtkBezierTetra_new () ;
extern "C" void vtkBezierTetra_destructor (vtkBezierTetra * sself) ;
extern "C" int vtk_bezier_tetra_get_cell_type(vtkBezierTetra* sself);
extern "C" vtkBezierTriangle * vtkBezierTriangle_new () ;
extern "C" void vtkBezierTriangle_destructor (vtkBezierTriangle * sself) ;
extern "C" int vtk_bezier_triangle_get_cell_type(vtkBezierTriangle* sself);
extern "C" vtkBezierWedge * vtkBezierWedge_new () ;
extern "C" void vtkBezierWedge_destructor (vtkBezierWedge * sself) ;
extern "C" int vtk_bezier_wedge_get_cell_type(vtkBezierWedge* sself);
extern "C" vtkBiQuadraticQuad * vtkBiQuadraticQuad_new () ;
extern "C" void vtkBiQuadraticQuad_destructor (vtkBiQuadraticQuad * sself) ;
extern "C" int vtk_bi_quadratic_quad_get_cell_type(vtkBiQuadraticQuad* sself);
extern "C" int vtk_bi_quadratic_quad_get_cell_dimension(vtkBiQuadraticQuad* sself);
extern "C" int vtk_bi_quadratic_quad_get_number_of_edges(vtkBiQuadraticQuad* sself);
extern "C" int vtk_bi_quadratic_quad_get_number_of_faces(vtkBiQuadraticQuad* sself);
extern "C" vtkBiQuadraticQuadraticHexahedron * vtkBiQuadraticQuadraticHexahedron_new () ;
extern "C" void vtkBiQuadraticQuadraticHexahedron_destructor (vtkBiQuadraticQuadraticHexahedron * sself) ;
extern "C" int vtk_bi_quadratic_quadratic_hexahedron_get_cell_type(vtkBiQuadraticQuadraticHexahedron* sself);
extern "C" int vtk_bi_quadratic_quadratic_hexahedron_get_cell_dimension(vtkBiQuadraticQuadraticHexahedron* sself);
extern "C" int vtk_bi_quadratic_quadratic_hexahedron_get_number_of_edges(vtkBiQuadraticQuadraticHexahedron* sself);
extern "C" int vtk_bi_quadratic_quadratic_hexahedron_get_number_of_faces(vtkBiQuadraticQuadraticHexahedron* sself);
extern "C" vtkBiQuadraticQuadraticWedge * vtkBiQuadraticQuadraticWedge_new () ;
extern "C" void vtkBiQuadraticQuadraticWedge_destructor (vtkBiQuadraticQuadraticWedge * sself) ;
extern "C" int vtk_bi_quadratic_quadratic_wedge_get_cell_type(vtkBiQuadraticQuadraticWedge* sself);
extern "C" int vtk_bi_quadratic_quadratic_wedge_get_cell_dimension(vtkBiQuadraticQuadraticWedge* sself);
extern "C" int vtk_bi_quadratic_quadratic_wedge_get_number_of_edges(vtkBiQuadraticQuadraticWedge* sself);
extern "C" int vtk_bi_quadratic_quadratic_wedge_get_number_of_faces(vtkBiQuadraticQuadraticWedge* sself);
extern "C" vtkBiQuadraticTriangle * vtkBiQuadraticTriangle_new () ;
extern "C" void vtkBiQuadraticTriangle_destructor (vtkBiQuadraticTriangle * sself) ;
extern "C" int vtk_bi_quadratic_triangle_get_cell_type(vtkBiQuadraticTriangle* sself);
extern "C" int vtk_bi_quadratic_triangle_get_cell_dimension(vtkBiQuadraticTriangle* sself);
extern "C" int vtk_bi_quadratic_triangle_get_number_of_edges(vtkBiQuadraticTriangle* sself);
extern "C" int vtk_bi_quadratic_triangle_get_number_of_faces(vtkBiQuadraticTriangle* sself);
extern "C" vtkBox * vtkBox_new () ;
extern "C" void vtkBox_destructor (vtkBox * sself) ;
extern "C" void vtk_box_set_x_min(vtkBox* sself, double x, double y, double z);
extern "C" void vtk_box_get_x_min(vtkBox* sself, double& x, double& y, double& z);
extern "C" void vtk_box_set_x_max(vtkBox* sself, double x, double y, double z);
extern "C" void vtk_box_get_x_max(vtkBox* sself, double& x, double& y, double& z);
extern "C" void vtk_box_set_bounds(vtkBox* sself, double xMin, double xMax, double yMin, double yMax, double zMin, double zMax);
extern "C" void vtk_box_get_bounds(vtkBox* sself, double& xMin, double& xMax, double& yMin, double& yMax, double& zMin, double& zMax);
extern "C" vtkCellArray * vtkCellArray_new () ;
extern "C" void vtkCellArray_destructor (vtkCellArray * sself) ;
extern "C" int vtk_cell_array_allocate(vtkCellArray* sself, long long sz, long long ext);
extern "C" bool vtk_cell_array_allocate_estimate(vtkCellArray* sself, long long numCells, long long maxCellSize);
extern "C" bool vtk_cell_array_allocate_exact(vtkCellArray* sself, long long numCells, long long connectivitySize);
extern "C" bool vtk_cell_array_resize_exact(vtkCellArray* sself, long long numCells, long long connectivitySize);
extern "C" void vtk_cell_array_initialize(vtkCellArray* sself);
extern "C" void vtk_cell_array_reset(vtkCellArray* sself);
extern "C" void vtk_cell_array_squeeze(vtkCellArray* sself);
extern "C" bool vtk_cell_array_is_valid(vtkCellArray* sself);
extern "C" long long vtk_cell_array_get_number_of_cells(vtkCellArray* sself);
extern "C" long long vtk_cell_array_get_number_of_offsets(vtkCellArray* sself);
extern "C" long long vtk_cell_array_get_number_of_connectivity_ids(vtkCellArray* sself);
extern "C" bool vtk_cell_array_is_storage_64_bit(vtkCellArray* sself);
extern "C" bool vtk_cell_array_is_storage_shareable(vtkCellArray* sself);
extern "C" void vtk_cell_array_use_32_bit_storage(vtkCellArray* sself);
extern "C" void vtk_cell_array_use_64_bit_storage(vtkCellArray* sself);
extern "C" void vtk_cell_array_use_default_storage(vtkCellArray* sself);
extern "C" bool vtk_cell_array_can_convert_to_32_bit_storage(vtkCellArray* sself);
extern "C" bool vtk_cell_array_can_convert_to_64_bit_storage(vtkCellArray* sself);
extern "C" bool vtk_cell_array_can_convert_to_default_storage(vtkCellArray* sself);
extern "C" bool vtk_cell_array_convert_to_32_bit_storage(vtkCellArray* sself);
extern "C" bool vtk_cell_array_convert_to_64_bit_storage(vtkCellArray* sself);
extern "C" bool vtk_cell_array_convert_to_default_storage(vtkCellArray* sself);
extern "C" bool vtk_cell_array_convert_to_smallest_storage(vtkCellArray* sself);
extern "C" long long vtk_cell_array_is_homogeneous(vtkCellArray* sself);
extern "C" void vtk_cell_array_init_traversal(vtkCellArray* sself);
extern "C" long long vtk_cell_array_get_cell_size(vtkCellArray* sself, const long long cellId);
extern "C" void vtk_cell_array_insert_cell_point(vtkCellArray* sself, long long id);
extern "C" void vtk_cell_array_update_cell_count(vtkCellArray* sself, int npts);
extern "C" long long vtk_cell_array_get_traversal_cell_id(vtkCellArray* sself);
extern "C" void vtk_cell_array_set_traversal_cell_id(vtkCellArray* sself, long long cellId);
extern "C" void vtk_cell_array_reverse_cell_at_id(vtkCellArray* sself, long long cellId);
extern "C" int vtk_cell_array_get_max_cell_size(vtkCellArray* sself);
extern "C" unsigned long vtk_cell_array_get_actual_memory_size(vtkCellArray* sself);
extern "C" void vtk_cell_array_set_number_of_cells(vtkCellArray* sself, long long p0);
extern "C" long long vtk_cell_array_estimate_size(vtkCellArray* sself, long long numCells, int maxPtsPerCell);
extern "C" long long vtk_cell_array_get_size(vtkCellArray* sself);
extern "C" long long vtk_cell_array_get_number_of_connectivity_entries(vtkCellArray* sself);
extern "C" long long vtk_cell_array_get_insert_location(vtkCellArray* sself, int npts);
extern "C" long long vtk_cell_array_get_traversal_location(vtkCellArray* sself);
extern "C" void vtk_cell_array_set_traversal_location(vtkCellArray* sself, long long loc);
extern "C" void vtk_cell_array_reverse_cell(vtkCellArray* sself, long long loc);
extern "C" vtkCellArrayIterator * vtkCellArrayIterator_new () ;
extern "C" void vtkCellArrayIterator_destructor (vtkCellArrayIterator * sself) ;
extern "C" void vtk_cell_array_iterator_go_to_cell(vtkCellArrayIterator* sself, long long cellId);
extern "C" void vtk_cell_array_iterator_go_to_first_cell(vtkCellArrayIterator* sself);
extern "C" void vtk_cell_array_iterator_go_to_next_cell(vtkCellArrayIterator* sself);
extern "C" bool vtk_cell_array_iterator_is_done_with_traversal(vtkCellArrayIterator* sself);
extern "C" long long vtk_cell_array_iterator_get_current_cell_id(vtkCellArrayIterator* sself);
extern "C" void vtk_cell_array_iterator_reverse_current_cell(vtkCellArrayIterator* sself);
extern "C" vtkCellData * vtkCellData_new () ;
extern "C" void vtkCellData_destructor (vtkCellData * sself) ;
extern "C" vtkCellLinks * vtkCellLinks_new () ;
extern "C" void vtkCellLinks_destructor (vtkCellLinks * sself) ;
extern "C" void vtk_cell_links_allocate(vtkCellLinks* sself, long long numLinks, long long ext);
extern "C" void vtk_cell_links_initialize(vtkCellLinks* sself);
extern "C" long long vtk_cell_links_get_ncells(vtkCellLinks* sself, long long ptId);
extern "C" long long vtk_cell_links_insert_next_point(vtkCellLinks* sself, int numLinks);
extern "C" void vtk_cell_links_insert_next_cell_reference(vtkCellLinks* sself, long long ptId, long long cellId);
extern "C" void vtk_cell_links_delete_point(vtkCellLinks* sself, long long ptId);
extern "C" void vtk_cell_links_remove_cell_reference(vtkCellLinks* sself, long long cellId, long long ptId);
extern "C" void vtk_cell_links_add_cell_reference(vtkCellLinks* sself, long long cellId, long long ptId);
extern "C" void vtk_cell_links_resize_cell_list(vtkCellLinks* sself, long long ptId, int size);
extern "C" void vtk_cell_links_squeeze(vtkCellLinks* sself);
extern "C" void vtk_cell_links_reset(vtkCellLinks* sself);
extern "C" unsigned long vtk_cell_links_get_actual_memory_size(vtkCellLinks* sself);
extern "C" vtkCellLocator * vtkCellLocator_new () ;
extern "C" void vtkCellLocator_destructor (vtkCellLocator * sself) ;
extern "C" void vtk_cell_locator_set_number_of_cells_per_bucket(vtkCellLocator* sself, int N);
extern "C" int vtk_cell_locator_get_number_of_cells_per_bucket(vtkCellLocator* sself);
extern "C" int vtk_cell_locator_get_number_of_buckets(vtkCellLocator* sself);
extern "C" void vtk_cell_locator_free_search_structure(vtkCellLocator* sself);
extern "C" void vtk_cell_locator_build_locator(vtkCellLocator* sself);
extern "C" void vtk_cell_locator_build_locator_if_needed(vtkCellLocator* sself);
extern "C" void vtk_cell_locator_force_build_locator(vtkCellLocator* sself);
extern "C" void vtk_cell_locator_build_locator_internal(vtkCellLocator* sself);
extern "C" vtkCellLocatorStrategy * vtkCellLocatorStrategy_new () ;
extern "C" void vtkCellLocatorStrategy_destructor (vtkCellLocatorStrategy * sself) ;
extern "C" vtkCellTypes * vtkCellTypes_new () ;
extern "C" void vtkCellTypes_destructor (vtkCellTypes * sself) ;
extern "C" int vtk_cell_types_allocate(vtkCellTypes* sself, long long sz, long long ext);
extern "C" void vtk_cell_types_insert_cell(vtkCellTypes* sself, long long id, unsigned char type, long long loc);
extern "C" long long vtk_cell_types_insert_next_cell(vtkCellTypes* sself, unsigned char type, long long loc);
extern "C" long long vtk_cell_types_get_cell_location(vtkCellTypes* sself, long long cellId);
extern "C" void vtk_cell_types_delete_cell(vtkCellTypes* sself, long long cellId);
extern "C" long long vtk_cell_types_get_number_of_types(vtkCellTypes* sself);
extern "C" int vtk_cell_types_is_type(vtkCellTypes* sself, unsigned char type);
extern "C" long long vtk_cell_types_insert_next_type(vtkCellTypes* sself, unsigned char type);
extern "C" unsigned char vtk_cell_types_get_cell_type(vtkCellTypes* sself, long long cellId);
extern "C" void vtk_cell_types_squeeze(vtkCellTypes* sself);
extern "C" void vtk_cell_types_reset(vtkCellTypes* sself);
extern "C" unsigned long vtk_cell_types_get_actual_memory_size(vtkCellTypes* sself);
extern "C" const char* vtk_cell_types_get_class_name_from_type_id(vtkCellTypes* sself, int typeId);
extern "C" int vtk_cell_types_get_type_id_from_class_name(vtkCellTypes* sself, const char* classname);
extern "C" int vtk_cell_types_is_linear(vtkCellTypes* sself, unsigned char type);
extern "C" vtkClosestNPointsStrategy * vtkClosestNPointsStrategy_new () ;
extern "C" void vtkClosestNPointsStrategy_destructor (vtkClosestNPointsStrategy * sself) ;
extern "C" void vtk_closest_n_points_strategy_set_closest_n_points(vtkClosestNPointsStrategy* sself, int _arg);
extern "C" int vtk_closest_n_points_strategy_get_closest_n_points_min_value(vtkClosestNPointsStrategy* sself);
extern "C" int vtk_closest_n_points_strategy_get_closest_n_points_max_value(vtkClosestNPointsStrategy* sself);
extern "C" int vtk_closest_n_points_strategy_get_closest_n_points(vtkClosestNPointsStrategy* sself);
extern "C" vtkClosestPointStrategy * vtkClosestPointStrategy_new () ;
extern "C" void vtkClosestPointStrategy_destructor (vtkClosestPointStrategy * sself) ;
extern "C" vtkCone * vtkCone_new () ;
extern "C" void vtkCone_destructor (vtkCone * sself) ;
extern "C" void vtk_cone_set_angle(vtkCone* sself, double _arg);
extern "C" double vtk_cone_get_angle_min_value(vtkCone* sself);
extern "C" double vtk_cone_get_angle_max_value(vtkCone* sself);
extern "C" double vtk_cone_get_angle(vtkCone* sself);
extern "C" vtkConvexPointSet * vtkConvexPointSet_new () ;
extern "C" void vtkConvexPointSet_destructor (vtkConvexPointSet * sself) ;
extern "C" int vtk_convex_point_set_has_fixed_topology(vtkConvexPointSet* sself);
extern "C" int vtk_convex_point_set_get_cell_type(vtkConvexPointSet* sself);
extern "C" int vtk_convex_point_set_requires_initialization(vtkConvexPointSet* sself);
extern "C" int vtk_convex_point_set_get_number_of_edges(vtkConvexPointSet* sself);
extern "C" int vtk_convex_point_set_get_number_of_faces(vtkConvexPointSet* sself);
extern "C" int vtk_convex_point_set_is_primary_cell(vtkConvexPointSet* sself);
extern "C" vtkCubicLine * vtkCubicLine_new () ;
extern "C" void vtkCubicLine_destructor (vtkCubicLine * sself) ;
extern "C" int vtk_cubic_line_get_cell_type(vtkCubicLine* sself);
extern "C" int vtk_cubic_line_get_cell_dimension(vtkCubicLine* sself);
extern "C" int vtk_cubic_line_get_number_of_edges(vtkCubicLine* sself);
extern "C" int vtk_cubic_line_get_number_of_faces(vtkCubicLine* sself);
extern "C" vtkCylinder * vtkCylinder_new () ;
extern "C" void vtkCylinder_destructor (vtkCylinder * sself) ;
extern "C" void vtk_cylinder_set_radius(vtkCylinder* sself, double _arg);
extern "C" double vtk_cylinder_get_radius(vtkCylinder* sself);
extern "C" void vtk_cylinder_set_center(vtkCylinder* sself, double _arg1, double _arg2, double _arg3);
extern "C" void vtk_cylinder_set_axis(vtkCylinder* sself, double ax, double ay, double az);
extern "C" vtkDataAssembly * vtkDataAssembly_new () ;
extern "C" void vtkDataAssembly_destructor (vtkDataAssembly * sself) ;
extern "C" void vtk_data_assembly_initialize(vtkDataAssembly* sself);
extern "C" bool vtk_data_assembly_initialize_from_xml(vtkDataAssembly* sself, const char* xmlcontents);
extern "C" int vtk_data_assembly_get_root_node(vtkDataAssembly* sself);
extern "C" void vtk_data_assembly_set_root_node_name(vtkDataAssembly* sself, const char* name);
extern "C" const char* vtk_data_assembly_get_root_node_name(vtkDataAssembly* sself);
extern "C" int vtk_data_assembly_add_node(vtkDataAssembly* sself, const char* name, int parent);
extern "C" bool vtk_data_assembly_remove_node(vtkDataAssembly* sself, int id);
extern "C" void vtk_data_assembly_set_node_name(vtkDataAssembly* sself, int id, const char* name);
extern "C" const char* vtk_data_assembly_get_node_name(vtkDataAssembly* sself, int id);
extern "C" int vtk_data_assembly_get_first_node_by_path(vtkDataAssembly* sself, const char* path);
extern "C" bool vtk_data_assembly_add_data_set_index(vtkDataAssembly* sself, int id, unsigned int dataset_index);
extern "C" bool vtk_data_assembly_add_data_set_index_range(vtkDataAssembly* sself, int id, unsigned int index_start, int count);
extern "C" bool vtk_data_assembly_remove_data_set_index(vtkDataAssembly* sself, int id, unsigned int dataset_index);
extern "C" bool vtk_data_assembly_remove_all_data_set_indices(vtkDataAssembly* sself, int id, bool traverse_subtree);
extern "C" int vtk_data_assembly_find_first_node_with_name(vtkDataAssembly* sself, const char* name, int traversal_order);
extern "C" int vtk_data_assembly_get_number_of_children(vtkDataAssembly* sself, int parent);
extern "C" int vtk_data_assembly_get_child(vtkDataAssembly* sself, int parent, int index);
extern "C" int vtk_data_assembly_get_child_index(vtkDataAssembly* sself, int parent, int child);
extern "C" int vtk_data_assembly_get_parent(vtkDataAssembly* sself, int id);
extern "C" bool vtk_data_assembly_has_attribute(vtkDataAssembly* sself, int id, const char* name);
extern "C" void vtk_data_assembly_set_attribute(vtkDataAssembly* sself, int id, const char* name, const char* value);
extern "C" bool vtk_data_assembly_get_attribute(vtkDataAssembly* sself, int id, const char* name, const char* value);
extern "C" const char* vtk_data_assembly_get_attribute_or_default(vtkDataAssembly* sself, int id, const char* name, const char* default_value);
extern "C" bool vtk_data_assembly_is_node_name_valid(vtkDataAssembly* sself, const char* name);
extern "C" bool vtk_data_assembly_is_node_name_reserved(vtkDataAssembly* sself, const char* name);
extern "C" vtkDataAssemblyUtilities * vtkDataAssemblyUtilities_new () ;
extern "C" void vtkDataAssemblyUtilities_destructor (vtkDataAssemblyUtilities * sself) ;
extern "C" const char* vtk_data_assembly_utilities_hierarchy_name(vtkDataAssemblyUtilities* sself);
extern "C" vtkDataObject * vtkDataObject_new () ;
extern "C" void vtkDataObject_destructor (vtkDataObject * sself) ;
extern "C" unsigned long vtk_data_object_get_m_time(vtkDataObject* sself);
extern "C" void vtk_data_object_initialize(vtkDataObject* sself);
extern "C" void vtk_data_object_release_data(vtkDataObject* sself);
extern "C" int vtk_data_object_get_data_released(vtkDataObject* sself);
extern "C" void vtk_data_object_set_global_release_data_flag(vtkDataObject* sself, int val);
extern "C" void vtk_data_object_global_release_data_flag_on(vtkDataObject* sself);
extern "C" void vtk_data_object_global_release_data_flag_off(vtkDataObject* sself);
extern "C" int vtk_data_object_get_global_release_data_flag(vtkDataObject* sself);
extern "C" int vtk_data_object_get_data_object_type(vtkDataObject* sself);
extern "C" unsigned long vtk_data_object_get_update_time(vtkDataObject* sself);
extern "C" unsigned long vtk_data_object_get_actual_memory_size(vtkDataObject* sself);
extern "C" void vtk_data_object_data_has_been_generated(vtkDataObject* sself);
extern "C" void vtk_data_object_prepare_for_new_data(vtkDataObject* sself);
extern "C" int vtk_data_object_get_extent_type(vtkDataObject* sself);
extern "C" long long vtk_data_object_get_number_of_elements(vtkDataObject* sself, int type);
extern "C" const char* vtk_data_object_get_association_type_as_string(vtkDataObject* sself, int associationType);
extern "C" int vtk_data_object_get_association_type_from_string(vtkDataObject* sself, const char* associationName);
extern "C" vtkDataObjectCollection * vtkDataObjectCollection_new () ;
extern "C" void vtkDataObjectCollection_destructor (vtkDataObjectCollection * sself) ;
extern "C" int vtk_data_object_collection_get_number_of_items(vtkDataObjectCollection* sself);
extern "C" vtkDataObjectTreeIterator * vtkDataObjectTreeIterator_new () ;
extern "C" void vtkDataObjectTreeIterator_destructor (vtkDataObjectTreeIterator * sself) ;
extern "C" void vtk_data_object_tree_iterator_go_to_first_item(vtkDataObjectTreeIterator* sself);
extern "C" void vtk_data_object_tree_iterator_go_to_next_item(vtkDataObjectTreeIterator* sself);
extern "C" int vtk_data_object_tree_iterator_is_done_with_traversal(vtkDataObjectTreeIterator* sself);
extern "C" int vtk_data_object_tree_iterator_has_current_meta_data(vtkDataObjectTreeIterator* sself);
extern "C" unsigned int vtk_data_object_tree_iterator_get_current_flat_index(vtkDataObjectTreeIterator* sself);
extern "C" void vtk_data_object_tree_iterator_set_visit_only_leaves(vtkDataObjectTreeIterator* sself, int _arg);
extern "C" int vtk_data_object_tree_iterator_get_visit_only_leaves(vtkDataObjectTreeIterator* sself);
extern "C" void vtk_data_object_tree_iterator_visit_only_leaves_on(vtkDataObjectTreeIterator* sself);
extern "C" void vtk_data_object_tree_iterator_visit_only_leaves_off(vtkDataObjectTreeIterator* sself);
extern "C" void vtk_data_object_tree_iterator_set_traverse_sub_tree(vtkDataObjectTreeIterator* sself, int _arg);
extern "C" int vtk_data_object_tree_iterator_get_traverse_sub_tree(vtkDataObjectTreeIterator* sself);
extern "C" void vtk_data_object_tree_iterator_traverse_sub_tree_on(vtkDataObjectTreeIterator* sself);
extern "C" void vtk_data_object_tree_iterator_traverse_sub_tree_off(vtkDataObjectTreeIterator* sself);
extern "C" vtkDataObjectTypes * vtkDataObjectTypes_new () ;
extern "C" void vtkDataObjectTypes_destructor (vtkDataObjectTypes * sself) ;
extern "C" const char* vtk_data_object_types_get_class_name_from_type_id(vtkDataObjectTypes* sself, int typeId);
extern "C" int vtk_data_object_types_get_type_id_from_class_name(vtkDataObjectTypes* sself, const char* classname);
extern "C" bool vtk_data_object_types_type_id_is_a(vtkDataObjectTypes* sself, int typeId, int targetTypeId);
extern "C" int vtk_data_object_types_get_common_base_type_id(vtkDataObjectTypes* sself, int typeA, int typeB);
extern "C" vtkDataSetAttributes * vtkDataSetAttributes_new () ;
extern "C" void vtkDataSetAttributes_destructor (vtkDataSetAttributes * sself) ;
extern "C" void vtk_data_set_attributes_initialize(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_update(vtkDataSetAttributes* sself);
extern "C" const char* vtk_data_set_attributes_ghost_array_name(vtkDataSetAttributes* sself);
extern "C" int vtk_data_set_attributes_set_active_scalars(vtkDataSetAttributes* sself, const char* name);
extern "C" int vtk_data_set_attributes_set_active_vectors(vtkDataSetAttributes* sself, const char* name);
extern "C" int vtk_data_set_attributes_set_active_normals(vtkDataSetAttributes* sself, const char* name);
extern "C" int vtk_data_set_attributes_set_active_tangents(vtkDataSetAttributes* sself, const char* name);
extern "C" int vtk_data_set_attributes_set_active_t_coords(vtkDataSetAttributes* sself, const char* name);
extern "C" int vtk_data_set_attributes_set_active_tensors(vtkDataSetAttributes* sself, const char* name);
extern "C" int vtk_data_set_attributes_set_active_global_ids(vtkDataSetAttributes* sself, const char* name);
extern "C" int vtk_data_set_attributes_set_active_pedigree_ids(vtkDataSetAttributes* sself, const char* name);
extern "C" int vtk_data_set_attributes_set_active_rational_weights(vtkDataSetAttributes* sself, const char* name);
extern "C" int vtk_data_set_attributes_set_active_higher_order_degrees(vtkDataSetAttributes* sself, const char* name);
extern "C" int vtk_data_set_attributes_set_active_attribute(vtkDataSetAttributes* sself, const char* name, int attributeType);
extern "C" int vtk_data_set_attributes_is_array_an_attribute(vtkDataSetAttributes* sself, int idx);
extern "C" const char* vtk_data_set_attributes_get_attribute_type_as_string(vtkDataSetAttributes* sself, int attributeType);
extern "C" const char* vtk_data_set_attributes_get_long_attribute_type_as_string(vtkDataSetAttributes* sself, int attributeType);
extern "C" void vtk_data_set_attributes_set_copy_attribute(vtkDataSetAttributes* sself, int index, int value, int ctype);
extern "C" int vtk_data_set_attributes_get_copy_attribute(vtkDataSetAttributes* sself, int index, int ctype);
extern "C" void vtk_data_set_attributes_set_copy_scalars(vtkDataSetAttributes* sself, int i, int ctype);
extern "C" int vtk_data_set_attributes_get_copy_scalars(vtkDataSetAttributes* sself, int ctype);
extern "C" void vtk_data_set_attributes_copy_scalars_on(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_copy_scalars_off(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_set_copy_vectors(vtkDataSetAttributes* sself, int i, int ctype);
extern "C" int vtk_data_set_attributes_get_copy_vectors(vtkDataSetAttributes* sself, int ctype);
extern "C" void vtk_data_set_attributes_copy_vectors_on(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_copy_vectors_off(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_set_copy_normals(vtkDataSetAttributes* sself, int i, int ctype);
extern "C" int vtk_data_set_attributes_get_copy_normals(vtkDataSetAttributes* sself, int ctype);
extern "C" void vtk_data_set_attributes_copy_normals_on(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_copy_normals_off(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_set_copy_tangents(vtkDataSetAttributes* sself, int i, int ctype);
extern "C" int vtk_data_set_attributes_get_copy_tangents(vtkDataSetAttributes* sself, int ctype);
extern "C" void vtk_data_set_attributes_copy_tangents_on(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_copy_tangents_off(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_set_copy_t_coords(vtkDataSetAttributes* sself, int i, int ctype);
extern "C" int vtk_data_set_attributes_get_copy_t_coords(vtkDataSetAttributes* sself, int ctype);
extern "C" void vtk_data_set_attributes_copy_t_coords_on(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_copy_t_coords_off(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_set_copy_tensors(vtkDataSetAttributes* sself, int i, int ctype);
extern "C" int vtk_data_set_attributes_get_copy_tensors(vtkDataSetAttributes* sself, int ctype);
extern "C" void vtk_data_set_attributes_copy_tensors_on(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_copy_tensors_off(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_set_copy_global_ids(vtkDataSetAttributes* sself, int i, int ctype);
extern "C" int vtk_data_set_attributes_get_copy_global_ids(vtkDataSetAttributes* sself, int ctype);
extern "C" void vtk_data_set_attributes_copy_global_ids_on(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_copy_global_ids_off(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_set_copy_pedigree_ids(vtkDataSetAttributes* sself, int i, int ctype);
extern "C" int vtk_data_set_attributes_get_copy_pedigree_ids(vtkDataSetAttributes* sself, int ctype);
extern "C" void vtk_data_set_attributes_copy_pedigree_ids_on(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_copy_pedigree_ids_off(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_set_copy_rational_weights(vtkDataSetAttributes* sself, int i, int ctype);
extern "C" int vtk_data_set_attributes_get_copy_rational_weights(vtkDataSetAttributes* sself, int ctype);
extern "C" void vtk_data_set_attributes_copy_rational_weights_on(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_copy_rational_weights_off(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_set_copy_higher_order_degrees(vtkDataSetAttributes* sself, int i, int ctype);
extern "C" int vtk_data_set_attributes_get_copy_higher_order_degrees(vtkDataSetAttributes* sself, int ctype);
extern "C" void vtk_data_set_attributes_copy_higher_order_degrees_on(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_copy_higher_order_degrees_off(vtkDataSetAttributes* sself);
extern "C" void vtk_data_set_attributes_copy_all_on(vtkDataSetAttributes* sself, int ctype);
extern "C" void vtk_data_set_attributes_copy_all_off(vtkDataSetAttributes* sself, int ctype);
extern "C" vtkDataSetCellIterator * vtkDataSetCellIterator_new () ;
extern "C" void vtkDataSetCellIterator_destructor (vtkDataSetCellIterator * sself) ;
extern "C" bool vtk_data_set_cell_iterator_is_done_with_traversal(vtkDataSetCellIterator* sself);
extern "C" long long vtk_data_set_cell_iterator_get_cell_id(vtkDataSetCellIterator* sself);
extern "C" vtkDataSetCollection * vtkDataSetCollection_new () ;
extern "C" void vtkDataSetCollection_destructor (vtkDataSetCollection * sself) ;
extern "C" int vtk_data_set_collection_get_number_of_items(vtkDataSetCollection* sself);
extern "C" vtkDirectedAcyclicGraph * vtkDirectedAcyclicGraph_new () ;
extern "C" void vtkDirectedAcyclicGraph_destructor (vtkDirectedAcyclicGraph * sself) ;
extern "C" vtkDirectedGraph * vtkDirectedGraph_new () ;
extern "C" void vtkDirectedGraph_destructor (vtkDirectedGraph * sself) ;
extern "C" vtkEdgeListIterator * vtkEdgeListIterator_new () ;
extern "C" void vtkEdgeListIterator_destructor (vtkEdgeListIterator * sself) ;
extern "C" bool vtk_edge_list_iterator_has_next(vtkEdgeListIterator* sself);
extern "C" vtkEdgeTable * vtkEdgeTable_new () ;
extern "C" void vtkEdgeTable_destructor (vtkEdgeTable * sself) ;
extern "C" void vtk_edge_table_initialize(vtkEdgeTable* sself);
extern "C" int vtk_edge_table_init_edge_insertion(vtkEdgeTable* sself, long long numPoints, int storeAttributes);
extern "C" long long vtk_edge_table_insert_edge(vtkEdgeTable* sself, long long p1, long long p2);
extern "C" long long vtk_edge_table_is_edge(vtkEdgeTable* sself, long long p1, long long p2);
extern "C" long long vtk_edge_table_get_number_of_edges(vtkEdgeTable* sself);
extern "C" void vtk_edge_table_init_traversal(vtkEdgeTable* sself);
extern "C" long long vtk_edge_table_get_next_edge(vtkEdgeTable* sself, long long& p1, long long& p2);
extern "C" void vtk_edge_table_reset(vtkEdgeTable* sself);
extern "C" vtkEmptyCell * vtkEmptyCell_new () ;
extern "C" void vtkEmptyCell_destructor (vtkEmptyCell * sself) ;
extern "C" int vtk_empty_cell_get_cell_type(vtkEmptyCell* sself);
extern "C" int vtk_empty_cell_get_cell_dimension(vtkEmptyCell* sself);
extern "C" int vtk_empty_cell_get_number_of_edges(vtkEmptyCell* sself);
extern "C" int vtk_empty_cell_get_number_of_faces(vtkEmptyCell* sself);
extern "C" vtkExplicitStructuredGrid * vtkExplicitStructuredGrid_new () ;
extern "C" void vtkExplicitStructuredGrid_destructor (vtkExplicitStructuredGrid * sself) ;
extern "C" int vtk_explicit_structured_grid_get_data_object_type(vtkExplicitStructuredGrid* sself);
extern "C" void vtk_explicit_structured_grid_initialize(vtkExplicitStructuredGrid* sself);
extern "C" int vtk_explicit_structured_grid_get_cell_type(vtkExplicitStructuredGrid* sself, long long cellId);
extern "C" int vtk_explicit_structured_grid_get_data_dimension(vtkExplicitStructuredGrid* sself);
extern "C" void vtk_explicit_structured_grid_set_dimensions(vtkExplicitStructuredGrid* sself, int i, int j, int k);
extern "C" int vtk_explicit_structured_grid_get_extent_type(vtkExplicitStructuredGrid* sself);
extern "C" void vtk_explicit_structured_grid_set_extent(vtkExplicitStructuredGrid* sself, int x0, int x1, int y0, int y1, int z0, int z1);
extern "C" void vtk_explicit_structured_grid_build_links(vtkExplicitStructuredGrid* sself);
extern "C" void vtk_explicit_structured_grid_compute_cell_structured_coords(vtkExplicitStructuredGrid* sself, long long cellId, int& i, int& j, int& k, bool adjustForExtent);
extern "C" long long vtk_explicit_structured_grid_compute_cell_id(vtkExplicitStructuredGrid* sself, int i, int j, int k, bool adjustForExtent);
extern "C" void vtk_explicit_structured_grid_compute_faces_connectivity_flags_array(vtkExplicitStructuredGrid* sself);
extern "C" void vtk_explicit_structured_grid_set_faces_connectivity_flags_array_name(vtkExplicitStructuredGrid* sself, const char* _arg);
extern "C" void vtk_explicit_structured_grid_blank_cell(vtkExplicitStructuredGrid* sself, long long cellId);
extern "C" void vtk_explicit_structured_grid_un_blank_cell(vtkExplicitStructuredGrid* sself, long long cellId);
extern "C" bool vtk_explicit_structured_grid_has_any_blank_cells(vtkExplicitStructuredGrid* sself);
extern "C" unsigned char vtk_explicit_structured_grid_is_cell_visible(vtkExplicitStructuredGrid* sself, long long cellId);
extern "C" unsigned char vtk_explicit_structured_grid_is_cell_ghost(vtkExplicitStructuredGrid* sself, long long cellId);
extern "C" bool vtk_explicit_structured_grid_has_any_ghost_cells(vtkExplicitStructuredGrid* sself);
extern "C" unsigned long vtk_explicit_structured_grid_get_actual_memory_size(vtkExplicitStructuredGrid* sself);
extern "C" void vtk_explicit_structured_grid_check_and_reorder_faces(vtkExplicitStructuredGrid* sself);
extern "C" vtkExtractStructuredGridHelper * vtkExtractStructuredGridHelper_new () ;
extern "C" void vtkExtractStructuredGridHelper_destructor (vtkExtractStructuredGridHelper * sself) ;
extern "C" bool vtk_extract_structured_grid_helper_is_valid(vtkExtractStructuredGridHelper* sself);
extern "C" int vtk_extract_structured_grid_helper_get_size(vtkExtractStructuredGridHelper* sself, const int dim);
extern "C" int vtk_extract_structured_grid_helper_get_mapped_index(vtkExtractStructuredGridHelper* sself, int dim, int outIdx);
extern "C" int vtk_extract_structured_grid_helper_get_mapped_index_from_extent_value(vtkExtractStructuredGridHelper* sself, int dim, int outExtVal);
extern "C" int vtk_extract_structured_grid_helper_get_mapped_extent_value(vtkExtractStructuredGridHelper* sself, int dim, int outExtVal);
extern "C" int vtk_extract_structured_grid_helper_get_mapped_extent_value_from_index(vtkExtractStructuredGridHelper* sself, int dim, int outIdx);
extern "C" vtkFieldData * vtkFieldData_new () ;
extern "C" void vtkFieldData_destructor (vtkFieldData * sself) ;
extern "C" void vtk_field_data_initialize(vtkFieldData* sself);
extern "C" int vtk_field_data_allocate(vtkFieldData* sself, long long sz, long long ext);
extern "C" void vtk_field_data_allocate_arrays(vtkFieldData* sself, int num);
extern "C" int vtk_field_data_get_number_of_arrays(vtkFieldData* sself);
extern "C" void vtk_field_data_null_data(vtkFieldData* sself, long long id);
extern "C" void vtk_field_data_remove_array(vtkFieldData* sself, const char* name);
extern "C" int vtk_field_data_has_array(vtkFieldData* sself, const char* name);
extern "C" const char* vtk_field_data_get_array_name(vtkFieldData* sself, int i);
extern "C" void vtk_field_data_copy_field_on(vtkFieldData* sself, const char* name);
extern "C" void vtk_field_data_copy_field_off(vtkFieldData* sself, const char* name);
extern "C" void vtk_field_data_copy_all_on(vtkFieldData* sself, int unused);
extern "C" void vtk_field_data_copy_all_off(vtkFieldData* sself, int unused);
extern "C" void vtk_field_data_squeeze(vtkFieldData* sself);
extern "C" void vtk_field_data_reset(vtkFieldData* sself);
extern "C" unsigned long vtk_field_data_get_actual_memory_size(vtkFieldData* sself);
extern "C" unsigned long vtk_field_data_get_m_time(vtkFieldData* sself);
extern "C" int vtk_field_data_get_array_containing_component(vtkFieldData* sself, int i, int& arrayComp);
extern "C" int vtk_field_data_get_number_of_components(vtkFieldData* sself);
extern "C" long long vtk_field_data_get_number_of_tuples(vtkFieldData* sself);
extern "C" void vtk_field_data_set_number_of_tuples(vtkFieldData* sself, const long long number);
extern "C" vtkGenericAttributeCollection * vtkGenericAttributeCollection_new () ;
extern "C" void vtkGenericAttributeCollection_destructor (vtkGenericAttributeCollection * sself) ;
extern "C" int vtk_generic_attribute_collection_get_number_of_attributes(vtkGenericAttributeCollection* sself);
extern "C" int vtk_generic_attribute_collection_get_number_of_components(vtkGenericAttributeCollection* sself);
extern "C" int vtk_generic_attribute_collection_get_number_of_point_centered_components(vtkGenericAttributeCollection* sself);
extern "C" int vtk_generic_attribute_collection_get_max_number_of_components(vtkGenericAttributeCollection* sself);
extern "C" unsigned long vtk_generic_attribute_collection_get_actual_memory_size(vtkGenericAttributeCollection* sself);
extern "C" int vtk_generic_attribute_collection_is_empty(vtkGenericAttributeCollection* sself);
extern "C" int vtk_generic_attribute_collection_find_attribute(vtkGenericAttributeCollection* sself, const char* name);
extern "C" int vtk_generic_attribute_collection_get_attribute_index(vtkGenericAttributeCollection* sself, int i);
extern "C" void vtk_generic_attribute_collection_remove_attribute(vtkGenericAttributeCollection* sself, int i);
extern "C" void vtk_generic_attribute_collection_reset(vtkGenericAttributeCollection* sself);
extern "C" unsigned long vtk_generic_attribute_collection_get_m_time(vtkGenericAttributeCollection* sself);
extern "C" int vtk_generic_attribute_collection_get_active_attribute(vtkGenericAttributeCollection* sself);
extern "C" int vtk_generic_attribute_collection_get_active_component(vtkGenericAttributeCollection* sself);
extern "C" void vtk_generic_attribute_collection_set_active_attribute(vtkGenericAttributeCollection* sself, int attribute, int component);
extern "C" int vtk_generic_attribute_collection_get_number_of_attributes_to_interpolate(vtkGenericAttributeCollection* sself);
extern "C" void vtk_generic_attribute_collection_set_attributes_to_interpolate_to_all(vtkGenericAttributeCollection* sself);
extern "C" vtkGenericCell * vtkGenericCell_new () ;
extern "C" void vtkGenericCell_destructor (vtkGenericCell * sself) ;
extern "C" int vtk_generic_cell_get_cell_type(vtkGenericCell* sself);
extern "C" int vtk_generic_cell_get_cell_dimension(vtkGenericCell* sself);
extern "C" int vtk_generic_cell_get_number_of_edges(vtkGenericCell* sself);
extern "C" int vtk_generic_cell_get_number_of_faces(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type(vtkGenericCell* sself, int cellType);
extern "C" void vtk_generic_cell_set_cell_type_to_empty_cell(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_vertex(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_poly_vertex(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_line(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_poly_line(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_triangle(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_triangle_strip(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_polygon(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_pixel(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_quad(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_tetra(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_voxel(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_hexahedron(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_wedge(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_pyramid(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_pentagonal_prism(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_hexagonal_prism(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_polyhedron(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_convex_point_set(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_edge(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_cubic_line(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_triangle(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_bi_quadratic_triangle(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_quad(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_polygon(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_tetra(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_hexahedron(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_wedge(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_pyramid(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_linear_quad(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_bi_quadratic_quad(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_linear_wedge(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_bi_quadratic_quadratic_wedge(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_tri_quadratic_hexahedron(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_tri_quadratic_pyramid(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_bi_quadratic_quadratic_hexahedron(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_triangle(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_tetra(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_curve(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_quadrilateral(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_hexahedron(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_wedge(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_triangle(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_tetra(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_curve(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_quadrilateral(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_hexahedron(vtkGenericCell* sself);
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_wedge(vtkGenericCell* sself);
extern "C" vtkGenericEdgeTable * vtkGenericEdgeTable_new () ;
extern "C" void vtkGenericEdgeTable_destructor (vtkGenericEdgeTable * sself) ;
extern "C" void vtk_generic_edge_table_insert_edge(vtkGenericEdgeTable* sself, long long e1, long long e2, long long cellId, int ref, long long& ptId);
extern "C" int vtk_generic_edge_table_remove_edge(vtkGenericEdgeTable* sself, long long e1, long long e2);
extern "C" int vtk_generic_edge_table_check_edge(vtkGenericEdgeTable* sself, long long e1, long long e2, long long& ptId);
extern "C" int vtk_generic_edge_table_increment_edge_reference_count(vtkGenericEdgeTable* sself, long long e1, long long e2, long long cellId);
extern "C" int vtk_generic_edge_table_check_edge_reference_count(vtkGenericEdgeTable* sself, long long e1, long long e2);
extern "C" void vtk_generic_edge_table_initialize(vtkGenericEdgeTable* sself, long long start);
extern "C" int vtk_generic_edge_table_get_number_of_components(vtkGenericEdgeTable* sself);
extern "C" void vtk_generic_edge_table_set_number_of_components(vtkGenericEdgeTable* sself, int count);
extern "C" int vtk_generic_edge_table_check_point(vtkGenericEdgeTable* sself, long long ptId);
extern "C" void vtk_generic_edge_table_remove_point(vtkGenericEdgeTable* sself, long long ptId);
extern "C" void vtk_generic_edge_table_increment_point_reference_count(vtkGenericEdgeTable* sself, long long ptId);
extern "C" void vtk_generic_edge_table_dump_table(vtkGenericEdgeTable* sself);
extern "C" void vtk_generic_edge_table_load_factor(vtkGenericEdgeTable* sself);
extern "C" vtkGenericInterpolatedVelocityField * vtkGenericInterpolatedVelocityField_new () ;
extern "C" void vtkGenericInterpolatedVelocityField_destructor (vtkGenericInterpolatedVelocityField * sself) ;
extern "C" void vtk_generic_interpolated_velocity_field_clear_last_cell(vtkGenericInterpolatedVelocityField* sself);
extern "C" int vtk_generic_interpolated_velocity_field_get_caching(vtkGenericInterpolatedVelocityField* sself);
extern "C" void vtk_generic_interpolated_velocity_field_set_caching(vtkGenericInterpolatedVelocityField* sself, int _arg);
extern "C" void vtk_generic_interpolated_velocity_field_caching_on(vtkGenericInterpolatedVelocityField* sself);
extern "C" void vtk_generic_interpolated_velocity_field_caching_off(vtkGenericInterpolatedVelocityField* sself);
extern "C" int vtk_generic_interpolated_velocity_field_get_cache_hit(vtkGenericInterpolatedVelocityField* sself);
extern "C" int vtk_generic_interpolated_velocity_field_get_cache_miss(vtkGenericInterpolatedVelocityField* sself);
extern "C" void vtk_generic_interpolated_velocity_field_select_vectors(vtkGenericInterpolatedVelocityField* sself, const char* fieldName);
extern "C" vtkGeometricErrorMetric * vtkGeometricErrorMetric_new () ;
extern "C" void vtkGeometricErrorMetric_destructor (vtkGeometricErrorMetric * sself) ;
extern "C" double vtk_geometric_error_metric_get_absolute_geometric_tolerance(vtkGeometricErrorMetric* sself);
extern "C" void vtk_geometric_error_metric_set_absolute_geometric_tolerance(vtkGeometricErrorMetric* sself, double value);
extern "C" int vtk_geometric_error_metric_get_relative(vtkGeometricErrorMetric* sself);
extern "C" vtkGraphEdge * vtkGraphEdge_new () ;
extern "C" void vtkGraphEdge_destructor (vtkGraphEdge * sself) ;
extern "C" void vtk_graph_edge_set_source(vtkGraphEdge* sself, long long _arg);
extern "C" long long vtk_graph_edge_get_source(vtkGraphEdge* sself);
extern "C" void vtk_graph_edge_set_target(vtkGraphEdge* sself, long long _arg);
extern "C" long long vtk_graph_edge_get_target(vtkGraphEdge* sself);
extern "C" void vtk_graph_edge_set_id(vtkGraphEdge* sself, long long _arg);
extern "C" long long vtk_graph_edge_get_id(vtkGraphEdge* sself);
extern "C" vtkGraphInternals * vtkGraphInternals_new () ;
extern "C" void vtkGraphInternals_destructor (vtkGraphInternals * sself) ;
extern "C" vtkHexagonalPrism * vtkHexagonalPrism_new () ;
extern "C" void vtkHexagonalPrism_destructor (vtkHexagonalPrism * sself) ;
extern "C" int vtk_hexagonal_prism_get_cell_type(vtkHexagonalPrism* sself);
extern "C" int vtk_hexagonal_prism_get_number_of_edges(vtkHexagonalPrism* sself);
extern "C" int vtk_hexagonal_prism_get_number_of_faces(vtkHexagonalPrism* sself);
extern "C" vtkHexahedron * vtkHexahedron_new () ;
extern "C" void vtkHexahedron_destructor (vtkHexahedron * sself) ;
extern "C" int vtk_hexahedron_get_cell_type(vtkHexahedron* sself);
extern "C" int vtk_hexahedron_get_number_of_edges(vtkHexahedron* sself);
extern "C" int vtk_hexahedron_get_number_of_faces(vtkHexahedron* sself);
extern "C" vtkHierarchicalBoxDataIterator * vtkHierarchicalBoxDataIterator_new () ;
extern "C" void vtkHierarchicalBoxDataIterator_destructor (vtkHierarchicalBoxDataIterator * sself) ;
extern "C" vtkHierarchicalBoxDataSet * vtkHierarchicalBoxDataSet_new () ;
extern "C" void vtkHierarchicalBoxDataSet_destructor (vtkHierarchicalBoxDataSet * sself) ;
extern "C" vtkHyperTreeGrid * vtkHyperTreeGrid_new () ;
extern "C" void vtkHyperTreeGrid_destructor (vtkHyperTreeGrid * sself) ;
extern "C" void vtk_hyper_tree_grid_set_mode_squeeze(vtkHyperTreeGrid* sself, const char* _arg);
extern "C" void vtk_hyper_tree_grid_squeeze(vtkHyperTreeGrid* sself);
extern "C" int vtk_hyper_tree_grid_get_data_object_type(vtkHyperTreeGrid* sself);
extern "C" void vtk_hyper_tree_grid_set_dimensions(vtkHyperTreeGrid* sself, unsigned int i, unsigned int j, unsigned int k);
extern "C" void vtk_hyper_tree_grid_set_extent(vtkHyperTreeGrid* sself, int x1, int x2, int y1, int y2, int z1, int z2);
extern "C" unsigned int vtk_hyper_tree_grid_get_dimension(vtkHyperTreeGrid* sself);
extern "C" void vtk_hyper_tree_grid_get_1_d_axis(vtkHyperTreeGrid* sself, unsigned int& axis);
extern "C" void vtk_hyper_tree_grid_get_2_d_axes(vtkHyperTreeGrid* sself, unsigned int& axis1, unsigned int& axis2);
extern "C" unsigned int vtk_hyper_tree_grid_get_number_of_children(vtkHyperTreeGrid* sself);
extern "C" void vtk_hyper_tree_grid_set_transposed_root_indexing(vtkHyperTreeGrid* sself, bool _arg);
extern "C" bool vtk_hyper_tree_grid_get_transposed_root_indexing(vtkHyperTreeGrid* sself);
extern "C" void vtk_hyper_tree_grid_set_indexing_mode_to_kji(vtkHyperTreeGrid* sself);
extern "C" void vtk_hyper_tree_grid_set_indexing_mode_to_ijk(vtkHyperTreeGrid* sself);
extern "C" unsigned int vtk_hyper_tree_grid_get_orientation(vtkHyperTreeGrid* sself);
extern "C" bool vtk_hyper_tree_grid_get_freeze_state(vtkHyperTreeGrid* sself);
extern "C" void vtk_hyper_tree_grid_set_branch_factor(vtkHyperTreeGrid* sself, unsigned int p0);
extern "C" unsigned int vtk_hyper_tree_grid_get_branch_factor(vtkHyperTreeGrid* sself);
extern "C" long long vtk_hyper_tree_grid_get_max_number_of_trees(vtkHyperTreeGrid* sself);
extern "C" long long vtk_hyper_tree_grid_get_number_of_vertices(vtkHyperTreeGrid* sself);
extern "C" long long vtk_hyper_tree_grid_get_number_of_non_empty_trees(vtkHyperTreeGrid* sself);
extern "C" long long vtk_hyper_tree_grid_get_number_of_leaves(vtkHyperTreeGrid* sself);
extern "C" unsigned int vtk_hyper_tree_grid_get_number_of_levels(vtkHyperTreeGrid* sself, long long p0);
extern "C" void vtk_hyper_tree_grid_set_fixed_coordinates(vtkHyperTreeGrid* sself, unsigned int axis, double value);
extern "C" bool vtk_hyper_tree_grid_has_mask(vtkHyperTreeGrid* sself);
extern "C" void vtk_hyper_tree_grid_set_has_interface(vtkHyperTreeGrid* sself, bool _arg);
extern "C" bool vtk_hyper_tree_grid_get_has_interface(vtkHyperTreeGrid* sself);
extern "C" void vtk_hyper_tree_grid_has_interface_on(vtkHyperTreeGrid* sself);
extern "C" void vtk_hyper_tree_grid_has_interface_off(vtkHyperTreeGrid* sself);
extern "C" void vtk_hyper_tree_grid_set_interface_normals_name(vtkHyperTreeGrid* sself, const char* _arg);
extern "C" void vtk_hyper_tree_grid_set_interface_intercepts_name(vtkHyperTreeGrid* sself, const char* _arg);
extern "C" void vtk_hyper_tree_grid_set_depth_limiter(vtkHyperTreeGrid* sself, unsigned int _arg);
extern "C" unsigned int vtk_hyper_tree_grid_get_depth_limiter(vtkHyperTreeGrid* sself);
extern "C" unsigned int vtk_hyper_tree_grid_find_dichotomic_x(vtkHyperTreeGrid* sself, double value);
extern "C" unsigned int vtk_hyper_tree_grid_find_dichotomic_y(vtkHyperTreeGrid* sself, double value);
extern "C" unsigned int vtk_hyper_tree_grid_find_dichotomic_z(vtkHyperTreeGrid* sself, double value);
extern "C" void vtk_hyper_tree_grid_initialize(vtkHyperTreeGrid* sself);
extern "C" int vtk_hyper_tree_grid_get_extent_type(vtkHyperTreeGrid* sself);
extern "C" unsigned long vtk_hyper_tree_grid_get_actual_memory_size_bytes(vtkHyperTreeGrid* sself);
extern "C" unsigned long vtk_hyper_tree_grid_get_actual_memory_size(vtkHyperTreeGrid* sself);
extern "C" unsigned int vtk_hyper_tree_grid_get_child_mask(vtkHyperTreeGrid* sself, unsigned int p0);
extern "C" void vtk_hyper_tree_grid_get_index_from_level_zero_coordinates(vtkHyperTreeGrid* sself, long long& p0, unsigned int p1, unsigned int p2, unsigned int p3);
extern "C" long long vtk_hyper_tree_grid_get_shifted_level_zero_index(vtkHyperTreeGrid* sself, long long p0, unsigned int p1, unsigned int p2, unsigned int p3);
extern "C" void vtk_hyper_tree_grid_get_level_zero_coordinates_from_index(vtkHyperTreeGrid* sself, long long p0, unsigned int& p1, unsigned int& p2, unsigned int& p3);
extern "C" long long vtk_hyper_tree_grid_get_global_node_index_max(vtkHyperTreeGrid* sself);
extern "C" void vtk_hyper_tree_grid_initialize_local_index_node(vtkHyperTreeGrid* sself);
extern "C" bool vtk_hyper_tree_grid_has_any_ghost_cells(vtkHyperTreeGrid* sself);
extern "C" long long vtk_hyper_tree_grid_get_number_of_elements(vtkHyperTreeGrid* sself, int type);
extern "C" vtkHyperTreeGridNonOrientedCursor * vtkHyperTreeGridNonOrientedCursor_new () ;
extern "C" void vtkHyperTreeGridNonOrientedCursor_destructor (vtkHyperTreeGridNonOrientedCursor * sself) ;
extern "C" bool vtk_hyper_tree_grid_non_oriented_cursor_has_tree(vtkHyperTreeGridNonOrientedCursor* sself);
extern "C" long long vtk_hyper_tree_grid_non_oriented_cursor_get_vertex_id(vtkHyperTreeGridNonOrientedCursor* sself);
extern "C" long long vtk_hyper_tree_grid_non_oriented_cursor_get_global_node_index(vtkHyperTreeGridNonOrientedCursor* sself);
extern "C" unsigned char vtk_hyper_tree_grid_non_oriented_cursor_get_dimension(vtkHyperTreeGridNonOrientedCursor* sself);
extern "C" unsigned char vtk_hyper_tree_grid_non_oriented_cursor_get_number_of_children(vtkHyperTreeGridNonOrientedCursor* sself);
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_set_global_index_start(vtkHyperTreeGridNonOrientedCursor* sself, long long index);
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_set_global_index_from_local(vtkHyperTreeGridNonOrientedCursor* sself, long long index);
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_set_mask(vtkHyperTreeGridNonOrientedCursor* sself, bool state);
extern "C" bool vtk_hyper_tree_grid_non_oriented_cursor_is_masked(vtkHyperTreeGridNonOrientedCursor* sself);
extern "C" bool vtk_hyper_tree_grid_non_oriented_cursor_is_leaf(vtkHyperTreeGridNonOrientedCursor* sself);
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_subdivide_leaf(vtkHyperTreeGridNonOrientedCursor* sself);
extern "C" bool vtk_hyper_tree_grid_non_oriented_cursor_is_root(vtkHyperTreeGridNonOrientedCursor* sself);
extern "C" unsigned int vtk_hyper_tree_grid_non_oriented_cursor_get_level(vtkHyperTreeGridNonOrientedCursor* sself);
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_to_child(vtkHyperTreeGridNonOrientedCursor* sself, unsigned char ichild);
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_to_root(vtkHyperTreeGridNonOrientedCursor* sself);
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_to_parent(vtkHyperTreeGridNonOrientedCursor* sself);
extern "C" vtkHyperTreeGridNonOrientedGeometryCursor * vtkHyperTreeGridNonOrientedGeometryCursor_new () ;
extern "C" void vtkHyperTreeGridNonOrientedGeometryCursor_destructor (vtkHyperTreeGridNonOrientedGeometryCursor * sself) ;
extern "C" bool vtk_hyper_tree_grid_non_oriented_geometry_cursor_has_tree(vtkHyperTreeGridNonOrientedGeometryCursor* sself);
extern "C" long long vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_vertex_id(vtkHyperTreeGridNonOrientedGeometryCursor* sself);
extern "C" long long vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_global_node_index(vtkHyperTreeGridNonOrientedGeometryCursor* sself);
extern "C" unsigned char vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_dimension(vtkHyperTreeGridNonOrientedGeometryCursor* sself);
extern "C" unsigned char vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_number_of_children(vtkHyperTreeGridNonOrientedGeometryCursor* sself);
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_set_global_index_start(vtkHyperTreeGridNonOrientedGeometryCursor* sself, long long index);
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_set_global_index_from_local(vtkHyperTreeGridNonOrientedGeometryCursor* sself, long long index);
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_set_mask(vtkHyperTreeGridNonOrientedGeometryCursor* sself, bool state);
extern "C" bool vtk_hyper_tree_grid_non_oriented_geometry_cursor_is_masked(vtkHyperTreeGridNonOrientedGeometryCursor* sself);
extern "C" bool vtk_hyper_tree_grid_non_oriented_geometry_cursor_is_leaf(vtkHyperTreeGridNonOrientedGeometryCursor* sself);
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_subdivide_leaf(vtkHyperTreeGridNonOrientedGeometryCursor* sself);
extern "C" bool vtk_hyper_tree_grid_non_oriented_geometry_cursor_is_root(vtkHyperTreeGridNonOrientedGeometryCursor* sself);
extern "C" unsigned int vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_level(vtkHyperTreeGridNonOrientedGeometryCursor* sself);
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_to_child(vtkHyperTreeGridNonOrientedGeometryCursor* sself, unsigned char ichild);
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_to_root(vtkHyperTreeGridNonOrientedGeometryCursor* sself);
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_to_parent(vtkHyperTreeGridNonOrientedGeometryCursor* sself);
extern "C" vtkHyperTreeGridNonOrientedMooreSuperCursor * vtkHyperTreeGridNonOrientedMooreSuperCursor_new () ;
extern "C" void vtkHyperTreeGridNonOrientedMooreSuperCursor_destructor (vtkHyperTreeGridNonOrientedMooreSuperCursor * sself) ;
extern "C" vtkHyperTreeGridNonOrientedMooreSuperCursorLight * vtkHyperTreeGridNonOrientedMooreSuperCursorLight_new () ;
extern "C" void vtkHyperTreeGridNonOrientedMooreSuperCursorLight_destructor (vtkHyperTreeGridNonOrientedMooreSuperCursorLight * sself) ;
extern "C" vtkHyperTreeGridNonOrientedVonNeumannSuperCursor * vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_new () ;
extern "C" void vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_destructor (vtkHyperTreeGridNonOrientedVonNeumannSuperCursor * sself) ;
extern "C" vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight * vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_new () ;
extern "C" void vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_destructor (vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight * sself) ;
extern "C" vtkHyperTreeGridOrientedCursor * vtkHyperTreeGridOrientedCursor_new () ;
extern "C" void vtkHyperTreeGridOrientedCursor_destructor (vtkHyperTreeGridOrientedCursor * sself) ;
extern "C" bool vtk_hyper_tree_grid_oriented_cursor_has_tree(vtkHyperTreeGridOrientedCursor* sself);
extern "C" long long vtk_hyper_tree_grid_oriented_cursor_get_vertex_id(vtkHyperTreeGridOrientedCursor* sself);
extern "C" long long vtk_hyper_tree_grid_oriented_cursor_get_global_node_index(vtkHyperTreeGridOrientedCursor* sself);
extern "C" unsigned char vtk_hyper_tree_grid_oriented_cursor_get_dimension(vtkHyperTreeGridOrientedCursor* sself);
extern "C" unsigned char vtk_hyper_tree_grid_oriented_cursor_get_number_of_children(vtkHyperTreeGridOrientedCursor* sself);
extern "C" void vtk_hyper_tree_grid_oriented_cursor_set_global_index_start(vtkHyperTreeGridOrientedCursor* sself, long long index);
extern "C" void vtk_hyper_tree_grid_oriented_cursor_set_global_index_from_local(vtkHyperTreeGridOrientedCursor* sself, long long index);
extern "C" void vtk_hyper_tree_grid_oriented_cursor_set_mask(vtkHyperTreeGridOrientedCursor* sself, bool state);
extern "C" bool vtk_hyper_tree_grid_oriented_cursor_is_masked(vtkHyperTreeGridOrientedCursor* sself);
extern "C" bool vtk_hyper_tree_grid_oriented_cursor_is_leaf(vtkHyperTreeGridOrientedCursor* sself);
extern "C" void vtk_hyper_tree_grid_oriented_cursor_subdivide_leaf(vtkHyperTreeGridOrientedCursor* sself);
extern "C" bool vtk_hyper_tree_grid_oriented_cursor_is_root(vtkHyperTreeGridOrientedCursor* sself);
extern "C" unsigned int vtk_hyper_tree_grid_oriented_cursor_get_level(vtkHyperTreeGridOrientedCursor* sself);
extern "C" void vtk_hyper_tree_grid_oriented_cursor_to_child(vtkHyperTreeGridOrientedCursor* sself, unsigned char ichild);
extern "C" vtkHyperTreeGridOrientedGeometryCursor * vtkHyperTreeGridOrientedGeometryCursor_new () ;
extern "C" void vtkHyperTreeGridOrientedGeometryCursor_destructor (vtkHyperTreeGridOrientedGeometryCursor * sself) ;
extern "C" bool vtk_hyper_tree_grid_oriented_geometry_cursor_has_tree(vtkHyperTreeGridOrientedGeometryCursor* sself);
extern "C" long long vtk_hyper_tree_grid_oriented_geometry_cursor_get_vertex_id(vtkHyperTreeGridOrientedGeometryCursor* sself);
extern "C" long long vtk_hyper_tree_grid_oriented_geometry_cursor_get_global_node_index(vtkHyperTreeGridOrientedGeometryCursor* sself);
extern "C" unsigned char vtk_hyper_tree_grid_oriented_geometry_cursor_get_dimension(vtkHyperTreeGridOrientedGeometryCursor* sself);
extern "C" unsigned char vtk_hyper_tree_grid_oriented_geometry_cursor_get_number_of_children(vtkHyperTreeGridOrientedGeometryCursor* sself);
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_set_global_index_start(vtkHyperTreeGridOrientedGeometryCursor* sself, long long index);
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_set_global_index_from_local(vtkHyperTreeGridOrientedGeometryCursor* sself, long long index);
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_set_mask(vtkHyperTreeGridOrientedGeometryCursor* sself, bool state);
extern "C" bool vtk_hyper_tree_grid_oriented_geometry_cursor_is_masked(vtkHyperTreeGridOrientedGeometryCursor* sself);
extern "C" bool vtk_hyper_tree_grid_oriented_geometry_cursor_is_leaf(vtkHyperTreeGridOrientedGeometryCursor* sself);
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_subdivide_leaf(vtkHyperTreeGridOrientedGeometryCursor* sself);
extern "C" bool vtk_hyper_tree_grid_oriented_geometry_cursor_is_root(vtkHyperTreeGridOrientedGeometryCursor* sself);
extern "C" unsigned int vtk_hyper_tree_grid_oriented_geometry_cursor_get_level(vtkHyperTreeGridOrientedGeometryCursor* sself);
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_to_child(vtkHyperTreeGridOrientedGeometryCursor* sself, unsigned char ichild);
extern "C" vtkImageData * vtkImageData_new () ;
extern "C" void vtkImageData_destructor (vtkImageData * sself) ;
extern "C" int vtk_image_data_get_data_object_type(vtkImageData* sself);
extern "C" long long vtk_image_data_get_number_of_cells(vtkImageData* sself);
extern "C" long long vtk_image_data_get_number_of_points(vtkImageData* sself);
extern "C" long long vtk_image_data_find_point(vtkImageData* sself, double x, double y, double z);
extern "C" int vtk_image_data_get_cell_type(vtkImageData* sself, long long cellId);
extern "C" int vtk_image_data_get_max_cell_size(vtkImageData* sself);
extern "C" void vtk_image_data_initialize(vtkImageData* sself);
extern "C" unsigned char vtk_image_data_is_point_visible(vtkImageData* sself, long long ptId);
extern "C" unsigned char vtk_image_data_is_cell_visible(vtkImageData* sself, long long cellId);
extern "C" bool vtk_image_data_has_any_blank_points(vtkImageData* sself);
extern "C" bool vtk_image_data_has_any_blank_cells(vtkImageData* sself);
extern "C" void vtk_image_data_set_dimensions(vtkImageData* sself, int i, int j, int k);
extern "C" int vtk_image_data_get_data_dimension(vtkImageData* sself);
extern "C" void vtk_image_data_set_extent(vtkImageData* sself, int x1, int x2, int y1, int y2, int z1, int z2);
extern "C" void* vtk_image_data_get_scalar_pointer(vtkImageData* sself, int x, int y, int z);
extern "C" long long vtk_image_data_get_scalar_index(vtkImageData* sself, int x, int y, int z);
extern "C" float vtk_image_data_get_scalar_component_as_float(vtkImageData* sself, int x, int y, int z, int component);
extern "C" void vtk_image_data_set_scalar_component_from_float(vtkImageData* sself, int x, int y, int z, int component, float v);
extern "C" double vtk_image_data_get_scalar_component_as_double(vtkImageData* sself, int x, int y, int z, int component);
extern "C" void vtk_image_data_set_scalar_component_from_double(vtkImageData* sself, int x, int y, int z, int component, double v);
extern "C" void vtk_image_data_allocate_scalars(vtkImageData* sself, int dataType, int numComponents);
extern "C" void vtk_image_data_set_spacing(vtkImageData* sself, double i, double j, double k);
extern "C" void vtk_image_data_set_origin(vtkImageData* sself, double i, double j, double k);
extern "C" const char* vtk_image_data_get_scalar_type_as_string(vtkImageData* sself);
extern "C" void vtk_image_data_prepare_for_new_data(vtkImageData* sself);
extern "C" int vtk_image_data_get_extent_type(vtkImageData* sself);
extern "C" vtkImageTransform * vtkImageTransform_new () ;
extern "C" void vtkImageTransform_destructor (vtkImageTransform * sself) ;
extern "C" vtkImplicitBoolean * vtkImplicitBoolean_new () ;
extern "C" void vtkImplicitBoolean_destructor (vtkImplicitBoolean * sself) ;
extern "C" unsigned long vtk_implicit_boolean_get_m_time(vtkImplicitBoolean* sself);
extern "C" void vtk_implicit_boolean_set_operation_type(vtkImplicitBoolean* sself, int _arg);
extern "C" int vtk_implicit_boolean_get_operation_type_min_value(vtkImplicitBoolean* sself);
extern "C" int vtk_implicit_boolean_get_operation_type_max_value(vtkImplicitBoolean* sself);
extern "C" int vtk_implicit_boolean_get_operation_type(vtkImplicitBoolean* sself);
extern "C" void vtk_implicit_boolean_set_operation_type_to_union(vtkImplicitBoolean* sself);
extern "C" void vtk_implicit_boolean_set_operation_type_to_intersection(vtkImplicitBoolean* sself);
extern "C" void vtk_implicit_boolean_set_operation_type_to_difference(vtkImplicitBoolean* sself);
extern "C" void vtk_implicit_boolean_set_operation_type_to_union_of_magnitudes(vtkImplicitBoolean* sself);
extern "C" const char* vtk_implicit_boolean_get_operation_type_as_string(vtkImplicitBoolean* sself);
extern "C" vtkImplicitDataSet * vtkImplicitDataSet_new () ;
extern "C" void vtkImplicitDataSet_destructor (vtkImplicitDataSet * sself) ;
extern "C" unsigned long vtk_implicit_data_set_get_m_time(vtkImplicitDataSet* sself);
extern "C" void vtk_implicit_data_set_set_out_value(vtkImplicitDataSet* sself, double _arg);
extern "C" double vtk_implicit_data_set_get_out_value(vtkImplicitDataSet* sself);
extern "C" void vtk_implicit_data_set_set_out_gradient(vtkImplicitDataSet* sself, double _arg1, double _arg2, double _arg3);
extern "C" vtkImplicitFunctionCollection * vtkImplicitFunctionCollection_new () ;
extern "C" void vtkImplicitFunctionCollection_destructor (vtkImplicitFunctionCollection * sself) ;
extern "C" vtkImplicitHalo * vtkImplicitHalo_new () ;
extern "C" void vtkImplicitHalo_destructor (vtkImplicitHalo * sself) ;
extern "C" void vtk_implicit_halo_set_radius(vtkImplicitHalo* sself, double _arg);
extern "C" double vtk_implicit_halo_get_radius(vtkImplicitHalo* sself);
extern "C" void vtk_implicit_halo_set_center(vtkImplicitHalo* sself, double _arg1, double _arg2, double _arg3);
extern "C" void vtk_implicit_halo_set_fade_out(vtkImplicitHalo* sself, double _arg);
extern "C" double vtk_implicit_halo_get_fade_out(vtkImplicitHalo* sself);
extern "C" vtkImplicitSelectionLoop * vtkImplicitSelectionLoop_new () ;
extern "C" void vtkImplicitSelectionLoop_destructor (vtkImplicitSelectionLoop * sself) ;
extern "C" void vtk_implicit_selection_loop_set_automatic_normal_generation(vtkImplicitSelectionLoop* sself, int _arg);
extern "C" int vtk_implicit_selection_loop_get_automatic_normal_generation(vtkImplicitSelectionLoop* sself);
extern "C" void vtk_implicit_selection_loop_automatic_normal_generation_on(vtkImplicitSelectionLoop* sself);
extern "C" void vtk_implicit_selection_loop_automatic_normal_generation_off(vtkImplicitSelectionLoop* sself);
extern "C" void vtk_implicit_selection_loop_set_normal(vtkImplicitSelectionLoop* sself, double _arg1, double _arg2, double _arg3);
extern "C" unsigned long vtk_implicit_selection_loop_get_m_time(vtkImplicitSelectionLoop* sself);
extern "C" vtkImplicitSum * vtkImplicitSum_new () ;
extern "C" void vtkImplicitSum_destructor (vtkImplicitSum * sself) ;
extern "C" unsigned long vtk_implicit_sum_get_m_time(vtkImplicitSum* sself);
extern "C" void vtk_implicit_sum_remove_all_functions(vtkImplicitSum* sself);
extern "C" void vtk_implicit_sum_set_normalize_by_weight(vtkImplicitSum* sself, int _arg);
extern "C" int vtk_implicit_sum_get_normalize_by_weight(vtkImplicitSum* sself);
extern "C" void vtk_implicit_sum_normalize_by_weight_on(vtkImplicitSum* sself);
extern "C" void vtk_implicit_sum_normalize_by_weight_off(vtkImplicitSum* sself);
extern "C" vtkImplicitVolume * vtkImplicitVolume_new () ;
extern "C" void vtkImplicitVolume_destructor (vtkImplicitVolume * sself) ;
extern "C" unsigned long vtk_implicit_volume_get_m_time(vtkImplicitVolume* sself);
extern "C" void vtk_implicit_volume_set_out_value(vtkImplicitVolume* sself, double _arg);
extern "C" double vtk_implicit_volume_get_out_value(vtkImplicitVolume* sself);
extern "C" void vtk_implicit_volume_set_out_gradient(vtkImplicitVolume* sself, double _arg1, double _arg2, double _arg3);
extern "C" vtkImplicitWindowFunction * vtkImplicitWindowFunction_new () ;
extern "C" void vtkImplicitWindowFunction_destructor (vtkImplicitWindowFunction * sself) ;
extern "C" void vtk_implicit_window_function_set_window_range(vtkImplicitWindowFunction* sself, double _arg1, double _arg2);
extern "C" void vtk_implicit_window_function_set_window_values(vtkImplicitWindowFunction* sself, double _arg1, double _arg2);
extern "C" unsigned long vtk_implicit_window_function_get_m_time(vtkImplicitWindowFunction* sself);
extern "C" vtkInEdgeIterator * vtkInEdgeIterator_new () ;
extern "C" void vtkInEdgeIterator_destructor (vtkInEdgeIterator * sself) ;
extern "C" long long vtk_in_edge_iterator_get_vertex(vtkInEdgeIterator* sself);
extern "C" bool vtk_in_edge_iterator_has_next(vtkInEdgeIterator* sself);
extern "C" vtkIncrementalOctreeNode * vtkIncrementalOctreeNode_new () ;
extern "C" void vtkIncrementalOctreeNode_destructor (vtkIncrementalOctreeNode * sself) ;
extern "C" int vtk_incremental_octree_node_get_number_of_points(vtkIncrementalOctreeNode* sself);
extern "C" void vtk_incremental_octree_node_delete_child_nodes(vtkIncrementalOctreeNode* sself);
extern "C" void vtk_incremental_octree_node_set_bounds(vtkIncrementalOctreeNode* sself, double x1, double x2, double y1, double y2, double z1, double z2);
extern "C" int vtk_incremental_octree_node_is_leaf(vtkIncrementalOctreeNode* sself);
extern "C" int vtk_incremental_octree_node_get_number_of_levels(vtkIncrementalOctreeNode* sself);
extern "C" int vtk_incremental_octree_node_get_id(vtkIncrementalOctreeNode* sself);
extern "C" vtkIncrementalOctreePointLocator * vtkIncrementalOctreePointLocator_new () ;
extern "C" void vtkIncrementalOctreePointLocator_destructor (vtkIncrementalOctreePointLocator * sself) ;
extern "C" void vtk_incremental_octree_point_locator_set_max_points_per_leaf(vtkIncrementalOctreePointLocator* sself, int _arg);
extern "C" int vtk_incremental_octree_point_locator_get_max_points_per_leaf_min_value(vtkIncrementalOctreePointLocator* sself);
extern "C" int vtk_incremental_octree_point_locator_get_max_points_per_leaf_max_value(vtkIncrementalOctreePointLocator* sself);
extern "C" int vtk_incremental_octree_point_locator_get_max_points_per_leaf(vtkIncrementalOctreePointLocator* sself);
extern "C" void vtk_incremental_octree_point_locator_set_build_cubic_octree(vtkIncrementalOctreePointLocator* sself, int _arg);
extern "C" int vtk_incremental_octree_point_locator_get_build_cubic_octree(vtkIncrementalOctreePointLocator* sself);
extern "C" void vtk_incremental_octree_point_locator_build_cubic_octree_on(vtkIncrementalOctreePointLocator* sself);
extern "C" void vtk_incremental_octree_point_locator_build_cubic_octree_off(vtkIncrementalOctreePointLocator* sself);
extern "C" void vtk_incremental_octree_point_locator_initialize(vtkIncrementalOctreePointLocator* sself);
extern "C" void vtk_incremental_octree_point_locator_free_search_structure(vtkIncrementalOctreePointLocator* sself);
extern "C" int vtk_incremental_octree_point_locator_get_number_of_points(vtkIncrementalOctreePointLocator* sself);
extern "C" int vtk_incremental_octree_point_locator_get_number_of_nodes(vtkIncrementalOctreePointLocator* sself);
extern "C" void vtk_incremental_octree_point_locator_build_locator(vtkIncrementalOctreePointLocator* sself);
extern "C" long long vtk_incremental_octree_point_locator_find_closest_point(vtkIncrementalOctreePointLocator* sself, double x, double y, double z);
extern "C" long long vtk_incremental_octree_point_locator_is_inserted_point(vtkIncrementalOctreePointLocator* sself, double x, double y, double z);
extern "C" int vtk_incremental_octree_point_locator_get_number_of_levels(vtkIncrementalOctreePointLocator* sself);
extern "C" vtkIterativeClosestPointTransform * vtkIterativeClosestPointTransform_new () ;
extern "C" void vtkIterativeClosestPointTransform_destructor (vtkIterativeClosestPointTransform * sself) ;
extern "C" void vtk_iterative_closest_point_transform_set_maximum_number_of_iterations(vtkIterativeClosestPointTransform* sself, int _arg);
extern "C" int vtk_iterative_closest_point_transform_get_maximum_number_of_iterations(vtkIterativeClosestPointTransform* sself);
extern "C" int vtk_iterative_closest_point_transform_get_number_of_iterations(vtkIterativeClosestPointTransform* sself);
extern "C" void vtk_iterative_closest_point_transform_set_check_mean_distance(vtkIterativeClosestPointTransform* sself, int _arg);
extern "C" int vtk_iterative_closest_point_transform_get_check_mean_distance(vtkIterativeClosestPointTransform* sself);
extern "C" void vtk_iterative_closest_point_transform_check_mean_distance_on(vtkIterativeClosestPointTransform* sself);
extern "C" void vtk_iterative_closest_point_transform_check_mean_distance_off(vtkIterativeClosestPointTransform* sself);
extern "C" void vtk_iterative_closest_point_transform_set_mean_distance_mode(vtkIterativeClosestPointTransform* sself, int _arg);
extern "C" int vtk_iterative_closest_point_transform_get_mean_distance_mode_min_value(vtkIterativeClosestPointTransform* sself);
extern "C" int vtk_iterative_closest_point_transform_get_mean_distance_mode_max_value(vtkIterativeClosestPointTransform* sself);
extern "C" int vtk_iterative_closest_point_transform_get_mean_distance_mode(vtkIterativeClosestPointTransform* sself);
extern "C" void vtk_iterative_closest_point_transform_set_mean_distance_mode_to_rms(vtkIterativeClosestPointTransform* sself);
extern "C" void vtk_iterative_closest_point_transform_set_mean_distance_mode_to_absolute_value(vtkIterativeClosestPointTransform* sself);
extern "C" const char* vtk_iterative_closest_point_transform_get_mean_distance_mode_as_string(vtkIterativeClosestPointTransform* sself);
extern "C" void vtk_iterative_closest_point_transform_set_maximum_mean_distance(vtkIterativeClosestPointTransform* sself, double _arg);
extern "C" double vtk_iterative_closest_point_transform_get_maximum_mean_distance(vtkIterativeClosestPointTransform* sself);
extern "C" double vtk_iterative_closest_point_transform_get_mean_distance(vtkIterativeClosestPointTransform* sself);
extern "C" void vtk_iterative_closest_point_transform_set_maximum_number_of_landmarks(vtkIterativeClosestPointTransform* sself, int _arg);
extern "C" int vtk_iterative_closest_point_transform_get_maximum_number_of_landmarks(vtkIterativeClosestPointTransform* sself);
extern "C" void vtk_iterative_closest_point_transform_set_start_by_matching_centroids(vtkIterativeClosestPointTransform* sself, int _arg);
extern "C" int vtk_iterative_closest_point_transform_get_start_by_matching_centroids(vtkIterativeClosestPointTransform* sself);
extern "C" void vtk_iterative_closest_point_transform_start_by_matching_centroids_on(vtkIterativeClosestPointTransform* sself);
extern "C" void vtk_iterative_closest_point_transform_start_by_matching_centroids_off(vtkIterativeClosestPointTransform* sself);
extern "C" void vtk_iterative_closest_point_transform_inverse(vtkIterativeClosestPointTransform* sself);
extern "C" vtkKdNode * vtkKdNode_new () ;
extern "C" void vtkKdNode_destructor (vtkKdNode * sself) ;
extern "C" void vtk_kd_node_set_dim(vtkKdNode* sself, int _arg);
extern "C" int vtk_kd_node_get_dim(vtkKdNode* sself);
extern "C" double vtk_kd_node_get_division_position(vtkKdNode* sself);
extern "C" void vtk_kd_node_set_number_of_points(vtkKdNode* sself, int _arg);
extern "C" int vtk_kd_node_get_number_of_points(vtkKdNode* sself);
extern "C" void vtk_kd_node_set_bounds(vtkKdNode* sself, double x1, double x2, double y1, double y2, double z1, double z2);
extern "C" void vtk_kd_node_set_data_bounds(vtkKdNode* sself, double x1, double x2, double y1, double y2, double z1, double z2);
extern "C" void vtk_kd_node_set_id(vtkKdNode* sself, int _arg);
extern "C" int vtk_kd_node_get_id(vtkKdNode* sself);
extern "C" int vtk_kd_node_get_min_id(vtkKdNode* sself);
extern "C" int vtk_kd_node_get_max_id(vtkKdNode* sself);
extern "C" void vtk_kd_node_set_min_id(vtkKdNode* sself, int _arg);
extern "C" void vtk_kd_node_set_max_id(vtkKdNode* sself, int _arg);
extern "C" void vtk_kd_node_delete_child_nodes(vtkKdNode* sself);
extern "C" int vtk_kd_node_intersects_box(vtkKdNode* sself, double x1, double x2, double y1, double y2, double z1, double z2, int useDataBounds);
extern "C" int vtk_kd_node_intersects_sphere_2(vtkKdNode* sself, double x, double y, double z, double rSquared, int useDataBounds);
extern "C" int vtk_kd_node_contains_box(vtkKdNode* sself, double x1, double x2, double y1, double y2, double z1, double z2, int useDataBounds);
extern "C" int vtk_kd_node_contains_point(vtkKdNode* sself, double x, double y, double z, int useDataBounds);
extern "C" double vtk_kd_node_get_distance_2_to_boundary(vtkKdNode* sself, double x, double y, double z, int useDataBounds);
extern "C" double vtk_kd_node_get_distance_2_to_inner_boundary(vtkKdNode* sself, double x, double y, double z);
extern "C" void vtk_kd_node_print_node(vtkKdNode* sself, int depth);
extern "C" void vtk_kd_node_print_verbose_node(vtkKdNode* sself, int depth);
extern "C" vtkKdTree * vtkKdTree_new () ;
extern "C" void vtkKdTree_destructor (vtkKdTree * sself) ;
extern "C" void vtk_kd_tree_timing_on(vtkKdTree* sself);
extern "C" void vtk_kd_tree_timing_off(vtkKdTree* sself);
extern "C" void vtk_kd_tree_set_timing(vtkKdTree* sself, int _arg);
extern "C" int vtk_kd_tree_get_timing(vtkKdTree* sself);
extern "C" void vtk_kd_tree_set_min_cells(vtkKdTree* sself, int _arg);
extern "C" int vtk_kd_tree_get_min_cells(vtkKdTree* sself);
extern "C" int vtk_kd_tree_get_number_of_regions_or_less(vtkKdTree* sself);
extern "C" void vtk_kd_tree_set_number_of_regions_or_less(vtkKdTree* sself, int _arg);
extern "C" int vtk_kd_tree_get_number_of_regions_or_more(vtkKdTree* sself);
extern "C" void vtk_kd_tree_set_number_of_regions_or_more(vtkKdTree* sself, int _arg);
extern "C" double vtk_kd_tree_get_fudge_factor(vtkKdTree* sself);
extern "C" void vtk_kd_tree_set_fudge_factor(vtkKdTree* sself, double _arg);
extern "C" void vtk_kd_tree_omit_x_partitioning(vtkKdTree* sself);
extern "C" void vtk_kd_tree_omit_y_partitioning(vtkKdTree* sself);
extern "C" void vtk_kd_tree_omit_z_partitioning(vtkKdTree* sself);
extern "C" void vtk_kd_tree_omit_xy_partitioning(vtkKdTree* sself);
extern "C" void vtk_kd_tree_omit_yz_partitioning(vtkKdTree* sself);
extern "C" void vtk_kd_tree_omit_zx_partitioning(vtkKdTree* sself);
extern "C" void vtk_kd_tree_omit_no_partitioning(vtkKdTree* sself);
extern "C" void vtk_kd_tree_remove_data_set(vtkKdTree* sself, int index);
extern "C" void vtk_kd_tree_remove_all_data_sets(vtkKdTree* sself);
extern "C" int vtk_kd_tree_get_number_of_data_sets(vtkKdTree* sself);
extern "C" int vtk_kd_tree_get_number_of_regions(vtkKdTree* sself);
extern "C" void vtk_kd_tree_print_tree(vtkKdTree* sself);
extern "C" void vtk_kd_tree_print_verbose_tree(vtkKdTree* sself);
extern "C" void vtk_kd_tree_print_region(vtkKdTree* sself, int id);
extern "C" void vtk_kd_tree_set_include_region_boundary_cells(vtkKdTree* sself, int _arg);
extern "C" int vtk_kd_tree_get_include_region_boundary_cells(vtkKdTree* sself);
extern "C" void vtk_kd_tree_include_region_boundary_cells_on(vtkKdTree* sself);
extern "C" void vtk_kd_tree_include_region_boundary_cells_off(vtkKdTree* sself);
extern "C" void vtk_kd_tree_delete_cell_lists(vtkKdTree* sself);
extern "C" int vtk_kd_tree_get_region_containing_point(vtkKdTree* sself, double x, double y, double z);
extern "C" void vtk_kd_tree_build_locator(vtkKdTree* sself);
extern "C" void vtk_kd_tree_free_search_structure(vtkKdTree* sself);
extern "C" void vtk_kd_tree_generate_representation_using_data_bounds_on(vtkKdTree* sself);
extern "C" void vtk_kd_tree_generate_representation_using_data_bounds_off(vtkKdTree* sself);
extern "C" void vtk_kd_tree_set_generate_representation_using_data_bounds(vtkKdTree* sself, int _arg);
extern "C" int vtk_kd_tree_get_generate_representation_using_data_bounds(vtkKdTree* sself);
extern "C" int vtk_kd_tree_new_geometry(vtkKdTree* sself);
extern "C" void vtk_kd_tree_invalidate_geometry(vtkKdTree* sself);
extern "C" vtkKdTreePointLocator * vtkKdTreePointLocator_new () ;
extern "C" void vtkKdTreePointLocator_destructor (vtkKdTreePointLocator * sself) ;
extern "C" void vtk_kd_tree_point_locator_free_search_structure(vtkKdTreePointLocator* sself);
extern "C" void vtk_kd_tree_point_locator_build_locator(vtkKdTreePointLocator* sself);
extern "C" vtkLagrangeCurve * vtkLagrangeCurve_new () ;
extern "C" void vtkLagrangeCurve_destructor (vtkLagrangeCurve * sself) ;
extern "C" int vtk_lagrange_curve_get_cell_type(vtkLagrangeCurve* sself);
extern "C" vtkLagrangeHexahedron * vtkLagrangeHexahedron_new () ;
extern "C" void vtkLagrangeHexahedron_destructor (vtkLagrangeHexahedron * sself) ;
extern "C" int vtk_lagrange_hexahedron_get_cell_type(vtkLagrangeHexahedron* sself);
extern "C" vtkLagrangeInterpolation * vtkLagrangeInterpolation_new () ;
extern "C" void vtkLagrangeInterpolation_destructor (vtkLagrangeInterpolation * sself) ;
extern "C" vtkLagrangeQuadrilateral * vtkLagrangeQuadrilateral_new () ;
extern "C" void vtkLagrangeQuadrilateral_destructor (vtkLagrangeQuadrilateral * sself) ;
extern "C" int vtk_lagrange_quadrilateral_get_cell_type(vtkLagrangeQuadrilateral* sself);
extern "C" vtkLagrangeTetra * vtkLagrangeTetra_new () ;
extern "C" void vtkLagrangeTetra_destructor (vtkLagrangeTetra * sself) ;
extern "C" int vtk_lagrange_tetra_get_cell_type(vtkLagrangeTetra* sself);
extern "C" vtkLagrangeTriangle * vtkLagrangeTriangle_new () ;
extern "C" void vtkLagrangeTriangle_destructor (vtkLagrangeTriangle * sself) ;
extern "C" int vtk_lagrange_triangle_get_cell_type(vtkLagrangeTriangle* sself);
extern "C" vtkLagrangeWedge * vtkLagrangeWedge_new () ;
extern "C" void vtkLagrangeWedge_destructor (vtkLagrangeWedge * sself) ;
extern "C" int vtk_lagrange_wedge_get_cell_type(vtkLagrangeWedge* sself);
extern "C" vtkLine * vtkLine_new () ;
extern "C" void vtkLine_destructor (vtkLine * sself) ;
extern "C" int vtk_line_get_cell_type(vtkLine* sself);
extern "C" int vtk_line_get_cell_dimension(vtkLine* sself);
extern "C" int vtk_line_get_number_of_edges(vtkLine* sself);
extern "C" int vtk_line_get_number_of_faces(vtkLine* sself);
extern "C" int vtk_line_inflate(vtkLine* sself, double dist);
extern "C" vtkMeanValueCoordinatesInterpolator * vtkMeanValueCoordinatesInterpolator_new () ;
extern "C" void vtkMeanValueCoordinatesInterpolator_destructor (vtkMeanValueCoordinatesInterpolator * sself) ;
extern "C" vtkMergePoints * vtkMergePoints_new () ;
extern "C" void vtkMergePoints_destructor (vtkMergePoints * sself) ;
extern "C" vtkMolecule * vtkMolecule_new () ;
extern "C" void vtkMolecule_destructor (vtkMolecule * sself) ;
extern "C" long long vtk_molecule_get_number_of_atoms(vtkMolecule* sself);
extern "C" long long vtk_molecule_get_number_of_bonds(vtkMolecule* sself);
extern "C" unsigned short vtk_molecule_get_atom_atomic_number(vtkMolecule* sself, long long atomId);
extern "C" void vtk_molecule_set_atom_atomic_number(vtkMolecule* sself, long long atomId, unsigned short atomicNum);
extern "C" void vtk_molecule_set_bond_order(vtkMolecule* sself, long long bondId, unsigned short order);
extern "C" unsigned short vtk_molecule_get_bond_order(vtkMolecule* sself, long long bondId);
extern "C" double vtk_molecule_get_bond_length(vtkMolecule* sself, long long bondId);
extern "C" bool vtk_molecule_has_lattice(vtkMolecule* sself);
extern "C" void vtk_molecule_clear_lattice(vtkMolecule* sself);
extern "C" void vtk_molecule_allocate_atom_ghost_array(vtkMolecule* sself);
extern "C" void vtk_molecule_allocate_bond_ghost_array(vtkMolecule* sself);
extern "C" long long vtk_molecule_get_bond_id(vtkMolecule* sself, long long a, long long b);
extern "C" void vtk_molecule_set_atomic_number_array_name(vtkMolecule* sself, const char* _arg);
extern "C" void vtk_molecule_set_bond_orders_array_name(vtkMolecule* sself, const char* _arg);
extern "C" vtkMultiBlockDataSet * vtkMultiBlockDataSet_new () ;
extern "C" void vtkMultiBlockDataSet_destructor (vtkMultiBlockDataSet * sself) ;
extern "C" void vtk_multi_block_data_set_set_number_of_blocks(vtkMultiBlockDataSet* sself, unsigned int numBlocks);
extern "C" unsigned int vtk_multi_block_data_set_get_number_of_blocks(vtkMultiBlockDataSet* sself);
extern "C" void vtk_multi_block_data_set_remove_block(vtkMultiBlockDataSet* sself, unsigned int blockno);
extern "C" int vtk_multi_block_data_set_has_meta_data(vtkMultiBlockDataSet* sself, unsigned int blockno);
extern "C" vtkMultiPieceDataSet * vtkMultiPieceDataSet_new () ;
extern "C" void vtkMultiPieceDataSet_destructor (vtkMultiPieceDataSet * sself) ;
extern "C" void vtk_multi_piece_data_set_set_number_of_pieces(vtkMultiPieceDataSet* sself, unsigned int numpieces);
extern "C" unsigned int vtk_multi_piece_data_set_get_number_of_pieces(vtkMultiPieceDataSet* sself);
extern "C" vtkMutableDirectedGraph * vtkMutableDirectedGraph_new () ;
extern "C" void vtkMutableDirectedGraph_destructor (vtkMutableDirectedGraph * sself) ;
extern "C" long long vtk_mutable_directed_graph_set_number_of_vertices(vtkMutableDirectedGraph* sself, long long numVerts);
extern "C" long long vtk_mutable_directed_graph_add_vertex(vtkMutableDirectedGraph* sself);
extern "C" void vtk_mutable_directed_graph_lazy_add_vertex(vtkMutableDirectedGraph* sself);
extern "C" void vtk_mutable_directed_graph_remove_vertex(vtkMutableDirectedGraph* sself, long long v);
extern "C" void vtk_mutable_directed_graph_remove_edge(vtkMutableDirectedGraph* sself, long long e);
extern "C" vtkMutableUndirectedGraph * vtkMutableUndirectedGraph_new () ;
extern "C" void vtkMutableUndirectedGraph_destructor (vtkMutableUndirectedGraph * sself) ;
extern "C" long long vtk_mutable_undirected_graph_set_number_of_vertices(vtkMutableUndirectedGraph* sself, long long numVerts);
extern "C" long long vtk_mutable_undirected_graph_add_vertex(vtkMutableUndirectedGraph* sself);
extern "C" void vtk_mutable_undirected_graph_lazy_add_vertex(vtkMutableUndirectedGraph* sself);
extern "C" void vtk_mutable_undirected_graph_lazy_add_edge(vtkMutableUndirectedGraph* sself, long long u, long long v);
extern "C" void vtk_mutable_undirected_graph_remove_vertex(vtkMutableUndirectedGraph* sself, long long v);
extern "C" void vtk_mutable_undirected_graph_remove_edge(vtkMutableUndirectedGraph* sself, long long e);
extern "C" vtkNonMergingPointLocator * vtkNonMergingPointLocator_new () ;
extern "C" void vtkNonMergingPointLocator_destructor (vtkNonMergingPointLocator * sself) ;
extern "C" long long vtk_non_merging_point_locator_is_inserted_point(vtkNonMergingPointLocator* sself, double p0, double p1, double p2);
extern "C" vtkNonOverlappingAMR * vtkNonOverlappingAMR_new () ;
extern "C" void vtkNonOverlappingAMR_destructor (vtkNonOverlappingAMR * sself) ;
extern "C" int vtk_non_overlapping_amr_get_data_object_type(vtkNonOverlappingAMR* sself);
extern "C" vtkOctreePointLocator * vtkOctreePointLocator_new () ;
extern "C" void vtkOctreePointLocator_destructor (vtkOctreePointLocator * sself) ;
extern "C" void vtk_octree_point_locator_set_maximum_points_per_region(vtkOctreePointLocator* sself, int _arg);
extern "C" int vtk_octree_point_locator_get_maximum_points_per_region(vtkOctreePointLocator* sself);
extern "C" void vtk_octree_point_locator_set_create_cubic_octants(vtkOctreePointLocator* sself, int _arg);
extern "C" int vtk_octree_point_locator_get_create_cubic_octants(vtkOctreePointLocator* sself);
extern "C" double vtk_octree_point_locator_get_fudge_factor(vtkOctreePointLocator* sself);
extern "C" void vtk_octree_point_locator_set_fudge_factor(vtkOctreePointLocator* sself, double _arg);
extern "C" int vtk_octree_point_locator_get_number_of_leaf_nodes(vtkOctreePointLocator* sself);
extern "C" int vtk_octree_point_locator_get_region_containing_point(vtkOctreePointLocator* sself, double x, double y, double z);
extern "C" void vtk_octree_point_locator_build_locator(vtkOctreePointLocator* sself);
extern "C" long long vtk_octree_point_locator_find_closest_point(vtkOctreePointLocator* sself, double x, double y, double z, double& dist2);
extern "C" void vtk_octree_point_locator_free_search_structure(vtkOctreePointLocator* sself);
extern "C" vtkOctreePointLocatorNode * vtkOctreePointLocatorNode_new () ;
extern "C" void vtkOctreePointLocatorNode_destructor (vtkOctreePointLocatorNode * sself) ;
extern "C" void vtk_octree_point_locator_node_set_number_of_points(vtkOctreePointLocatorNode* sself, int numberOfPoints);
extern "C" int vtk_octree_point_locator_node_get_number_of_points(vtkOctreePointLocatorNode* sself);
extern "C" void vtk_octree_point_locator_node_set_bounds(vtkOctreePointLocatorNode* sself, double xMin, double xMax, double yMin, double yMax, double zMin, double zMax);
extern "C" void vtk_octree_point_locator_node_set_data_bounds(vtkOctreePointLocatorNode* sself, double xMin, double xMax, double yMin, double yMax, double zMin, double zMax);
extern "C" int vtk_octree_point_locator_node_get_id(vtkOctreePointLocatorNode* sself);
extern "C" int vtk_octree_point_locator_node_get_min_id(vtkOctreePointLocatorNode* sself);
extern "C" void vtk_octree_point_locator_node_create_child_nodes(vtkOctreePointLocatorNode* sself);
extern "C" void vtk_octree_point_locator_node_delete_child_nodes(vtkOctreePointLocatorNode* sself);
extern "C" int vtk_octree_point_locator_node_contains_point(vtkOctreePointLocatorNode* sself, double x, double y, double z, int useDataBounds);
extern "C" vtkOrderedTriangulator * vtkOrderedTriangulator_new () ;
extern "C" void vtkOrderedTriangulator_destructor (vtkOrderedTriangulator * sself) ;
extern "C" void vtk_ordered_triangulator_init_triangulation(vtkOrderedTriangulator* sself, double xmin, double xmax, double ymin, double ymax, double zmin, double zmax, int numPts);
extern "C" void vtk_ordered_triangulator_triangulate(vtkOrderedTriangulator* sself);
extern "C" void vtk_ordered_triangulator_template_triangulate(vtkOrderedTriangulator* sself, int cellType, int numPts, int numEdges);
extern "C" void vtk_ordered_triangulator_update_point_type(vtkOrderedTriangulator* sself, long long internalId, int type);
extern "C" long long vtk_ordered_triangulator_get_point_id(vtkOrderedTriangulator* sself, long long internalId);
extern "C" int vtk_ordered_triangulator_get_number_of_points(vtkOrderedTriangulator* sself);
extern "C" void vtk_ordered_triangulator_set_use_templates(vtkOrderedTriangulator* sself, int _arg);
extern "C" int vtk_ordered_triangulator_get_use_templates(vtkOrderedTriangulator* sself);
extern "C" void vtk_ordered_triangulator_use_templates_on(vtkOrderedTriangulator* sself);
extern "C" void vtk_ordered_triangulator_use_templates_off(vtkOrderedTriangulator* sself);
extern "C" void vtk_ordered_triangulator_set_pre_sorted(vtkOrderedTriangulator* sself, int _arg);
extern "C" int vtk_ordered_triangulator_get_pre_sorted(vtkOrderedTriangulator* sself);
extern "C" void vtk_ordered_triangulator_pre_sorted_on(vtkOrderedTriangulator* sself);
extern "C" void vtk_ordered_triangulator_pre_sorted_off(vtkOrderedTriangulator* sself);
extern "C" void vtk_ordered_triangulator_set_use_two_sort_ids(vtkOrderedTriangulator* sself, int _arg);
extern "C" int vtk_ordered_triangulator_get_use_two_sort_ids(vtkOrderedTriangulator* sself);
extern "C" void vtk_ordered_triangulator_use_two_sort_ids_on(vtkOrderedTriangulator* sself);
extern "C" void vtk_ordered_triangulator_use_two_sort_ids_off(vtkOrderedTriangulator* sself);
extern "C" void vtk_ordered_triangulator_init_tetra_traversal(vtkOrderedTriangulator* sself);
extern "C" vtkOutEdgeIterator * vtkOutEdgeIterator_new () ;
extern "C" void vtkOutEdgeIterator_destructor (vtkOutEdgeIterator * sself) ;
extern "C" long long vtk_out_edge_iterator_get_vertex(vtkOutEdgeIterator* sself);
extern "C" bool vtk_out_edge_iterator_has_next(vtkOutEdgeIterator* sself);
extern "C" vtkOverlappingAMR * vtkOverlappingAMR_new () ;
extern "C" void vtkOverlappingAMR_destructor (vtkOverlappingAMR * sself) ;
extern "C" void vtk_overlapping_amr_set_refinement_ratio(vtkOverlappingAMR* sself, unsigned int level, int refRatio);
extern "C" int vtk_overlapping_amr_get_refinement_ratio(vtkOverlappingAMR* sself, unsigned int level);
extern "C" void vtk_overlapping_amr_set_amr_block_source_index(vtkOverlappingAMR* sself, unsigned int level, unsigned int id, int sourceId);
extern "C" int vtk_overlapping_amr_get_amr_block_source_index(vtkOverlappingAMR* sself, unsigned int level, unsigned int id);
extern "C" bool vtk_overlapping_amr_has_children_information(vtkOverlappingAMR* sself);
extern "C" void vtk_overlapping_amr_generate_parent_child_information(vtkOverlappingAMR* sself);
extern "C" void vtk_overlapping_amr_print_parent_child_info(vtkOverlappingAMR* sself, unsigned int level, unsigned int index);
extern "C" void vtk_overlapping_amr_audit(vtkOverlappingAMR* sself);
extern "C" vtkPartitionedDataSet * vtkPartitionedDataSet_new () ;
extern "C" void vtkPartitionedDataSet_destructor (vtkPartitionedDataSet * sself) ;
extern "C" void vtk_partitioned_data_set_set_number_of_partitions(vtkPartitionedDataSet* sself, unsigned int numPartitions);
extern "C" unsigned int vtk_partitioned_data_set_get_number_of_partitions(vtkPartitionedDataSet* sself);
extern "C" int vtk_partitioned_data_set_has_meta_data(vtkPartitionedDataSet* sself, unsigned int idx);
extern "C" void vtk_partitioned_data_set_remove_null_partitions(vtkPartitionedDataSet* sself);
extern "C" vtkPartitionedDataSetCollection * vtkPartitionedDataSetCollection_new () ;
extern "C" void vtkPartitionedDataSetCollection_destructor (vtkPartitionedDataSetCollection * sself) ;
extern "C" void vtk_partitioned_data_set_collection_set_number_of_partitioned_data_sets(vtkPartitionedDataSetCollection* sself, unsigned int numDataSets);
extern "C" unsigned int vtk_partitioned_data_set_collection_get_number_of_partitioned_data_sets(vtkPartitionedDataSetCollection* sself);
extern "C" void vtk_partitioned_data_set_collection_remove_partitioned_data_set(vtkPartitionedDataSetCollection* sself, unsigned int idx);
extern "C" unsigned int vtk_partitioned_data_set_collection_get_number_of_partitions(vtkPartitionedDataSetCollection* sself, unsigned int idx);
extern "C" void vtk_partitioned_data_set_collection_set_number_of_partitions(vtkPartitionedDataSetCollection* sself, unsigned int idx, unsigned int numPartitions);
extern "C" int vtk_partitioned_data_set_collection_has_meta_data(vtkPartitionedDataSetCollection* sself, unsigned int idx);
extern "C" unsigned int vtk_partitioned_data_set_collection_get_composite_index(vtkPartitionedDataSetCollection* sself, unsigned int idx);
extern "C" unsigned long vtk_partitioned_data_set_collection_get_m_time(vtkPartitionedDataSetCollection* sself);
extern "C" vtkPath * vtkPath_new () ;
extern "C" void vtkPath_destructor (vtkPath * sself) ;
extern "C" int vtk_path_get_data_object_type(vtkPath* sself);
extern "C" void vtk_path_insert_next_point(vtkPath* sself, double x, double y, double z, int code);
extern "C" long long vtk_path_get_number_of_cells(vtkPath* sself);
extern "C" int vtk_path_get_max_cell_size(vtkPath* sself);
extern "C" void vtk_path_allocate(vtkPath* sself, long long size, int extSize);
extern "C" void vtk_path_reset(vtkPath* sself);
extern "C" vtkPentagonalPrism * vtkPentagonalPrism_new () ;
extern "C" void vtkPentagonalPrism_destructor (vtkPentagonalPrism * sself) ;
extern "C" int vtk_pentagonal_prism_get_cell_type(vtkPentagonalPrism* sself);
extern "C" int vtk_pentagonal_prism_get_number_of_edges(vtkPentagonalPrism* sself);
extern "C" int vtk_pentagonal_prism_get_number_of_faces(vtkPentagonalPrism* sself);
extern "C" vtkPerlinNoise * vtkPerlinNoise_new () ;
extern "C" void vtkPerlinNoise_destructor (vtkPerlinNoise * sself) ;
extern "C" void vtk_perlin_noise_set_frequency(vtkPerlinNoise* sself, double _arg1, double _arg2, double _arg3);
extern "C" void vtk_perlin_noise_set_phase(vtkPerlinNoise* sself, double _arg1, double _arg2, double _arg3);
extern "C" void vtk_perlin_noise_set_amplitude(vtkPerlinNoise* sself, double _arg);
extern "C" double vtk_perlin_noise_get_amplitude(vtkPerlinNoise* sself);
extern "C" vtkPiecewiseFunction * vtkPiecewiseFunction_new () ;
extern "C" void vtkPiecewiseFunction_destructor (vtkPiecewiseFunction * sself) ;
extern "C" int vtk_piecewise_function_get_data_object_type(vtkPiecewiseFunction* sself);
extern "C" int vtk_piecewise_function_get_size(vtkPiecewiseFunction* sself);
extern "C" int vtk_piecewise_function_add_point(vtkPiecewiseFunction* sself, double x, double y);
extern "C" bool vtk_piecewise_function_remove_point_by_index(vtkPiecewiseFunction* sself, size_t id);
extern "C" int vtk_piecewise_function_remove_point(vtkPiecewiseFunction* sself, double x);
extern "C" void vtk_piecewise_function_remove_all_points(vtkPiecewiseFunction* sself);
extern "C" void vtk_piecewise_function_add_segment(vtkPiecewiseFunction* sself, double x1, double y1, double x2, double y2);
extern "C" double vtk_piecewise_function_get_value(vtkPiecewiseFunction* sself, double x);
extern "C" void vtk_piecewise_function_set_clamping(vtkPiecewiseFunction* sself, int _arg);
extern "C" int vtk_piecewise_function_get_clamping(vtkPiecewiseFunction* sself);
extern "C" void vtk_piecewise_function_clamping_on(vtkPiecewiseFunction* sself);
extern "C" void vtk_piecewise_function_clamping_off(vtkPiecewiseFunction* sself);
extern "C" void vtk_piecewise_function_set_use_log_scale(vtkPiecewiseFunction* sself, bool _arg);
extern "C" bool vtk_piecewise_function_get_use_log_scale(vtkPiecewiseFunction* sself);
extern "C" void vtk_piecewise_function_use_log_scale_on(vtkPiecewiseFunction* sself);
extern "C" void vtk_piecewise_function_use_log_scale_off(vtkPiecewiseFunction* sself);
extern "C" const char* vtk_piecewise_function_get_type(vtkPiecewiseFunction* sself);
extern "C" double vtk_piecewise_function_get_first_non_zero_value(vtkPiecewiseFunction* sself);
extern "C" void vtk_piecewise_function_initialize(vtkPiecewiseFunction* sself);
extern "C" void vtk_piecewise_function_set_allow_duplicate_scalars(vtkPiecewiseFunction* sself, int _arg);
extern "C" int vtk_piecewise_function_get_allow_duplicate_scalars(vtkPiecewiseFunction* sself);
extern "C" void vtk_piecewise_function_allow_duplicate_scalars_on(vtkPiecewiseFunction* sself);
extern "C" void vtk_piecewise_function_allow_duplicate_scalars_off(vtkPiecewiseFunction* sself);
extern "C" int vtk_piecewise_function_estimate_min_number_of_samples(vtkPiecewiseFunction* sself, const double& x1, const double& x2);
extern "C" vtkPixel * vtkPixel_new () ;
extern "C" void vtkPixel_destructor (vtkPixel * sself) ;
extern "C" int vtk_pixel_get_cell_type(vtkPixel* sself);
extern "C" int vtk_pixel_get_cell_dimension(vtkPixel* sself);
extern "C" int vtk_pixel_get_number_of_edges(vtkPixel* sself);
extern "C" int vtk_pixel_get_number_of_faces(vtkPixel* sself);
extern "C" int vtk_pixel_inflate(vtkPixel* sself, double dist);
extern "C" vtkPlane * vtkPlane_new () ;
extern "C" void vtkPlane_destructor (vtkPlane * sself) ;
extern "C" void vtk_plane_set_normal(vtkPlane* sself, double _arg1, double _arg2, double _arg3);
extern "C" void vtk_plane_set_origin(vtkPlane* sself, double _arg1, double _arg2, double _arg3);
extern "C" void vtk_plane_push(vtkPlane* sself, double distance);
extern "C" vtkPlaneCollection * vtkPlaneCollection_new () ;
extern "C" void vtkPlaneCollection_destructor (vtkPlaneCollection * sself) ;
extern "C" int vtk_plane_collection_get_number_of_items(vtkPlaneCollection* sself);
extern "C" vtkPlanes * vtkPlanes_new () ;
extern "C" void vtkPlanes_destructor (vtkPlanes * sself) ;
extern "C" void vtk_planes_set_bounds(vtkPlanes* sself, double xmin, double xmax, double ymin, double ymax, double zmin, double zmax);
extern "C" int vtk_planes_get_number_of_planes(vtkPlanes* sself);
extern "C" vtkPlanesIntersection * vtkPlanesIntersection_new () ;
extern "C" void vtkPlanesIntersection_destructor (vtkPlanesIntersection * sself) ;
extern "C" int vtk_planes_intersection_get_number_of_region_vertices(vtkPlanesIntersection* sself);
extern "C" int vtk_planes_intersection_get_num_region_vertices(vtkPlanesIntersection* sself);
extern "C" vtkPointData * vtkPointData_new () ;
extern "C" void vtkPointData_destructor (vtkPointData * sself) ;
extern "C" void vtk_point_data_null_point(vtkPointData* sself, long long ptId);
extern "C" vtkPointLocator * vtkPointLocator_new () ;
extern "C" void vtkPointLocator_destructor (vtkPointLocator * sself) ;
extern "C" void vtk_point_locator_set_divisions(vtkPointLocator* sself, int _arg1, int _arg2, int _arg3);
extern "C" void vtk_point_locator_set_number_of_points_per_bucket(vtkPointLocator* sself, int _arg);
extern "C" int vtk_point_locator_get_number_of_points_per_bucket_min_value(vtkPointLocator* sself);
extern "C" int vtk_point_locator_get_number_of_points_per_bucket_max_value(vtkPointLocator* sself);
extern "C" int vtk_point_locator_get_number_of_points_per_bucket(vtkPointLocator* sself);
extern "C" long long vtk_point_locator_is_inserted_point(vtkPointLocator* sself, double x, double y, double z);
extern "C" void vtk_point_locator_initialize(vtkPointLocator* sself);
extern "C" void vtk_point_locator_free_search_structure(vtkPointLocator* sself);
extern "C" void vtk_point_locator_build_locator(vtkPointLocator* sself);
extern "C" vtkPointSet * vtkPointSet_new () ;
extern "C" void vtkPointSet_destructor (vtkPointSet * sself) ;
extern "C" void vtk_point_set_set_editable(vtkPointSet* sself, bool _arg);
extern "C" bool vtk_point_set_get_editable(vtkPointSet* sself);
extern "C" void vtk_point_set_editable_on(vtkPointSet* sself);
extern "C" void vtk_point_set_editable_off(vtkPointSet* sself);
extern "C" void vtk_point_set_initialize(vtkPointSet* sself);
extern "C" long long vtk_point_set_get_number_of_points(vtkPointSet* sself);
extern "C" long long vtk_point_set_get_number_of_cells(vtkPointSet* sself);
extern "C" int vtk_point_set_get_max_cell_size(vtkPointSet* sself);
extern "C" int vtk_point_set_get_cell_type(vtkPointSet* sself, long long p0);
extern "C" void vtk_point_set_build_point_locator(vtkPointSet* sself);
extern "C" void vtk_point_set_build_locator(vtkPointSet* sself);
extern "C" void vtk_point_set_build_cell_locator(vtkPointSet* sself);
extern "C" unsigned long vtk_point_set_get_m_time(vtkPointSet* sself);
extern "C" void vtk_point_set_compute_bounds(vtkPointSet* sself);
extern "C" void vtk_point_set_squeeze(vtkPointSet* sself);
extern "C" vtkPointSetCellIterator * vtkPointSetCellIterator_new () ;
extern "C" void vtkPointSetCellIterator_destructor (vtkPointSetCellIterator * sself) ;
extern "C" bool vtk_point_set_cell_iterator_is_done_with_traversal(vtkPointSetCellIterator* sself);
extern "C" long long vtk_point_set_cell_iterator_get_cell_id(vtkPointSetCellIterator* sself);
extern "C" vtkPointsProjectedHull * vtkPointsProjectedHull_new () ;
extern "C" void vtkPointsProjectedHull_destructor (vtkPointsProjectedHull * sself) ;
extern "C" int vtk_points_projected_hull_get_size_ccw_hull_x(vtkPointsProjectedHull* sself);
extern "C" int vtk_points_projected_hull_get_size_ccw_hull_y(vtkPointsProjectedHull* sself);
extern "C" int vtk_points_projected_hull_get_size_ccw_hull_z(vtkPointsProjectedHull* sself);
extern "C" void vtk_points_projected_hull_update(vtkPointsProjectedHull* sself);
extern "C" vtkPolyData * vtkPolyData_new () ;
extern "C" void vtkPolyData_destructor (vtkPolyData * sself) ;
extern "C" int vtk_poly_data_get_data_object_type(vtkPolyData* sself);
extern "C" long long vtk_poly_data_get_number_of_cells(vtkPolyData* sself);
extern "C" int vtk_poly_data_get_cell_type(vtkPolyData* sself, long long cellId);
extern "C" void vtk_poly_data_compute_cells_bounds(vtkPolyData* sself);
extern "C" void vtk_poly_data_squeeze(vtkPolyData* sself);
extern "C" int vtk_poly_data_get_max_cell_size(vtkPolyData* sself);
extern "C" long long vtk_poly_data_get_cell_id_relative_to_cell_array(vtkPolyData* sself, long long cellId);
extern "C" long long vtk_poly_data_get_number_of_verts(vtkPolyData* sself);
extern "C" long long vtk_poly_data_get_number_of_lines(vtkPolyData* sself);
extern "C" long long vtk_poly_data_get_number_of_polys(vtkPolyData* sself);
extern "C" long long vtk_poly_data_get_number_of_strips(vtkPolyData* sself);
extern "C" bool vtk_poly_data_allocate_estimate(vtkPolyData* sself, long long numCells, long long maxCellSize);
extern "C" bool vtk_poly_data_allocate_exact(vtkPolyData* sself, long long numCells, long long connectivitySize);
extern "C" void vtk_poly_data_allocate(vtkPolyData* sself, long long numCells, int extSize);
extern "C" void vtk_poly_data_reset(vtkPolyData* sself);
extern "C" void vtk_poly_data_build_cells(vtkPolyData* sself);
extern "C" bool vtk_poly_data_need_to_build_cells(vtkPolyData* sself);
extern "C" void vtk_poly_data_build_links(vtkPolyData* sself, int initialSize);
extern "C" void vtk_poly_data_delete_cells(vtkPolyData* sself);
extern "C" void vtk_poly_data_delete_links(vtkPolyData* sself);
extern "C" int vtk_poly_data_is_triangle(vtkPolyData* sself, int v1, int v2, int v3);
extern "C" int vtk_poly_data_is_edge(vtkPolyData* sself, long long p1, long long p2);
extern "C" int vtk_poly_data_is_point_used_by_cell(vtkPolyData* sself, long long ptId, long long cellId);
extern "C" void vtk_poly_data_replace_cell_point(vtkPolyData* sself, long long cellId, long long oldPtId, long long newPtId);
extern "C" void vtk_poly_data_reverse_cell(vtkPolyData* sself, long long cellId);
extern "C" void vtk_poly_data_delete_point(vtkPolyData* sself, long long ptId);
extern "C" void vtk_poly_data_delete_cell(vtkPolyData* sself, long long cellId);
extern "C" void vtk_poly_data_remove_deleted_cells(vtkPolyData* sself);
extern "C" long long vtk_poly_data_insert_next_linked_point(vtkPolyData* sself, int numLinks);
extern "C" void vtk_poly_data_remove_cell_reference(vtkPolyData* sself, long long cellId);
extern "C" void vtk_poly_data_add_cell_reference(vtkPolyData* sself, long long cellId);
extern "C" void vtk_poly_data_remove_reference_to_cell(vtkPolyData* sself, long long ptId, long long cellId);
extern "C" void vtk_poly_data_add_reference_to_cell(vtkPolyData* sself, long long ptId, long long cellId);
extern "C" void vtk_poly_data_resize_cell_list(vtkPolyData* sself, long long ptId, int size);
extern "C" void vtk_poly_data_initialize(vtkPolyData* sself);
extern "C" int vtk_poly_data_get_piece(vtkPolyData* sself);
extern "C" int vtk_poly_data_get_number_of_pieces(vtkPolyData* sself);
extern "C" int vtk_poly_data_get_ghost_level(vtkPolyData* sself);
extern "C" void vtk_poly_data_remove_ghost_cells(vtkPolyData* sself);
extern "C" unsigned long vtk_poly_data_get_mesh_m_time(vtkPolyData* sself);
extern "C" unsigned long vtk_poly_data_get_m_time(vtkPolyData* sself);
extern "C" vtkPolyDataCollection * vtkPolyDataCollection_new () ;
extern "C" void vtkPolyDataCollection_destructor (vtkPolyDataCollection * sself) ;
extern "C" vtkPolyLine * vtkPolyLine_new () ;
extern "C" void vtkPolyLine_destructor (vtkPolyLine * sself) ;
extern "C" int vtk_poly_line_get_cell_type(vtkPolyLine* sself);
extern "C" int vtk_poly_line_get_cell_dimension(vtkPolyLine* sself);
extern "C" int vtk_poly_line_get_number_of_edges(vtkPolyLine* sself);
extern "C" int vtk_poly_line_get_number_of_faces(vtkPolyLine* sself);
extern "C" vtkPolyPlane * vtkPolyPlane_new () ;
extern "C" void vtkPolyPlane_destructor (vtkPolyPlane * sself) ;
extern "C" unsigned long vtk_poly_plane_get_m_time(vtkPolyPlane* sself);
extern "C" vtkPolyVertex * vtkPolyVertex_new () ;
extern "C" void vtkPolyVertex_destructor (vtkPolyVertex * sself) ;
extern "C" int vtk_poly_vertex_get_cell_type(vtkPolyVertex* sself);
extern "C" int vtk_poly_vertex_get_cell_dimension(vtkPolyVertex* sself);
extern "C" int vtk_poly_vertex_get_number_of_edges(vtkPolyVertex* sself);
extern "C" int vtk_poly_vertex_get_number_of_faces(vtkPolyVertex* sself);
extern "C" vtkPolygon * vtkPolygon_new () ;
extern "C" void vtkPolygon_destructor (vtkPolygon * sself) ;
extern "C" int vtk_polygon_get_cell_type(vtkPolygon* sself);
extern "C" int vtk_polygon_get_cell_dimension(vtkPolygon* sself);
extern "C" int vtk_polygon_get_number_of_edges(vtkPolygon* sself);
extern "C" int vtk_polygon_get_number_of_faces(vtkPolygon* sself);
extern "C" double vtk_polygon_compute_area(vtkPolygon* sself);
extern "C" bool vtk_polygon_is_convex(vtkPolygon* sself);
extern "C" bool vtk_polygon_get_use_mvc_interpolation(vtkPolygon* sself);
extern "C" void vtk_polygon_set_use_mvc_interpolation(vtkPolygon* sself, bool _arg);
extern "C" void vtk_polygon_set_tolerance(vtkPolygon* sself, double _arg);
extern "C" double vtk_polygon_get_tolerance_min_value(vtkPolygon* sself);
extern "C" double vtk_polygon_get_tolerance_max_value(vtkPolygon* sself);
extern "C" double vtk_polygon_get_tolerance(vtkPolygon* sself);
extern "C" int vtk_polygon_ear_cut_triangulation(vtkPolygon* sself, int measure);
extern "C" int vtk_polygon_unbiased_ear_cut_triangulation(vtkPolygon* sself, int seed, int measure);
extern "C" vtkPolyhedron * vtkPolyhedron_new () ;
extern "C" void vtkPolyhedron_destructor (vtkPolyhedron * sself) ;
extern "C" int vtk_polyhedron_get_cell_type(vtkPolyhedron* sself);
extern "C" int vtk_polyhedron_requires_initialization(vtkPolyhedron* sself);
extern "C" int vtk_polyhedron_get_number_of_edges(vtkPolyhedron* sself);
extern "C" int vtk_polyhedron_get_number_of_faces(vtkPolyhedron* sself);
extern "C" int vtk_polyhedron_is_primary_cell(vtkPolyhedron* sself);
extern "C" int vtk_polyhedron_requires_explicit_face_representation(vtkPolyhedron* sself);
extern "C" bool vtk_polyhedron_is_convex(vtkPolyhedron* sself);
extern "C" vtkPyramid * vtkPyramid_new () ;
extern "C" void vtkPyramid_destructor (vtkPyramid * sself) ;
extern "C" int vtk_pyramid_get_cell_type(vtkPyramid* sself);
extern "C" int vtk_pyramid_get_number_of_edges(vtkPyramid* sself);
extern "C" int vtk_pyramid_get_number_of_faces(vtkPyramid* sself);
extern "C" vtkQuad * vtkQuad_new () ;
extern "C" void vtkQuad_destructor (vtkQuad * sself) ;
extern "C" int vtk_quad_get_cell_type(vtkQuad* sself);
extern "C" int vtk_quad_get_cell_dimension(vtkQuad* sself);
extern "C" int vtk_quad_get_number_of_edges(vtkQuad* sself);
extern "C" int vtk_quad_get_number_of_faces(vtkQuad* sself);
extern "C" vtkQuadraticEdge * vtkQuadraticEdge_new () ;
extern "C" void vtkQuadraticEdge_destructor (vtkQuadraticEdge * sself) ;
extern "C" int vtk_quadratic_edge_get_cell_type(vtkQuadraticEdge* sself);
extern "C" int vtk_quadratic_edge_get_cell_dimension(vtkQuadraticEdge* sself);
extern "C" int vtk_quadratic_edge_get_number_of_edges(vtkQuadraticEdge* sself);
extern "C" int vtk_quadratic_edge_get_number_of_faces(vtkQuadraticEdge* sself);
extern "C" vtkQuadraticHexahedron * vtkQuadraticHexahedron_new () ;
extern "C" void vtkQuadraticHexahedron_destructor (vtkQuadraticHexahedron * sself) ;
extern "C" int vtk_quadratic_hexahedron_get_cell_type(vtkQuadraticHexahedron* sself);
extern "C" int vtk_quadratic_hexahedron_get_cell_dimension(vtkQuadraticHexahedron* sself);
extern "C" int vtk_quadratic_hexahedron_get_number_of_edges(vtkQuadraticHexahedron* sself);
extern "C" int vtk_quadratic_hexahedron_get_number_of_faces(vtkQuadraticHexahedron* sself);
extern "C" vtkQuadraticLinearQuad * vtkQuadraticLinearQuad_new () ;
extern "C" void vtkQuadraticLinearQuad_destructor (vtkQuadraticLinearQuad * sself) ;
extern "C" int vtk_quadratic_linear_quad_get_cell_type(vtkQuadraticLinearQuad* sself);
extern "C" int vtk_quadratic_linear_quad_get_cell_dimension(vtkQuadraticLinearQuad* sself);
extern "C" int vtk_quadratic_linear_quad_get_number_of_edges(vtkQuadraticLinearQuad* sself);
extern "C" int vtk_quadratic_linear_quad_get_number_of_faces(vtkQuadraticLinearQuad* sself);
extern "C" vtkQuadraticLinearWedge * vtkQuadraticLinearWedge_new () ;
extern "C" void vtkQuadraticLinearWedge_destructor (vtkQuadraticLinearWedge * sself) ;
extern "C" int vtk_quadratic_linear_wedge_get_cell_type(vtkQuadraticLinearWedge* sself);
extern "C" int vtk_quadratic_linear_wedge_get_cell_dimension(vtkQuadraticLinearWedge* sself);
extern "C" int vtk_quadratic_linear_wedge_get_number_of_edges(vtkQuadraticLinearWedge* sself);
extern "C" int vtk_quadratic_linear_wedge_get_number_of_faces(vtkQuadraticLinearWedge* sself);
extern "C" vtkQuadraticPolygon * vtkQuadraticPolygon_new () ;
extern "C" void vtkQuadraticPolygon_destructor (vtkQuadraticPolygon * sself) ;
extern "C" int vtk_quadratic_polygon_get_cell_type(vtkQuadraticPolygon* sself);
extern "C" int vtk_quadratic_polygon_get_cell_dimension(vtkQuadraticPolygon* sself);
extern "C" int vtk_quadratic_polygon_get_number_of_edges(vtkQuadraticPolygon* sself);
extern "C" int vtk_quadratic_polygon_get_number_of_faces(vtkQuadraticPolygon* sself);
extern "C" bool vtk_quadratic_polygon_get_use_mvc_interpolation(vtkQuadraticPolygon* sself);
extern "C" void vtk_quadratic_polygon_set_use_mvc_interpolation(vtkQuadraticPolygon* sself, bool _arg);
extern "C" vtkQuadraticPyramid * vtkQuadraticPyramid_new () ;
extern "C" void vtkQuadraticPyramid_destructor (vtkQuadraticPyramid * sself) ;
extern "C" int vtk_quadratic_pyramid_get_cell_type(vtkQuadraticPyramid* sself);
extern "C" int vtk_quadratic_pyramid_get_cell_dimension(vtkQuadraticPyramid* sself);
extern "C" int vtk_quadratic_pyramid_get_number_of_edges(vtkQuadraticPyramid* sself);
extern "C" int vtk_quadratic_pyramid_get_number_of_faces(vtkQuadraticPyramid* sself);
extern "C" vtkQuadraticQuad * vtkQuadraticQuad_new () ;
extern "C" void vtkQuadraticQuad_destructor (vtkQuadraticQuad * sself) ;
extern "C" int vtk_quadratic_quad_get_cell_type(vtkQuadraticQuad* sself);
extern "C" int vtk_quadratic_quad_get_cell_dimension(vtkQuadraticQuad* sself);
extern "C" int vtk_quadratic_quad_get_number_of_edges(vtkQuadraticQuad* sself);
extern "C" int vtk_quadratic_quad_get_number_of_faces(vtkQuadraticQuad* sself);
extern "C" vtkQuadraticTetra * vtkQuadraticTetra_new () ;
extern "C" void vtkQuadraticTetra_destructor (vtkQuadraticTetra * sself) ;
extern "C" int vtk_quadratic_tetra_get_cell_type(vtkQuadraticTetra* sself);
extern "C" int vtk_quadratic_tetra_get_cell_dimension(vtkQuadraticTetra* sself);
extern "C" int vtk_quadratic_tetra_get_number_of_edges(vtkQuadraticTetra* sself);
extern "C" int vtk_quadratic_tetra_get_number_of_faces(vtkQuadraticTetra* sself);
extern "C" vtkQuadraticTriangle * vtkQuadraticTriangle_new () ;
extern "C" void vtkQuadraticTriangle_destructor (vtkQuadraticTriangle * sself) ;
extern "C" int vtk_quadratic_triangle_get_cell_type(vtkQuadraticTriangle* sself);
extern "C" int vtk_quadratic_triangle_get_cell_dimension(vtkQuadraticTriangle* sself);
extern "C" int vtk_quadratic_triangle_get_number_of_edges(vtkQuadraticTriangle* sself);
extern "C" int vtk_quadratic_triangle_get_number_of_faces(vtkQuadraticTriangle* sself);
extern "C" vtkQuadraticWedge * vtkQuadraticWedge_new () ;
extern "C" void vtkQuadraticWedge_destructor (vtkQuadraticWedge * sself) ;
extern "C" int vtk_quadratic_wedge_get_cell_type(vtkQuadraticWedge* sself);
extern "C" int vtk_quadratic_wedge_get_cell_dimension(vtkQuadraticWedge* sself);
extern "C" int vtk_quadratic_wedge_get_number_of_edges(vtkQuadraticWedge* sself);
extern "C" int vtk_quadratic_wedge_get_number_of_faces(vtkQuadraticWedge* sself);
extern "C" vtkQuadratureSchemeDefinition * vtkQuadratureSchemeDefinition_new () ;
extern "C" void vtkQuadratureSchemeDefinition_destructor (vtkQuadratureSchemeDefinition * sself) ;
extern "C" void vtk_quadrature_scheme_definition_clear(vtkQuadratureSchemeDefinition* sself);
extern "C" int vtk_quadrature_scheme_definition_get_cell_type(vtkQuadratureSchemeDefinition* sself);
extern "C" int vtk_quadrature_scheme_definition_get_quadrature_key(vtkQuadratureSchemeDefinition* sself);
extern "C" int vtk_quadrature_scheme_definition_get_number_of_nodes(vtkQuadratureSchemeDefinition* sself);
extern "C" int vtk_quadrature_scheme_definition_get_number_of_quadrature_points(vtkQuadratureSchemeDefinition* sself);
extern "C" vtkQuadric * vtkQuadric_new () ;
extern "C" void vtkQuadric_destructor (vtkQuadric * sself) ;
extern "C" void vtk_quadric_set_coefficients(vtkQuadric* sself, double a0, double a1, double a2, double a3, double a4, double a5, double a6, double a7, double a8, double a9);
extern "C" vtkRectilinearGrid * vtkRectilinearGrid_new () ;
extern "C" void vtkRectilinearGrid_destructor (vtkRectilinearGrid * sself) ;
extern "C" int vtk_rectilinear_grid_get_data_object_type(vtkRectilinearGrid* sself);
extern "C" void vtk_rectilinear_grid_initialize(vtkRectilinearGrid* sself);
extern "C" long long vtk_rectilinear_grid_get_number_of_cells(vtkRectilinearGrid* sself);
extern "C" long long vtk_rectilinear_grid_get_number_of_points(vtkRectilinearGrid* sself);
extern "C" int vtk_rectilinear_grid_get_cell_type(vtkRectilinearGrid* sself, long long cellId);
extern "C" int vtk_rectilinear_grid_get_max_cell_size(vtkRectilinearGrid* sself);
extern "C" unsigned char vtk_rectilinear_grid_is_point_visible(vtkRectilinearGrid* sself, long long ptId);
extern "C" unsigned char vtk_rectilinear_grid_is_cell_visible(vtkRectilinearGrid* sself, long long cellId);
extern "C" bool vtk_rectilinear_grid_has_any_blank_points(vtkRectilinearGrid* sself);
extern "C" bool vtk_rectilinear_grid_has_any_blank_cells(vtkRectilinearGrid* sself);
extern "C" void vtk_rectilinear_grid_set_dimensions(vtkRectilinearGrid* sself, int i, int j, int k);
extern "C" int vtk_rectilinear_grid_get_data_dimension(vtkRectilinearGrid* sself);
extern "C" void vtk_rectilinear_grid_set_extent(vtkRectilinearGrid* sself, int xMin, int xMax, int yMin, int yMax, int zMin, int zMax);
extern "C" int vtk_rectilinear_grid_get_extent_type(vtkRectilinearGrid* sself);
extern "C" const char* vtk_rectilinear_grid_get_scalar_type_as_string(vtkRectilinearGrid* sself);
extern "C" vtkReebGraph * vtkReebGraph_new () ;
extern "C" void vtkReebGraph_destructor (vtkReebGraph * sself) ;
extern "C" int vtk_reeb_graph_stream_triangle(vtkReebGraph* sself, long long vertex0Id, double scalar0, long long vertex1Id, double scalar1, long long vertex2Id, double scalar2);
extern "C" int vtk_reeb_graph_stream_tetrahedron(vtkReebGraph* sself, long long vertex0Id, double scalar0, long long vertex1Id, double scalar1, long long vertex2Id, double scalar2, long long vertex3Id, double scalar3);
extern "C" void vtk_reeb_graph_close_stream(vtkReebGraph* sself);
extern "C" vtkReebGraphSimplificationMetric * vtkReebGraphSimplificationMetric_new () ;
extern "C" void vtkReebGraphSimplificationMetric_destructor (vtkReebGraphSimplificationMetric * sself) ;
extern "C" void vtk_reeb_graph_simplification_metric_set_lower_bound(vtkReebGraphSimplificationMetric* sself, double _arg);
extern "C" double vtk_reeb_graph_simplification_metric_get_lower_bound(vtkReebGraphSimplificationMetric* sself);
extern "C" void vtk_reeb_graph_simplification_metric_set_upper_bound(vtkReebGraphSimplificationMetric* sself, double _arg);
extern "C" double vtk_reeb_graph_simplification_metric_get_upper_bound(vtkReebGraphSimplificationMetric* sself);
extern "C" vtkSelection * vtkSelection_new () ;
extern "C" void vtkSelection_destructor (vtkSelection * sself) ;
extern "C" int vtk_selection_get_data_object_type(vtkSelection* sself);
extern "C" unsigned int vtk_selection_get_number_of_nodes(vtkSelection* sself);
extern "C" void vtk_selection_remove_node(vtkSelection* sself, unsigned int idx);
extern "C" void vtk_selection_remove_all_nodes(vtkSelection* sself);
extern "C" void vtk_selection_set_expression(vtkSelection* sself, const char* _arg);
extern "C" unsigned long vtk_selection_get_m_time(vtkSelection* sself);
extern "C" void vtk_selection_dump(vtkSelection* sself);
extern "C" vtkSelectionNode * vtkSelectionNode_new () ;
extern "C" void vtkSelectionNode_destructor (vtkSelectionNode * sself) ;
extern "C" void vtk_selection_node_initialize(vtkSelectionNode* sself);
extern "C" unsigned long vtk_selection_node_get_m_time(vtkSelectionNode* sself);
extern "C" void vtk_selection_node_set_content_type(vtkSelectionNode* sself, int type);
extern "C" int vtk_selection_node_get_content_type(vtkSelectionNode* sself);
extern "C" const char* vtk_selection_node_get_content_type_as_string(vtkSelectionNode* sself, int type);
extern "C" void vtk_selection_node_set_field_type(vtkSelectionNode* sself, int type);
extern "C" int vtk_selection_node_get_field_type(vtkSelectionNode* sself);
extern "C" const char* vtk_selection_node_get_field_type_as_string(vtkSelectionNode* sself, int type);
extern "C" int vtk_selection_node_get_field_type_from_string(vtkSelectionNode* sself, const char* type);
extern "C" int vtk_selection_node_convert_selection_field_to_attribute_type(vtkSelectionNode* sself, int val);
extern "C" int vtk_selection_node_convert_attribute_type_to_selection_field(vtkSelectionNode* sself, int val);
extern "C" void vtk_selection_node_set_query_string(vtkSelectionNode* sself, const char* _arg);
extern "C" vtkSimpleCellTessellator * vtkSimpleCellTessellator_new () ;
extern "C" void vtkSimpleCellTessellator_destructor (vtkSimpleCellTessellator * sself) ;
extern "C" void vtk_simple_cell_tessellator_reset(vtkSimpleCellTessellator* sself);
extern "C" int vtk_simple_cell_tessellator_get_fixed_subdivisions(vtkSimpleCellTessellator* sself);
extern "C" int vtk_simple_cell_tessellator_get_max_subdivision_level(vtkSimpleCellTessellator* sself);
extern "C" int vtk_simple_cell_tessellator_get_max_adaptive_subdivisions(vtkSimpleCellTessellator* sself);
extern "C" void vtk_simple_cell_tessellator_set_fixed_subdivisions(vtkSimpleCellTessellator* sself, int level);
extern "C" void vtk_simple_cell_tessellator_set_max_subdivision_level(vtkSimpleCellTessellator* sself, int level);
extern "C" void vtk_simple_cell_tessellator_set_subdivision_levels(vtkSimpleCellTessellator* sself, int fixed, int maxLevel);
extern "C" vtkSmoothErrorMetric * vtkSmoothErrorMetric_new () ;
extern "C" void vtkSmoothErrorMetric_destructor (vtkSmoothErrorMetric * sself) ;
extern "C" double vtk_smooth_error_metric_get_angle_tolerance(vtkSmoothErrorMetric* sself);
extern "C" void vtk_smooth_error_metric_set_angle_tolerance(vtkSmoothErrorMetric* sself, double value);
extern "C" vtkSortFieldData * vtkSortFieldData_new () ;
extern "C" void vtkSortFieldData_destructor (vtkSortFieldData * sself) ;
extern "C" vtkSphere * vtkSphere_new () ;
extern "C" void vtkSphere_destructor (vtkSphere * sself) ;
extern "C" void vtk_sphere_set_radius(vtkSphere* sself, double _arg);
extern "C" double vtk_sphere_get_radius(vtkSphere* sself);
extern "C" void vtk_sphere_set_center(vtkSphere* sself, double _arg1, double _arg2, double _arg3);
extern "C" vtkSpheres * vtkSpheres_new () ;
extern "C" void vtkSpheres_destructor (vtkSpheres * sself) ;
extern "C" int vtk_spheres_get_number_of_spheres(vtkSpheres* sself);
extern "C" vtkStaticCellLinks * vtkStaticCellLinks_new () ;
extern "C" void vtkStaticCellLinks_destructor (vtkStaticCellLinks * sself) ;
extern "C" long long vtk_static_cell_links_get_number_of_cells(vtkStaticCellLinks* sself, long long ptId);
extern "C" long long vtk_static_cell_links_get_ncells(vtkStaticCellLinks* sself, long long ptId);
extern "C" void vtk_static_cell_links_initialize(vtkStaticCellLinks* sself);
extern "C" void vtk_static_cell_links_squeeze(vtkStaticCellLinks* sself);
extern "C" void vtk_static_cell_links_reset(vtkStaticCellLinks* sself);
extern "C" unsigned long vtk_static_cell_links_get_actual_memory_size(vtkStaticCellLinks* sself);
extern "C" vtkStaticCellLocator * vtkStaticCellLocator_new () ;
extern "C" void vtkStaticCellLocator_destructor (vtkStaticCellLocator * sself) ;
extern "C" void vtk_static_cell_locator_set_divisions(vtkStaticCellLocator* sself, int _arg1, int _arg2, int _arg3);
extern "C" void vtk_static_cell_locator_free_search_structure(vtkStaticCellLocator* sself);
extern "C" void vtk_static_cell_locator_build_locator(vtkStaticCellLocator* sself);
extern "C" void vtk_static_cell_locator_set_max_number_of_buckets(vtkStaticCellLocator* sself, long long _arg);
extern "C" long long vtk_static_cell_locator_get_max_number_of_buckets_min_value(vtkStaticCellLocator* sself);
extern "C" long long vtk_static_cell_locator_get_max_number_of_buckets_max_value(vtkStaticCellLocator* sself);
extern "C" long long vtk_static_cell_locator_get_max_number_of_buckets(vtkStaticCellLocator* sself);
extern "C" bool vtk_static_cell_locator_get_large_ids(vtkStaticCellLocator* sself);
extern "C" void vtk_static_cell_locator_set_use_diagonal_length_tolerance(vtkStaticCellLocator* sself, bool _arg);
extern "C" bool vtk_static_cell_locator_get_use_diagonal_length_tolerance(vtkStaticCellLocator* sself);
extern "C" void vtk_static_cell_locator_use_diagonal_length_tolerance_on(vtkStaticCellLocator* sself);
extern "C" void vtk_static_cell_locator_use_diagonal_length_tolerance_off(vtkStaticCellLocator* sself);
extern "C" vtkStaticPointLocator * vtkStaticPointLocator_new () ;
extern "C" void vtkStaticPointLocator_destructor (vtkStaticPointLocator * sself) ;
extern "C" void vtk_static_point_locator_set_number_of_points_per_bucket(vtkStaticPointLocator* sself, int _arg);
extern "C" int vtk_static_point_locator_get_number_of_points_per_bucket_min_value(vtkStaticPointLocator* sself);
extern "C" int vtk_static_point_locator_get_number_of_points_per_bucket_max_value(vtkStaticPointLocator* sself);
extern "C" int vtk_static_point_locator_get_number_of_points_per_bucket(vtkStaticPointLocator* sself);
extern "C" void vtk_static_point_locator_set_divisions(vtkStaticPointLocator* sself, int _arg1, int _arg2, int _arg3);
extern "C" void vtk_static_point_locator_initialize(vtkStaticPointLocator* sself);
extern "C" void vtk_static_point_locator_free_search_structure(vtkStaticPointLocator* sself);
extern "C" void vtk_static_point_locator_build_locator(vtkStaticPointLocator* sself);
extern "C" long long vtk_static_point_locator_get_number_of_points_in_bucket(vtkStaticPointLocator* sself, long long bNum);
extern "C" void vtk_static_point_locator_set_max_number_of_buckets(vtkStaticPointLocator* sself, long long _arg);
extern "C" long long vtk_static_point_locator_get_max_number_of_buckets_min_value(vtkStaticPointLocator* sself);
extern "C" long long vtk_static_point_locator_get_max_number_of_buckets_max_value(vtkStaticPointLocator* sself);
extern "C" long long vtk_static_point_locator_get_max_number_of_buckets(vtkStaticPointLocator* sself);
extern "C" bool vtk_static_point_locator_get_large_ids(vtkStaticPointLocator* sself);
extern "C" vtkStaticPointLocator2D * vtkStaticPointLocator2D_new () ;
extern "C" void vtkStaticPointLocator2D_destructor (vtkStaticPointLocator2D * sself) ;
extern "C" void vtk_static_point_locator_2_d_set_number_of_points_per_bucket(vtkStaticPointLocator2D* sself, int _arg);
extern "C" int vtk_static_point_locator_2_d_get_number_of_points_per_bucket_min_value(vtkStaticPointLocator2D* sself);
extern "C" int vtk_static_point_locator_2_d_get_number_of_points_per_bucket_max_value(vtkStaticPointLocator2D* sself);
extern "C" int vtk_static_point_locator_2_d_get_number_of_points_per_bucket(vtkStaticPointLocator2D* sself);
extern "C" void vtk_static_point_locator_2_d_set_divisions(vtkStaticPointLocator2D* sself, int _arg1, int _arg2);
extern "C" void vtk_static_point_locator_2_d_initialize(vtkStaticPointLocator2D* sself);
extern "C" void vtk_static_point_locator_2_d_free_search_structure(vtkStaticPointLocator2D* sself);
extern "C" void vtk_static_point_locator_2_d_build_locator(vtkStaticPointLocator2D* sself);
extern "C" long long vtk_static_point_locator_2_d_get_number_of_points_in_bucket(vtkStaticPointLocator2D* sself, long long bNum);
extern "C" void vtk_static_point_locator_2_d_set_max_number_of_buckets(vtkStaticPointLocator2D* sself, long long _arg);
extern "C" long long vtk_static_point_locator_2_d_get_max_number_of_buckets_min_value(vtkStaticPointLocator2D* sself);
extern "C" long long vtk_static_point_locator_2_d_get_max_number_of_buckets_max_value(vtkStaticPointLocator2D* sself);
extern "C" long long vtk_static_point_locator_2_d_get_max_number_of_buckets(vtkStaticPointLocator2D* sself);
extern "C" bool vtk_static_point_locator_2_d_get_large_ids(vtkStaticPointLocator2D* sself);
extern "C" vtkStructuredExtent * vtkStructuredExtent_new () ;
extern "C" void vtkStructuredExtent_destructor (vtkStructuredExtent * sself) ;
extern "C" vtkStructuredGrid * vtkStructuredGrid_new () ;
extern "C" void vtkStructuredGrid_destructor (vtkStructuredGrid * sself) ;
extern "C" int vtk_structured_grid_get_data_object_type(vtkStructuredGrid* sself);
extern "C" long long vtk_structured_grid_get_number_of_points(vtkStructuredGrid* sself);
extern "C" int vtk_structured_grid_get_cell_type(vtkStructuredGrid* sself, long long cellId);
extern "C" void vtk_structured_grid_set_dimensions(vtkStructuredGrid* sself, int i, int j, int k);
extern "C" int vtk_structured_grid_get_data_dimension(vtkStructuredGrid* sself);
extern "C" void vtk_structured_grid_set_extent(vtkStructuredGrid* sself, int xMin, int xMax, int yMin, int yMax, int zMin, int zMax);
extern "C" int vtk_structured_grid_get_extent_type(vtkStructuredGrid* sself);
extern "C" void vtk_structured_grid_blank_point(vtkStructuredGrid* sself, long long ptId);
extern "C" void vtk_structured_grid_un_blank_point(vtkStructuredGrid* sself, long long ptId);
extern "C" void vtk_structured_grid_blank_cell(vtkStructuredGrid* sself, long long ptId);
extern "C" void vtk_structured_grid_un_blank_cell(vtkStructuredGrid* sself, long long ptId);
extern "C" unsigned char vtk_structured_grid_is_point_visible(vtkStructuredGrid* sself, long long ptId);
extern "C" unsigned char vtk_structured_grid_is_cell_visible(vtkStructuredGrid* sself, long long cellId);
extern "C" bool vtk_structured_grid_has_any_blank_points(vtkStructuredGrid* sself);
extern "C" bool vtk_structured_grid_has_any_blank_cells(vtkStructuredGrid* sself);
extern "C" vtkStructuredPoints * vtkStructuredPoints_new () ;
extern "C" void vtkStructuredPoints_destructor (vtkStructuredPoints * sself) ;
extern "C" int vtk_structured_points_get_data_object_type(vtkStructuredPoints* sself);
extern "C" vtkStructuredPointsCollection * vtkStructuredPointsCollection_new () ;
extern "C" void vtkStructuredPointsCollection_destructor (vtkStructuredPointsCollection * sself) ;
extern "C" vtkSuperquadric * vtkSuperquadric_new () ;
extern "C" void vtkSuperquadric_destructor (vtkSuperquadric * sself) ;
extern "C" void vtk_superquadric_set_center(vtkSuperquadric* sself, double _arg1, double _arg2, double _arg3);
extern "C" void vtk_superquadric_set_scale(vtkSuperquadric* sself, double _arg1, double _arg2, double _arg3);
extern "C" double vtk_superquadric_get_thickness(vtkSuperquadric* sself);
extern "C" void vtk_superquadric_set_thickness(vtkSuperquadric* sself, double _arg);
extern "C" double vtk_superquadric_get_thickness_min_value(vtkSuperquadric* sself);
extern "C" double vtk_superquadric_get_thickness_max_value(vtkSuperquadric* sself);
extern "C" double vtk_superquadric_get_phi_roundness(vtkSuperquadric* sself);
extern "C" void vtk_superquadric_set_phi_roundness(vtkSuperquadric* sself, double e);
extern "C" double vtk_superquadric_get_theta_roundness(vtkSuperquadric* sself);
extern "C" void vtk_superquadric_set_theta_roundness(vtkSuperquadric* sself, double e);
extern "C" void vtk_superquadric_set_size(vtkSuperquadric* sself, double _arg);
extern "C" double vtk_superquadric_get_size(vtkSuperquadric* sself);
extern "C" void vtk_superquadric_toroidal_on(vtkSuperquadric* sself);
extern "C" void vtk_superquadric_toroidal_off(vtkSuperquadric* sself);
extern "C" int vtk_superquadric_get_toroidal(vtkSuperquadric* sself);
extern "C" void vtk_superquadric_set_toroidal(vtkSuperquadric* sself, int _arg);
extern "C" vtkTable * vtkTable_new () ;
extern "C" void vtkTable_destructor (vtkTable * sself) ;
extern "C" void vtk_table_dump(vtkTable* sself, unsigned int colWidth, int rowLimit);
extern "C" int vtk_table_get_data_object_type(vtkTable* sself);
extern "C" long long vtk_table_get_number_of_rows(vtkTable* sself);
extern "C" void vtk_table_set_number_of_rows(vtkTable* sself, const long long p0);
extern "C" long long vtk_table_insert_next_blank_row(vtkTable* sself, double default_num_val);
extern "C" void vtk_table_remove_row(vtkTable* sself, long long row);
extern "C" long long vtk_table_get_number_of_columns(vtkTable* sself);
extern "C" const char* vtk_table_get_column_name(vtkTable* sself, long long col);
extern "C" void vtk_table_remove_column_by_name(vtkTable* sself, const char* name);
extern "C" void vtk_table_remove_column(vtkTable* sself, long long col);
extern "C" void vtk_table_initialize(vtkTable* sself);
extern "C" long long vtk_table_get_number_of_elements(vtkTable* sself, int type);
extern "C" vtkTetra * vtkTetra_new () ;
extern "C" void vtkTetra_destructor (vtkTetra * sself) ;
extern "C" int vtk_tetra_get_cell_type(vtkTetra* sself);
extern "C" int vtk_tetra_get_number_of_edges(vtkTetra* sself);
extern "C" int vtk_tetra_get_number_of_faces(vtkTetra* sself);
extern "C" vtkTree * vtkTree_new () ;
extern "C" void vtkTree_destructor (vtkTree * sself) ;
extern "C" long long vtk_tree_get_root(vtkTree* sself);
extern "C" long long vtk_tree_get_number_of_children(vtkTree* sself, long long v);
extern "C" long long vtk_tree_get_child(vtkTree* sself, long long v, long long i);
extern "C" long long vtk_tree_get_parent(vtkTree* sself, long long v);
extern "C" long long vtk_tree_get_level(vtkTree* sself, long long v);
extern "C" bool vtk_tree_is_leaf(vtkTree* sself, long long vertex);
extern "C" vtkTreeBFSIterator * vtkTreeBFSIterator_new () ;
extern "C" void vtkTreeBFSIterator_destructor (vtkTreeBFSIterator * sself) ;
extern "C" vtkTreeDFSIterator * vtkTreeDFSIterator_new () ;
extern "C" void vtkTreeDFSIterator_destructor (vtkTreeDFSIterator * sself) ;
extern "C" void vtk_tree_dfs_iterator_set_mode(vtkTreeDFSIterator* sself, int mode);
extern "C" int vtk_tree_dfs_iterator_get_mode(vtkTreeDFSIterator* sself);
extern "C" vtkTriQuadraticHexahedron * vtkTriQuadraticHexahedron_new () ;
extern "C" void vtkTriQuadraticHexahedron_destructor (vtkTriQuadraticHexahedron * sself) ;
extern "C" int vtk_tri_quadratic_hexahedron_get_cell_type(vtkTriQuadraticHexahedron* sself);
extern "C" int vtk_tri_quadratic_hexahedron_get_cell_dimension(vtkTriQuadraticHexahedron* sself);
extern "C" int vtk_tri_quadratic_hexahedron_get_number_of_edges(vtkTriQuadraticHexahedron* sself);
extern "C" int vtk_tri_quadratic_hexahedron_get_number_of_faces(vtkTriQuadraticHexahedron* sself);
extern "C" vtkTriQuadraticPyramid * vtkTriQuadraticPyramid_new () ;
extern "C" void vtkTriQuadraticPyramid_destructor (vtkTriQuadraticPyramid * sself) ;
extern "C" int vtk_tri_quadratic_pyramid_get_cell_type(vtkTriQuadraticPyramid* sself);
extern "C" int vtk_tri_quadratic_pyramid_get_cell_dimension(vtkTriQuadraticPyramid* sself);
extern "C" int vtk_tri_quadratic_pyramid_get_number_of_edges(vtkTriQuadraticPyramid* sself);
extern "C" int vtk_tri_quadratic_pyramid_get_number_of_faces(vtkTriQuadraticPyramid* sself);
extern "C" vtkTriangle * vtkTriangle_new () ;
extern "C" void vtkTriangle_destructor (vtkTriangle * sself) ;
extern "C" int vtk_triangle_get_cell_type(vtkTriangle* sself);
extern "C" int vtk_triangle_get_cell_dimension(vtkTriangle* sself);
extern "C" int vtk_triangle_get_number_of_edges(vtkTriangle* sself);
extern "C" int vtk_triangle_get_number_of_faces(vtkTriangle* sself);
extern "C" double vtk_triangle_compute_area(vtkTriangle* sself);
extern "C" vtkTriangleStrip * vtkTriangleStrip_new () ;
extern "C" void vtkTriangleStrip_destructor (vtkTriangleStrip * sself) ;
extern "C" int vtk_triangle_strip_get_cell_type(vtkTriangleStrip* sself);
extern "C" int vtk_triangle_strip_get_cell_dimension(vtkTriangleStrip* sself);
extern "C" int vtk_triangle_strip_get_number_of_edges(vtkTriangleStrip* sself);
extern "C" int vtk_triangle_strip_get_number_of_faces(vtkTriangleStrip* sself);
extern "C" vtkUndirectedGraph * vtkUndirectedGraph_new () ;
extern "C" void vtkUndirectedGraph_destructor (vtkUndirectedGraph * sself) ;
extern "C" long long vtk_undirected_graph_get_in_degree(vtkUndirectedGraph* sself, long long v);
extern "C" vtkUniformGrid * vtkUniformGrid_new () ;
extern "C" void vtkUniformGrid_destructor (vtkUniformGrid * sself) ;
extern "C" int vtk_uniform_grid_get_grid_description(vtkUniformGrid* sself);
extern "C" void vtk_uniform_grid_blank_point(vtkUniformGrid* sself, long long ptId);
extern "C" void vtk_uniform_grid_un_blank_point(vtkUniformGrid* sself, long long ptId);
extern "C" void vtk_uniform_grid_blank_cell(vtkUniformGrid* sself, long long ptId);
extern "C" void vtk_uniform_grid_un_blank_cell(vtkUniformGrid* sself, long long ptId);
extern "C" unsigned char vtk_uniform_grid_is_point_visible(vtkUniformGrid* sself, long long pointId);
extern "C" unsigned char vtk_uniform_grid_is_cell_visible(vtkUniformGrid* sself, long long cellId);
extern "C" vtkUniformGridAMR * vtkUniformGridAMR_new () ;
extern "C" void vtkUniformGridAMR_destructor (vtkUniformGridAMR * sself) ;
extern "C" int vtk_uniform_grid_amr_get_data_object_type(vtkUniformGridAMR* sself);
extern "C" void vtk_uniform_grid_amr_initialize(vtkUniformGridAMR* sself);
extern "C" void vtk_uniform_grid_amr_set_grid_description(vtkUniformGridAMR* sself, int gridDescription);
extern "C" int vtk_uniform_grid_amr_get_grid_description(vtkUniformGridAMR* sself);
extern "C" unsigned int vtk_uniform_grid_amr_get_number_of_levels(vtkUniformGridAMR* sself);
extern "C" unsigned int vtk_uniform_grid_amr_get_total_number_of_blocks(vtkUniformGridAMR* sself);
extern "C" unsigned int vtk_uniform_grid_amr_get_number_of_data_sets(vtkUniformGridAMR* sself, const unsigned int level);
extern "C" int vtk_uniform_grid_amr_get_composite_index(vtkUniformGridAMR* sself, const unsigned int level, const unsigned int index);
extern "C" void vtk_uniform_grid_amr_get_level_and_index(vtkUniformGridAMR* sself, const unsigned int compositeIdx, unsigned int& level, unsigned int& idx);
extern "C" vtkUniformGridAMRDataIterator * vtkUniformGridAMRDataIterator_new () ;
extern "C" void vtkUniformGridAMRDataIterator_destructor (vtkUniformGridAMRDataIterator * sself) ;
extern "C" int vtk_uniform_grid_amr_data_iterator_has_current_meta_data(vtkUniformGridAMRDataIterator* sself);
extern "C" unsigned int vtk_uniform_grid_amr_data_iterator_get_current_flat_index(vtkUniformGridAMRDataIterator* sself);
extern "C" unsigned int vtk_uniform_grid_amr_data_iterator_get_current_level(vtkUniformGridAMRDataIterator* sself);
extern "C" unsigned int vtk_uniform_grid_amr_data_iterator_get_current_index(vtkUniformGridAMRDataIterator* sself);
extern "C" void vtk_uniform_grid_amr_data_iterator_go_to_first_item(vtkUniformGridAMRDataIterator* sself);
extern "C" void vtk_uniform_grid_amr_data_iterator_go_to_next_item(vtkUniformGridAMRDataIterator* sself);
extern "C" int vtk_uniform_grid_amr_data_iterator_is_done_with_traversal(vtkUniformGridAMRDataIterator* sself);
extern "C" vtkUniformHyperTreeGrid * vtkUniformHyperTreeGrid_new () ;
extern "C" void vtkUniformHyperTreeGrid_destructor (vtkUniformHyperTreeGrid * sself) ;
extern "C" void vtk_uniform_hyper_tree_grid_set_origin(vtkUniformHyperTreeGrid* sself, double _arg1, double _arg2, double _arg3);
extern "C" void vtk_uniform_hyper_tree_grid_set_grid_scale(vtkUniformHyperTreeGrid* sself, double p0, double p1, double p2);
extern "C" unsigned long vtk_uniform_hyper_tree_grid_get_actual_memory_size_bytes(vtkUniformHyperTreeGrid* sself);
extern "C" vtkUnstructuredGrid * vtkUnstructuredGrid_new () ;
extern "C" void vtkUnstructuredGrid_destructor (vtkUnstructuredGrid * sself) ;
extern "C" int vtk_unstructured_grid_get_data_object_type(vtkUnstructuredGrid* sself);
extern "C" bool vtk_unstructured_grid_allocate_estimate(vtkUnstructuredGrid* sself, long long numCells, long long maxCellSize);
extern "C" bool vtk_unstructured_grid_allocate_exact(vtkUnstructuredGrid* sself, long long numCells, long long connectivitySize);
extern "C" void vtk_unstructured_grid_allocate(vtkUnstructuredGrid* sself, long long numCells, int extSize);
extern "C" void vtk_unstructured_grid_reset(vtkUnstructuredGrid* sself);
extern "C" int vtk_unstructured_grid_get_cell_type(vtkUnstructuredGrid* sself, long long cellId);
extern "C" void vtk_unstructured_grid_squeeze(vtkUnstructuredGrid* sself);
extern "C" void vtk_unstructured_grid_initialize(vtkUnstructuredGrid* sself);
extern "C" int vtk_unstructured_grid_get_max_cell_size(vtkUnstructuredGrid* sself);
extern "C" void vtk_unstructured_grid_build_links(vtkUnstructuredGrid* sself);
extern "C" void vtk_unstructured_grid_remove_reference_to_cell(vtkUnstructuredGrid* sself, long long ptId, long long cellId);
extern "C" void vtk_unstructured_grid_add_reference_to_cell(vtkUnstructuredGrid* sself, long long ptId, long long cellId);
extern "C" void vtk_unstructured_grid_resize_cell_list(vtkUnstructuredGrid* sself, long long ptId, int size);
extern "C" int vtk_unstructured_grid_get_piece(vtkUnstructuredGrid* sself);
extern "C" int vtk_unstructured_grid_get_number_of_pieces(vtkUnstructuredGrid* sself);
extern "C" int vtk_unstructured_grid_get_ghost_level(vtkUnstructuredGrid* sself);
extern "C" int vtk_unstructured_grid_is_homogeneous(vtkUnstructuredGrid* sself);
extern "C" void vtk_unstructured_grid_remove_ghost_cells(vtkUnstructuredGrid* sself);
extern "C" int vtk_unstructured_grid_initialize_faces_representation(vtkUnstructuredGrid* sself, long long numPrevCells);
extern "C" unsigned long vtk_unstructured_grid_get_mesh_m_time(vtkUnstructuredGrid* sself);
extern "C" vtkUnstructuredGridCellIterator * vtkUnstructuredGridCellIterator_new () ;
extern "C" void vtkUnstructuredGridCellIterator_destructor (vtkUnstructuredGridCellIterator * sself) ;
extern "C" bool vtk_unstructured_grid_cell_iterator_is_done_with_traversal(vtkUnstructuredGridCellIterator* sself);
extern "C" long long vtk_unstructured_grid_cell_iterator_get_cell_id(vtkUnstructuredGridCellIterator* sself);
extern "C" void vtk_unstructured_grid_cell_iterator_go_to_cell(vtkUnstructuredGridCellIterator* sself, long long cellId);
extern "C" vtkVertex * vtkVertex_new () ;
extern "C" void vtkVertex_destructor (vtkVertex * sself) ;
extern "C" int vtk_vertex_get_cell_type(vtkVertex* sself);
extern "C" int vtk_vertex_get_cell_dimension(vtkVertex* sself);
extern "C" int vtk_vertex_get_number_of_edges(vtkVertex* sself);
extern "C" int vtk_vertex_get_number_of_faces(vtkVertex* sself);
extern "C" int vtk_vertex_inflate(vtkVertex* sself, double p0);
extern "C" vtkVertexListIterator * vtkVertexListIterator_new () ;
extern "C" void vtkVertexListIterator_destructor (vtkVertexListIterator * sself) ;
extern "C" long long vtk_vertex_list_iterator_next(vtkVertexListIterator* sself);
extern "C" bool vtk_vertex_list_iterator_has_next(vtkVertexListIterator* sself);
extern "C" vtkVoxel * vtkVoxel_new () ;
extern "C" void vtkVoxel_destructor (vtkVoxel * sself) ;
extern "C" int vtk_voxel_get_cell_type(vtkVoxel* sself);
extern "C" int vtk_voxel_get_number_of_edges(vtkVoxel* sself);
extern "C" int vtk_voxel_get_number_of_faces(vtkVoxel* sself);
extern "C" int vtk_voxel_inflate(vtkVoxel* sself, double dist);
extern "C" vtkWedge * vtkWedge_new () ;
extern "C" void vtkWedge_destructor (vtkWedge * sself) ;
extern "C" int vtk_wedge_get_cell_type(vtkWedge* sself);
extern "C" int vtk_wedge_get_number_of_edges(vtkWedge* sself);
extern "C" int vtk_wedge_get_number_of_faces(vtkWedge* sself);
extern "C" vtkXMLDataElement * vtkXMLDataElement_new () ;
extern "C" void vtkXMLDataElement_destructor (vtkXMLDataElement * sself) ;
extern "C" void vtk_xml_data_element_set_name(vtkXMLDataElement* sself, const char* _arg);
extern "C" void vtk_xml_data_element_set_id(vtkXMLDataElement* sself, const char* _arg);
extern "C" const char* vtk_xml_data_element_get_attribute(vtkXMLDataElement* sself, const char* name);
extern "C" void vtk_xml_data_element_set_attribute(vtkXMLDataElement* sself, const char* name, const char* value);
extern "C" void vtk_xml_data_element_set_character_data(vtkXMLDataElement* sself, const char* data, int length);
extern "C" void vtk_xml_data_element_add_character_data(vtkXMLDataElement* sself, const char* c, size_t length);
extern "C" int vtk_xml_data_element_get_scalar_attribute(vtkXMLDataElement* sself, const char* name, int& value);
extern "C" void vtk_xml_data_element_set_int_attribute(vtkXMLDataElement* sself, const char* name, int value);
extern "C" void vtk_xml_data_element_set_float_attribute(vtkXMLDataElement* sself, const char* name, float value);
extern "C" void vtk_xml_data_element_set_double_attribute(vtkXMLDataElement* sself, const char* name, double value);
extern "C" void vtk_xml_data_element_set_unsigned_long_attribute(vtkXMLDataElement* sself, const char* name, unsigned long value);
extern "C" int vtk_xml_data_element_get_word_type_attribute(vtkXMLDataElement* sself, const char* name, int& value);
extern "C" int vtk_xml_data_element_get_number_of_attributes(vtkXMLDataElement* sself);
extern "C" const char* vtk_xml_data_element_get_attribute_name(vtkXMLDataElement* sself, int idx);
extern "C" const char* vtk_xml_data_element_get_attribute_value(vtkXMLDataElement* sself, int idx);
extern "C" void vtk_xml_data_element_remove_attribute(vtkXMLDataElement* sself, const char* name);
extern "C" void vtk_xml_data_element_remove_all_attributes(vtkXMLDataElement* sself);
extern "C" int vtk_xml_data_element_get_number_of_nested_elements(vtkXMLDataElement* sself);
extern "C" void vtk_xml_data_element_remove_all_nested_elements(vtkXMLDataElement* sself);
extern "C" long long vtk_xml_data_element_get_xml_byte_index(vtkXMLDataElement* sself);
extern "C" void vtk_xml_data_element_set_xml_byte_index(vtkXMLDataElement* sself, long long _arg);
extern "C" void vtk_xml_data_element_set_attribute_encoding(vtkXMLDataElement* sself, int _arg);
extern "C" int vtk_xml_data_element_get_attribute_encoding_min_value(vtkXMLDataElement* sself);
extern "C" int vtk_xml_data_element_get_attribute_encoding_max_value(vtkXMLDataElement* sself);
extern "C" int vtk_xml_data_element_get_attribute_encoding(vtkXMLDataElement* sself);
extern "C" void vtk_xml_data_element_print_xml(vtkXMLDataElement* sself, const char* fname);
extern "C" int vtk_xml_data_element_get_character_data_width(vtkXMLDataElement* sself);
extern "C" void vtk_xml_data_element_set_character_data_width(vtkXMLDataElement* sself, int _arg);
