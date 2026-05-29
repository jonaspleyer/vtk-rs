// Include header file
#include<vtk_common_data_model.h>

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

// Implement declared functions
extern "C" vtkAMRDataInternals * vtkAMRDataInternals_new () {return vtkAMRDataInternals :: New () ;}
extern "C" void vtkAMRDataInternals_destructor (vtkAMRDataInternals * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_amr_data_internals_initialize(vtkAMRDataInternals* sself) { sself->Initialize(); }
extern "C" bool vtk_amr_data_internals_empty(vtkAMRDataInternals* sself) { return sself->Empty(); }
extern "C" unsigned int vtk_amr_data_internals_get_number_of_blocks(vtkAMRDataInternals* sself) { return sself->GetNumberOfBlocks(); }
extern "C" vtkAdjacentVertexIterator * vtkAdjacentVertexIterator_new () {return vtkAdjacentVertexIterator :: New () ;}
extern "C" void vtkAdjacentVertexIterator_destructor (vtkAdjacentVertexIterator * sself) {sself -> Delete () ; return ;}
extern "C" long long vtk_adjacent_vertex_iterator_get_vertex(vtkAdjacentVertexIterator* sself) { return sself->GetVertex(); }
extern "C" long long vtk_adjacent_vertex_iterator_next(vtkAdjacentVertexIterator* sself) { return sself->Next(); }
extern "C" bool vtk_adjacent_vertex_iterator_has_next(vtkAdjacentVertexIterator* sself) { return sself->HasNext(); }
extern "C" vtkAnimationScene * vtkAnimationScene_new () {return vtkAnimationScene :: New () ;}
extern "C" void vtkAnimationScene_destructor (vtkAnimationScene * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_animation_scene_set_play_mode(vtkAnimationScene* sself, int _arg) { sself->SetPlayMode(_arg); }
extern "C" void vtk_animation_scene_set_mode_to_sequence(vtkAnimationScene* sself) { sself->SetModeToSequence(); }
extern "C" void vtk_animation_scene_set_mode_to_real_time(vtkAnimationScene* sself) { sself->SetModeToRealTime(); }
extern "C" int vtk_animation_scene_get_play_mode(vtkAnimationScene* sself) { return sself->GetPlayMode(); }
extern "C" void vtk_animation_scene_set_frame_rate(vtkAnimationScene* sself, double _arg) { sself->SetFrameRate(_arg); }
extern "C" double vtk_animation_scene_get_frame_rate(vtkAnimationScene* sself) { return sself->GetFrameRate(); }
extern "C" void vtk_animation_scene_remove_all_cues(vtkAnimationScene* sself) { sself->RemoveAllCues(); }
extern "C" int vtk_animation_scene_get_number_of_cues(vtkAnimationScene* sself) { return sself->GetNumberOfCues(); }
extern "C" void vtk_animation_scene_play(vtkAnimationScene* sself) { sself->Play(); }
extern "C" void vtk_animation_scene_stop(vtkAnimationScene* sself) { sself->Stop(); }
extern "C" void vtk_animation_scene_set_loop(vtkAnimationScene* sself, int _arg) { sself->SetLoop(_arg); }
extern "C" int vtk_animation_scene_get_loop(vtkAnimationScene* sself) { return sself->GetLoop(); }
extern "C" void vtk_animation_scene_set_animation_time(vtkAnimationScene* sself, double time) { sself->SetAnimationTime(time); }
extern "C" void vtk_animation_scene_set_time_mode(vtkAnimationScene* sself, int mode) { sself->SetTimeMode(mode); }
extern "C" int vtk_animation_scene_is_in_play(vtkAnimationScene* sself) { return sself->IsInPlay(); }
extern "C" vtkAnnotation * vtkAnnotation_new () {return vtkAnnotation :: New () ;}
extern "C" void vtkAnnotation_destructor (vtkAnnotation * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_annotation_get_data_object_type(vtkAnnotation* sself) { return sself->GetDataObjectType(); }
extern "C" void vtk_annotation_initialize(vtkAnnotation* sself) { sself->Initialize(); }
extern "C" unsigned long vtk_annotation_get_m_time(vtkAnnotation* sself) { return sself->GetMTime(); }
extern "C" vtkAnnotationLayers * vtkAnnotationLayers_new () {return vtkAnnotationLayers :: New () ;}
extern "C" void vtkAnnotationLayers_destructor (vtkAnnotationLayers * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_annotation_layers_get_data_object_type(vtkAnnotationLayers* sself) { return sself->GetDataObjectType(); }
extern "C" unsigned int vtk_annotation_layers_get_number_of_annotations(vtkAnnotationLayers* sself) { return sself->GetNumberOfAnnotations(); }
extern "C" void vtk_annotation_layers_initialize(vtkAnnotationLayers* sself) { sself->Initialize(); }
extern "C" unsigned long vtk_annotation_layers_get_m_time(vtkAnnotationLayers* sself) { return sself->GetMTime(); }
extern "C" vtkArrayData * vtkArrayData_new () {return vtkArrayData :: New () ;}
extern "C" void vtkArrayData_destructor (vtkArrayData * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_array_data_clear_arrays(vtkArrayData* sself) { sself->ClearArrays(); }
extern "C" long long vtk_array_data_get_number_of_arrays(vtkArrayData* sself) { return sself->GetNumberOfArrays(); }
extern "C" int vtk_array_data_get_data_object_type(vtkArrayData* sself) { return sself->GetDataObjectType(); }
extern "C" vtkAttributesErrorMetric * vtkAttributesErrorMetric_new () {return vtkAttributesErrorMetric :: New () ;}
extern "C" void vtkAttributesErrorMetric_destructor (vtkAttributesErrorMetric * sself) {sself -> Delete () ; return ;}
extern "C" double vtk_attributes_error_metric_get_absolute_attribute_tolerance(vtkAttributesErrorMetric* sself) { return sself->GetAbsoluteAttributeTolerance(); }
extern "C" void vtk_attributes_error_metric_set_absolute_attribute_tolerance(vtkAttributesErrorMetric* sself, double value) { sself->SetAbsoluteAttributeTolerance(value); }
extern "C" double vtk_attributes_error_metric_get_attribute_tolerance(vtkAttributesErrorMetric* sself) { return sself->GetAttributeTolerance(); }
extern "C" void vtk_attributes_error_metric_set_attribute_tolerance(vtkAttributesErrorMetric* sself, double value) { sself->SetAttributeTolerance(value); }
extern "C" vtkBSPCuts * vtkBSPCuts_new () {return vtkBSPCuts :: New () ;}
extern "C" void vtkBSPCuts_destructor (vtkBSPCuts * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_bsp_cuts_get_data_object_type(vtkBSPCuts* sself) { return sself->GetDataObjectType(); }
extern "C" int vtk_bsp_cuts_get_number_of_cuts(vtkBSPCuts* sself) { return sself->GetNumberOfCuts(); }
extern "C" void vtk_bsp_cuts_print_tree(vtkBSPCuts* sself) { sself->PrintTree(); }
extern "C" void vtk_bsp_cuts_print_arrays(vtkBSPCuts* sself) { sself->PrintArrays(); }
extern "C" vtkBSPIntersections * vtkBSPIntersections_new () {return vtkBSPIntersections :: New () ;}
extern "C" void vtkBSPIntersections_destructor (vtkBSPIntersections * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_bsp_intersections_get_number_of_regions(vtkBSPIntersections* sself) { return sself->GetNumberOfRegions(); }
extern "C" int vtk_bsp_intersections_intersects_sphere_2(vtkBSPIntersections* sself, int regionId, double x, double y, double z, double rSquared) { return sself->IntersectsSphere2(regionId, x, y, z, rSquared); }
extern "C" int vtk_bsp_intersections_get_compute_intersections_using_data_bounds(vtkBSPIntersections* sself) { return sself->GetComputeIntersectionsUsingDataBounds(); }
extern "C" void vtk_bsp_intersections_set_compute_intersections_using_data_bounds(vtkBSPIntersections* sself, int c) { sself->SetComputeIntersectionsUsingDataBounds(c); }
extern "C" void vtk_bsp_intersections_compute_intersections_using_data_bounds_on(vtkBSPIntersections* sself) { sself->ComputeIntersectionsUsingDataBoundsOn(); }
extern "C" void vtk_bsp_intersections_compute_intersections_using_data_bounds_off(vtkBSPIntersections* sself) { sself->ComputeIntersectionsUsingDataBoundsOff(); }
extern "C" vtkBezierCurve * vtkBezierCurve_new () {return vtkBezierCurve :: New () ;}
extern "C" void vtkBezierCurve_destructor (vtkBezierCurve * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_bezier_curve_get_cell_type(vtkBezierCurve* sself) { return sself->GetCellType(); }
extern "C" vtkBezierHexahedron * vtkBezierHexahedron_new () {return vtkBezierHexahedron :: New () ;}
extern "C" void vtkBezierHexahedron_destructor (vtkBezierHexahedron * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_bezier_hexahedron_get_cell_type(vtkBezierHexahedron* sself) { return sself->GetCellType(); }
extern "C" vtkBezierInterpolation * vtkBezierInterpolation_new () {return vtkBezierInterpolation :: New () ;}
extern "C" void vtkBezierInterpolation_destructor (vtkBezierInterpolation * sself) {sself -> Delete () ; return ;}
extern "C" vtkBezierQuadrilateral * vtkBezierQuadrilateral_new () {return vtkBezierQuadrilateral :: New () ;}
extern "C" void vtkBezierQuadrilateral_destructor (vtkBezierQuadrilateral * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_bezier_quadrilateral_get_cell_type(vtkBezierQuadrilateral* sself) { return sself->GetCellType(); }
extern "C" vtkBezierTetra * vtkBezierTetra_new () {return vtkBezierTetra :: New () ;}
extern "C" void vtkBezierTetra_destructor (vtkBezierTetra * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_bezier_tetra_get_cell_type(vtkBezierTetra* sself) { return sself->GetCellType(); }
extern "C" vtkBezierTriangle * vtkBezierTriangle_new () {return vtkBezierTriangle :: New () ;}
extern "C" void vtkBezierTriangle_destructor (vtkBezierTriangle * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_bezier_triangle_get_cell_type(vtkBezierTriangle* sself) { return sself->GetCellType(); }
extern "C" vtkBezierWedge * vtkBezierWedge_new () {return vtkBezierWedge :: New () ;}
extern "C" void vtkBezierWedge_destructor (vtkBezierWedge * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_bezier_wedge_get_cell_type(vtkBezierWedge* sself) { return sself->GetCellType(); }
extern "C" vtkBiQuadraticQuad * vtkBiQuadraticQuad_new () {return vtkBiQuadraticQuad :: New () ;}
extern "C" void vtkBiQuadraticQuad_destructor (vtkBiQuadraticQuad * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_bi_quadratic_quad_get_cell_type(vtkBiQuadraticQuad* sself) { return sself->GetCellType(); }
extern "C" int vtk_bi_quadratic_quad_get_cell_dimension(vtkBiQuadraticQuad* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_bi_quadratic_quad_get_number_of_edges(vtkBiQuadraticQuad* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_bi_quadratic_quad_get_number_of_faces(vtkBiQuadraticQuad* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkBiQuadraticQuadraticHexahedron * vtkBiQuadraticQuadraticHexahedron_new () {return vtkBiQuadraticQuadraticHexahedron :: New () ;}
extern "C" void vtkBiQuadraticQuadraticHexahedron_destructor (vtkBiQuadraticQuadraticHexahedron * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_bi_quadratic_quadratic_hexahedron_get_cell_type(vtkBiQuadraticQuadraticHexahedron* sself) { return sself->GetCellType(); }
extern "C" int vtk_bi_quadratic_quadratic_hexahedron_get_cell_dimension(vtkBiQuadraticQuadraticHexahedron* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_bi_quadratic_quadratic_hexahedron_get_number_of_edges(vtkBiQuadraticQuadraticHexahedron* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_bi_quadratic_quadratic_hexahedron_get_number_of_faces(vtkBiQuadraticQuadraticHexahedron* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkBiQuadraticQuadraticWedge * vtkBiQuadraticQuadraticWedge_new () {return vtkBiQuadraticQuadraticWedge :: New () ;}
extern "C" void vtkBiQuadraticQuadraticWedge_destructor (vtkBiQuadraticQuadraticWedge * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_bi_quadratic_quadratic_wedge_get_cell_type(vtkBiQuadraticQuadraticWedge* sself) { return sself->GetCellType(); }
extern "C" int vtk_bi_quadratic_quadratic_wedge_get_cell_dimension(vtkBiQuadraticQuadraticWedge* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_bi_quadratic_quadratic_wedge_get_number_of_edges(vtkBiQuadraticQuadraticWedge* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_bi_quadratic_quadratic_wedge_get_number_of_faces(vtkBiQuadraticQuadraticWedge* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkBiQuadraticTriangle * vtkBiQuadraticTriangle_new () {return vtkBiQuadraticTriangle :: New () ;}
extern "C" void vtkBiQuadraticTriangle_destructor (vtkBiQuadraticTriangle * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_bi_quadratic_triangle_get_cell_type(vtkBiQuadraticTriangle* sself) { return sself->GetCellType(); }
extern "C" int vtk_bi_quadratic_triangle_get_cell_dimension(vtkBiQuadraticTriangle* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_bi_quadratic_triangle_get_number_of_edges(vtkBiQuadraticTriangle* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_bi_quadratic_triangle_get_number_of_faces(vtkBiQuadraticTriangle* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkBox * vtkBox_new () {return vtkBox :: New () ;}
extern "C" void vtkBox_destructor (vtkBox * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_box_set_x_min(vtkBox* sself, double x, double y, double z) { sself->SetXMin(x, y, z); }
extern "C" void vtk_box_get_x_min(vtkBox* sself, double& x, double& y, double& z) { sself->GetXMin(x, y, z); }
extern "C" void vtk_box_set_x_max(vtkBox* sself, double x, double y, double z) { sself->SetXMax(x, y, z); }
extern "C" void vtk_box_get_x_max(vtkBox* sself, double& x, double& y, double& z) { sself->GetXMax(x, y, z); }
extern "C" void vtk_box_set_bounds(vtkBox* sself, double xMin, double xMax, double yMin, double yMax, double zMin, double zMax) { sself->SetBounds(xMin, xMax, yMin, yMax, zMin, zMax); }
extern "C" void vtk_box_get_bounds(vtkBox* sself, double& xMin, double& xMax, double& yMin, double& yMax, double& zMin, double& zMax) { sself->GetBounds(xMin, xMax, yMin, yMax, zMin, zMax); }
extern "C" vtkCellArray * vtkCellArray_new () {return vtkCellArray :: New () ;}
extern "C" void vtkCellArray_destructor (vtkCellArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_cell_array_allocate(vtkCellArray* sself, long long sz, long long ext) { return sself->Allocate(sz, ext); }
extern "C" bool vtk_cell_array_allocate_estimate(vtkCellArray* sself, long long numCells, long long maxCellSize) { return sself->AllocateEstimate(numCells, maxCellSize); }
extern "C" bool vtk_cell_array_allocate_exact(vtkCellArray* sself, long long numCells, long long connectivitySize) { return sself->AllocateExact(numCells, connectivitySize); }
extern "C" bool vtk_cell_array_resize_exact(vtkCellArray* sself, long long numCells, long long connectivitySize) { return sself->ResizeExact(numCells, connectivitySize); }
extern "C" void vtk_cell_array_initialize(vtkCellArray* sself) { sself->Initialize(); }
extern "C" void vtk_cell_array_reset(vtkCellArray* sself) { sself->Reset(); }
extern "C" void vtk_cell_array_squeeze(vtkCellArray* sself) { sself->Squeeze(); }
extern "C" bool vtk_cell_array_is_valid(vtkCellArray* sself) { return sself->IsValid(); }
extern "C" long long vtk_cell_array_get_number_of_cells(vtkCellArray* sself) { return sself->GetNumberOfCells(); }
extern "C" long long vtk_cell_array_get_number_of_offsets(vtkCellArray* sself) { return sself->GetNumberOfOffsets(); }
extern "C" long long vtk_cell_array_get_number_of_connectivity_ids(vtkCellArray* sself) { return sself->GetNumberOfConnectivityIds(); }
extern "C" bool vtk_cell_array_is_storage_64_bit(vtkCellArray* sself) { return sself->IsStorage64Bit(); }
extern "C" bool vtk_cell_array_is_storage_shareable(vtkCellArray* sself) { return sself->IsStorageShareable(); }
extern "C" void vtk_cell_array_use_32_bit_storage(vtkCellArray* sself) { sself->Use32BitStorage(); }
extern "C" void vtk_cell_array_use_64_bit_storage(vtkCellArray* sself) { sself->Use64BitStorage(); }
extern "C" void vtk_cell_array_use_default_storage(vtkCellArray* sself) { sself->UseDefaultStorage(); }
extern "C" bool vtk_cell_array_can_convert_to_32_bit_storage(vtkCellArray* sself) { return sself->CanConvertTo32BitStorage(); }
extern "C" bool vtk_cell_array_can_convert_to_64_bit_storage(vtkCellArray* sself) { return sself->CanConvertTo64BitStorage(); }
extern "C" bool vtk_cell_array_can_convert_to_default_storage(vtkCellArray* sself) { return sself->CanConvertToDefaultStorage(); }
extern "C" bool vtk_cell_array_convert_to_32_bit_storage(vtkCellArray* sself) { return sself->ConvertTo32BitStorage(); }
extern "C" bool vtk_cell_array_convert_to_64_bit_storage(vtkCellArray* sself) { return sself->ConvertTo64BitStorage(); }
extern "C" bool vtk_cell_array_convert_to_default_storage(vtkCellArray* sself) { return sself->ConvertToDefaultStorage(); }
extern "C" bool vtk_cell_array_convert_to_smallest_storage(vtkCellArray* sself) { return sself->ConvertToSmallestStorage(); }
extern "C" long long vtk_cell_array_is_homogeneous(vtkCellArray* sself) { return sself->IsHomogeneous(); }
extern "C" void vtk_cell_array_init_traversal(vtkCellArray* sself) { sself->InitTraversal(); }
extern "C" long long vtk_cell_array_get_cell_size(vtkCellArray* sself, const long long cellId) { return sself->GetCellSize(cellId); }
extern "C" void vtk_cell_array_insert_cell_point(vtkCellArray* sself, long long id) { sself->InsertCellPoint(id); }
extern "C" void vtk_cell_array_update_cell_count(vtkCellArray* sself, int npts) { sself->UpdateCellCount(npts); }
extern "C" long long vtk_cell_array_get_traversal_cell_id(vtkCellArray* sself) { return sself->GetTraversalCellId(); }
extern "C" void vtk_cell_array_set_traversal_cell_id(vtkCellArray* sself, long long cellId) { sself->SetTraversalCellId(cellId); }
extern "C" void vtk_cell_array_reverse_cell_at_id(vtkCellArray* sself, long long cellId) { sself->ReverseCellAtId(cellId); }
extern "C" int vtk_cell_array_get_max_cell_size(vtkCellArray* sself) { return sself->GetMaxCellSize(); }
extern "C" unsigned long vtk_cell_array_get_actual_memory_size(vtkCellArray* sself) { return sself->GetActualMemorySize(); }
extern "C" void vtk_cell_array_set_number_of_cells(vtkCellArray* sself, long long p0) { sself->SetNumberOfCells(p0); }
extern "C" long long vtk_cell_array_estimate_size(vtkCellArray* sself, long long numCells, int maxPtsPerCell) { return sself->EstimateSize(numCells, maxPtsPerCell); }
extern "C" long long vtk_cell_array_get_size(vtkCellArray* sself) { return sself->GetSize(); }
extern "C" long long vtk_cell_array_get_number_of_connectivity_entries(vtkCellArray* sself) { return sself->GetNumberOfConnectivityEntries(); }
extern "C" long long vtk_cell_array_get_insert_location(vtkCellArray* sself, int npts) { return sself->GetInsertLocation(npts); }
extern "C" long long vtk_cell_array_get_traversal_location(vtkCellArray* sself) { return sself->GetTraversalLocation(); }
extern "C" void vtk_cell_array_set_traversal_location(vtkCellArray* sself, long long loc) { sself->SetTraversalLocation(loc); }
extern "C" void vtk_cell_array_reverse_cell(vtkCellArray* sself, long long loc) { sself->ReverseCell(loc); }
extern "C" vtkCellArrayIterator * vtkCellArrayIterator_new () {return vtkCellArrayIterator :: New () ;}
extern "C" void vtkCellArrayIterator_destructor (vtkCellArrayIterator * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_cell_array_iterator_go_to_cell(vtkCellArrayIterator* sself, long long cellId) { sself->GoToCell(cellId); }
extern "C" void vtk_cell_array_iterator_go_to_first_cell(vtkCellArrayIterator* sself) { sself->GoToFirstCell(); }
extern "C" void vtk_cell_array_iterator_go_to_next_cell(vtkCellArrayIterator* sself) { sself->GoToNextCell(); }
extern "C" bool vtk_cell_array_iterator_is_done_with_traversal(vtkCellArrayIterator* sself) { return sself->IsDoneWithTraversal(); }
extern "C" long long vtk_cell_array_iterator_get_current_cell_id(vtkCellArrayIterator* sself) { return sself->GetCurrentCellId(); }
extern "C" void vtk_cell_array_iterator_reverse_current_cell(vtkCellArrayIterator* sself) { sself->ReverseCurrentCell(); }
extern "C" vtkCellData * vtkCellData_new () {return vtkCellData :: New () ;}
extern "C" void vtkCellData_destructor (vtkCellData * sself) {sself -> Delete () ; return ;}
extern "C" vtkCellLinks * vtkCellLinks_new () {return vtkCellLinks :: New () ;}
extern "C" void vtkCellLinks_destructor (vtkCellLinks * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_cell_links_allocate(vtkCellLinks* sself, long long numLinks, long long ext) { sself->Allocate(numLinks, ext); }
extern "C" void vtk_cell_links_initialize(vtkCellLinks* sself) { sself->Initialize(); }
extern "C" long long vtk_cell_links_get_ncells(vtkCellLinks* sself, long long ptId) { return sself->GetNcells(ptId); }
extern "C" long long vtk_cell_links_insert_next_point(vtkCellLinks* sself, int numLinks) { return sself->InsertNextPoint(numLinks); }
extern "C" void vtk_cell_links_insert_next_cell_reference(vtkCellLinks* sself, long long ptId, long long cellId) { sself->InsertNextCellReference(ptId, cellId); }
extern "C" void vtk_cell_links_delete_point(vtkCellLinks* sself, long long ptId) { sself->DeletePoint(ptId); }
extern "C" void vtk_cell_links_remove_cell_reference(vtkCellLinks* sself, long long cellId, long long ptId) { sself->RemoveCellReference(cellId, ptId); }
extern "C" void vtk_cell_links_add_cell_reference(vtkCellLinks* sself, long long cellId, long long ptId) { sself->AddCellReference(cellId, ptId); }
extern "C" void vtk_cell_links_resize_cell_list(vtkCellLinks* sself, long long ptId, int size) { sself->ResizeCellList(ptId, size); }
extern "C" void vtk_cell_links_squeeze(vtkCellLinks* sself) { sself->Squeeze(); }
extern "C" void vtk_cell_links_reset(vtkCellLinks* sself) { sself->Reset(); }
extern "C" unsigned long vtk_cell_links_get_actual_memory_size(vtkCellLinks* sself) { return sself->GetActualMemorySize(); }
extern "C" vtkCellLocator * vtkCellLocator_new () {return vtkCellLocator :: New () ;}
extern "C" void vtkCellLocator_destructor (vtkCellLocator * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_cell_locator_set_number_of_cells_per_bucket(vtkCellLocator* sself, int N) { sself->SetNumberOfCellsPerBucket(N); }
extern "C" int vtk_cell_locator_get_number_of_cells_per_bucket(vtkCellLocator* sself) { return sself->GetNumberOfCellsPerBucket(); }
extern "C" int vtk_cell_locator_get_number_of_buckets(vtkCellLocator* sself) { return sself->GetNumberOfBuckets(); }
extern "C" void vtk_cell_locator_free_search_structure(vtkCellLocator* sself) { sself->FreeSearchStructure(); }
extern "C" void vtk_cell_locator_build_locator(vtkCellLocator* sself) { sself->BuildLocator(); }
extern "C" void vtk_cell_locator_build_locator_if_needed(vtkCellLocator* sself) { sself->BuildLocatorIfNeeded(); }
extern "C" void vtk_cell_locator_force_build_locator(vtkCellLocator* sself) { sself->ForceBuildLocator(); }
extern "C" void vtk_cell_locator_build_locator_internal(vtkCellLocator* sself) { sself->BuildLocatorInternal(); }
extern "C" vtkCellLocatorStrategy * vtkCellLocatorStrategy_new () {return vtkCellLocatorStrategy :: New () ;}
extern "C" void vtkCellLocatorStrategy_destructor (vtkCellLocatorStrategy * sself) {sself -> Delete () ; return ;}
extern "C" vtkCellTypes * vtkCellTypes_new () {return vtkCellTypes :: New () ;}
extern "C" void vtkCellTypes_destructor (vtkCellTypes * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_cell_types_allocate(vtkCellTypes* sself, long long sz, long long ext) { return sself->Allocate(sz, ext); }
extern "C" void vtk_cell_types_insert_cell(vtkCellTypes* sself, long long id, unsigned char type, long long loc) { sself->InsertCell(id, type, loc); }
extern "C" long long vtk_cell_types_insert_next_cell(vtkCellTypes* sself, unsigned char type, long long loc) { return sself->InsertNextCell(type, loc); }
extern "C" long long vtk_cell_types_get_cell_location(vtkCellTypes* sself, long long cellId) { return sself->GetCellLocation(cellId); }
extern "C" void vtk_cell_types_delete_cell(vtkCellTypes* sself, long long cellId) { sself->DeleteCell(cellId); }
extern "C" long long vtk_cell_types_get_number_of_types(vtkCellTypes* sself) { return sself->GetNumberOfTypes(); }
extern "C" int vtk_cell_types_is_type(vtkCellTypes* sself, unsigned char type) { return sself->IsType(type); }
extern "C" long long vtk_cell_types_insert_next_type(vtkCellTypes* sself, unsigned char type) { return sself->InsertNextType(type); }
extern "C" unsigned char vtk_cell_types_get_cell_type(vtkCellTypes* sself, long long cellId) { return sself->GetCellType(cellId); }
extern "C" void vtk_cell_types_squeeze(vtkCellTypes* sself) { sself->Squeeze(); }
extern "C" void vtk_cell_types_reset(vtkCellTypes* sself) { sself->Reset(); }
extern "C" unsigned long vtk_cell_types_get_actual_memory_size(vtkCellTypes* sself) { return sself->GetActualMemorySize(); }
extern "C" const char* vtk_cell_types_get_class_name_from_type_id(vtkCellTypes* sself, int typeId) { return sself->GetClassNameFromTypeId(typeId); }
extern "C" int vtk_cell_types_get_type_id_from_class_name(vtkCellTypes* sself, const char* classname) { return sself->GetTypeIdFromClassName(classname); }
extern "C" int vtk_cell_types_is_linear(vtkCellTypes* sself, unsigned char type) { return sself->IsLinear(type); }
extern "C" vtkClosestNPointsStrategy * vtkClosestNPointsStrategy_new () {return vtkClosestNPointsStrategy :: New () ;}
extern "C" void vtkClosestNPointsStrategy_destructor (vtkClosestNPointsStrategy * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_closest_n_points_strategy_set_closest_n_points(vtkClosestNPointsStrategy* sself, int _arg) { sself->SetClosestNPoints(_arg); }
extern "C" int vtk_closest_n_points_strategy_get_closest_n_points_min_value(vtkClosestNPointsStrategy* sself) { return sself->GetClosestNPointsMinValue(); }
extern "C" int vtk_closest_n_points_strategy_get_closest_n_points_max_value(vtkClosestNPointsStrategy* sself) { return sself->GetClosestNPointsMaxValue(); }
extern "C" int vtk_closest_n_points_strategy_get_closest_n_points(vtkClosestNPointsStrategy* sself) { return sself->GetClosestNPoints(); }
extern "C" vtkClosestPointStrategy * vtkClosestPointStrategy_new () {return vtkClosestPointStrategy :: New () ;}
extern "C" void vtkClosestPointStrategy_destructor (vtkClosestPointStrategy * sself) {sself -> Delete () ; return ;}
extern "C" vtkCone * vtkCone_new () {return vtkCone :: New () ;}
extern "C" void vtkCone_destructor (vtkCone * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_cone_set_angle(vtkCone* sself, double _arg) { sself->SetAngle(_arg); }
extern "C" double vtk_cone_get_angle_min_value(vtkCone* sself) { return sself->GetAngleMinValue(); }
extern "C" double vtk_cone_get_angle_max_value(vtkCone* sself) { return sself->GetAngleMaxValue(); }
extern "C" double vtk_cone_get_angle(vtkCone* sself) { return sself->GetAngle(); }
extern "C" vtkConvexPointSet * vtkConvexPointSet_new () {return vtkConvexPointSet :: New () ;}
extern "C" void vtkConvexPointSet_destructor (vtkConvexPointSet * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_convex_point_set_has_fixed_topology(vtkConvexPointSet* sself) { return sself->HasFixedTopology(); }
extern "C" int vtk_convex_point_set_get_cell_type(vtkConvexPointSet* sself) { return sself->GetCellType(); }
extern "C" int vtk_convex_point_set_requires_initialization(vtkConvexPointSet* sself) { return sself->RequiresInitialization(); }
extern "C" int vtk_convex_point_set_get_number_of_edges(vtkConvexPointSet* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_convex_point_set_get_number_of_faces(vtkConvexPointSet* sself) { return sself->GetNumberOfFaces(); }
extern "C" int vtk_convex_point_set_is_primary_cell(vtkConvexPointSet* sself) { return sself->IsPrimaryCell(); }
extern "C" vtkCubicLine * vtkCubicLine_new () {return vtkCubicLine :: New () ;}
extern "C" void vtkCubicLine_destructor (vtkCubicLine * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_cubic_line_get_cell_type(vtkCubicLine* sself) { return sself->GetCellType(); }
extern "C" int vtk_cubic_line_get_cell_dimension(vtkCubicLine* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_cubic_line_get_number_of_edges(vtkCubicLine* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_cubic_line_get_number_of_faces(vtkCubicLine* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkCylinder * vtkCylinder_new () {return vtkCylinder :: New () ;}
extern "C" void vtkCylinder_destructor (vtkCylinder * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_cylinder_set_radius(vtkCylinder* sself, double _arg) { sself->SetRadius(_arg); }
extern "C" double vtk_cylinder_get_radius(vtkCylinder* sself) { return sself->GetRadius(); }
extern "C" void vtk_cylinder_set_center(vtkCylinder* sself, double _arg1, double _arg2, double _arg3) { sself->SetCenter(_arg1, _arg2, _arg3); }
extern "C" void vtk_cylinder_set_axis(vtkCylinder* sself, double ax, double ay, double az) { sself->SetAxis(ax, ay, az); }
extern "C" vtkDataAssembly * vtkDataAssembly_new () {return vtkDataAssembly :: New () ;}
extern "C" void vtkDataAssembly_destructor (vtkDataAssembly * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_data_assembly_initialize(vtkDataAssembly* sself) { sself->Initialize(); }
extern "C" bool vtk_data_assembly_initialize_from_xml(vtkDataAssembly* sself, const char* xmlcontents) { return sself->InitializeFromXML(xmlcontents); }
extern "C" int vtk_data_assembly_get_root_node(vtkDataAssembly* sself) { return sself->GetRootNode(); }
extern "C" void vtk_data_assembly_set_root_node_name(vtkDataAssembly* sself, const char* name) { sself->SetRootNodeName(name); }
extern "C" const char* vtk_data_assembly_get_root_node_name(vtkDataAssembly* sself) { return sself->GetRootNodeName(); }
extern "C" int vtk_data_assembly_add_node(vtkDataAssembly* sself, const char* name, int parent) { return sself->AddNode(name, parent); }
extern "C" bool vtk_data_assembly_remove_node(vtkDataAssembly* sself, int id) { return sself->RemoveNode(id); }
extern "C" void vtk_data_assembly_set_node_name(vtkDataAssembly* sself, int id, const char* name) { sself->SetNodeName(id, name); }
extern "C" const char* vtk_data_assembly_get_node_name(vtkDataAssembly* sself, int id) { return sself->GetNodeName(id); }
extern "C" int vtk_data_assembly_get_first_node_by_path(vtkDataAssembly* sself, const char* path) { return sself->GetFirstNodeByPath(path); }
extern "C" bool vtk_data_assembly_add_data_set_index(vtkDataAssembly* sself, int id, unsigned int dataset_index) { return sself->AddDataSetIndex(id, dataset_index); }
extern "C" bool vtk_data_assembly_add_data_set_index_range(vtkDataAssembly* sself, int id, unsigned int index_start, int count) { return sself->AddDataSetIndexRange(id, index_start, count); }
extern "C" bool vtk_data_assembly_remove_data_set_index(vtkDataAssembly* sself, int id, unsigned int dataset_index) { return sself->RemoveDataSetIndex(id, dataset_index); }
extern "C" bool vtk_data_assembly_remove_all_data_set_indices(vtkDataAssembly* sself, int id, bool traverse_subtree) { return sself->RemoveAllDataSetIndices(id, traverse_subtree); }
extern "C" int vtk_data_assembly_find_first_node_with_name(vtkDataAssembly* sself, const char* name, int traversal_order) { return sself->FindFirstNodeWithName(name, traversal_order); }
extern "C" int vtk_data_assembly_get_number_of_children(vtkDataAssembly* sself, int parent) { return sself->GetNumberOfChildren(parent); }
extern "C" int vtk_data_assembly_get_child(vtkDataAssembly* sself, int parent, int index) { return sself->GetChild(parent, index); }
extern "C" int vtk_data_assembly_get_child_index(vtkDataAssembly* sself, int parent, int child) { return sself->GetChildIndex(parent, child); }
extern "C" int vtk_data_assembly_get_parent(vtkDataAssembly* sself, int id) { return sself->GetParent(id); }
extern "C" bool vtk_data_assembly_has_attribute(vtkDataAssembly* sself, int id, const char* name) { return sself->HasAttribute(id, name); }
extern "C" void vtk_data_assembly_set_attribute(vtkDataAssembly* sself, int id, const char* name, const char* value) { sself->SetAttribute(id, name, value); }
extern "C" bool vtk_data_assembly_get_attribute(vtkDataAssembly* sself, int id, const char* name, const char* value) { return sself->GetAttribute(id, name, value); }
extern "C" const char* vtk_data_assembly_get_attribute_or_default(vtkDataAssembly* sself, int id, const char* name, const char* default_value) { return sself->GetAttributeOrDefault(id, name, default_value); }
extern "C" bool vtk_data_assembly_is_node_name_valid(vtkDataAssembly* sself, const char* name) { return sself->IsNodeNameValid(name); }
extern "C" bool vtk_data_assembly_is_node_name_reserved(vtkDataAssembly* sself, const char* name) { return sself->IsNodeNameReserved(name); }
extern "C" vtkDataAssemblyUtilities * vtkDataAssemblyUtilities_new () {return vtkDataAssemblyUtilities :: New () ;}
extern "C" void vtkDataAssemblyUtilities_destructor (vtkDataAssemblyUtilities * sself) {sself -> Delete () ; return ;}
extern "C" const char* vtk_data_assembly_utilities_hierarchy_name(vtkDataAssemblyUtilities* sself) { return sself->HierarchyName(); }
extern "C" vtkDataObject * vtkDataObject_new () {return vtkDataObject :: New () ;}
extern "C" void vtkDataObject_destructor (vtkDataObject * sself) {sself -> Delete () ; return ;}
extern "C" unsigned long vtk_data_object_get_m_time(vtkDataObject* sself) { return sself->GetMTime(); }
extern "C" void vtk_data_object_initialize(vtkDataObject* sself) { sself->Initialize(); }
extern "C" void vtk_data_object_release_data(vtkDataObject* sself) { sself->ReleaseData(); }
extern "C" int vtk_data_object_get_data_released(vtkDataObject* sself) { return sself->GetDataReleased(); }
extern "C" void vtk_data_object_set_global_release_data_flag(vtkDataObject* sself, int val) { sself->SetGlobalReleaseDataFlag(val); }
extern "C" void vtk_data_object_global_release_data_flag_on(vtkDataObject* sself) { sself->GlobalReleaseDataFlagOn(); }
extern "C" void vtk_data_object_global_release_data_flag_off(vtkDataObject* sself) { sself->GlobalReleaseDataFlagOff(); }
extern "C" int vtk_data_object_get_global_release_data_flag(vtkDataObject* sself) { return sself->GetGlobalReleaseDataFlag(); }
extern "C" int vtk_data_object_get_data_object_type(vtkDataObject* sself) { return sself->GetDataObjectType(); }
extern "C" unsigned long vtk_data_object_get_update_time(vtkDataObject* sself) { return sself->GetUpdateTime(); }
extern "C" unsigned long vtk_data_object_get_actual_memory_size(vtkDataObject* sself) { return sself->GetActualMemorySize(); }
extern "C" void vtk_data_object_data_has_been_generated(vtkDataObject* sself) { sself->DataHasBeenGenerated(); }
extern "C" void vtk_data_object_prepare_for_new_data(vtkDataObject* sself) { sself->PrepareForNewData(); }
extern "C" int vtk_data_object_get_extent_type(vtkDataObject* sself) { return sself->GetExtentType(); }
extern "C" long long vtk_data_object_get_number_of_elements(vtkDataObject* sself, int type) { return sself->GetNumberOfElements(type); }
extern "C" const char* vtk_data_object_get_association_type_as_string(vtkDataObject* sself, int associationType) { return sself->GetAssociationTypeAsString(associationType); }
extern "C" int vtk_data_object_get_association_type_from_string(vtkDataObject* sself, const char* associationName) { return sself->GetAssociationTypeFromString(associationName); }
extern "C" vtkDataObjectCollection * vtkDataObjectCollection_new () {return vtkDataObjectCollection :: New () ;}
extern "C" void vtkDataObjectCollection_destructor (vtkDataObjectCollection * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_data_object_collection_get_number_of_items(vtkDataObjectCollection* sself) { return sself->GetNumberOfItems(); }
extern "C" vtkDataObjectTreeIterator * vtkDataObjectTreeIterator_new () {return vtkDataObjectTreeIterator :: New () ;}
extern "C" void vtkDataObjectTreeIterator_destructor (vtkDataObjectTreeIterator * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_data_object_tree_iterator_go_to_first_item(vtkDataObjectTreeIterator* sself) { sself->GoToFirstItem(); }
extern "C" void vtk_data_object_tree_iterator_go_to_next_item(vtkDataObjectTreeIterator* sself) { sself->GoToNextItem(); }
extern "C" int vtk_data_object_tree_iterator_is_done_with_traversal(vtkDataObjectTreeIterator* sself) { return sself->IsDoneWithTraversal(); }
extern "C" int vtk_data_object_tree_iterator_has_current_meta_data(vtkDataObjectTreeIterator* sself) { return sself->HasCurrentMetaData(); }
extern "C" unsigned int vtk_data_object_tree_iterator_get_current_flat_index(vtkDataObjectTreeIterator* sself) { return sself->GetCurrentFlatIndex(); }
extern "C" void vtk_data_object_tree_iterator_set_visit_only_leaves(vtkDataObjectTreeIterator* sself, int _arg) { sself->SetVisitOnlyLeaves(_arg); }
extern "C" int vtk_data_object_tree_iterator_get_visit_only_leaves(vtkDataObjectTreeIterator* sself) { return sself->GetVisitOnlyLeaves(); }
extern "C" void vtk_data_object_tree_iterator_visit_only_leaves_on(vtkDataObjectTreeIterator* sself) { sself->VisitOnlyLeavesOn(); }
extern "C" void vtk_data_object_tree_iterator_visit_only_leaves_off(vtkDataObjectTreeIterator* sself) { sself->VisitOnlyLeavesOff(); }
extern "C" void vtk_data_object_tree_iterator_set_traverse_sub_tree(vtkDataObjectTreeIterator* sself, int _arg) { sself->SetTraverseSubTree(_arg); }
extern "C" int vtk_data_object_tree_iterator_get_traverse_sub_tree(vtkDataObjectTreeIterator* sself) { return sself->GetTraverseSubTree(); }
extern "C" void vtk_data_object_tree_iterator_traverse_sub_tree_on(vtkDataObjectTreeIterator* sself) { sself->TraverseSubTreeOn(); }
extern "C" void vtk_data_object_tree_iterator_traverse_sub_tree_off(vtkDataObjectTreeIterator* sself) { sself->TraverseSubTreeOff(); }
extern "C" vtkDataObjectTypes * vtkDataObjectTypes_new () {return vtkDataObjectTypes :: New () ;}
extern "C" void vtkDataObjectTypes_destructor (vtkDataObjectTypes * sself) {sself -> Delete () ; return ;}
extern "C" const char* vtk_data_object_types_get_class_name_from_type_id(vtkDataObjectTypes* sself, int typeId) { return sself->GetClassNameFromTypeId(typeId); }
extern "C" int vtk_data_object_types_get_type_id_from_class_name(vtkDataObjectTypes* sself, const char* classname) { return sself->GetTypeIdFromClassName(classname); }
extern "C" bool vtk_data_object_types_type_id_is_a(vtkDataObjectTypes* sself, int typeId, int targetTypeId) { return sself->TypeIdIsA(typeId, targetTypeId); }
extern "C" int vtk_data_object_types_get_common_base_type_id(vtkDataObjectTypes* sself, int typeA, int typeB) { return sself->GetCommonBaseTypeId(typeA, typeB); }
extern "C" vtkDataSetAttributes * vtkDataSetAttributes_new () {return vtkDataSetAttributes :: New () ;}
extern "C" void vtkDataSetAttributes_destructor (vtkDataSetAttributes * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_data_set_attributes_initialize(vtkDataSetAttributes* sself) { sself->Initialize(); }
extern "C" void vtk_data_set_attributes_update(vtkDataSetAttributes* sself) { sself->Update(); }
extern "C" const char* vtk_data_set_attributes_ghost_array_name(vtkDataSetAttributes* sself) { return sself->GhostArrayName(); }
extern "C" int vtk_data_set_attributes_set_active_scalars(vtkDataSetAttributes* sself, const char* name) { return sself->SetActiveScalars(name); }
extern "C" int vtk_data_set_attributes_set_active_vectors(vtkDataSetAttributes* sself, const char* name) { return sself->SetActiveVectors(name); }
extern "C" int vtk_data_set_attributes_set_active_normals(vtkDataSetAttributes* sself, const char* name) { return sself->SetActiveNormals(name); }
extern "C" int vtk_data_set_attributes_set_active_tangents(vtkDataSetAttributes* sself, const char* name) { return sself->SetActiveTangents(name); }
extern "C" int vtk_data_set_attributes_set_active_t_coords(vtkDataSetAttributes* sself, const char* name) { return sself->SetActiveTCoords(name); }
extern "C" int vtk_data_set_attributes_set_active_tensors(vtkDataSetAttributes* sself, const char* name) { return sself->SetActiveTensors(name); }
extern "C" int vtk_data_set_attributes_set_active_global_ids(vtkDataSetAttributes* sself, const char* name) { return sself->SetActiveGlobalIds(name); }
extern "C" int vtk_data_set_attributes_set_active_pedigree_ids(vtkDataSetAttributes* sself, const char* name) { return sself->SetActivePedigreeIds(name); }
extern "C" int vtk_data_set_attributes_set_active_rational_weights(vtkDataSetAttributes* sself, const char* name) { return sself->SetActiveRationalWeights(name); }
extern "C" int vtk_data_set_attributes_set_active_higher_order_degrees(vtkDataSetAttributes* sself, const char* name) { return sself->SetActiveHigherOrderDegrees(name); }
extern "C" int vtk_data_set_attributes_set_active_attribute(vtkDataSetAttributes* sself, const char* name, int attributeType) { return sself->SetActiveAttribute(name, attributeType); }
extern "C" int vtk_data_set_attributes_is_array_an_attribute(vtkDataSetAttributes* sself, int idx) { return sself->IsArrayAnAttribute(idx); }
extern "C" const char* vtk_data_set_attributes_get_attribute_type_as_string(vtkDataSetAttributes* sself, int attributeType) { return sself->GetAttributeTypeAsString(attributeType); }
extern "C" const char* vtk_data_set_attributes_get_long_attribute_type_as_string(vtkDataSetAttributes* sself, int attributeType) { return sself->GetLongAttributeTypeAsString(attributeType); }
extern "C" void vtk_data_set_attributes_set_copy_attribute(vtkDataSetAttributes* sself, int index, int value, int ctype) { sself->SetCopyAttribute(index, value, ctype); }
extern "C" int vtk_data_set_attributes_get_copy_attribute(vtkDataSetAttributes* sself, int index, int ctype) { return sself->GetCopyAttribute(index, ctype); }
extern "C" void vtk_data_set_attributes_set_copy_scalars(vtkDataSetAttributes* sself, int i, int ctype) { sself->SetCopyScalars(i, ctype); }
extern "C" int vtk_data_set_attributes_get_copy_scalars(vtkDataSetAttributes* sself, int ctype) { return sself->GetCopyScalars(ctype); }
extern "C" void vtk_data_set_attributes_copy_scalars_on(vtkDataSetAttributes* sself) { sself->CopyScalarsOn(); }
extern "C" void vtk_data_set_attributes_copy_scalars_off(vtkDataSetAttributes* sself) { sself->CopyScalarsOff(); }
extern "C" void vtk_data_set_attributes_set_copy_vectors(vtkDataSetAttributes* sself, int i, int ctype) { sself->SetCopyVectors(i, ctype); }
extern "C" int vtk_data_set_attributes_get_copy_vectors(vtkDataSetAttributes* sself, int ctype) { return sself->GetCopyVectors(ctype); }
extern "C" void vtk_data_set_attributes_copy_vectors_on(vtkDataSetAttributes* sself) { sself->CopyVectorsOn(); }
extern "C" void vtk_data_set_attributes_copy_vectors_off(vtkDataSetAttributes* sself) { sself->CopyVectorsOff(); }
extern "C" void vtk_data_set_attributes_set_copy_normals(vtkDataSetAttributes* sself, int i, int ctype) { sself->SetCopyNormals(i, ctype); }
extern "C" int vtk_data_set_attributes_get_copy_normals(vtkDataSetAttributes* sself, int ctype) { return sself->GetCopyNormals(ctype); }
extern "C" void vtk_data_set_attributes_copy_normals_on(vtkDataSetAttributes* sself) { sself->CopyNormalsOn(); }
extern "C" void vtk_data_set_attributes_copy_normals_off(vtkDataSetAttributes* sself) { sself->CopyNormalsOff(); }
extern "C" void vtk_data_set_attributes_set_copy_tangents(vtkDataSetAttributes* sself, int i, int ctype) { sself->SetCopyTangents(i, ctype); }
extern "C" int vtk_data_set_attributes_get_copy_tangents(vtkDataSetAttributes* sself, int ctype) { return sself->GetCopyTangents(ctype); }
extern "C" void vtk_data_set_attributes_copy_tangents_on(vtkDataSetAttributes* sself) { sself->CopyTangentsOn(); }
extern "C" void vtk_data_set_attributes_copy_tangents_off(vtkDataSetAttributes* sself) { sself->CopyTangentsOff(); }
extern "C" void vtk_data_set_attributes_set_copy_t_coords(vtkDataSetAttributes* sself, int i, int ctype) { sself->SetCopyTCoords(i, ctype); }
extern "C" int vtk_data_set_attributes_get_copy_t_coords(vtkDataSetAttributes* sself, int ctype) { return sself->GetCopyTCoords(ctype); }
extern "C" void vtk_data_set_attributes_copy_t_coords_on(vtkDataSetAttributes* sself) { sself->CopyTCoordsOn(); }
extern "C" void vtk_data_set_attributes_copy_t_coords_off(vtkDataSetAttributes* sself) { sself->CopyTCoordsOff(); }
extern "C" void vtk_data_set_attributes_set_copy_tensors(vtkDataSetAttributes* sself, int i, int ctype) { sself->SetCopyTensors(i, ctype); }
extern "C" int vtk_data_set_attributes_get_copy_tensors(vtkDataSetAttributes* sself, int ctype) { return sself->GetCopyTensors(ctype); }
extern "C" void vtk_data_set_attributes_copy_tensors_on(vtkDataSetAttributes* sself) { sself->CopyTensorsOn(); }
extern "C" void vtk_data_set_attributes_copy_tensors_off(vtkDataSetAttributes* sself) { sself->CopyTensorsOff(); }
extern "C" void vtk_data_set_attributes_set_copy_global_ids(vtkDataSetAttributes* sself, int i, int ctype) { sself->SetCopyGlobalIds(i, ctype); }
extern "C" int vtk_data_set_attributes_get_copy_global_ids(vtkDataSetAttributes* sself, int ctype) { return sself->GetCopyGlobalIds(ctype); }
extern "C" void vtk_data_set_attributes_copy_global_ids_on(vtkDataSetAttributes* sself) { sself->CopyGlobalIdsOn(); }
extern "C" void vtk_data_set_attributes_copy_global_ids_off(vtkDataSetAttributes* sself) { sself->CopyGlobalIdsOff(); }
extern "C" void vtk_data_set_attributes_set_copy_pedigree_ids(vtkDataSetAttributes* sself, int i, int ctype) { sself->SetCopyPedigreeIds(i, ctype); }
extern "C" int vtk_data_set_attributes_get_copy_pedigree_ids(vtkDataSetAttributes* sself, int ctype) { return sself->GetCopyPedigreeIds(ctype); }
extern "C" void vtk_data_set_attributes_copy_pedigree_ids_on(vtkDataSetAttributes* sself) { sself->CopyPedigreeIdsOn(); }
extern "C" void vtk_data_set_attributes_copy_pedigree_ids_off(vtkDataSetAttributes* sself) { sself->CopyPedigreeIdsOff(); }
extern "C" void vtk_data_set_attributes_set_copy_rational_weights(vtkDataSetAttributes* sself, int i, int ctype) { sself->SetCopyRationalWeights(i, ctype); }
extern "C" int vtk_data_set_attributes_get_copy_rational_weights(vtkDataSetAttributes* sself, int ctype) { return sself->GetCopyRationalWeights(ctype); }
extern "C" void vtk_data_set_attributes_copy_rational_weights_on(vtkDataSetAttributes* sself) { sself->CopyRationalWeightsOn(); }
extern "C" void vtk_data_set_attributes_copy_rational_weights_off(vtkDataSetAttributes* sself) { sself->CopyRationalWeightsOff(); }
extern "C" void vtk_data_set_attributes_set_copy_higher_order_degrees(vtkDataSetAttributes* sself, int i, int ctype) { sself->SetCopyHigherOrderDegrees(i, ctype); }
extern "C" int vtk_data_set_attributes_get_copy_higher_order_degrees(vtkDataSetAttributes* sself, int ctype) { return sself->GetCopyHigherOrderDegrees(ctype); }
extern "C" void vtk_data_set_attributes_copy_higher_order_degrees_on(vtkDataSetAttributes* sself) { sself->CopyHigherOrderDegreesOn(); }
extern "C" void vtk_data_set_attributes_copy_higher_order_degrees_off(vtkDataSetAttributes* sself) { sself->CopyHigherOrderDegreesOff(); }
extern "C" void vtk_data_set_attributes_copy_all_on(vtkDataSetAttributes* sself, int ctype) { sself->CopyAllOn(ctype); }
extern "C" void vtk_data_set_attributes_copy_all_off(vtkDataSetAttributes* sself, int ctype) { sself->CopyAllOff(ctype); }
extern "C" vtkDataSetCellIterator * vtkDataSetCellIterator_new () {return vtkDataSetCellIterator :: New () ;}
extern "C" void vtkDataSetCellIterator_destructor (vtkDataSetCellIterator * sself) {sself -> Delete () ; return ;}
extern "C" bool vtk_data_set_cell_iterator_is_done_with_traversal(vtkDataSetCellIterator* sself) { return sself->IsDoneWithTraversal(); }
extern "C" long long vtk_data_set_cell_iterator_get_cell_id(vtkDataSetCellIterator* sself) { return sself->GetCellId(); }
extern "C" vtkDataSetCollection * vtkDataSetCollection_new () {return vtkDataSetCollection :: New () ;}
extern "C" void vtkDataSetCollection_destructor (vtkDataSetCollection * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_data_set_collection_get_number_of_items(vtkDataSetCollection* sself) { return sself->GetNumberOfItems(); }
extern "C" vtkDirectedAcyclicGraph * vtkDirectedAcyclicGraph_new () {return vtkDirectedAcyclicGraph :: New () ;}
extern "C" void vtkDirectedAcyclicGraph_destructor (vtkDirectedAcyclicGraph * sself) {sself -> Delete () ; return ;}
extern "C" vtkDirectedGraph * vtkDirectedGraph_new () {return vtkDirectedGraph :: New () ;}
extern "C" void vtkDirectedGraph_destructor (vtkDirectedGraph * sself) {sself -> Delete () ; return ;}
extern "C" vtkEdgeListIterator * vtkEdgeListIterator_new () {return vtkEdgeListIterator :: New () ;}
extern "C" void vtkEdgeListIterator_destructor (vtkEdgeListIterator * sself) {sself -> Delete () ; return ;}
extern "C" bool vtk_edge_list_iterator_has_next(vtkEdgeListIterator* sself) { return sself->HasNext(); }
extern "C" vtkEdgeTable * vtkEdgeTable_new () {return vtkEdgeTable :: New () ;}
extern "C" void vtkEdgeTable_destructor (vtkEdgeTable * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_edge_table_initialize(vtkEdgeTable* sself) { sself->Initialize(); }
extern "C" int vtk_edge_table_init_edge_insertion(vtkEdgeTable* sself, long long numPoints, int storeAttributes) { return sself->InitEdgeInsertion(numPoints, storeAttributes); }
extern "C" long long vtk_edge_table_insert_edge(vtkEdgeTable* sself, long long p1, long long p2) { return sself->InsertEdge(p1, p2); }
extern "C" long long vtk_edge_table_is_edge(vtkEdgeTable* sself, long long p1, long long p2) { return sself->IsEdge(p1, p2); }
extern "C" long long vtk_edge_table_get_number_of_edges(vtkEdgeTable* sself) { return sself->GetNumberOfEdges(); }
extern "C" void vtk_edge_table_init_traversal(vtkEdgeTable* sself) { sself->InitTraversal(); }
extern "C" long long vtk_edge_table_get_next_edge(vtkEdgeTable* sself, long long& p1, long long& p2) { return sself->GetNextEdge(p1, p2); }
extern "C" void vtk_edge_table_reset(vtkEdgeTable* sself) { sself->Reset(); }
extern "C" vtkEmptyCell * vtkEmptyCell_new () {return vtkEmptyCell :: New () ;}
extern "C" void vtkEmptyCell_destructor (vtkEmptyCell * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_empty_cell_get_cell_type(vtkEmptyCell* sself) { return sself->GetCellType(); }
extern "C" int vtk_empty_cell_get_cell_dimension(vtkEmptyCell* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_empty_cell_get_number_of_edges(vtkEmptyCell* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_empty_cell_get_number_of_faces(vtkEmptyCell* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkExplicitStructuredGrid * vtkExplicitStructuredGrid_new () {return vtkExplicitStructuredGrid :: New () ;}
extern "C" void vtkExplicitStructuredGrid_destructor (vtkExplicitStructuredGrid * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_explicit_structured_grid_get_data_object_type(vtkExplicitStructuredGrid* sself) { return sself->GetDataObjectType(); }
extern "C" void vtk_explicit_structured_grid_initialize(vtkExplicitStructuredGrid* sself) { sself->Initialize(); }
extern "C" int vtk_explicit_structured_grid_get_cell_type(vtkExplicitStructuredGrid* sself, long long cellId) { return sself->GetCellType(cellId); }
extern "C" int vtk_explicit_structured_grid_get_data_dimension(vtkExplicitStructuredGrid* sself) { return sself->GetDataDimension(); }
extern "C" void vtk_explicit_structured_grid_set_dimensions(vtkExplicitStructuredGrid* sself, int i, int j, int k) { sself->SetDimensions(i, j, k); }
extern "C" int vtk_explicit_structured_grid_get_extent_type(vtkExplicitStructuredGrid* sself) { return sself->GetExtentType(); }
extern "C" void vtk_explicit_structured_grid_set_extent(vtkExplicitStructuredGrid* sself, int x0, int x1, int y0, int y1, int z0, int z1) { sself->SetExtent(x0, x1, y0, y1, z0, z1); }
extern "C" void vtk_explicit_structured_grid_build_links(vtkExplicitStructuredGrid* sself) { sself->BuildLinks(); }
extern "C" void vtk_explicit_structured_grid_compute_cell_structured_coords(vtkExplicitStructuredGrid* sself, long long cellId, int& i, int& j, int& k, bool adjustForExtent) { sself->ComputeCellStructuredCoords(cellId, i, j, k, adjustForExtent); }
extern "C" long long vtk_explicit_structured_grid_compute_cell_id(vtkExplicitStructuredGrid* sself, int i, int j, int k, bool adjustForExtent) { return sself->ComputeCellId(i, j, k, adjustForExtent); }
extern "C" void vtk_explicit_structured_grid_compute_faces_connectivity_flags_array(vtkExplicitStructuredGrid* sself) { sself->ComputeFacesConnectivityFlagsArray(); }
extern "C" void vtk_explicit_structured_grid_set_faces_connectivity_flags_array_name(vtkExplicitStructuredGrid* sself, const char* _arg) { sself->SetFacesConnectivityFlagsArrayName(_arg); }
extern "C" void vtk_explicit_structured_grid_blank_cell(vtkExplicitStructuredGrid* sself, long long cellId) { sself->BlankCell(cellId); }
extern "C" void vtk_explicit_structured_grid_un_blank_cell(vtkExplicitStructuredGrid* sself, long long cellId) { sself->UnBlankCell(cellId); }
extern "C" bool vtk_explicit_structured_grid_has_any_blank_cells(vtkExplicitStructuredGrid* sself) { return sself->HasAnyBlankCells(); }
extern "C" unsigned char vtk_explicit_structured_grid_is_cell_visible(vtkExplicitStructuredGrid* sself, long long cellId) { return sself->IsCellVisible(cellId); }
extern "C" unsigned char vtk_explicit_structured_grid_is_cell_ghost(vtkExplicitStructuredGrid* sself, long long cellId) { return sself->IsCellGhost(cellId); }
extern "C" bool vtk_explicit_structured_grid_has_any_ghost_cells(vtkExplicitStructuredGrid* sself) { return sself->HasAnyGhostCells(); }
extern "C" unsigned long vtk_explicit_structured_grid_get_actual_memory_size(vtkExplicitStructuredGrid* sself) { return sself->GetActualMemorySize(); }
extern "C" void vtk_explicit_structured_grid_check_and_reorder_faces(vtkExplicitStructuredGrid* sself) { sself->CheckAndReorderFaces(); }
extern "C" vtkExtractStructuredGridHelper * vtkExtractStructuredGridHelper_new () {return vtkExtractStructuredGridHelper :: New () ;}
extern "C" void vtkExtractStructuredGridHelper_destructor (vtkExtractStructuredGridHelper * sself) {sself -> Delete () ; return ;}
extern "C" bool vtk_extract_structured_grid_helper_is_valid(vtkExtractStructuredGridHelper* sself) { return sself->IsValid(); }
extern "C" int vtk_extract_structured_grid_helper_get_size(vtkExtractStructuredGridHelper* sself, const int dim) { return sself->GetSize(dim); }
extern "C" int vtk_extract_structured_grid_helper_get_mapped_index(vtkExtractStructuredGridHelper* sself, int dim, int outIdx) { return sself->GetMappedIndex(dim, outIdx); }
extern "C" int vtk_extract_structured_grid_helper_get_mapped_index_from_extent_value(vtkExtractStructuredGridHelper* sself, int dim, int outExtVal) { return sself->GetMappedIndexFromExtentValue(dim, outExtVal); }
extern "C" int vtk_extract_structured_grid_helper_get_mapped_extent_value(vtkExtractStructuredGridHelper* sself, int dim, int outExtVal) { return sself->GetMappedExtentValue(dim, outExtVal); }
extern "C" int vtk_extract_structured_grid_helper_get_mapped_extent_value_from_index(vtkExtractStructuredGridHelper* sself, int dim, int outIdx) { return sself->GetMappedExtentValueFromIndex(dim, outIdx); }
extern "C" vtkFieldData * vtkFieldData_new () {return vtkFieldData :: New () ;}
extern "C" void vtkFieldData_destructor (vtkFieldData * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_field_data_initialize(vtkFieldData* sself) { sself->Initialize(); }
extern "C" int vtk_field_data_allocate(vtkFieldData* sself, long long sz, long long ext) { return sself->Allocate(sz, ext); }
extern "C" void vtk_field_data_allocate_arrays(vtkFieldData* sself, int num) { sself->AllocateArrays(num); }
extern "C" int vtk_field_data_get_number_of_arrays(vtkFieldData* sself) { return sself->GetNumberOfArrays(); }
extern "C" void vtk_field_data_null_data(vtkFieldData* sself, long long id) { sself->NullData(id); }
extern "C" void vtk_field_data_remove_array(vtkFieldData* sself, const char* name) { sself->RemoveArray(name); }
extern "C" int vtk_field_data_has_array(vtkFieldData* sself, const char* name) { return sself->HasArray(name); }
extern "C" const char* vtk_field_data_get_array_name(vtkFieldData* sself, int i) { return sself->GetArrayName(i); }
extern "C" void vtk_field_data_copy_field_on(vtkFieldData* sself, const char* name) { sself->CopyFieldOn(name); }
extern "C" void vtk_field_data_copy_field_off(vtkFieldData* sself, const char* name) { sself->CopyFieldOff(name); }
extern "C" void vtk_field_data_copy_all_on(vtkFieldData* sself, int unused) { sself->CopyAllOn(unused); }
extern "C" void vtk_field_data_copy_all_off(vtkFieldData* sself, int unused) { sself->CopyAllOff(unused); }
extern "C" void vtk_field_data_squeeze(vtkFieldData* sself) { sself->Squeeze(); }
extern "C" void vtk_field_data_reset(vtkFieldData* sself) { sself->Reset(); }
extern "C" unsigned long vtk_field_data_get_actual_memory_size(vtkFieldData* sself) { return sself->GetActualMemorySize(); }
extern "C" unsigned long vtk_field_data_get_m_time(vtkFieldData* sself) { return sself->GetMTime(); }
extern "C" int vtk_field_data_get_array_containing_component(vtkFieldData* sself, int i, int& arrayComp) { return sself->GetArrayContainingComponent(i, arrayComp); }
extern "C" int vtk_field_data_get_number_of_components(vtkFieldData* sself) { return sself->GetNumberOfComponents(); }
extern "C" long long vtk_field_data_get_number_of_tuples(vtkFieldData* sself) { return sself->GetNumberOfTuples(); }
extern "C" void vtk_field_data_set_number_of_tuples(vtkFieldData* sself, const long long number) { sself->SetNumberOfTuples(number); }
extern "C" vtkGenericAttributeCollection * vtkGenericAttributeCollection_new () {return vtkGenericAttributeCollection :: New () ;}
extern "C" void vtkGenericAttributeCollection_destructor (vtkGenericAttributeCollection * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_generic_attribute_collection_get_number_of_attributes(vtkGenericAttributeCollection* sself) { return sself->GetNumberOfAttributes(); }
extern "C" int vtk_generic_attribute_collection_get_number_of_components(vtkGenericAttributeCollection* sself) { return sself->GetNumberOfComponents(); }
extern "C" int vtk_generic_attribute_collection_get_number_of_point_centered_components(vtkGenericAttributeCollection* sself) { return sself->GetNumberOfPointCenteredComponents(); }
extern "C" int vtk_generic_attribute_collection_get_max_number_of_components(vtkGenericAttributeCollection* sself) { return sself->GetMaxNumberOfComponents(); }
extern "C" unsigned long vtk_generic_attribute_collection_get_actual_memory_size(vtkGenericAttributeCollection* sself) { return sself->GetActualMemorySize(); }
extern "C" int vtk_generic_attribute_collection_is_empty(vtkGenericAttributeCollection* sself) { return sself->IsEmpty(); }
extern "C" int vtk_generic_attribute_collection_find_attribute(vtkGenericAttributeCollection* sself, const char* name) { return sself->FindAttribute(name); }
extern "C" int vtk_generic_attribute_collection_get_attribute_index(vtkGenericAttributeCollection* sself, int i) { return sself->GetAttributeIndex(i); }
extern "C" void vtk_generic_attribute_collection_remove_attribute(vtkGenericAttributeCollection* sself, int i) { sself->RemoveAttribute(i); }
extern "C" void vtk_generic_attribute_collection_reset(vtkGenericAttributeCollection* sself) { sself->Reset(); }
extern "C" unsigned long vtk_generic_attribute_collection_get_m_time(vtkGenericAttributeCollection* sself) { return sself->GetMTime(); }
extern "C" int vtk_generic_attribute_collection_get_active_attribute(vtkGenericAttributeCollection* sself) { return sself->GetActiveAttribute(); }
extern "C" int vtk_generic_attribute_collection_get_active_component(vtkGenericAttributeCollection* sself) { return sself->GetActiveComponent(); }
extern "C" void vtk_generic_attribute_collection_set_active_attribute(vtkGenericAttributeCollection* sself, int attribute, int component) { sself->SetActiveAttribute(attribute, component); }
extern "C" int vtk_generic_attribute_collection_get_number_of_attributes_to_interpolate(vtkGenericAttributeCollection* sself) { return sself->GetNumberOfAttributesToInterpolate(); }
extern "C" void vtk_generic_attribute_collection_set_attributes_to_interpolate_to_all(vtkGenericAttributeCollection* sself) { sself->SetAttributesToInterpolateToAll(); }
extern "C" vtkGenericCell * vtkGenericCell_new () {return vtkGenericCell :: New () ;}
extern "C" void vtkGenericCell_destructor (vtkGenericCell * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_generic_cell_get_cell_type(vtkGenericCell* sself) { return sself->GetCellType(); }
extern "C" int vtk_generic_cell_get_cell_dimension(vtkGenericCell* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_generic_cell_get_number_of_edges(vtkGenericCell* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_generic_cell_get_number_of_faces(vtkGenericCell* sself) { return sself->GetNumberOfFaces(); }
extern "C" void vtk_generic_cell_set_cell_type(vtkGenericCell* sself, int cellType) { sself->SetCellType(cellType); }
extern "C" void vtk_generic_cell_set_cell_type_to_empty_cell(vtkGenericCell* sself) { sself->SetCellTypeToEmptyCell(); }
extern "C" void vtk_generic_cell_set_cell_type_to_vertex(vtkGenericCell* sself) { sself->SetCellTypeToVertex(); }
extern "C" void vtk_generic_cell_set_cell_type_to_poly_vertex(vtkGenericCell* sself) { sself->SetCellTypeToPolyVertex(); }
extern "C" void vtk_generic_cell_set_cell_type_to_line(vtkGenericCell* sself) { sself->SetCellTypeToLine(); }
extern "C" void vtk_generic_cell_set_cell_type_to_poly_line(vtkGenericCell* sself) { sself->SetCellTypeToPolyLine(); }
extern "C" void vtk_generic_cell_set_cell_type_to_triangle(vtkGenericCell* sself) { sself->SetCellTypeToTriangle(); }
extern "C" void vtk_generic_cell_set_cell_type_to_triangle_strip(vtkGenericCell* sself) { sself->SetCellTypeToTriangleStrip(); }
extern "C" void vtk_generic_cell_set_cell_type_to_polygon(vtkGenericCell* sself) { sself->SetCellTypeToPolygon(); }
extern "C" void vtk_generic_cell_set_cell_type_to_pixel(vtkGenericCell* sself) { sself->SetCellTypeToPixel(); }
extern "C" void vtk_generic_cell_set_cell_type_to_quad(vtkGenericCell* sself) { sself->SetCellTypeToQuad(); }
extern "C" void vtk_generic_cell_set_cell_type_to_tetra(vtkGenericCell* sself) { sself->SetCellTypeToTetra(); }
extern "C" void vtk_generic_cell_set_cell_type_to_voxel(vtkGenericCell* sself) { sself->SetCellTypeToVoxel(); }
extern "C" void vtk_generic_cell_set_cell_type_to_hexahedron(vtkGenericCell* sself) { sself->SetCellTypeToHexahedron(); }
extern "C" void vtk_generic_cell_set_cell_type_to_wedge(vtkGenericCell* sself) { sself->SetCellTypeToWedge(); }
extern "C" void vtk_generic_cell_set_cell_type_to_pyramid(vtkGenericCell* sself) { sself->SetCellTypeToPyramid(); }
extern "C" void vtk_generic_cell_set_cell_type_to_pentagonal_prism(vtkGenericCell* sself) { sself->SetCellTypeToPentagonalPrism(); }
extern "C" void vtk_generic_cell_set_cell_type_to_hexagonal_prism(vtkGenericCell* sself) { sself->SetCellTypeToHexagonalPrism(); }
extern "C" void vtk_generic_cell_set_cell_type_to_polyhedron(vtkGenericCell* sself) { sself->SetCellTypeToPolyhedron(); }
extern "C" void vtk_generic_cell_set_cell_type_to_convex_point_set(vtkGenericCell* sself) { sself->SetCellTypeToConvexPointSet(); }
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_edge(vtkGenericCell* sself) { sself->SetCellTypeToQuadraticEdge(); }
extern "C" void vtk_generic_cell_set_cell_type_to_cubic_line(vtkGenericCell* sself) { sself->SetCellTypeToCubicLine(); }
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_triangle(vtkGenericCell* sself) { sself->SetCellTypeToQuadraticTriangle(); }
extern "C" void vtk_generic_cell_set_cell_type_to_bi_quadratic_triangle(vtkGenericCell* sself) { sself->SetCellTypeToBiQuadraticTriangle(); }
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_quad(vtkGenericCell* sself) { sself->SetCellTypeToQuadraticQuad(); }
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_polygon(vtkGenericCell* sself) { sself->SetCellTypeToQuadraticPolygon(); }
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_tetra(vtkGenericCell* sself) { sself->SetCellTypeToQuadraticTetra(); }
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_hexahedron(vtkGenericCell* sself) { sself->SetCellTypeToQuadraticHexahedron(); }
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_wedge(vtkGenericCell* sself) { sself->SetCellTypeToQuadraticWedge(); }
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_pyramid(vtkGenericCell* sself) { sself->SetCellTypeToQuadraticPyramid(); }
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_linear_quad(vtkGenericCell* sself) { sself->SetCellTypeToQuadraticLinearQuad(); }
extern "C" void vtk_generic_cell_set_cell_type_to_bi_quadratic_quad(vtkGenericCell* sself) { sself->SetCellTypeToBiQuadraticQuad(); }
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_linear_wedge(vtkGenericCell* sself) { sself->SetCellTypeToQuadraticLinearWedge(); }
extern "C" void vtk_generic_cell_set_cell_type_to_bi_quadratic_quadratic_wedge(vtkGenericCell* sself) { sself->SetCellTypeToBiQuadraticQuadraticWedge(); }
extern "C" void vtk_generic_cell_set_cell_type_to_tri_quadratic_hexahedron(vtkGenericCell* sself) { sself->SetCellTypeToTriQuadraticHexahedron(); }
extern "C" void vtk_generic_cell_set_cell_type_to_tri_quadratic_pyramid(vtkGenericCell* sself) { sself->SetCellTypeToTriQuadraticPyramid(); }
extern "C" void vtk_generic_cell_set_cell_type_to_bi_quadratic_quadratic_hexahedron(vtkGenericCell* sself) { sself->SetCellTypeToBiQuadraticQuadraticHexahedron(); }
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_triangle(vtkGenericCell* sself) { sself->SetCellTypeToLagrangeTriangle(); }
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_tetra(vtkGenericCell* sself) { sself->SetCellTypeToLagrangeTetra(); }
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_curve(vtkGenericCell* sself) { sself->SetCellTypeToLagrangeCurve(); }
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_quadrilateral(vtkGenericCell* sself) { sself->SetCellTypeToLagrangeQuadrilateral(); }
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_hexahedron(vtkGenericCell* sself) { sself->SetCellTypeToLagrangeHexahedron(); }
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_wedge(vtkGenericCell* sself) { sself->SetCellTypeToLagrangeWedge(); }
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_triangle(vtkGenericCell* sself) { sself->SetCellTypeToBezierTriangle(); }
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_tetra(vtkGenericCell* sself) { sself->SetCellTypeToBezierTetra(); }
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_curve(vtkGenericCell* sself) { sself->SetCellTypeToBezierCurve(); }
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_quadrilateral(vtkGenericCell* sself) { sself->SetCellTypeToBezierQuadrilateral(); }
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_hexahedron(vtkGenericCell* sself) { sself->SetCellTypeToBezierHexahedron(); }
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_wedge(vtkGenericCell* sself) { sself->SetCellTypeToBezierWedge(); }
extern "C" vtkGenericEdgeTable * vtkGenericEdgeTable_new () {return vtkGenericEdgeTable :: New () ;}
extern "C" void vtkGenericEdgeTable_destructor (vtkGenericEdgeTable * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_generic_edge_table_insert_edge(vtkGenericEdgeTable* sself, long long e1, long long e2, long long cellId, int ref, long long& ptId) { sself->InsertEdge(e1, e2, cellId, ref, ptId); }
extern "C" int vtk_generic_edge_table_remove_edge(vtkGenericEdgeTable* sself, long long e1, long long e2) { return sself->RemoveEdge(e1, e2); }
extern "C" int vtk_generic_edge_table_check_edge(vtkGenericEdgeTable* sself, long long e1, long long e2, long long& ptId) { return sself->CheckEdge(e1, e2, ptId); }
extern "C" int vtk_generic_edge_table_increment_edge_reference_count(vtkGenericEdgeTable* sself, long long e1, long long e2, long long cellId) { return sself->IncrementEdgeReferenceCount(e1, e2, cellId); }
extern "C" int vtk_generic_edge_table_check_edge_reference_count(vtkGenericEdgeTable* sself, long long e1, long long e2) { return sself->CheckEdgeReferenceCount(e1, e2); }
extern "C" void vtk_generic_edge_table_initialize(vtkGenericEdgeTable* sself, long long start) { sself->Initialize(start); }
extern "C" int vtk_generic_edge_table_get_number_of_components(vtkGenericEdgeTable* sself) { return sself->GetNumberOfComponents(); }
extern "C" void vtk_generic_edge_table_set_number_of_components(vtkGenericEdgeTable* sself, int count) { sself->SetNumberOfComponents(count); }
extern "C" int vtk_generic_edge_table_check_point(vtkGenericEdgeTable* sself, long long ptId) { return sself->CheckPoint(ptId); }
extern "C" void vtk_generic_edge_table_remove_point(vtkGenericEdgeTable* sself, long long ptId) { sself->RemovePoint(ptId); }
extern "C" void vtk_generic_edge_table_increment_point_reference_count(vtkGenericEdgeTable* sself, long long ptId) { sself->IncrementPointReferenceCount(ptId); }
extern "C" void vtk_generic_edge_table_dump_table(vtkGenericEdgeTable* sself) { sself->DumpTable(); }
extern "C" void vtk_generic_edge_table_load_factor(vtkGenericEdgeTable* sself) { sself->LoadFactor(); }
extern "C" vtkGenericInterpolatedVelocityField * vtkGenericInterpolatedVelocityField_new () {return vtkGenericInterpolatedVelocityField :: New () ;}
extern "C" void vtkGenericInterpolatedVelocityField_destructor (vtkGenericInterpolatedVelocityField * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_generic_interpolated_velocity_field_clear_last_cell(vtkGenericInterpolatedVelocityField* sself) { sself->ClearLastCell(); }
extern "C" int vtk_generic_interpolated_velocity_field_get_caching(vtkGenericInterpolatedVelocityField* sself) { return sself->GetCaching(); }
extern "C" void vtk_generic_interpolated_velocity_field_set_caching(vtkGenericInterpolatedVelocityField* sself, int _arg) { sself->SetCaching(_arg); }
extern "C" void vtk_generic_interpolated_velocity_field_caching_on(vtkGenericInterpolatedVelocityField* sself) { sself->CachingOn(); }
extern "C" void vtk_generic_interpolated_velocity_field_caching_off(vtkGenericInterpolatedVelocityField* sself) { sself->CachingOff(); }
extern "C" int vtk_generic_interpolated_velocity_field_get_cache_hit(vtkGenericInterpolatedVelocityField* sself) { return sself->GetCacheHit(); }
extern "C" int vtk_generic_interpolated_velocity_field_get_cache_miss(vtkGenericInterpolatedVelocityField* sself) { return sself->GetCacheMiss(); }
extern "C" void vtk_generic_interpolated_velocity_field_select_vectors(vtkGenericInterpolatedVelocityField* sself, const char* fieldName) { sself->SelectVectors(fieldName); }
extern "C" vtkGeometricErrorMetric * vtkGeometricErrorMetric_new () {return vtkGeometricErrorMetric :: New () ;}
extern "C" void vtkGeometricErrorMetric_destructor (vtkGeometricErrorMetric * sself) {sself -> Delete () ; return ;}
extern "C" double vtk_geometric_error_metric_get_absolute_geometric_tolerance(vtkGeometricErrorMetric* sself) { return sself->GetAbsoluteGeometricTolerance(); }
extern "C" void vtk_geometric_error_metric_set_absolute_geometric_tolerance(vtkGeometricErrorMetric* sself, double value) { sself->SetAbsoluteGeometricTolerance(value); }
extern "C" int vtk_geometric_error_metric_get_relative(vtkGeometricErrorMetric* sself) { return sself->GetRelative(); }
extern "C" vtkGraphEdge * vtkGraphEdge_new () {return vtkGraphEdge :: New () ;}
extern "C" void vtkGraphEdge_destructor (vtkGraphEdge * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_graph_edge_set_source(vtkGraphEdge* sself, long long _arg) { sself->SetSource(_arg); }
extern "C" long long vtk_graph_edge_get_source(vtkGraphEdge* sself) { return sself->GetSource(); }
extern "C" void vtk_graph_edge_set_target(vtkGraphEdge* sself, long long _arg) { sself->SetTarget(_arg); }
extern "C" long long vtk_graph_edge_get_target(vtkGraphEdge* sself) { return sself->GetTarget(); }
extern "C" void vtk_graph_edge_set_id(vtkGraphEdge* sself, long long _arg) { sself->SetId(_arg); }
extern "C" long long vtk_graph_edge_get_id(vtkGraphEdge* sself) { return sself->GetId(); }
extern "C" vtkGraphInternals * vtkGraphInternals_new () {return vtkGraphInternals :: New () ;}
extern "C" void vtkGraphInternals_destructor (vtkGraphInternals * sself) {sself -> Delete () ; return ;}
extern "C" vtkHexagonalPrism * vtkHexagonalPrism_new () {return vtkHexagonalPrism :: New () ;}
extern "C" void vtkHexagonalPrism_destructor (vtkHexagonalPrism * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_hexagonal_prism_get_cell_type(vtkHexagonalPrism* sself) { return sself->GetCellType(); }
extern "C" int vtk_hexagonal_prism_get_number_of_edges(vtkHexagonalPrism* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_hexagonal_prism_get_number_of_faces(vtkHexagonalPrism* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkHexahedron * vtkHexahedron_new () {return vtkHexahedron :: New () ;}
extern "C" void vtkHexahedron_destructor (vtkHexahedron * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_hexahedron_get_cell_type(vtkHexahedron* sself) { return sself->GetCellType(); }
extern "C" int vtk_hexahedron_get_number_of_edges(vtkHexahedron* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_hexahedron_get_number_of_faces(vtkHexahedron* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkHierarchicalBoxDataIterator * vtkHierarchicalBoxDataIterator_new () {return vtkHierarchicalBoxDataIterator :: New () ;}
extern "C" void vtkHierarchicalBoxDataIterator_destructor (vtkHierarchicalBoxDataIterator * sself) {sself -> Delete () ; return ;}
extern "C" vtkHierarchicalBoxDataSet * vtkHierarchicalBoxDataSet_new () {return vtkHierarchicalBoxDataSet :: New () ;}
extern "C" void vtkHierarchicalBoxDataSet_destructor (vtkHierarchicalBoxDataSet * sself) {sself -> Delete () ; return ;}
extern "C" vtkHyperTreeGrid * vtkHyperTreeGrid_new () {return vtkHyperTreeGrid :: New () ;}
extern "C" void vtkHyperTreeGrid_destructor (vtkHyperTreeGrid * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_hyper_tree_grid_set_mode_squeeze(vtkHyperTreeGrid* sself, const char* _arg) { sself->SetModeSqueeze(_arg); }
extern "C" void vtk_hyper_tree_grid_squeeze(vtkHyperTreeGrid* sself) { sself->Squeeze(); }
extern "C" int vtk_hyper_tree_grid_get_data_object_type(vtkHyperTreeGrid* sself) { return sself->GetDataObjectType(); }
extern "C" void vtk_hyper_tree_grid_set_dimensions(vtkHyperTreeGrid* sself, unsigned int i, unsigned int j, unsigned int k) { sself->SetDimensions(i, j, k); }
extern "C" void vtk_hyper_tree_grid_set_extent(vtkHyperTreeGrid* sself, int x1, int x2, int y1, int y2, int z1, int z2) { sself->SetExtent(x1, x2, y1, y2, z1, z2); }
extern "C" unsigned int vtk_hyper_tree_grid_get_dimension(vtkHyperTreeGrid* sself) { return sself->GetDimension(); }
extern "C" void vtk_hyper_tree_grid_get_1_d_axis(vtkHyperTreeGrid* sself, unsigned int& axis) { sself->Get1DAxis(axis); }
extern "C" void vtk_hyper_tree_grid_get_2_d_axes(vtkHyperTreeGrid* sself, unsigned int& axis1, unsigned int& axis2) { sself->Get2DAxes(axis1, axis2); }
extern "C" unsigned int vtk_hyper_tree_grid_get_number_of_children(vtkHyperTreeGrid* sself) { return sself->GetNumberOfChildren(); }
extern "C" void vtk_hyper_tree_grid_set_transposed_root_indexing(vtkHyperTreeGrid* sself, bool _arg) { sself->SetTransposedRootIndexing(_arg); }
extern "C" bool vtk_hyper_tree_grid_get_transposed_root_indexing(vtkHyperTreeGrid* sself) { return sself->GetTransposedRootIndexing(); }
extern "C" void vtk_hyper_tree_grid_set_indexing_mode_to_kji(vtkHyperTreeGrid* sself) { sself->SetIndexingModeToKJI(); }
extern "C" void vtk_hyper_tree_grid_set_indexing_mode_to_ijk(vtkHyperTreeGrid* sself) { sself->SetIndexingModeToIJK(); }
extern "C" unsigned int vtk_hyper_tree_grid_get_orientation(vtkHyperTreeGrid* sself) { return sself->GetOrientation(); }
extern "C" bool vtk_hyper_tree_grid_get_freeze_state(vtkHyperTreeGrid* sself) { return sself->GetFreezeState(); }
extern "C" void vtk_hyper_tree_grid_set_branch_factor(vtkHyperTreeGrid* sself, unsigned int p0) { sself->SetBranchFactor(p0); }
extern "C" unsigned int vtk_hyper_tree_grid_get_branch_factor(vtkHyperTreeGrid* sself) { return sself->GetBranchFactor(); }
extern "C" long long vtk_hyper_tree_grid_get_max_number_of_trees(vtkHyperTreeGrid* sself) { return sself->GetMaxNumberOfTrees(); }
extern "C" long long vtk_hyper_tree_grid_get_number_of_vertices(vtkHyperTreeGrid* sself) { return sself->GetNumberOfVertices(); }
extern "C" long long vtk_hyper_tree_grid_get_number_of_non_empty_trees(vtkHyperTreeGrid* sself) { return sself->GetNumberOfNonEmptyTrees(); }
extern "C" long long vtk_hyper_tree_grid_get_number_of_leaves(vtkHyperTreeGrid* sself) { return sself->GetNumberOfLeaves(); }
extern "C" unsigned int vtk_hyper_tree_grid_get_number_of_levels(vtkHyperTreeGrid* sself, long long p0) { return sself->GetNumberOfLevels(p0); }
extern "C" void vtk_hyper_tree_grid_set_fixed_coordinates(vtkHyperTreeGrid* sself, unsigned int axis, double value) { sself->SetFixedCoordinates(axis, value); }
extern "C" bool vtk_hyper_tree_grid_has_mask(vtkHyperTreeGrid* sself) { return sself->HasMask(); }
extern "C" void vtk_hyper_tree_grid_set_has_interface(vtkHyperTreeGrid* sself, bool _arg) { sself->SetHasInterface(_arg); }
extern "C" bool vtk_hyper_tree_grid_get_has_interface(vtkHyperTreeGrid* sself) { return sself->GetHasInterface(); }
extern "C" void vtk_hyper_tree_grid_has_interface_on(vtkHyperTreeGrid* sself) { sself->HasInterfaceOn(); }
extern "C" void vtk_hyper_tree_grid_has_interface_off(vtkHyperTreeGrid* sself) { sself->HasInterfaceOff(); }
extern "C" void vtk_hyper_tree_grid_set_interface_normals_name(vtkHyperTreeGrid* sself, const char* _arg) { sself->SetInterfaceNormalsName(_arg); }
extern "C" void vtk_hyper_tree_grid_set_interface_intercepts_name(vtkHyperTreeGrid* sself, const char* _arg) { sself->SetInterfaceInterceptsName(_arg); }
extern "C" void vtk_hyper_tree_grid_set_depth_limiter(vtkHyperTreeGrid* sself, unsigned int _arg) { sself->SetDepthLimiter(_arg); }
extern "C" unsigned int vtk_hyper_tree_grid_get_depth_limiter(vtkHyperTreeGrid* sself) { return sself->GetDepthLimiter(); }
extern "C" unsigned int vtk_hyper_tree_grid_find_dichotomic_x(vtkHyperTreeGrid* sself, double value) { return sself->FindDichotomicX(value); }
extern "C" unsigned int vtk_hyper_tree_grid_find_dichotomic_y(vtkHyperTreeGrid* sself, double value) { return sself->FindDichotomicY(value); }
extern "C" unsigned int vtk_hyper_tree_grid_find_dichotomic_z(vtkHyperTreeGrid* sself, double value) { return sself->FindDichotomicZ(value); }
extern "C" void vtk_hyper_tree_grid_initialize(vtkHyperTreeGrid* sself) { sself->Initialize(); }
extern "C" int vtk_hyper_tree_grid_get_extent_type(vtkHyperTreeGrid* sself) { return sself->GetExtentType(); }
extern "C" unsigned long vtk_hyper_tree_grid_get_actual_memory_size_bytes(vtkHyperTreeGrid* sself) { return sself->GetActualMemorySizeBytes(); }
extern "C" unsigned long vtk_hyper_tree_grid_get_actual_memory_size(vtkHyperTreeGrid* sself) { return sself->GetActualMemorySize(); }
extern "C" unsigned int vtk_hyper_tree_grid_get_child_mask(vtkHyperTreeGrid* sself, unsigned int p0) { return sself->GetChildMask(p0); }
extern "C" void vtk_hyper_tree_grid_get_index_from_level_zero_coordinates(vtkHyperTreeGrid* sself, long long& p0, unsigned int p1, unsigned int p2, unsigned int p3) { sself->GetIndexFromLevelZeroCoordinates(p0, p1, p2, p3); }
extern "C" long long vtk_hyper_tree_grid_get_shifted_level_zero_index(vtkHyperTreeGrid* sself, long long p0, unsigned int p1, unsigned int p2, unsigned int p3) { return sself->GetShiftedLevelZeroIndex(p0, p1, p2, p3); }
extern "C" void vtk_hyper_tree_grid_get_level_zero_coordinates_from_index(vtkHyperTreeGrid* sself, long long p0, unsigned int& p1, unsigned int& p2, unsigned int& p3) { sself->GetLevelZeroCoordinatesFromIndex(p0, p1, p2, p3); }
extern "C" long long vtk_hyper_tree_grid_get_global_node_index_max(vtkHyperTreeGrid* sself) { return sself->GetGlobalNodeIndexMax(); }
extern "C" void vtk_hyper_tree_grid_initialize_local_index_node(vtkHyperTreeGrid* sself) { sself->InitializeLocalIndexNode(); }
extern "C" bool vtk_hyper_tree_grid_has_any_ghost_cells(vtkHyperTreeGrid* sself) { return sself->HasAnyGhostCells(); }
extern "C" long long vtk_hyper_tree_grid_get_number_of_elements(vtkHyperTreeGrid* sself, int type) { return sself->GetNumberOfElements(type); }
extern "C" vtkHyperTreeGridNonOrientedCursor * vtkHyperTreeGridNonOrientedCursor_new () {return vtkHyperTreeGridNonOrientedCursor :: New () ;}
extern "C" void vtkHyperTreeGridNonOrientedCursor_destructor (vtkHyperTreeGridNonOrientedCursor * sself) {sself -> Delete () ; return ;}
extern "C" bool vtk_hyper_tree_grid_non_oriented_cursor_has_tree(vtkHyperTreeGridNonOrientedCursor* sself) { return sself->HasTree(); }
extern "C" long long vtk_hyper_tree_grid_non_oriented_cursor_get_vertex_id(vtkHyperTreeGridNonOrientedCursor* sself) { return sself->GetVertexId(); }
extern "C" long long vtk_hyper_tree_grid_non_oriented_cursor_get_global_node_index(vtkHyperTreeGridNonOrientedCursor* sself) { return sself->GetGlobalNodeIndex(); }
extern "C" unsigned char vtk_hyper_tree_grid_non_oriented_cursor_get_dimension(vtkHyperTreeGridNonOrientedCursor* sself) { return sself->GetDimension(); }
extern "C" unsigned char vtk_hyper_tree_grid_non_oriented_cursor_get_number_of_children(vtkHyperTreeGridNonOrientedCursor* sself) { return sself->GetNumberOfChildren(); }
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_set_global_index_start(vtkHyperTreeGridNonOrientedCursor* sself, long long index) { sself->SetGlobalIndexStart(index); }
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_set_global_index_from_local(vtkHyperTreeGridNonOrientedCursor* sself, long long index) { sself->SetGlobalIndexFromLocal(index); }
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_set_mask(vtkHyperTreeGridNonOrientedCursor* sself, bool state) { sself->SetMask(state); }
extern "C" bool vtk_hyper_tree_grid_non_oriented_cursor_is_masked(vtkHyperTreeGridNonOrientedCursor* sself) { return sself->IsMasked(); }
extern "C" bool vtk_hyper_tree_grid_non_oriented_cursor_is_leaf(vtkHyperTreeGridNonOrientedCursor* sself) { return sself->IsLeaf(); }
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_subdivide_leaf(vtkHyperTreeGridNonOrientedCursor* sself) { sself->SubdivideLeaf(); }
extern "C" bool vtk_hyper_tree_grid_non_oriented_cursor_is_root(vtkHyperTreeGridNonOrientedCursor* sself) { return sself->IsRoot(); }
extern "C" unsigned int vtk_hyper_tree_grid_non_oriented_cursor_get_level(vtkHyperTreeGridNonOrientedCursor* sself) { return sself->GetLevel(); }
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_to_child(vtkHyperTreeGridNonOrientedCursor* sself, unsigned char ichild) { sself->ToChild(ichild); }
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_to_root(vtkHyperTreeGridNonOrientedCursor* sself) { sself->ToRoot(); }
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_to_parent(vtkHyperTreeGridNonOrientedCursor* sself) { sself->ToParent(); }
extern "C" vtkHyperTreeGridNonOrientedGeometryCursor * vtkHyperTreeGridNonOrientedGeometryCursor_new () {return vtkHyperTreeGridNonOrientedGeometryCursor :: New () ;}
extern "C" void vtkHyperTreeGridNonOrientedGeometryCursor_destructor (vtkHyperTreeGridNonOrientedGeometryCursor * sself) {sself -> Delete () ; return ;}
extern "C" bool vtk_hyper_tree_grid_non_oriented_geometry_cursor_has_tree(vtkHyperTreeGridNonOrientedGeometryCursor* sself) { return sself->HasTree(); }
extern "C" long long vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_vertex_id(vtkHyperTreeGridNonOrientedGeometryCursor* sself) { return sself->GetVertexId(); }
extern "C" long long vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_global_node_index(vtkHyperTreeGridNonOrientedGeometryCursor* sself) { return sself->GetGlobalNodeIndex(); }
extern "C" unsigned char vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_dimension(vtkHyperTreeGridNonOrientedGeometryCursor* sself) { return sself->GetDimension(); }
extern "C" unsigned char vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_number_of_children(vtkHyperTreeGridNonOrientedGeometryCursor* sself) { return sself->GetNumberOfChildren(); }
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_set_global_index_start(vtkHyperTreeGridNonOrientedGeometryCursor* sself, long long index) { sself->SetGlobalIndexStart(index); }
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_set_global_index_from_local(vtkHyperTreeGridNonOrientedGeometryCursor* sself, long long index) { sself->SetGlobalIndexFromLocal(index); }
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_set_mask(vtkHyperTreeGridNonOrientedGeometryCursor* sself, bool state) { sself->SetMask(state); }
extern "C" bool vtk_hyper_tree_grid_non_oriented_geometry_cursor_is_masked(vtkHyperTreeGridNonOrientedGeometryCursor* sself) { return sself->IsMasked(); }
extern "C" bool vtk_hyper_tree_grid_non_oriented_geometry_cursor_is_leaf(vtkHyperTreeGridNonOrientedGeometryCursor* sself) { return sself->IsLeaf(); }
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_subdivide_leaf(vtkHyperTreeGridNonOrientedGeometryCursor* sself) { sself->SubdivideLeaf(); }
extern "C" bool vtk_hyper_tree_grid_non_oriented_geometry_cursor_is_root(vtkHyperTreeGridNonOrientedGeometryCursor* sself) { return sself->IsRoot(); }
extern "C" unsigned int vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_level(vtkHyperTreeGridNonOrientedGeometryCursor* sself) { return sself->GetLevel(); }
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_to_child(vtkHyperTreeGridNonOrientedGeometryCursor* sself, unsigned char ichild) { sself->ToChild(ichild); }
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_to_root(vtkHyperTreeGridNonOrientedGeometryCursor* sself) { sself->ToRoot(); }
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_to_parent(vtkHyperTreeGridNonOrientedGeometryCursor* sself) { sself->ToParent(); }
extern "C" vtkHyperTreeGridNonOrientedMooreSuperCursor * vtkHyperTreeGridNonOrientedMooreSuperCursor_new () {return vtkHyperTreeGridNonOrientedMooreSuperCursor :: New () ;}
extern "C" void vtkHyperTreeGridNonOrientedMooreSuperCursor_destructor (vtkHyperTreeGridNonOrientedMooreSuperCursor * sself) {sself -> Delete () ; return ;}
extern "C" vtkHyperTreeGridNonOrientedMooreSuperCursorLight * vtkHyperTreeGridNonOrientedMooreSuperCursorLight_new () {return vtkHyperTreeGridNonOrientedMooreSuperCursorLight :: New () ;}
extern "C" void vtkHyperTreeGridNonOrientedMooreSuperCursorLight_destructor (vtkHyperTreeGridNonOrientedMooreSuperCursorLight * sself) {sself -> Delete () ; return ;}
extern "C" vtkHyperTreeGridNonOrientedVonNeumannSuperCursor * vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_new () {return vtkHyperTreeGridNonOrientedVonNeumannSuperCursor :: New () ;}
extern "C" void vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_destructor (vtkHyperTreeGridNonOrientedVonNeumannSuperCursor * sself) {sself -> Delete () ; return ;}
extern "C" vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight * vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_new () {return vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight :: New () ;}
extern "C" void vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_destructor (vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight * sself) {sself -> Delete () ; return ;}
extern "C" vtkHyperTreeGridOrientedCursor * vtkHyperTreeGridOrientedCursor_new () {return vtkHyperTreeGridOrientedCursor :: New () ;}
extern "C" void vtkHyperTreeGridOrientedCursor_destructor (vtkHyperTreeGridOrientedCursor * sself) {sself -> Delete () ; return ;}
extern "C" bool vtk_hyper_tree_grid_oriented_cursor_has_tree(vtkHyperTreeGridOrientedCursor* sself) { return sself->HasTree(); }
extern "C" long long vtk_hyper_tree_grid_oriented_cursor_get_vertex_id(vtkHyperTreeGridOrientedCursor* sself) { return sself->GetVertexId(); }
extern "C" long long vtk_hyper_tree_grid_oriented_cursor_get_global_node_index(vtkHyperTreeGridOrientedCursor* sself) { return sself->GetGlobalNodeIndex(); }
extern "C" unsigned char vtk_hyper_tree_grid_oriented_cursor_get_dimension(vtkHyperTreeGridOrientedCursor* sself) { return sself->GetDimension(); }
extern "C" unsigned char vtk_hyper_tree_grid_oriented_cursor_get_number_of_children(vtkHyperTreeGridOrientedCursor* sself) { return sself->GetNumberOfChildren(); }
extern "C" void vtk_hyper_tree_grid_oriented_cursor_set_global_index_start(vtkHyperTreeGridOrientedCursor* sself, long long index) { sself->SetGlobalIndexStart(index); }
extern "C" void vtk_hyper_tree_grid_oriented_cursor_set_global_index_from_local(vtkHyperTreeGridOrientedCursor* sself, long long index) { sself->SetGlobalIndexFromLocal(index); }
extern "C" void vtk_hyper_tree_grid_oriented_cursor_set_mask(vtkHyperTreeGridOrientedCursor* sself, bool state) { sself->SetMask(state); }
extern "C" bool vtk_hyper_tree_grid_oriented_cursor_is_masked(vtkHyperTreeGridOrientedCursor* sself) { return sself->IsMasked(); }
extern "C" bool vtk_hyper_tree_grid_oriented_cursor_is_leaf(vtkHyperTreeGridOrientedCursor* sself) { return sself->IsLeaf(); }
extern "C" void vtk_hyper_tree_grid_oriented_cursor_subdivide_leaf(vtkHyperTreeGridOrientedCursor* sself) { sself->SubdivideLeaf(); }
extern "C" bool vtk_hyper_tree_grid_oriented_cursor_is_root(vtkHyperTreeGridOrientedCursor* sself) { return sself->IsRoot(); }
extern "C" unsigned int vtk_hyper_tree_grid_oriented_cursor_get_level(vtkHyperTreeGridOrientedCursor* sself) { return sself->GetLevel(); }
extern "C" void vtk_hyper_tree_grid_oriented_cursor_to_child(vtkHyperTreeGridOrientedCursor* sself, unsigned char ichild) { sself->ToChild(ichild); }
extern "C" vtkHyperTreeGridOrientedGeometryCursor * vtkHyperTreeGridOrientedGeometryCursor_new () {return vtkHyperTreeGridOrientedGeometryCursor :: New () ;}
extern "C" void vtkHyperTreeGridOrientedGeometryCursor_destructor (vtkHyperTreeGridOrientedGeometryCursor * sself) {sself -> Delete () ; return ;}
extern "C" bool vtk_hyper_tree_grid_oriented_geometry_cursor_has_tree(vtkHyperTreeGridOrientedGeometryCursor* sself) { return sself->HasTree(); }
extern "C" long long vtk_hyper_tree_grid_oriented_geometry_cursor_get_vertex_id(vtkHyperTreeGridOrientedGeometryCursor* sself) { return sself->GetVertexId(); }
extern "C" long long vtk_hyper_tree_grid_oriented_geometry_cursor_get_global_node_index(vtkHyperTreeGridOrientedGeometryCursor* sself) { return sself->GetGlobalNodeIndex(); }
extern "C" unsigned char vtk_hyper_tree_grid_oriented_geometry_cursor_get_dimension(vtkHyperTreeGridOrientedGeometryCursor* sself) { return sself->GetDimension(); }
extern "C" unsigned char vtk_hyper_tree_grid_oriented_geometry_cursor_get_number_of_children(vtkHyperTreeGridOrientedGeometryCursor* sself) { return sself->GetNumberOfChildren(); }
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_set_global_index_start(vtkHyperTreeGridOrientedGeometryCursor* sself, long long index) { sself->SetGlobalIndexStart(index); }
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_set_global_index_from_local(vtkHyperTreeGridOrientedGeometryCursor* sself, long long index) { sself->SetGlobalIndexFromLocal(index); }
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_set_mask(vtkHyperTreeGridOrientedGeometryCursor* sself, bool state) { sself->SetMask(state); }
extern "C" bool vtk_hyper_tree_grid_oriented_geometry_cursor_is_masked(vtkHyperTreeGridOrientedGeometryCursor* sself) { return sself->IsMasked(); }
extern "C" bool vtk_hyper_tree_grid_oriented_geometry_cursor_is_leaf(vtkHyperTreeGridOrientedGeometryCursor* sself) { return sself->IsLeaf(); }
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_subdivide_leaf(vtkHyperTreeGridOrientedGeometryCursor* sself) { sself->SubdivideLeaf(); }
extern "C" bool vtk_hyper_tree_grid_oriented_geometry_cursor_is_root(vtkHyperTreeGridOrientedGeometryCursor* sself) { return sself->IsRoot(); }
extern "C" unsigned int vtk_hyper_tree_grid_oriented_geometry_cursor_get_level(vtkHyperTreeGridOrientedGeometryCursor* sself) { return sself->GetLevel(); }
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_to_child(vtkHyperTreeGridOrientedGeometryCursor* sself, unsigned char ichild) { sself->ToChild(ichild); }
extern "C" vtkImageData * vtkImageData_new () {return vtkImageData :: New () ;}
extern "C" void vtkImageData_destructor (vtkImageData * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_image_data_get_data_object_type(vtkImageData* sself) { return sself->GetDataObjectType(); }
extern "C" long long vtk_image_data_get_number_of_cells(vtkImageData* sself) { return sself->GetNumberOfCells(); }
extern "C" long long vtk_image_data_get_number_of_points(vtkImageData* sself) { return sself->GetNumberOfPoints(); }
extern "C" long long vtk_image_data_find_point(vtkImageData* sself, double x, double y, double z) { return sself->FindPoint(x, y, z); }
extern "C" int vtk_image_data_get_cell_type(vtkImageData* sself, long long cellId) { return sself->GetCellType(cellId); }
extern "C" int vtk_image_data_get_max_cell_size(vtkImageData* sself) { return sself->GetMaxCellSize(); }
extern "C" void vtk_image_data_initialize(vtkImageData* sself) { sself->Initialize(); }
extern "C" unsigned char vtk_image_data_is_point_visible(vtkImageData* sself, long long ptId) { return sself->IsPointVisible(ptId); }
extern "C" unsigned char vtk_image_data_is_cell_visible(vtkImageData* sself, long long cellId) { return sself->IsCellVisible(cellId); }
extern "C" bool vtk_image_data_has_any_blank_points(vtkImageData* sself) { return sself->HasAnyBlankPoints(); }
extern "C" bool vtk_image_data_has_any_blank_cells(vtkImageData* sself) { return sself->HasAnyBlankCells(); }
extern "C" void vtk_image_data_set_dimensions(vtkImageData* sself, int i, int j, int k) { sself->SetDimensions(i, j, k); }
extern "C" int vtk_image_data_get_data_dimension(vtkImageData* sself) { return sself->GetDataDimension(); }
extern "C" void vtk_image_data_set_extent(vtkImageData* sself, int x1, int x2, int y1, int y2, int z1, int z2) { sself->SetExtent(x1, x2, y1, y2, z1, z2); }
extern "C" void* vtk_image_data_get_scalar_pointer(vtkImageData* sself, int x, int y, int z) { return sself->GetScalarPointer(x, y, z); }
extern "C" long long vtk_image_data_get_scalar_index(vtkImageData* sself, int x, int y, int z) { return sself->GetScalarIndex(x, y, z); }
extern "C" float vtk_image_data_get_scalar_component_as_float(vtkImageData* sself, int x, int y, int z, int component) { return sself->GetScalarComponentAsFloat(x, y, z, component); }
extern "C" void vtk_image_data_set_scalar_component_from_float(vtkImageData* sself, int x, int y, int z, int component, float v) { sself->SetScalarComponentFromFloat(x, y, z, component, v); }
extern "C" double vtk_image_data_get_scalar_component_as_double(vtkImageData* sself, int x, int y, int z, int component) { return sself->GetScalarComponentAsDouble(x, y, z, component); }
extern "C" void vtk_image_data_set_scalar_component_from_double(vtkImageData* sself, int x, int y, int z, int component, double v) { sself->SetScalarComponentFromDouble(x, y, z, component, v); }
extern "C" void vtk_image_data_allocate_scalars(vtkImageData* sself, int dataType, int numComponents) { sself->AllocateScalars(dataType, numComponents); }
extern "C" void vtk_image_data_set_spacing(vtkImageData* sself, double i, double j, double k) { sself->SetSpacing(i, j, k); }
extern "C" void vtk_image_data_set_origin(vtkImageData* sself, double i, double j, double k) { sself->SetOrigin(i, j, k); }
extern "C" const char* vtk_image_data_get_scalar_type_as_string(vtkImageData* sself) { return sself->GetScalarTypeAsString(); }
extern "C" void vtk_image_data_prepare_for_new_data(vtkImageData* sself) { sself->PrepareForNewData(); }
extern "C" int vtk_image_data_get_extent_type(vtkImageData* sself) { return sself->GetExtentType(); }
extern "C" vtkImageTransform * vtkImageTransform_new () {return vtkImageTransform :: New () ;}
extern "C" void vtkImageTransform_destructor (vtkImageTransform * sself) {sself -> Delete () ; return ;}
extern "C" vtkImplicitBoolean * vtkImplicitBoolean_new () {return vtkImplicitBoolean :: New () ;}
extern "C" void vtkImplicitBoolean_destructor (vtkImplicitBoolean * sself) {sself -> Delete () ; return ;}
extern "C" unsigned long vtk_implicit_boolean_get_m_time(vtkImplicitBoolean* sself) { return sself->GetMTime(); }
extern "C" void vtk_implicit_boolean_set_operation_type(vtkImplicitBoolean* sself, int _arg) { sself->SetOperationType(_arg); }
extern "C" int vtk_implicit_boolean_get_operation_type_min_value(vtkImplicitBoolean* sself) { return sself->GetOperationTypeMinValue(); }
extern "C" int vtk_implicit_boolean_get_operation_type_max_value(vtkImplicitBoolean* sself) { return sself->GetOperationTypeMaxValue(); }
extern "C" int vtk_implicit_boolean_get_operation_type(vtkImplicitBoolean* sself) { return sself->GetOperationType(); }
extern "C" void vtk_implicit_boolean_set_operation_type_to_union(vtkImplicitBoolean* sself) { sself->SetOperationTypeToUnion(); }
extern "C" void vtk_implicit_boolean_set_operation_type_to_intersection(vtkImplicitBoolean* sself) { sself->SetOperationTypeToIntersection(); }
extern "C" void vtk_implicit_boolean_set_operation_type_to_difference(vtkImplicitBoolean* sself) { sself->SetOperationTypeToDifference(); }
extern "C" void vtk_implicit_boolean_set_operation_type_to_union_of_magnitudes(vtkImplicitBoolean* sself) { sself->SetOperationTypeToUnionOfMagnitudes(); }
extern "C" const char* vtk_implicit_boolean_get_operation_type_as_string(vtkImplicitBoolean* sself) { return sself->GetOperationTypeAsString(); }
extern "C" vtkImplicitDataSet * vtkImplicitDataSet_new () {return vtkImplicitDataSet :: New () ;}
extern "C" void vtkImplicitDataSet_destructor (vtkImplicitDataSet * sself) {sself -> Delete () ; return ;}
extern "C" unsigned long vtk_implicit_data_set_get_m_time(vtkImplicitDataSet* sself) { return sself->GetMTime(); }
extern "C" void vtk_implicit_data_set_set_out_value(vtkImplicitDataSet* sself, double _arg) { sself->SetOutValue(_arg); }
extern "C" double vtk_implicit_data_set_get_out_value(vtkImplicitDataSet* sself) { return sself->GetOutValue(); }
extern "C" void vtk_implicit_data_set_set_out_gradient(vtkImplicitDataSet* sself, double _arg1, double _arg2, double _arg3) { sself->SetOutGradient(_arg1, _arg2, _arg3); }
extern "C" vtkImplicitFunctionCollection * vtkImplicitFunctionCollection_new () {return vtkImplicitFunctionCollection :: New () ;}
extern "C" void vtkImplicitFunctionCollection_destructor (vtkImplicitFunctionCollection * sself) {sself -> Delete () ; return ;}
extern "C" vtkImplicitHalo * vtkImplicitHalo_new () {return vtkImplicitHalo :: New () ;}
extern "C" void vtkImplicitHalo_destructor (vtkImplicitHalo * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_implicit_halo_set_radius(vtkImplicitHalo* sself, double _arg) { sself->SetRadius(_arg); }
extern "C" double vtk_implicit_halo_get_radius(vtkImplicitHalo* sself) { return sself->GetRadius(); }
extern "C" void vtk_implicit_halo_set_center(vtkImplicitHalo* sself, double _arg1, double _arg2, double _arg3) { sself->SetCenter(_arg1, _arg2, _arg3); }
extern "C" void vtk_implicit_halo_set_fade_out(vtkImplicitHalo* sself, double _arg) { sself->SetFadeOut(_arg); }
extern "C" double vtk_implicit_halo_get_fade_out(vtkImplicitHalo* sself) { return sself->GetFadeOut(); }
extern "C" vtkImplicitSelectionLoop * vtkImplicitSelectionLoop_new () {return vtkImplicitSelectionLoop :: New () ;}
extern "C" void vtkImplicitSelectionLoop_destructor (vtkImplicitSelectionLoop * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_implicit_selection_loop_set_automatic_normal_generation(vtkImplicitSelectionLoop* sself, int _arg) { sself->SetAutomaticNormalGeneration(_arg); }
extern "C" int vtk_implicit_selection_loop_get_automatic_normal_generation(vtkImplicitSelectionLoop* sself) { return sself->GetAutomaticNormalGeneration(); }
extern "C" void vtk_implicit_selection_loop_automatic_normal_generation_on(vtkImplicitSelectionLoop* sself) { sself->AutomaticNormalGenerationOn(); }
extern "C" void vtk_implicit_selection_loop_automatic_normal_generation_off(vtkImplicitSelectionLoop* sself) { sself->AutomaticNormalGenerationOff(); }
extern "C" void vtk_implicit_selection_loop_set_normal(vtkImplicitSelectionLoop* sself, double _arg1, double _arg2, double _arg3) { sself->SetNormal(_arg1, _arg2, _arg3); }
extern "C" unsigned long vtk_implicit_selection_loop_get_m_time(vtkImplicitSelectionLoop* sself) { return sself->GetMTime(); }
extern "C" vtkImplicitSum * vtkImplicitSum_new () {return vtkImplicitSum :: New () ;}
extern "C" void vtkImplicitSum_destructor (vtkImplicitSum * sself) {sself -> Delete () ; return ;}
extern "C" unsigned long vtk_implicit_sum_get_m_time(vtkImplicitSum* sself) { return sself->GetMTime(); }
extern "C" void vtk_implicit_sum_remove_all_functions(vtkImplicitSum* sself) { sself->RemoveAllFunctions(); }
extern "C" void vtk_implicit_sum_set_normalize_by_weight(vtkImplicitSum* sself, int _arg) { sself->SetNormalizeByWeight(_arg); }
extern "C" int vtk_implicit_sum_get_normalize_by_weight(vtkImplicitSum* sself) { return sself->GetNormalizeByWeight(); }
extern "C" void vtk_implicit_sum_normalize_by_weight_on(vtkImplicitSum* sself) { sself->NormalizeByWeightOn(); }
extern "C" void vtk_implicit_sum_normalize_by_weight_off(vtkImplicitSum* sself) { sself->NormalizeByWeightOff(); }
extern "C" vtkImplicitVolume * vtkImplicitVolume_new () {return vtkImplicitVolume :: New () ;}
extern "C" void vtkImplicitVolume_destructor (vtkImplicitVolume * sself) {sself -> Delete () ; return ;}
extern "C" unsigned long vtk_implicit_volume_get_m_time(vtkImplicitVolume* sself) { return sself->GetMTime(); }
extern "C" void vtk_implicit_volume_set_out_value(vtkImplicitVolume* sself, double _arg) { sself->SetOutValue(_arg); }
extern "C" double vtk_implicit_volume_get_out_value(vtkImplicitVolume* sself) { return sself->GetOutValue(); }
extern "C" void vtk_implicit_volume_set_out_gradient(vtkImplicitVolume* sself, double _arg1, double _arg2, double _arg3) { sself->SetOutGradient(_arg1, _arg2, _arg3); }
extern "C" vtkImplicitWindowFunction * vtkImplicitWindowFunction_new () {return vtkImplicitWindowFunction :: New () ;}
extern "C" void vtkImplicitWindowFunction_destructor (vtkImplicitWindowFunction * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_implicit_window_function_set_window_range(vtkImplicitWindowFunction* sself, double _arg1, double _arg2) { sself->SetWindowRange(_arg1, _arg2); }
extern "C" void vtk_implicit_window_function_set_window_values(vtkImplicitWindowFunction* sself, double _arg1, double _arg2) { sself->SetWindowValues(_arg1, _arg2); }
extern "C" unsigned long vtk_implicit_window_function_get_m_time(vtkImplicitWindowFunction* sself) { return sself->GetMTime(); }
extern "C" vtkInEdgeIterator * vtkInEdgeIterator_new () {return vtkInEdgeIterator :: New () ;}
extern "C" void vtkInEdgeIterator_destructor (vtkInEdgeIterator * sself) {sself -> Delete () ; return ;}
extern "C" long long vtk_in_edge_iterator_get_vertex(vtkInEdgeIterator* sself) { return sself->GetVertex(); }
extern "C" bool vtk_in_edge_iterator_has_next(vtkInEdgeIterator* sself) { return sself->HasNext(); }
extern "C" vtkIncrementalOctreeNode * vtkIncrementalOctreeNode_new () {return vtkIncrementalOctreeNode :: New () ;}
extern "C" void vtkIncrementalOctreeNode_destructor (vtkIncrementalOctreeNode * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_incremental_octree_node_get_number_of_points(vtkIncrementalOctreeNode* sself) { return sself->GetNumberOfPoints(); }
extern "C" void vtk_incremental_octree_node_delete_child_nodes(vtkIncrementalOctreeNode* sself) { sself->DeleteChildNodes(); }
extern "C" void vtk_incremental_octree_node_set_bounds(vtkIncrementalOctreeNode* sself, double x1, double x2, double y1, double y2, double z1, double z2) { sself->SetBounds(x1, x2, y1, y2, z1, z2); }
extern "C" int vtk_incremental_octree_node_is_leaf(vtkIncrementalOctreeNode* sself) { return sself->IsLeaf(); }
extern "C" int vtk_incremental_octree_node_get_number_of_levels(vtkIncrementalOctreeNode* sself) { return sself->GetNumberOfLevels(); }
extern "C" int vtk_incremental_octree_node_get_id(vtkIncrementalOctreeNode* sself) { return sself->GetID(); }
extern "C" vtkIncrementalOctreePointLocator * vtkIncrementalOctreePointLocator_new () {return vtkIncrementalOctreePointLocator :: New () ;}
extern "C" void vtkIncrementalOctreePointLocator_destructor (vtkIncrementalOctreePointLocator * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_incremental_octree_point_locator_set_max_points_per_leaf(vtkIncrementalOctreePointLocator* sself, int _arg) { sself->SetMaxPointsPerLeaf(_arg); }
extern "C" int vtk_incremental_octree_point_locator_get_max_points_per_leaf_min_value(vtkIncrementalOctreePointLocator* sself) { return sself->GetMaxPointsPerLeafMinValue(); }
extern "C" int vtk_incremental_octree_point_locator_get_max_points_per_leaf_max_value(vtkIncrementalOctreePointLocator* sself) { return sself->GetMaxPointsPerLeafMaxValue(); }
extern "C" int vtk_incremental_octree_point_locator_get_max_points_per_leaf(vtkIncrementalOctreePointLocator* sself) { return sself->GetMaxPointsPerLeaf(); }
extern "C" void vtk_incremental_octree_point_locator_set_build_cubic_octree(vtkIncrementalOctreePointLocator* sself, int _arg) { sself->SetBuildCubicOctree(_arg); }
extern "C" int vtk_incremental_octree_point_locator_get_build_cubic_octree(vtkIncrementalOctreePointLocator* sself) { return sself->GetBuildCubicOctree(); }
extern "C" void vtk_incremental_octree_point_locator_build_cubic_octree_on(vtkIncrementalOctreePointLocator* sself) { sself->BuildCubicOctreeOn(); }
extern "C" void vtk_incremental_octree_point_locator_build_cubic_octree_off(vtkIncrementalOctreePointLocator* sself) { sself->BuildCubicOctreeOff(); }
extern "C" void vtk_incremental_octree_point_locator_initialize(vtkIncrementalOctreePointLocator* sself) { sself->Initialize(); }
extern "C" void vtk_incremental_octree_point_locator_free_search_structure(vtkIncrementalOctreePointLocator* sself) { sself->FreeSearchStructure(); }
extern "C" int vtk_incremental_octree_point_locator_get_number_of_points(vtkIncrementalOctreePointLocator* sself) { return sself->GetNumberOfPoints(); }
extern "C" int vtk_incremental_octree_point_locator_get_number_of_nodes(vtkIncrementalOctreePointLocator* sself) { return sself->GetNumberOfNodes(); }
extern "C" void vtk_incremental_octree_point_locator_build_locator(vtkIncrementalOctreePointLocator* sself) { sself->BuildLocator(); }
extern "C" long long vtk_incremental_octree_point_locator_find_closest_point(vtkIncrementalOctreePointLocator* sself, double x, double y, double z) { return sself->FindClosestPoint(x, y, z); }
extern "C" long long vtk_incremental_octree_point_locator_is_inserted_point(vtkIncrementalOctreePointLocator* sself, double x, double y, double z) { return sself->IsInsertedPoint(x, y, z); }
extern "C" int vtk_incremental_octree_point_locator_get_number_of_levels(vtkIncrementalOctreePointLocator* sself) { return sself->GetNumberOfLevels(); }
extern "C" vtkIterativeClosestPointTransform * vtkIterativeClosestPointTransform_new () {return vtkIterativeClosestPointTransform :: New () ;}
extern "C" void vtkIterativeClosestPointTransform_destructor (vtkIterativeClosestPointTransform * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_iterative_closest_point_transform_set_maximum_number_of_iterations(vtkIterativeClosestPointTransform* sself, int _arg) { sself->SetMaximumNumberOfIterations(_arg); }
extern "C" int vtk_iterative_closest_point_transform_get_maximum_number_of_iterations(vtkIterativeClosestPointTransform* sself) { return sself->GetMaximumNumberOfIterations(); }
extern "C" int vtk_iterative_closest_point_transform_get_number_of_iterations(vtkIterativeClosestPointTransform* sself) { return sself->GetNumberOfIterations(); }
extern "C" void vtk_iterative_closest_point_transform_set_check_mean_distance(vtkIterativeClosestPointTransform* sself, int _arg) { sself->SetCheckMeanDistance(_arg); }
extern "C" int vtk_iterative_closest_point_transform_get_check_mean_distance(vtkIterativeClosestPointTransform* sself) { return sself->GetCheckMeanDistance(); }
extern "C" void vtk_iterative_closest_point_transform_check_mean_distance_on(vtkIterativeClosestPointTransform* sself) { sself->CheckMeanDistanceOn(); }
extern "C" void vtk_iterative_closest_point_transform_check_mean_distance_off(vtkIterativeClosestPointTransform* sself) { sself->CheckMeanDistanceOff(); }
extern "C" void vtk_iterative_closest_point_transform_set_mean_distance_mode(vtkIterativeClosestPointTransform* sself, int _arg) { sself->SetMeanDistanceMode(_arg); }
extern "C" int vtk_iterative_closest_point_transform_get_mean_distance_mode_min_value(vtkIterativeClosestPointTransform* sself) { return sself->GetMeanDistanceModeMinValue(); }
extern "C" int vtk_iterative_closest_point_transform_get_mean_distance_mode_max_value(vtkIterativeClosestPointTransform* sself) { return sself->GetMeanDistanceModeMaxValue(); }
extern "C" int vtk_iterative_closest_point_transform_get_mean_distance_mode(vtkIterativeClosestPointTransform* sself) { return sself->GetMeanDistanceMode(); }
extern "C" void vtk_iterative_closest_point_transform_set_mean_distance_mode_to_rms(vtkIterativeClosestPointTransform* sself) { sself->SetMeanDistanceModeToRMS(); }
extern "C" void vtk_iterative_closest_point_transform_set_mean_distance_mode_to_absolute_value(vtkIterativeClosestPointTransform* sself) { sself->SetMeanDistanceModeToAbsoluteValue(); }
extern "C" const char* vtk_iterative_closest_point_transform_get_mean_distance_mode_as_string(vtkIterativeClosestPointTransform* sself) { return sself->GetMeanDistanceModeAsString(); }
extern "C" void vtk_iterative_closest_point_transform_set_maximum_mean_distance(vtkIterativeClosestPointTransform* sself, double _arg) { sself->SetMaximumMeanDistance(_arg); }
extern "C" double vtk_iterative_closest_point_transform_get_maximum_mean_distance(vtkIterativeClosestPointTransform* sself) { return sself->GetMaximumMeanDistance(); }
extern "C" double vtk_iterative_closest_point_transform_get_mean_distance(vtkIterativeClosestPointTransform* sself) { return sself->GetMeanDistance(); }
extern "C" void vtk_iterative_closest_point_transform_set_maximum_number_of_landmarks(vtkIterativeClosestPointTransform* sself, int _arg) { sself->SetMaximumNumberOfLandmarks(_arg); }
extern "C" int vtk_iterative_closest_point_transform_get_maximum_number_of_landmarks(vtkIterativeClosestPointTransform* sself) { return sself->GetMaximumNumberOfLandmarks(); }
extern "C" void vtk_iterative_closest_point_transform_set_start_by_matching_centroids(vtkIterativeClosestPointTransform* sself, int _arg) { sself->SetStartByMatchingCentroids(_arg); }
extern "C" int vtk_iterative_closest_point_transform_get_start_by_matching_centroids(vtkIterativeClosestPointTransform* sself) { return sself->GetStartByMatchingCentroids(); }
extern "C" void vtk_iterative_closest_point_transform_start_by_matching_centroids_on(vtkIterativeClosestPointTransform* sself) { sself->StartByMatchingCentroidsOn(); }
extern "C" void vtk_iterative_closest_point_transform_start_by_matching_centroids_off(vtkIterativeClosestPointTransform* sself) { sself->StartByMatchingCentroidsOff(); }
extern "C" void vtk_iterative_closest_point_transform_inverse(vtkIterativeClosestPointTransform* sself) { sself->Inverse(); }
extern "C" vtkKdNode * vtkKdNode_new () {return vtkKdNode :: New () ;}
extern "C" void vtkKdNode_destructor (vtkKdNode * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_kd_node_set_dim(vtkKdNode* sself, int _arg) { sself->SetDim(_arg); }
extern "C" int vtk_kd_node_get_dim(vtkKdNode* sself) { return sself->GetDim(); }
extern "C" double vtk_kd_node_get_division_position(vtkKdNode* sself) { return sself->GetDivisionPosition(); }
extern "C" void vtk_kd_node_set_number_of_points(vtkKdNode* sself, int _arg) { sself->SetNumberOfPoints(_arg); }
extern "C" int vtk_kd_node_get_number_of_points(vtkKdNode* sself) { return sself->GetNumberOfPoints(); }
extern "C" void vtk_kd_node_set_bounds(vtkKdNode* sself, double x1, double x2, double y1, double y2, double z1, double z2) { sself->SetBounds(x1, x2, y1, y2, z1, z2); }
extern "C" void vtk_kd_node_set_data_bounds(vtkKdNode* sself, double x1, double x2, double y1, double y2, double z1, double z2) { sself->SetDataBounds(x1, x2, y1, y2, z1, z2); }
extern "C" void vtk_kd_node_set_id(vtkKdNode* sself, int _arg) { sself->SetID(_arg); }
extern "C" int vtk_kd_node_get_id(vtkKdNode* sself) { return sself->GetID(); }
extern "C" int vtk_kd_node_get_min_id(vtkKdNode* sself) { return sself->GetMinID(); }
extern "C" int vtk_kd_node_get_max_id(vtkKdNode* sself) { return sself->GetMaxID(); }
extern "C" void vtk_kd_node_set_min_id(vtkKdNode* sself, int _arg) { sself->SetMinID(_arg); }
extern "C" void vtk_kd_node_set_max_id(vtkKdNode* sself, int _arg) { sself->SetMaxID(_arg); }
extern "C" void vtk_kd_node_delete_child_nodes(vtkKdNode* sself) { sself->DeleteChildNodes(); }
extern "C" int vtk_kd_node_intersects_box(vtkKdNode* sself, double x1, double x2, double y1, double y2, double z1, double z2, int useDataBounds) { return sself->IntersectsBox(x1, x2, y1, y2, z1, z2, useDataBounds); }
extern "C" int vtk_kd_node_intersects_sphere_2(vtkKdNode* sself, double x, double y, double z, double rSquared, int useDataBounds) { return sself->IntersectsSphere2(x, y, z, rSquared, useDataBounds); }
extern "C" int vtk_kd_node_contains_box(vtkKdNode* sself, double x1, double x2, double y1, double y2, double z1, double z2, int useDataBounds) { return sself->ContainsBox(x1, x2, y1, y2, z1, z2, useDataBounds); }
extern "C" int vtk_kd_node_contains_point(vtkKdNode* sself, double x, double y, double z, int useDataBounds) { return sself->ContainsPoint(x, y, z, useDataBounds); }
extern "C" double vtk_kd_node_get_distance_2_to_boundary(vtkKdNode* sself, double x, double y, double z, int useDataBounds) { return sself->GetDistance2ToBoundary(x, y, z, useDataBounds); }
extern "C" double vtk_kd_node_get_distance_2_to_inner_boundary(vtkKdNode* sself, double x, double y, double z) { return sself->GetDistance2ToInnerBoundary(x, y, z); }
extern "C" void vtk_kd_node_print_node(vtkKdNode* sself, int depth) { sself->PrintNode(depth); }
extern "C" void vtk_kd_node_print_verbose_node(vtkKdNode* sself, int depth) { sself->PrintVerboseNode(depth); }
extern "C" vtkKdTree * vtkKdTree_new () {return vtkKdTree :: New () ;}
extern "C" void vtkKdTree_destructor (vtkKdTree * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_kd_tree_timing_on(vtkKdTree* sself) { sself->TimingOn(); }
extern "C" void vtk_kd_tree_timing_off(vtkKdTree* sself) { sself->TimingOff(); }
extern "C" void vtk_kd_tree_set_timing(vtkKdTree* sself, int _arg) { sself->SetTiming(_arg); }
extern "C" int vtk_kd_tree_get_timing(vtkKdTree* sself) { return sself->GetTiming(); }
extern "C" void vtk_kd_tree_set_min_cells(vtkKdTree* sself, int _arg) { sself->SetMinCells(_arg); }
extern "C" int vtk_kd_tree_get_min_cells(vtkKdTree* sself) { return sself->GetMinCells(); }
extern "C" int vtk_kd_tree_get_number_of_regions_or_less(vtkKdTree* sself) { return sself->GetNumberOfRegionsOrLess(); }
extern "C" void vtk_kd_tree_set_number_of_regions_or_less(vtkKdTree* sself, int _arg) { sself->SetNumberOfRegionsOrLess(_arg); }
extern "C" int vtk_kd_tree_get_number_of_regions_or_more(vtkKdTree* sself) { return sself->GetNumberOfRegionsOrMore(); }
extern "C" void vtk_kd_tree_set_number_of_regions_or_more(vtkKdTree* sself, int _arg) { sself->SetNumberOfRegionsOrMore(_arg); }
extern "C" double vtk_kd_tree_get_fudge_factor(vtkKdTree* sself) { return sself->GetFudgeFactor(); }
extern "C" void vtk_kd_tree_set_fudge_factor(vtkKdTree* sself, double _arg) { sself->SetFudgeFactor(_arg); }
extern "C" void vtk_kd_tree_omit_x_partitioning(vtkKdTree* sself) { sself->OmitXPartitioning(); }
extern "C" void vtk_kd_tree_omit_y_partitioning(vtkKdTree* sself) { sself->OmitYPartitioning(); }
extern "C" void vtk_kd_tree_omit_z_partitioning(vtkKdTree* sself) { sself->OmitZPartitioning(); }
extern "C" void vtk_kd_tree_omit_xy_partitioning(vtkKdTree* sself) { sself->OmitXYPartitioning(); }
extern "C" void vtk_kd_tree_omit_yz_partitioning(vtkKdTree* sself) { sself->OmitYZPartitioning(); }
extern "C" void vtk_kd_tree_omit_zx_partitioning(vtkKdTree* sself) { sself->OmitZXPartitioning(); }
extern "C" void vtk_kd_tree_omit_no_partitioning(vtkKdTree* sself) { sself->OmitNoPartitioning(); }
extern "C" void vtk_kd_tree_remove_data_set(vtkKdTree* sself, int index) { sself->RemoveDataSet(index); }
extern "C" void vtk_kd_tree_remove_all_data_sets(vtkKdTree* sself) { sself->RemoveAllDataSets(); }
extern "C" int vtk_kd_tree_get_number_of_data_sets(vtkKdTree* sself) { return sself->GetNumberOfDataSets(); }
extern "C" int vtk_kd_tree_get_number_of_regions(vtkKdTree* sself) { return sself->GetNumberOfRegions(); }
extern "C" void vtk_kd_tree_print_tree(vtkKdTree* sself) { sself->PrintTree(); }
extern "C" void vtk_kd_tree_print_verbose_tree(vtkKdTree* sself) { sself->PrintVerboseTree(); }
extern "C" void vtk_kd_tree_print_region(vtkKdTree* sself, int id) { sself->PrintRegion(id); }
extern "C" void vtk_kd_tree_set_include_region_boundary_cells(vtkKdTree* sself, int _arg) { sself->SetIncludeRegionBoundaryCells(_arg); }
extern "C" int vtk_kd_tree_get_include_region_boundary_cells(vtkKdTree* sself) { return sself->GetIncludeRegionBoundaryCells(); }
extern "C" void vtk_kd_tree_include_region_boundary_cells_on(vtkKdTree* sself) { sself->IncludeRegionBoundaryCellsOn(); }
extern "C" void vtk_kd_tree_include_region_boundary_cells_off(vtkKdTree* sself) { sself->IncludeRegionBoundaryCellsOff(); }
extern "C" void vtk_kd_tree_delete_cell_lists(vtkKdTree* sself) { sself->DeleteCellLists(); }
extern "C" int vtk_kd_tree_get_region_containing_point(vtkKdTree* sself, double x, double y, double z) { return sself->GetRegionContainingPoint(x, y, z); }
extern "C" void vtk_kd_tree_build_locator(vtkKdTree* sself) { sself->BuildLocator(); }
extern "C" void vtk_kd_tree_free_search_structure(vtkKdTree* sself) { sself->FreeSearchStructure(); }
extern "C" void vtk_kd_tree_generate_representation_using_data_bounds_on(vtkKdTree* sself) { sself->GenerateRepresentationUsingDataBoundsOn(); }
extern "C" void vtk_kd_tree_generate_representation_using_data_bounds_off(vtkKdTree* sself) { sself->GenerateRepresentationUsingDataBoundsOff(); }
extern "C" void vtk_kd_tree_set_generate_representation_using_data_bounds(vtkKdTree* sself, int _arg) { sself->SetGenerateRepresentationUsingDataBounds(_arg); }
extern "C" int vtk_kd_tree_get_generate_representation_using_data_bounds(vtkKdTree* sself) { return sself->GetGenerateRepresentationUsingDataBounds(); }
extern "C" int vtk_kd_tree_new_geometry(vtkKdTree* sself) { return sself->NewGeometry(); }
extern "C" void vtk_kd_tree_invalidate_geometry(vtkKdTree* sself) { sself->InvalidateGeometry(); }
extern "C" vtkKdTreePointLocator * vtkKdTreePointLocator_new () {return vtkKdTreePointLocator :: New () ;}
extern "C" void vtkKdTreePointLocator_destructor (vtkKdTreePointLocator * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_kd_tree_point_locator_free_search_structure(vtkKdTreePointLocator* sself) { sself->FreeSearchStructure(); }
extern "C" void vtk_kd_tree_point_locator_build_locator(vtkKdTreePointLocator* sself) { sself->BuildLocator(); }
extern "C" vtkLagrangeCurve * vtkLagrangeCurve_new () {return vtkLagrangeCurve :: New () ;}
extern "C" void vtkLagrangeCurve_destructor (vtkLagrangeCurve * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_lagrange_curve_get_cell_type(vtkLagrangeCurve* sself) { return sself->GetCellType(); }
extern "C" vtkLagrangeHexahedron * vtkLagrangeHexahedron_new () {return vtkLagrangeHexahedron :: New () ;}
extern "C" void vtkLagrangeHexahedron_destructor (vtkLagrangeHexahedron * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_lagrange_hexahedron_get_cell_type(vtkLagrangeHexahedron* sself) { return sself->GetCellType(); }
extern "C" vtkLagrangeInterpolation * vtkLagrangeInterpolation_new () {return vtkLagrangeInterpolation :: New () ;}
extern "C" void vtkLagrangeInterpolation_destructor (vtkLagrangeInterpolation * sself) {sself -> Delete () ; return ;}
extern "C" vtkLagrangeQuadrilateral * vtkLagrangeQuadrilateral_new () {return vtkLagrangeQuadrilateral :: New () ;}
extern "C" void vtkLagrangeQuadrilateral_destructor (vtkLagrangeQuadrilateral * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_lagrange_quadrilateral_get_cell_type(vtkLagrangeQuadrilateral* sself) { return sself->GetCellType(); }
extern "C" vtkLagrangeTetra * vtkLagrangeTetra_new () {return vtkLagrangeTetra :: New () ;}
extern "C" void vtkLagrangeTetra_destructor (vtkLagrangeTetra * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_lagrange_tetra_get_cell_type(vtkLagrangeTetra* sself) { return sself->GetCellType(); }
extern "C" vtkLagrangeTriangle * vtkLagrangeTriangle_new () {return vtkLagrangeTriangle :: New () ;}
extern "C" void vtkLagrangeTriangle_destructor (vtkLagrangeTriangle * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_lagrange_triangle_get_cell_type(vtkLagrangeTriangle* sself) { return sself->GetCellType(); }
extern "C" vtkLagrangeWedge * vtkLagrangeWedge_new () {return vtkLagrangeWedge :: New () ;}
extern "C" void vtkLagrangeWedge_destructor (vtkLagrangeWedge * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_lagrange_wedge_get_cell_type(vtkLagrangeWedge* sself) { return sself->GetCellType(); }
extern "C" vtkLine * vtkLine_new () {return vtkLine :: New () ;}
extern "C" void vtkLine_destructor (vtkLine * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_line_get_cell_type(vtkLine* sself) { return sself->GetCellType(); }
extern "C" int vtk_line_get_cell_dimension(vtkLine* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_line_get_number_of_edges(vtkLine* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_line_get_number_of_faces(vtkLine* sself) { return sself->GetNumberOfFaces(); }
extern "C" int vtk_line_inflate(vtkLine* sself, double dist) { return sself->Inflate(dist); }
extern "C" vtkMeanValueCoordinatesInterpolator * vtkMeanValueCoordinatesInterpolator_new () {return vtkMeanValueCoordinatesInterpolator :: New () ;}
extern "C" void vtkMeanValueCoordinatesInterpolator_destructor (vtkMeanValueCoordinatesInterpolator * sself) {sself -> Delete () ; return ;}
extern "C" vtkMergePoints * vtkMergePoints_new () {return vtkMergePoints :: New () ;}
extern "C" void vtkMergePoints_destructor (vtkMergePoints * sself) {sself -> Delete () ; return ;}
extern "C" vtkMolecule * vtkMolecule_new () {return vtkMolecule :: New () ;}
extern "C" void vtkMolecule_destructor (vtkMolecule * sself) {sself -> Delete () ; return ;}
extern "C" long long vtk_molecule_get_number_of_atoms(vtkMolecule* sself) { return sself->GetNumberOfAtoms(); }
extern "C" long long vtk_molecule_get_number_of_bonds(vtkMolecule* sself) { return sself->GetNumberOfBonds(); }
extern "C" unsigned short vtk_molecule_get_atom_atomic_number(vtkMolecule* sself, long long atomId) { return sself->GetAtomAtomicNumber(atomId); }
extern "C" void vtk_molecule_set_atom_atomic_number(vtkMolecule* sself, long long atomId, unsigned short atomicNum) { sself->SetAtomAtomicNumber(atomId, atomicNum); }
extern "C" void vtk_molecule_set_bond_order(vtkMolecule* sself, long long bondId, unsigned short order) { sself->SetBondOrder(bondId, order); }
extern "C" unsigned short vtk_molecule_get_bond_order(vtkMolecule* sself, long long bondId) { return sself->GetBondOrder(bondId); }
extern "C" double vtk_molecule_get_bond_length(vtkMolecule* sself, long long bondId) { return sself->GetBondLength(bondId); }
extern "C" bool vtk_molecule_has_lattice(vtkMolecule* sself) { return sself->HasLattice(); }
extern "C" void vtk_molecule_clear_lattice(vtkMolecule* sself) { sself->ClearLattice(); }
extern "C" void vtk_molecule_allocate_atom_ghost_array(vtkMolecule* sself) { sself->AllocateAtomGhostArray(); }
extern "C" void vtk_molecule_allocate_bond_ghost_array(vtkMolecule* sself) { sself->AllocateBondGhostArray(); }
extern "C" long long vtk_molecule_get_bond_id(vtkMolecule* sself, long long a, long long b) { return sself->GetBondId(a, b); }
extern "C" void vtk_molecule_set_atomic_number_array_name(vtkMolecule* sself, const char* _arg) { sself->SetAtomicNumberArrayName(_arg); }
extern "C" void vtk_molecule_set_bond_orders_array_name(vtkMolecule* sself, const char* _arg) { sself->SetBondOrdersArrayName(_arg); }
extern "C" vtkMultiBlockDataSet * vtkMultiBlockDataSet_new () {return vtkMultiBlockDataSet :: New () ;}
extern "C" void vtkMultiBlockDataSet_destructor (vtkMultiBlockDataSet * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_multi_block_data_set_set_number_of_blocks(vtkMultiBlockDataSet* sself, unsigned int numBlocks) { sself->SetNumberOfBlocks(numBlocks); }
extern "C" unsigned int vtk_multi_block_data_set_get_number_of_blocks(vtkMultiBlockDataSet* sself) { return sself->GetNumberOfBlocks(); }
extern "C" void vtk_multi_block_data_set_remove_block(vtkMultiBlockDataSet* sself, unsigned int blockno) { sself->RemoveBlock(blockno); }
extern "C" int vtk_multi_block_data_set_has_meta_data(vtkMultiBlockDataSet* sself, unsigned int blockno) { return sself->HasMetaData(blockno); }
extern "C" vtkMultiPieceDataSet * vtkMultiPieceDataSet_new () {return vtkMultiPieceDataSet :: New () ;}
extern "C" void vtkMultiPieceDataSet_destructor (vtkMultiPieceDataSet * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_multi_piece_data_set_set_number_of_pieces(vtkMultiPieceDataSet* sself, unsigned int numpieces) { sself->SetNumberOfPieces(numpieces); }
extern "C" unsigned int vtk_multi_piece_data_set_get_number_of_pieces(vtkMultiPieceDataSet* sself) { return sself->GetNumberOfPieces(); }
extern "C" vtkMutableDirectedGraph * vtkMutableDirectedGraph_new () {return vtkMutableDirectedGraph :: New () ;}
extern "C" void vtkMutableDirectedGraph_destructor (vtkMutableDirectedGraph * sself) {sself -> Delete () ; return ;}
extern "C" long long vtk_mutable_directed_graph_set_number_of_vertices(vtkMutableDirectedGraph* sself, long long numVerts) { return sself->SetNumberOfVertices(numVerts); }
extern "C" long long vtk_mutable_directed_graph_add_vertex(vtkMutableDirectedGraph* sself) { return sself->AddVertex(); }
extern "C" void vtk_mutable_directed_graph_lazy_add_vertex(vtkMutableDirectedGraph* sself) { sself->LazyAddVertex(); }
extern "C" void vtk_mutable_directed_graph_remove_vertex(vtkMutableDirectedGraph* sself, long long v) { sself->RemoveVertex(v); }
extern "C" void vtk_mutable_directed_graph_remove_edge(vtkMutableDirectedGraph* sself, long long e) { sself->RemoveEdge(e); }
extern "C" vtkMutableUndirectedGraph * vtkMutableUndirectedGraph_new () {return vtkMutableUndirectedGraph :: New () ;}
extern "C" void vtkMutableUndirectedGraph_destructor (vtkMutableUndirectedGraph * sself) {sself -> Delete () ; return ;}
extern "C" long long vtk_mutable_undirected_graph_set_number_of_vertices(vtkMutableUndirectedGraph* sself, long long numVerts) { return sself->SetNumberOfVertices(numVerts); }
extern "C" long long vtk_mutable_undirected_graph_add_vertex(vtkMutableUndirectedGraph* sself) { return sself->AddVertex(); }
extern "C" void vtk_mutable_undirected_graph_lazy_add_vertex(vtkMutableUndirectedGraph* sself) { sself->LazyAddVertex(); }
extern "C" void vtk_mutable_undirected_graph_lazy_add_edge(vtkMutableUndirectedGraph* sself, long long u, long long v) { sself->LazyAddEdge(u, v); }
extern "C" void vtk_mutable_undirected_graph_remove_vertex(vtkMutableUndirectedGraph* sself, long long v) { sself->RemoveVertex(v); }
extern "C" void vtk_mutable_undirected_graph_remove_edge(vtkMutableUndirectedGraph* sself, long long e) { sself->RemoveEdge(e); }
extern "C" vtkNonMergingPointLocator * vtkNonMergingPointLocator_new () {return vtkNonMergingPointLocator :: New () ;}
extern "C" void vtkNonMergingPointLocator_destructor (vtkNonMergingPointLocator * sself) {sself -> Delete () ; return ;}
extern "C" long long vtk_non_merging_point_locator_is_inserted_point(vtkNonMergingPointLocator* sself, double p0, double p1, double p2) { return sself->IsInsertedPoint(p0, p1, p2); }
extern "C" vtkNonOverlappingAMR * vtkNonOverlappingAMR_new () {return vtkNonOverlappingAMR :: New () ;}
extern "C" void vtkNonOverlappingAMR_destructor (vtkNonOverlappingAMR * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_non_overlapping_amr_get_data_object_type(vtkNonOverlappingAMR* sself) { return sself->GetDataObjectType(); }
extern "C" vtkOctreePointLocator * vtkOctreePointLocator_new () {return vtkOctreePointLocator :: New () ;}
extern "C" void vtkOctreePointLocator_destructor (vtkOctreePointLocator * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_octree_point_locator_set_maximum_points_per_region(vtkOctreePointLocator* sself, int _arg) { sself->SetMaximumPointsPerRegion(_arg); }
extern "C" int vtk_octree_point_locator_get_maximum_points_per_region(vtkOctreePointLocator* sself) { return sself->GetMaximumPointsPerRegion(); }
extern "C" void vtk_octree_point_locator_set_create_cubic_octants(vtkOctreePointLocator* sself, int _arg) { sself->SetCreateCubicOctants(_arg); }
extern "C" int vtk_octree_point_locator_get_create_cubic_octants(vtkOctreePointLocator* sself) { return sself->GetCreateCubicOctants(); }
extern "C" double vtk_octree_point_locator_get_fudge_factor(vtkOctreePointLocator* sself) { return sself->GetFudgeFactor(); }
extern "C" void vtk_octree_point_locator_set_fudge_factor(vtkOctreePointLocator* sself, double _arg) { sself->SetFudgeFactor(_arg); }
extern "C" int vtk_octree_point_locator_get_number_of_leaf_nodes(vtkOctreePointLocator* sself) { return sself->GetNumberOfLeafNodes(); }
extern "C" int vtk_octree_point_locator_get_region_containing_point(vtkOctreePointLocator* sself, double x, double y, double z) { return sself->GetRegionContainingPoint(x, y, z); }
extern "C" void vtk_octree_point_locator_build_locator(vtkOctreePointLocator* sself) { sself->BuildLocator(); }
extern "C" long long vtk_octree_point_locator_find_closest_point(vtkOctreePointLocator* sself, double x, double y, double z, double& dist2) { return sself->FindClosestPoint(x, y, z, dist2); }
extern "C" void vtk_octree_point_locator_free_search_structure(vtkOctreePointLocator* sself) { sself->FreeSearchStructure(); }
extern "C" vtkOctreePointLocatorNode * vtkOctreePointLocatorNode_new () {return vtkOctreePointLocatorNode :: New () ;}
extern "C" void vtkOctreePointLocatorNode_destructor (vtkOctreePointLocatorNode * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_octree_point_locator_node_set_number_of_points(vtkOctreePointLocatorNode* sself, int numberOfPoints) { sself->SetNumberOfPoints(numberOfPoints); }
extern "C" int vtk_octree_point_locator_node_get_number_of_points(vtkOctreePointLocatorNode* sself) { return sself->GetNumberOfPoints(); }
extern "C" void vtk_octree_point_locator_node_set_bounds(vtkOctreePointLocatorNode* sself, double xMin, double xMax, double yMin, double yMax, double zMin, double zMax) { sself->SetBounds(xMin, xMax, yMin, yMax, zMin, zMax); }
extern "C" void vtk_octree_point_locator_node_set_data_bounds(vtkOctreePointLocatorNode* sself, double xMin, double xMax, double yMin, double yMax, double zMin, double zMax) { sself->SetDataBounds(xMin, xMax, yMin, yMax, zMin, zMax); }
extern "C" int vtk_octree_point_locator_node_get_id(vtkOctreePointLocatorNode* sself) { return sself->GetID(); }
extern "C" int vtk_octree_point_locator_node_get_min_id(vtkOctreePointLocatorNode* sself) { return sself->GetMinID(); }
extern "C" void vtk_octree_point_locator_node_create_child_nodes(vtkOctreePointLocatorNode* sself) { sself->CreateChildNodes(); }
extern "C" void vtk_octree_point_locator_node_delete_child_nodes(vtkOctreePointLocatorNode* sself) { sself->DeleteChildNodes(); }
extern "C" int vtk_octree_point_locator_node_contains_point(vtkOctreePointLocatorNode* sself, double x, double y, double z, int useDataBounds) { return sself->ContainsPoint(x, y, z, useDataBounds); }
extern "C" vtkOrderedTriangulator * vtkOrderedTriangulator_new () {return vtkOrderedTriangulator :: New () ;}
extern "C" void vtkOrderedTriangulator_destructor (vtkOrderedTriangulator * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_ordered_triangulator_init_triangulation(vtkOrderedTriangulator* sself, double xmin, double xmax, double ymin, double ymax, double zmin, double zmax, int numPts) { sself->InitTriangulation(xmin, xmax, ymin, ymax, zmin, zmax, numPts); }
extern "C" void vtk_ordered_triangulator_triangulate(vtkOrderedTriangulator* sself) { sself->Triangulate(); }
extern "C" void vtk_ordered_triangulator_template_triangulate(vtkOrderedTriangulator* sself, int cellType, int numPts, int numEdges) { sself->TemplateTriangulate(cellType, numPts, numEdges); }
extern "C" void vtk_ordered_triangulator_update_point_type(vtkOrderedTriangulator* sself, long long internalId, int type) { sself->UpdatePointType(internalId, type); }
extern "C" long long vtk_ordered_triangulator_get_point_id(vtkOrderedTriangulator* sself, long long internalId) { return sself->GetPointId(internalId); }
extern "C" int vtk_ordered_triangulator_get_number_of_points(vtkOrderedTriangulator* sself) { return sself->GetNumberOfPoints(); }
extern "C" void vtk_ordered_triangulator_set_use_templates(vtkOrderedTriangulator* sself, int _arg) { sself->SetUseTemplates(_arg); }
extern "C" int vtk_ordered_triangulator_get_use_templates(vtkOrderedTriangulator* sself) { return sself->GetUseTemplates(); }
extern "C" void vtk_ordered_triangulator_use_templates_on(vtkOrderedTriangulator* sself) { sself->UseTemplatesOn(); }
extern "C" void vtk_ordered_triangulator_use_templates_off(vtkOrderedTriangulator* sself) { sself->UseTemplatesOff(); }
extern "C" void vtk_ordered_triangulator_set_pre_sorted(vtkOrderedTriangulator* sself, int _arg) { sself->SetPreSorted(_arg); }
extern "C" int vtk_ordered_triangulator_get_pre_sorted(vtkOrderedTriangulator* sself) { return sself->GetPreSorted(); }
extern "C" void vtk_ordered_triangulator_pre_sorted_on(vtkOrderedTriangulator* sself) { sself->PreSortedOn(); }
extern "C" void vtk_ordered_triangulator_pre_sorted_off(vtkOrderedTriangulator* sself) { sself->PreSortedOff(); }
extern "C" void vtk_ordered_triangulator_set_use_two_sort_ids(vtkOrderedTriangulator* sself, int _arg) { sself->SetUseTwoSortIds(_arg); }
extern "C" int vtk_ordered_triangulator_get_use_two_sort_ids(vtkOrderedTriangulator* sself) { return sself->GetUseTwoSortIds(); }
extern "C" void vtk_ordered_triangulator_use_two_sort_ids_on(vtkOrderedTriangulator* sself) { sself->UseTwoSortIdsOn(); }
extern "C" void vtk_ordered_triangulator_use_two_sort_ids_off(vtkOrderedTriangulator* sself) { sself->UseTwoSortIdsOff(); }
extern "C" void vtk_ordered_triangulator_init_tetra_traversal(vtkOrderedTriangulator* sself) { sself->InitTetraTraversal(); }
extern "C" vtkOutEdgeIterator * vtkOutEdgeIterator_new () {return vtkOutEdgeIterator :: New () ;}
extern "C" void vtkOutEdgeIterator_destructor (vtkOutEdgeIterator * sself) {sself -> Delete () ; return ;}
extern "C" long long vtk_out_edge_iterator_get_vertex(vtkOutEdgeIterator* sself) { return sself->GetVertex(); }
extern "C" bool vtk_out_edge_iterator_has_next(vtkOutEdgeIterator* sself) { return sself->HasNext(); }
extern "C" vtkOverlappingAMR * vtkOverlappingAMR_new () {return vtkOverlappingAMR :: New () ;}
extern "C" void vtkOverlappingAMR_destructor (vtkOverlappingAMR * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_overlapping_amr_set_refinement_ratio(vtkOverlappingAMR* sself, unsigned int level, int refRatio) { sself->SetRefinementRatio(level, refRatio); }
extern "C" int vtk_overlapping_amr_get_refinement_ratio(vtkOverlappingAMR* sself, unsigned int level) { return sself->GetRefinementRatio(level); }
extern "C" void vtk_overlapping_amr_set_amr_block_source_index(vtkOverlappingAMR* sself, unsigned int level, unsigned int id, int sourceId) { sself->SetAMRBlockSourceIndex(level, id, sourceId); }
extern "C" int vtk_overlapping_amr_get_amr_block_source_index(vtkOverlappingAMR* sself, unsigned int level, unsigned int id) { return sself->GetAMRBlockSourceIndex(level, id); }
extern "C" bool vtk_overlapping_amr_has_children_information(vtkOverlappingAMR* sself) { return sself->HasChildrenInformation(); }
extern "C" void vtk_overlapping_amr_generate_parent_child_information(vtkOverlappingAMR* sself) { sself->GenerateParentChildInformation(); }
extern "C" void vtk_overlapping_amr_print_parent_child_info(vtkOverlappingAMR* sself, unsigned int level, unsigned int index) { sself->PrintParentChildInfo(level, index); }
extern "C" void vtk_overlapping_amr_audit(vtkOverlappingAMR* sself) { sself->Audit(); }
extern "C" vtkPartitionedDataSet * vtkPartitionedDataSet_new () {return vtkPartitionedDataSet :: New () ;}
extern "C" void vtkPartitionedDataSet_destructor (vtkPartitionedDataSet * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_partitioned_data_set_set_number_of_partitions(vtkPartitionedDataSet* sself, unsigned int numPartitions) { sself->SetNumberOfPartitions(numPartitions); }
extern "C" unsigned int vtk_partitioned_data_set_get_number_of_partitions(vtkPartitionedDataSet* sself) { return sself->GetNumberOfPartitions(); }
extern "C" int vtk_partitioned_data_set_has_meta_data(vtkPartitionedDataSet* sself, unsigned int idx) { return sself->HasMetaData(idx); }
extern "C" void vtk_partitioned_data_set_remove_null_partitions(vtkPartitionedDataSet* sself) { sself->RemoveNullPartitions(); }
extern "C" vtkPartitionedDataSetCollection * vtkPartitionedDataSetCollection_new () {return vtkPartitionedDataSetCollection :: New () ;}
extern "C" void vtkPartitionedDataSetCollection_destructor (vtkPartitionedDataSetCollection * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_partitioned_data_set_collection_set_number_of_partitioned_data_sets(vtkPartitionedDataSetCollection* sself, unsigned int numDataSets) { sself->SetNumberOfPartitionedDataSets(numDataSets); }
extern "C" unsigned int vtk_partitioned_data_set_collection_get_number_of_partitioned_data_sets(vtkPartitionedDataSetCollection* sself) { return sself->GetNumberOfPartitionedDataSets(); }
extern "C" void vtk_partitioned_data_set_collection_remove_partitioned_data_set(vtkPartitionedDataSetCollection* sself, unsigned int idx) { sself->RemovePartitionedDataSet(idx); }
extern "C" unsigned int vtk_partitioned_data_set_collection_get_number_of_partitions(vtkPartitionedDataSetCollection* sself, unsigned int idx) { return sself->GetNumberOfPartitions(idx); }
extern "C" void vtk_partitioned_data_set_collection_set_number_of_partitions(vtkPartitionedDataSetCollection* sself, unsigned int idx, unsigned int numPartitions) { sself->SetNumberOfPartitions(idx, numPartitions); }
extern "C" int vtk_partitioned_data_set_collection_has_meta_data(vtkPartitionedDataSetCollection* sself, unsigned int idx) { return sself->HasMetaData(idx); }
extern "C" unsigned int vtk_partitioned_data_set_collection_get_composite_index(vtkPartitionedDataSetCollection* sself, unsigned int idx) { return sself->GetCompositeIndex(idx); }
extern "C" unsigned long vtk_partitioned_data_set_collection_get_m_time(vtkPartitionedDataSetCollection* sself) { return sself->GetMTime(); }
extern "C" vtkPath * vtkPath_new () {return vtkPath :: New () ;}
extern "C" void vtkPath_destructor (vtkPath * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_path_get_data_object_type(vtkPath* sself) { return sself->GetDataObjectType(); }
extern "C" void vtk_path_insert_next_point(vtkPath* sself, double x, double y, double z, int code) { sself->InsertNextPoint(x, y, z, code); }
extern "C" long long vtk_path_get_number_of_cells(vtkPath* sself) { return sself->GetNumberOfCells(); }
extern "C" int vtk_path_get_max_cell_size(vtkPath* sself) { return sself->GetMaxCellSize(); }
extern "C" void vtk_path_allocate(vtkPath* sself, long long size, int extSize) { sself->Allocate(size, extSize); }
extern "C" void vtk_path_reset(vtkPath* sself) { sself->Reset(); }
extern "C" vtkPentagonalPrism * vtkPentagonalPrism_new () {return vtkPentagonalPrism :: New () ;}
extern "C" void vtkPentagonalPrism_destructor (vtkPentagonalPrism * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_pentagonal_prism_get_cell_type(vtkPentagonalPrism* sself) { return sself->GetCellType(); }
extern "C" int vtk_pentagonal_prism_get_number_of_edges(vtkPentagonalPrism* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_pentagonal_prism_get_number_of_faces(vtkPentagonalPrism* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkPerlinNoise * vtkPerlinNoise_new () {return vtkPerlinNoise :: New () ;}
extern "C" void vtkPerlinNoise_destructor (vtkPerlinNoise * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_perlin_noise_set_frequency(vtkPerlinNoise* sself, double _arg1, double _arg2, double _arg3) { sself->SetFrequency(_arg1, _arg2, _arg3); }
extern "C" void vtk_perlin_noise_set_phase(vtkPerlinNoise* sself, double _arg1, double _arg2, double _arg3) { sself->SetPhase(_arg1, _arg2, _arg3); }
extern "C" void vtk_perlin_noise_set_amplitude(vtkPerlinNoise* sself, double _arg) { sself->SetAmplitude(_arg); }
extern "C" double vtk_perlin_noise_get_amplitude(vtkPerlinNoise* sself) { return sself->GetAmplitude(); }
extern "C" vtkPiecewiseFunction * vtkPiecewiseFunction_new () {return vtkPiecewiseFunction :: New () ;}
extern "C" void vtkPiecewiseFunction_destructor (vtkPiecewiseFunction * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_piecewise_function_get_data_object_type(vtkPiecewiseFunction* sself) { return sself->GetDataObjectType(); }
extern "C" int vtk_piecewise_function_get_size(vtkPiecewiseFunction* sself) { return sself->GetSize(); }
extern "C" int vtk_piecewise_function_add_point(vtkPiecewiseFunction* sself, double x, double y) { return sself->AddPoint(x, y); }
extern "C" bool vtk_piecewise_function_remove_point_by_index(vtkPiecewiseFunction* sself, size_t id) { return sself->RemovePointByIndex(id); }
extern "C" int vtk_piecewise_function_remove_point(vtkPiecewiseFunction* sself, double x) { return sself->RemovePoint(x); }
extern "C" void vtk_piecewise_function_remove_all_points(vtkPiecewiseFunction* sself) { sself->RemoveAllPoints(); }
extern "C" void vtk_piecewise_function_add_segment(vtkPiecewiseFunction* sself, double x1, double y1, double x2, double y2) { sself->AddSegment(x1, y1, x2, y2); }
extern "C" double vtk_piecewise_function_get_value(vtkPiecewiseFunction* sself, double x) { return sself->GetValue(x); }
extern "C" void vtk_piecewise_function_set_clamping(vtkPiecewiseFunction* sself, int _arg) { sself->SetClamping(_arg); }
extern "C" int vtk_piecewise_function_get_clamping(vtkPiecewiseFunction* sself) { return sself->GetClamping(); }
extern "C" void vtk_piecewise_function_clamping_on(vtkPiecewiseFunction* sself) { sself->ClampingOn(); }
extern "C" void vtk_piecewise_function_clamping_off(vtkPiecewiseFunction* sself) { sself->ClampingOff(); }
extern "C" void vtk_piecewise_function_set_use_log_scale(vtkPiecewiseFunction* sself, bool _arg) { sself->SetUseLogScale(_arg); }
extern "C" bool vtk_piecewise_function_get_use_log_scale(vtkPiecewiseFunction* sself) { return sself->GetUseLogScale(); }
extern "C" void vtk_piecewise_function_use_log_scale_on(vtkPiecewiseFunction* sself) { sself->UseLogScaleOn(); }
extern "C" void vtk_piecewise_function_use_log_scale_off(vtkPiecewiseFunction* sself) { sself->UseLogScaleOff(); }
extern "C" const char* vtk_piecewise_function_get_type(vtkPiecewiseFunction* sself) { return sself->GetType(); }
extern "C" double vtk_piecewise_function_get_first_non_zero_value(vtkPiecewiseFunction* sself) { return sself->GetFirstNonZeroValue(); }
extern "C" void vtk_piecewise_function_initialize(vtkPiecewiseFunction* sself) { sself->Initialize(); }
extern "C" void vtk_piecewise_function_set_allow_duplicate_scalars(vtkPiecewiseFunction* sself, int _arg) { sself->SetAllowDuplicateScalars(_arg); }
extern "C" int vtk_piecewise_function_get_allow_duplicate_scalars(vtkPiecewiseFunction* sself) { return sself->GetAllowDuplicateScalars(); }
extern "C" void vtk_piecewise_function_allow_duplicate_scalars_on(vtkPiecewiseFunction* sself) { sself->AllowDuplicateScalarsOn(); }
extern "C" void vtk_piecewise_function_allow_duplicate_scalars_off(vtkPiecewiseFunction* sself) { sself->AllowDuplicateScalarsOff(); }
extern "C" int vtk_piecewise_function_estimate_min_number_of_samples(vtkPiecewiseFunction* sself, const double& x1, const double& x2) { return sself->EstimateMinNumberOfSamples(x1, x2); }
extern "C" vtkPixel * vtkPixel_new () {return vtkPixel :: New () ;}
extern "C" void vtkPixel_destructor (vtkPixel * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_pixel_get_cell_type(vtkPixel* sself) { return sself->GetCellType(); }
extern "C" int vtk_pixel_get_cell_dimension(vtkPixel* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_pixel_get_number_of_edges(vtkPixel* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_pixel_get_number_of_faces(vtkPixel* sself) { return sself->GetNumberOfFaces(); }
extern "C" int vtk_pixel_inflate(vtkPixel* sself, double dist) { return sself->Inflate(dist); }
extern "C" vtkPlane * vtkPlane_new () {return vtkPlane :: New () ;}
extern "C" void vtkPlane_destructor (vtkPlane * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_plane_set_normal(vtkPlane* sself, double _arg1, double _arg2, double _arg3) { sself->SetNormal(_arg1, _arg2, _arg3); }
extern "C" void vtk_plane_set_origin(vtkPlane* sself, double _arg1, double _arg2, double _arg3) { sself->SetOrigin(_arg1, _arg2, _arg3); }
extern "C" void vtk_plane_push(vtkPlane* sself, double distance) { sself->Push(distance); }
extern "C" vtkPlaneCollection * vtkPlaneCollection_new () {return vtkPlaneCollection :: New () ;}
extern "C" void vtkPlaneCollection_destructor (vtkPlaneCollection * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_plane_collection_get_number_of_items(vtkPlaneCollection* sself) { return sself->GetNumberOfItems(); }
extern "C" vtkPlanes * vtkPlanes_new () {return vtkPlanes :: New () ;}
extern "C" void vtkPlanes_destructor (vtkPlanes * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_planes_set_bounds(vtkPlanes* sself, double xmin, double xmax, double ymin, double ymax, double zmin, double zmax) { sself->SetBounds(xmin, xmax, ymin, ymax, zmin, zmax); }
extern "C" int vtk_planes_get_number_of_planes(vtkPlanes* sself) { return sself->GetNumberOfPlanes(); }
extern "C" vtkPlanesIntersection * vtkPlanesIntersection_new () {return vtkPlanesIntersection :: New () ;}
extern "C" void vtkPlanesIntersection_destructor (vtkPlanesIntersection * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_planes_intersection_get_number_of_region_vertices(vtkPlanesIntersection* sself) { return sself->GetNumberOfRegionVertices(); }
extern "C" int vtk_planes_intersection_get_num_region_vertices(vtkPlanesIntersection* sself) { return sself->GetNumRegionVertices(); }
extern "C" vtkPointData * vtkPointData_new () {return vtkPointData :: New () ;}
extern "C" void vtkPointData_destructor (vtkPointData * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_point_data_null_point(vtkPointData* sself, long long ptId) { sself->NullPoint(ptId); }
extern "C" vtkPointLocator * vtkPointLocator_new () {return vtkPointLocator :: New () ;}
extern "C" void vtkPointLocator_destructor (vtkPointLocator * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_point_locator_set_divisions(vtkPointLocator* sself, int _arg1, int _arg2, int _arg3) { sself->SetDivisions(_arg1, _arg2, _arg3); }
extern "C" void vtk_point_locator_set_number_of_points_per_bucket(vtkPointLocator* sself, int _arg) { sself->SetNumberOfPointsPerBucket(_arg); }
extern "C" int vtk_point_locator_get_number_of_points_per_bucket_min_value(vtkPointLocator* sself) { return sself->GetNumberOfPointsPerBucketMinValue(); }
extern "C" int vtk_point_locator_get_number_of_points_per_bucket_max_value(vtkPointLocator* sself) { return sself->GetNumberOfPointsPerBucketMaxValue(); }
extern "C" int vtk_point_locator_get_number_of_points_per_bucket(vtkPointLocator* sself) { return sself->GetNumberOfPointsPerBucket(); }
extern "C" long long vtk_point_locator_is_inserted_point(vtkPointLocator* sself, double x, double y, double z) { return sself->IsInsertedPoint(x, y, z); }
extern "C" void vtk_point_locator_initialize(vtkPointLocator* sself) { sself->Initialize(); }
extern "C" void vtk_point_locator_free_search_structure(vtkPointLocator* sself) { sself->FreeSearchStructure(); }
extern "C" void vtk_point_locator_build_locator(vtkPointLocator* sself) { sself->BuildLocator(); }
extern "C" vtkPointSet * vtkPointSet_new () {return vtkPointSet :: New () ;}
extern "C" void vtkPointSet_destructor (vtkPointSet * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_point_set_set_editable(vtkPointSet* sself, bool _arg) { sself->SetEditable(_arg); }
extern "C" bool vtk_point_set_get_editable(vtkPointSet* sself) { return sself->GetEditable(); }
extern "C" void vtk_point_set_editable_on(vtkPointSet* sself) { sself->EditableOn(); }
extern "C" void vtk_point_set_editable_off(vtkPointSet* sself) { sself->EditableOff(); }
extern "C" void vtk_point_set_initialize(vtkPointSet* sself) { sself->Initialize(); }
extern "C" long long vtk_point_set_get_number_of_points(vtkPointSet* sself) { return sself->GetNumberOfPoints(); }
extern "C" long long vtk_point_set_get_number_of_cells(vtkPointSet* sself) { return sself->GetNumberOfCells(); }
extern "C" int vtk_point_set_get_max_cell_size(vtkPointSet* sself) { return sself->GetMaxCellSize(); }
extern "C" int vtk_point_set_get_cell_type(vtkPointSet* sself, long long p0) { return sself->GetCellType(p0); }
extern "C" void vtk_point_set_build_point_locator(vtkPointSet* sself) { sself->BuildPointLocator(); }
extern "C" void vtk_point_set_build_locator(vtkPointSet* sself) { sself->BuildLocator(); }
extern "C" void vtk_point_set_build_cell_locator(vtkPointSet* sself) { sself->BuildCellLocator(); }
extern "C" unsigned long vtk_point_set_get_m_time(vtkPointSet* sself) { return sself->GetMTime(); }
extern "C" void vtk_point_set_compute_bounds(vtkPointSet* sself) { sself->ComputeBounds(); }
extern "C" void vtk_point_set_squeeze(vtkPointSet* sself) { sself->Squeeze(); }
extern "C" vtkPointSetCellIterator * vtkPointSetCellIterator_new () {return vtkPointSetCellIterator :: New () ;}
extern "C" void vtkPointSetCellIterator_destructor (vtkPointSetCellIterator * sself) {sself -> Delete () ; return ;}
extern "C" bool vtk_point_set_cell_iterator_is_done_with_traversal(vtkPointSetCellIterator* sself) { return sself->IsDoneWithTraversal(); }
extern "C" long long vtk_point_set_cell_iterator_get_cell_id(vtkPointSetCellIterator* sself) { return sself->GetCellId(); }
extern "C" vtkPointsProjectedHull * vtkPointsProjectedHull_new () {return vtkPointsProjectedHull :: New () ;}
extern "C" void vtkPointsProjectedHull_destructor (vtkPointsProjectedHull * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_points_projected_hull_get_size_ccw_hull_x(vtkPointsProjectedHull* sself) { return sself->GetSizeCCWHullX(); }
extern "C" int vtk_points_projected_hull_get_size_ccw_hull_y(vtkPointsProjectedHull* sself) { return sself->GetSizeCCWHullY(); }
extern "C" int vtk_points_projected_hull_get_size_ccw_hull_z(vtkPointsProjectedHull* sself) { return sself->GetSizeCCWHullZ(); }
extern "C" void vtk_points_projected_hull_update(vtkPointsProjectedHull* sself) { sself->Update(); }
extern "C" vtkPolyData * vtkPolyData_new () {return vtkPolyData :: New () ;}
extern "C" void vtkPolyData_destructor (vtkPolyData * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_poly_data_get_data_object_type(vtkPolyData* sself) { return sself->GetDataObjectType(); }
extern "C" long long vtk_poly_data_get_number_of_cells(vtkPolyData* sself) { return sself->GetNumberOfCells(); }
extern "C" int vtk_poly_data_get_cell_type(vtkPolyData* sself, long long cellId) { return sself->GetCellType(cellId); }
extern "C" void vtk_poly_data_compute_cells_bounds(vtkPolyData* sself) { sself->ComputeCellsBounds(); }
extern "C" void vtk_poly_data_squeeze(vtkPolyData* sself) { sself->Squeeze(); }
extern "C" int vtk_poly_data_get_max_cell_size(vtkPolyData* sself) { return sself->GetMaxCellSize(); }
extern "C" long long vtk_poly_data_get_cell_id_relative_to_cell_array(vtkPolyData* sself, long long cellId) { return sself->GetCellIdRelativeToCellArray(cellId); }
extern "C" long long vtk_poly_data_get_number_of_verts(vtkPolyData* sself) { return sself->GetNumberOfVerts(); }
extern "C" long long vtk_poly_data_get_number_of_lines(vtkPolyData* sself) { return sself->GetNumberOfLines(); }
extern "C" long long vtk_poly_data_get_number_of_polys(vtkPolyData* sself) { return sself->GetNumberOfPolys(); }
extern "C" long long vtk_poly_data_get_number_of_strips(vtkPolyData* sself) { return sself->GetNumberOfStrips(); }
extern "C" bool vtk_poly_data_allocate_estimate(vtkPolyData* sself, long long numCells, long long maxCellSize) { return sself->AllocateEstimate(numCells, maxCellSize); }
extern "C" bool vtk_poly_data_allocate_exact(vtkPolyData* sself, long long numCells, long long connectivitySize) { return sself->AllocateExact(numCells, connectivitySize); }
extern "C" void vtk_poly_data_allocate(vtkPolyData* sself, long long numCells, int extSize) { sself->Allocate(numCells, extSize); }
extern "C" void vtk_poly_data_reset(vtkPolyData* sself) { sself->Reset(); }
extern "C" void vtk_poly_data_build_cells(vtkPolyData* sself) { sself->BuildCells(); }
extern "C" bool vtk_poly_data_need_to_build_cells(vtkPolyData* sself) { return sself->NeedToBuildCells(); }
extern "C" void vtk_poly_data_build_links(vtkPolyData* sself, int initialSize) { sself->BuildLinks(initialSize); }
extern "C" void vtk_poly_data_delete_cells(vtkPolyData* sself) { sself->DeleteCells(); }
extern "C" void vtk_poly_data_delete_links(vtkPolyData* sself) { sself->DeleteLinks(); }
extern "C" int vtk_poly_data_is_triangle(vtkPolyData* sself, int v1, int v2, int v3) { return sself->IsTriangle(v1, v2, v3); }
extern "C" int vtk_poly_data_is_edge(vtkPolyData* sself, long long p1, long long p2) { return sself->IsEdge(p1, p2); }
extern "C" int vtk_poly_data_is_point_used_by_cell(vtkPolyData* sself, long long ptId, long long cellId) { return sself->IsPointUsedByCell(ptId, cellId); }
extern "C" void vtk_poly_data_replace_cell_point(vtkPolyData* sself, long long cellId, long long oldPtId, long long newPtId) { sself->ReplaceCellPoint(cellId, oldPtId, newPtId); }
extern "C" void vtk_poly_data_reverse_cell(vtkPolyData* sself, long long cellId) { sself->ReverseCell(cellId); }
extern "C" void vtk_poly_data_delete_point(vtkPolyData* sself, long long ptId) { sself->DeletePoint(ptId); }
extern "C" void vtk_poly_data_delete_cell(vtkPolyData* sself, long long cellId) { sself->DeleteCell(cellId); }
extern "C" void vtk_poly_data_remove_deleted_cells(vtkPolyData* sself) { sself->RemoveDeletedCells(); }
extern "C" long long vtk_poly_data_insert_next_linked_point(vtkPolyData* sself, int numLinks) { return sself->InsertNextLinkedPoint(numLinks); }
extern "C" void vtk_poly_data_remove_cell_reference(vtkPolyData* sself, long long cellId) { sself->RemoveCellReference(cellId); }
extern "C" void vtk_poly_data_add_cell_reference(vtkPolyData* sself, long long cellId) { sself->AddCellReference(cellId); }
extern "C" void vtk_poly_data_remove_reference_to_cell(vtkPolyData* sself, long long ptId, long long cellId) { sself->RemoveReferenceToCell(ptId, cellId); }
extern "C" void vtk_poly_data_add_reference_to_cell(vtkPolyData* sself, long long ptId, long long cellId) { sself->AddReferenceToCell(ptId, cellId); }
extern "C" void vtk_poly_data_resize_cell_list(vtkPolyData* sself, long long ptId, int size) { sself->ResizeCellList(ptId, size); }
extern "C" void vtk_poly_data_initialize(vtkPolyData* sself) { sself->Initialize(); }
extern "C" int vtk_poly_data_get_piece(vtkPolyData* sself) { return sself->GetPiece(); }
extern "C" int vtk_poly_data_get_number_of_pieces(vtkPolyData* sself) { return sself->GetNumberOfPieces(); }
extern "C" int vtk_poly_data_get_ghost_level(vtkPolyData* sself) { return sself->GetGhostLevel(); }
extern "C" void vtk_poly_data_remove_ghost_cells(vtkPolyData* sself) { sself->RemoveGhostCells(); }
extern "C" unsigned long vtk_poly_data_get_mesh_m_time(vtkPolyData* sself) { return sself->GetMeshMTime(); }
extern "C" unsigned long vtk_poly_data_get_m_time(vtkPolyData* sself) { return sself->GetMTime(); }
extern "C" vtkPolyDataCollection * vtkPolyDataCollection_new () {return vtkPolyDataCollection :: New () ;}
extern "C" void vtkPolyDataCollection_destructor (vtkPolyDataCollection * sself) {sself -> Delete () ; return ;}
extern "C" vtkPolyLine * vtkPolyLine_new () {return vtkPolyLine :: New () ;}
extern "C" void vtkPolyLine_destructor (vtkPolyLine * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_poly_line_get_cell_type(vtkPolyLine* sself) { return sself->GetCellType(); }
extern "C" int vtk_poly_line_get_cell_dimension(vtkPolyLine* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_poly_line_get_number_of_edges(vtkPolyLine* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_poly_line_get_number_of_faces(vtkPolyLine* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkPolyPlane * vtkPolyPlane_new () {return vtkPolyPlane :: New () ;}
extern "C" void vtkPolyPlane_destructor (vtkPolyPlane * sself) {sself -> Delete () ; return ;}
extern "C" unsigned long vtk_poly_plane_get_m_time(vtkPolyPlane* sself) { return sself->GetMTime(); }
extern "C" vtkPolyVertex * vtkPolyVertex_new () {return vtkPolyVertex :: New () ;}
extern "C" void vtkPolyVertex_destructor (vtkPolyVertex * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_poly_vertex_get_cell_type(vtkPolyVertex* sself) { return sself->GetCellType(); }
extern "C" int vtk_poly_vertex_get_cell_dimension(vtkPolyVertex* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_poly_vertex_get_number_of_edges(vtkPolyVertex* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_poly_vertex_get_number_of_faces(vtkPolyVertex* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkPolygon * vtkPolygon_new () {return vtkPolygon :: New () ;}
extern "C" void vtkPolygon_destructor (vtkPolygon * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_polygon_get_cell_type(vtkPolygon* sself) { return sself->GetCellType(); }
extern "C" int vtk_polygon_get_cell_dimension(vtkPolygon* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_polygon_get_number_of_edges(vtkPolygon* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_polygon_get_number_of_faces(vtkPolygon* sself) { return sself->GetNumberOfFaces(); }
extern "C" double vtk_polygon_compute_area(vtkPolygon* sself) { return sself->ComputeArea(); }
extern "C" bool vtk_polygon_is_convex(vtkPolygon* sself) { return sself->IsConvex(); }
extern "C" bool vtk_polygon_get_use_mvc_interpolation(vtkPolygon* sself) { return sself->GetUseMVCInterpolation(); }
extern "C" void vtk_polygon_set_use_mvc_interpolation(vtkPolygon* sself, bool _arg) { sself->SetUseMVCInterpolation(_arg); }
extern "C" void vtk_polygon_set_tolerance(vtkPolygon* sself, double _arg) { sself->SetTolerance(_arg); }
extern "C" double vtk_polygon_get_tolerance_min_value(vtkPolygon* sself) { return sself->GetToleranceMinValue(); }
extern "C" double vtk_polygon_get_tolerance_max_value(vtkPolygon* sself) { return sself->GetToleranceMaxValue(); }
extern "C" double vtk_polygon_get_tolerance(vtkPolygon* sself) { return sself->GetTolerance(); }
extern "C" int vtk_polygon_ear_cut_triangulation(vtkPolygon* sself, int measure) { return sself->EarCutTriangulation(measure); }
extern "C" int vtk_polygon_unbiased_ear_cut_triangulation(vtkPolygon* sself, int seed, int measure) { return sself->UnbiasedEarCutTriangulation(seed, measure); }
extern "C" vtkPolyhedron * vtkPolyhedron_new () {return vtkPolyhedron :: New () ;}
extern "C" void vtkPolyhedron_destructor (vtkPolyhedron * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_polyhedron_get_cell_type(vtkPolyhedron* sself) { return sself->GetCellType(); }
extern "C" int vtk_polyhedron_requires_initialization(vtkPolyhedron* sself) { return sself->RequiresInitialization(); }
extern "C" int vtk_polyhedron_get_number_of_edges(vtkPolyhedron* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_polyhedron_get_number_of_faces(vtkPolyhedron* sself) { return sself->GetNumberOfFaces(); }
extern "C" int vtk_polyhedron_is_primary_cell(vtkPolyhedron* sself) { return sself->IsPrimaryCell(); }
extern "C" int vtk_polyhedron_requires_explicit_face_representation(vtkPolyhedron* sself) { return sself->RequiresExplicitFaceRepresentation(); }
extern "C" bool vtk_polyhedron_is_convex(vtkPolyhedron* sself) { return sself->IsConvex(); }
extern "C" vtkPyramid * vtkPyramid_new () {return vtkPyramid :: New () ;}
extern "C" void vtkPyramid_destructor (vtkPyramid * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_pyramid_get_cell_type(vtkPyramid* sself) { return sself->GetCellType(); }
extern "C" int vtk_pyramid_get_number_of_edges(vtkPyramid* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_pyramid_get_number_of_faces(vtkPyramid* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkQuad * vtkQuad_new () {return vtkQuad :: New () ;}
extern "C" void vtkQuad_destructor (vtkQuad * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_quad_get_cell_type(vtkQuad* sself) { return sself->GetCellType(); }
extern "C" int vtk_quad_get_cell_dimension(vtkQuad* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_quad_get_number_of_edges(vtkQuad* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_quad_get_number_of_faces(vtkQuad* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkQuadraticEdge * vtkQuadraticEdge_new () {return vtkQuadraticEdge :: New () ;}
extern "C" void vtkQuadraticEdge_destructor (vtkQuadraticEdge * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_quadratic_edge_get_cell_type(vtkQuadraticEdge* sself) { return sself->GetCellType(); }
extern "C" int vtk_quadratic_edge_get_cell_dimension(vtkQuadraticEdge* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_quadratic_edge_get_number_of_edges(vtkQuadraticEdge* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_quadratic_edge_get_number_of_faces(vtkQuadraticEdge* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkQuadraticHexahedron * vtkQuadraticHexahedron_new () {return vtkQuadraticHexahedron :: New () ;}
extern "C" void vtkQuadraticHexahedron_destructor (vtkQuadraticHexahedron * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_quadratic_hexahedron_get_cell_type(vtkQuadraticHexahedron* sself) { return sself->GetCellType(); }
extern "C" int vtk_quadratic_hexahedron_get_cell_dimension(vtkQuadraticHexahedron* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_quadratic_hexahedron_get_number_of_edges(vtkQuadraticHexahedron* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_quadratic_hexahedron_get_number_of_faces(vtkQuadraticHexahedron* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkQuadraticLinearQuad * vtkQuadraticLinearQuad_new () {return vtkQuadraticLinearQuad :: New () ;}
extern "C" void vtkQuadraticLinearQuad_destructor (vtkQuadraticLinearQuad * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_quadratic_linear_quad_get_cell_type(vtkQuadraticLinearQuad* sself) { return sself->GetCellType(); }
extern "C" int vtk_quadratic_linear_quad_get_cell_dimension(vtkQuadraticLinearQuad* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_quadratic_linear_quad_get_number_of_edges(vtkQuadraticLinearQuad* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_quadratic_linear_quad_get_number_of_faces(vtkQuadraticLinearQuad* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkQuadraticLinearWedge * vtkQuadraticLinearWedge_new () {return vtkQuadraticLinearWedge :: New () ;}
extern "C" void vtkQuadraticLinearWedge_destructor (vtkQuadraticLinearWedge * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_quadratic_linear_wedge_get_cell_type(vtkQuadraticLinearWedge* sself) { return sself->GetCellType(); }
extern "C" int vtk_quadratic_linear_wedge_get_cell_dimension(vtkQuadraticLinearWedge* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_quadratic_linear_wedge_get_number_of_edges(vtkQuadraticLinearWedge* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_quadratic_linear_wedge_get_number_of_faces(vtkQuadraticLinearWedge* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkQuadraticPolygon * vtkQuadraticPolygon_new () {return vtkQuadraticPolygon :: New () ;}
extern "C" void vtkQuadraticPolygon_destructor (vtkQuadraticPolygon * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_quadratic_polygon_get_cell_type(vtkQuadraticPolygon* sself) { return sself->GetCellType(); }
extern "C" int vtk_quadratic_polygon_get_cell_dimension(vtkQuadraticPolygon* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_quadratic_polygon_get_number_of_edges(vtkQuadraticPolygon* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_quadratic_polygon_get_number_of_faces(vtkQuadraticPolygon* sself) { return sself->GetNumberOfFaces(); }
extern "C" bool vtk_quadratic_polygon_get_use_mvc_interpolation(vtkQuadraticPolygon* sself) { return sself->GetUseMVCInterpolation(); }
extern "C" void vtk_quadratic_polygon_set_use_mvc_interpolation(vtkQuadraticPolygon* sself, bool _arg) { sself->SetUseMVCInterpolation(_arg); }
extern "C" vtkQuadraticPyramid * vtkQuadraticPyramid_new () {return vtkQuadraticPyramid :: New () ;}
extern "C" void vtkQuadraticPyramid_destructor (vtkQuadraticPyramid * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_quadratic_pyramid_get_cell_type(vtkQuadraticPyramid* sself) { return sself->GetCellType(); }
extern "C" int vtk_quadratic_pyramid_get_cell_dimension(vtkQuadraticPyramid* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_quadratic_pyramid_get_number_of_edges(vtkQuadraticPyramid* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_quadratic_pyramid_get_number_of_faces(vtkQuadraticPyramid* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkQuadraticQuad * vtkQuadraticQuad_new () {return vtkQuadraticQuad :: New () ;}
extern "C" void vtkQuadraticQuad_destructor (vtkQuadraticQuad * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_quadratic_quad_get_cell_type(vtkQuadraticQuad* sself) { return sself->GetCellType(); }
extern "C" int vtk_quadratic_quad_get_cell_dimension(vtkQuadraticQuad* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_quadratic_quad_get_number_of_edges(vtkQuadraticQuad* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_quadratic_quad_get_number_of_faces(vtkQuadraticQuad* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkQuadraticTetra * vtkQuadraticTetra_new () {return vtkQuadraticTetra :: New () ;}
extern "C" void vtkQuadraticTetra_destructor (vtkQuadraticTetra * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_quadratic_tetra_get_cell_type(vtkQuadraticTetra* sself) { return sself->GetCellType(); }
extern "C" int vtk_quadratic_tetra_get_cell_dimension(vtkQuadraticTetra* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_quadratic_tetra_get_number_of_edges(vtkQuadraticTetra* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_quadratic_tetra_get_number_of_faces(vtkQuadraticTetra* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkQuadraticTriangle * vtkQuadraticTriangle_new () {return vtkQuadraticTriangle :: New () ;}
extern "C" void vtkQuadraticTriangle_destructor (vtkQuadraticTriangle * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_quadratic_triangle_get_cell_type(vtkQuadraticTriangle* sself) { return sself->GetCellType(); }
extern "C" int vtk_quadratic_triangle_get_cell_dimension(vtkQuadraticTriangle* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_quadratic_triangle_get_number_of_edges(vtkQuadraticTriangle* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_quadratic_triangle_get_number_of_faces(vtkQuadraticTriangle* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkQuadraticWedge * vtkQuadraticWedge_new () {return vtkQuadraticWedge :: New () ;}
extern "C" void vtkQuadraticWedge_destructor (vtkQuadraticWedge * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_quadratic_wedge_get_cell_type(vtkQuadraticWedge* sself) { return sself->GetCellType(); }
extern "C" int vtk_quadratic_wedge_get_cell_dimension(vtkQuadraticWedge* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_quadratic_wedge_get_number_of_edges(vtkQuadraticWedge* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_quadratic_wedge_get_number_of_faces(vtkQuadraticWedge* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkQuadratureSchemeDefinition * vtkQuadratureSchemeDefinition_new () {return vtkQuadratureSchemeDefinition :: New () ;}
extern "C" void vtkQuadratureSchemeDefinition_destructor (vtkQuadratureSchemeDefinition * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_quadrature_scheme_definition_clear(vtkQuadratureSchemeDefinition* sself) { sself->Clear(); }
extern "C" int vtk_quadrature_scheme_definition_get_cell_type(vtkQuadratureSchemeDefinition* sself) { return sself->GetCellType(); }
extern "C" int vtk_quadrature_scheme_definition_get_quadrature_key(vtkQuadratureSchemeDefinition* sself) { return sself->GetQuadratureKey(); }
extern "C" int vtk_quadrature_scheme_definition_get_number_of_nodes(vtkQuadratureSchemeDefinition* sself) { return sself->GetNumberOfNodes(); }
extern "C" int vtk_quadrature_scheme_definition_get_number_of_quadrature_points(vtkQuadratureSchemeDefinition* sself) { return sself->GetNumberOfQuadraturePoints(); }
extern "C" vtkQuadric * vtkQuadric_new () {return vtkQuadric :: New () ;}
extern "C" void vtkQuadric_destructor (vtkQuadric * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_quadric_set_coefficients(vtkQuadric* sself, double a0, double a1, double a2, double a3, double a4, double a5, double a6, double a7, double a8, double a9) { sself->SetCoefficients(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9); }
extern "C" vtkRectilinearGrid * vtkRectilinearGrid_new () {return vtkRectilinearGrid :: New () ;}
extern "C" void vtkRectilinearGrid_destructor (vtkRectilinearGrid * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_rectilinear_grid_get_data_object_type(vtkRectilinearGrid* sself) { return sself->GetDataObjectType(); }
extern "C" void vtk_rectilinear_grid_initialize(vtkRectilinearGrid* sself) { sself->Initialize(); }
extern "C" long long vtk_rectilinear_grid_get_number_of_cells(vtkRectilinearGrid* sself) { return sself->GetNumberOfCells(); }
extern "C" long long vtk_rectilinear_grid_get_number_of_points(vtkRectilinearGrid* sself) { return sself->GetNumberOfPoints(); }
extern "C" int vtk_rectilinear_grid_get_cell_type(vtkRectilinearGrid* sself, long long cellId) { return sself->GetCellType(cellId); }
extern "C" int vtk_rectilinear_grid_get_max_cell_size(vtkRectilinearGrid* sself) { return sself->GetMaxCellSize(); }
extern "C" unsigned char vtk_rectilinear_grid_is_point_visible(vtkRectilinearGrid* sself, long long ptId) { return sself->IsPointVisible(ptId); }
extern "C" unsigned char vtk_rectilinear_grid_is_cell_visible(vtkRectilinearGrid* sself, long long cellId) { return sself->IsCellVisible(cellId); }
extern "C" bool vtk_rectilinear_grid_has_any_blank_points(vtkRectilinearGrid* sself) { return sself->HasAnyBlankPoints(); }
extern "C" bool vtk_rectilinear_grid_has_any_blank_cells(vtkRectilinearGrid* sself) { return sself->HasAnyBlankCells(); }
extern "C" void vtk_rectilinear_grid_set_dimensions(vtkRectilinearGrid* sself, int i, int j, int k) { sself->SetDimensions(i, j, k); }
extern "C" int vtk_rectilinear_grid_get_data_dimension(vtkRectilinearGrid* sself) { return sself->GetDataDimension(); }
extern "C" void vtk_rectilinear_grid_set_extent(vtkRectilinearGrid* sself, int xMin, int xMax, int yMin, int yMax, int zMin, int zMax) { sself->SetExtent(xMin, xMax, yMin, yMax, zMin, zMax); }
extern "C" int vtk_rectilinear_grid_get_extent_type(vtkRectilinearGrid* sself) { return sself->GetExtentType(); }
extern "C" const char* vtk_rectilinear_grid_get_scalar_type_as_string(vtkRectilinearGrid* sself) { return sself->GetScalarTypeAsString(); }
extern "C" vtkReebGraph * vtkReebGraph_new () {return vtkReebGraph :: New () ;}
extern "C" void vtkReebGraph_destructor (vtkReebGraph * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_reeb_graph_stream_triangle(vtkReebGraph* sself, long long vertex0Id, double scalar0, long long vertex1Id, double scalar1, long long vertex2Id, double scalar2) { return sself->StreamTriangle(vertex0Id, scalar0, vertex1Id, scalar1, vertex2Id, scalar2); }
extern "C" int vtk_reeb_graph_stream_tetrahedron(vtkReebGraph* sself, long long vertex0Id, double scalar0, long long vertex1Id, double scalar1, long long vertex2Id, double scalar2, long long vertex3Id, double scalar3) { return sself->StreamTetrahedron(vertex0Id, scalar0, vertex1Id, scalar1, vertex2Id, scalar2, vertex3Id, scalar3); }
extern "C" void vtk_reeb_graph_close_stream(vtkReebGraph* sself) { sself->CloseStream(); }
extern "C" vtkReebGraphSimplificationMetric * vtkReebGraphSimplificationMetric_new () {return vtkReebGraphSimplificationMetric :: New () ;}
extern "C" void vtkReebGraphSimplificationMetric_destructor (vtkReebGraphSimplificationMetric * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_reeb_graph_simplification_metric_set_lower_bound(vtkReebGraphSimplificationMetric* sself, double _arg) { sself->SetLowerBound(_arg); }
extern "C" double vtk_reeb_graph_simplification_metric_get_lower_bound(vtkReebGraphSimplificationMetric* sself) { return sself->GetLowerBound(); }
extern "C" void vtk_reeb_graph_simplification_metric_set_upper_bound(vtkReebGraphSimplificationMetric* sself, double _arg) { sself->SetUpperBound(_arg); }
extern "C" double vtk_reeb_graph_simplification_metric_get_upper_bound(vtkReebGraphSimplificationMetric* sself) { return sself->GetUpperBound(); }
extern "C" vtkSelection * vtkSelection_new () {return vtkSelection :: New () ;}
extern "C" void vtkSelection_destructor (vtkSelection * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_selection_get_data_object_type(vtkSelection* sself) { return sself->GetDataObjectType(); }
extern "C" unsigned int vtk_selection_get_number_of_nodes(vtkSelection* sself) { return sself->GetNumberOfNodes(); }
extern "C" void vtk_selection_remove_node(vtkSelection* sself, unsigned int idx) { sself->RemoveNode(idx); }
extern "C" void vtk_selection_remove_all_nodes(vtkSelection* sself) { sself->RemoveAllNodes(); }
extern "C" void vtk_selection_set_expression(vtkSelection* sself, const char* _arg) { sself->SetExpression(_arg); }
extern "C" unsigned long vtk_selection_get_m_time(vtkSelection* sself) { return sself->GetMTime(); }
extern "C" void vtk_selection_dump(vtkSelection* sself) { sself->Dump(); }
extern "C" vtkSelectionNode * vtkSelectionNode_new () {return vtkSelectionNode :: New () ;}
extern "C" void vtkSelectionNode_destructor (vtkSelectionNode * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_selection_node_initialize(vtkSelectionNode* sself) { sself->Initialize(); }
extern "C" unsigned long vtk_selection_node_get_m_time(vtkSelectionNode* sself) { return sself->GetMTime(); }
extern "C" void vtk_selection_node_set_content_type(vtkSelectionNode* sself, int type) { sself->SetContentType(type); }
extern "C" int vtk_selection_node_get_content_type(vtkSelectionNode* sself) { return sself->GetContentType(); }
extern "C" const char* vtk_selection_node_get_content_type_as_string(vtkSelectionNode* sself, int type) { return sself->GetContentTypeAsString(type); }
extern "C" void vtk_selection_node_set_field_type(vtkSelectionNode* sself, int type) { sself->SetFieldType(type); }
extern "C" int vtk_selection_node_get_field_type(vtkSelectionNode* sself) { return sself->GetFieldType(); }
extern "C" const char* vtk_selection_node_get_field_type_as_string(vtkSelectionNode* sself, int type) { return sself->GetFieldTypeAsString(type); }
extern "C" int vtk_selection_node_get_field_type_from_string(vtkSelectionNode* sself, const char* type) { return sself->GetFieldTypeFromString(type); }
extern "C" int vtk_selection_node_convert_selection_field_to_attribute_type(vtkSelectionNode* sself, int val) { return sself->ConvertSelectionFieldToAttributeType(val); }
extern "C" int vtk_selection_node_convert_attribute_type_to_selection_field(vtkSelectionNode* sself, int val) { return sself->ConvertAttributeTypeToSelectionField(val); }
extern "C" void vtk_selection_node_set_query_string(vtkSelectionNode* sself, const char* _arg) { sself->SetQueryString(_arg); }
extern "C" vtkSimpleCellTessellator * vtkSimpleCellTessellator_new () {return vtkSimpleCellTessellator :: New () ;}
extern "C" void vtkSimpleCellTessellator_destructor (vtkSimpleCellTessellator * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_simple_cell_tessellator_reset(vtkSimpleCellTessellator* sself) { sself->Reset(); }
extern "C" int vtk_simple_cell_tessellator_get_fixed_subdivisions(vtkSimpleCellTessellator* sself) { return sself->GetFixedSubdivisions(); }
extern "C" int vtk_simple_cell_tessellator_get_max_subdivision_level(vtkSimpleCellTessellator* sself) { return sself->GetMaxSubdivisionLevel(); }
extern "C" int vtk_simple_cell_tessellator_get_max_adaptive_subdivisions(vtkSimpleCellTessellator* sself) { return sself->GetMaxAdaptiveSubdivisions(); }
extern "C" void vtk_simple_cell_tessellator_set_fixed_subdivisions(vtkSimpleCellTessellator* sself, int level) { sself->SetFixedSubdivisions(level); }
extern "C" void vtk_simple_cell_tessellator_set_max_subdivision_level(vtkSimpleCellTessellator* sself, int level) { sself->SetMaxSubdivisionLevel(level); }
extern "C" void vtk_simple_cell_tessellator_set_subdivision_levels(vtkSimpleCellTessellator* sself, int fixed, int maxLevel) { sself->SetSubdivisionLevels(fixed, maxLevel); }
extern "C" vtkSmoothErrorMetric * vtkSmoothErrorMetric_new () {return vtkSmoothErrorMetric :: New () ;}
extern "C" void vtkSmoothErrorMetric_destructor (vtkSmoothErrorMetric * sself) {sself -> Delete () ; return ;}
extern "C" double vtk_smooth_error_metric_get_angle_tolerance(vtkSmoothErrorMetric* sself) { return sself->GetAngleTolerance(); }
extern "C" void vtk_smooth_error_metric_set_angle_tolerance(vtkSmoothErrorMetric* sself, double value) { sself->SetAngleTolerance(value); }
extern "C" vtkSortFieldData * vtkSortFieldData_new () {return vtkSortFieldData :: New () ;}
extern "C" void vtkSortFieldData_destructor (vtkSortFieldData * sself) {sself -> Delete () ; return ;}
extern "C" vtkSphere * vtkSphere_new () {return vtkSphere :: New () ;}
extern "C" void vtkSphere_destructor (vtkSphere * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_sphere_set_radius(vtkSphere* sself, double _arg) { sself->SetRadius(_arg); }
extern "C" double vtk_sphere_get_radius(vtkSphere* sself) { return sself->GetRadius(); }
extern "C" void vtk_sphere_set_center(vtkSphere* sself, double _arg1, double _arg2, double _arg3) { sself->SetCenter(_arg1, _arg2, _arg3); }
extern "C" vtkSpheres * vtkSpheres_new () {return vtkSpheres :: New () ;}
extern "C" void vtkSpheres_destructor (vtkSpheres * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_spheres_get_number_of_spheres(vtkSpheres* sself) { return sself->GetNumberOfSpheres(); }
extern "C" vtkStaticCellLinks * vtkStaticCellLinks_new () {return vtkStaticCellLinks :: New () ;}
extern "C" void vtkStaticCellLinks_destructor (vtkStaticCellLinks * sself) {sself -> Delete () ; return ;}
extern "C" long long vtk_static_cell_links_get_number_of_cells(vtkStaticCellLinks* sself, long long ptId) { return sself->GetNumberOfCells(ptId); }
extern "C" long long vtk_static_cell_links_get_ncells(vtkStaticCellLinks* sself, long long ptId) { return sself->GetNcells(ptId); }
extern "C" void vtk_static_cell_links_initialize(vtkStaticCellLinks* sself) { sself->Initialize(); }
extern "C" void vtk_static_cell_links_squeeze(vtkStaticCellLinks* sself) { sself->Squeeze(); }
extern "C" void vtk_static_cell_links_reset(vtkStaticCellLinks* sself) { sself->Reset(); }
extern "C" unsigned long vtk_static_cell_links_get_actual_memory_size(vtkStaticCellLinks* sself) { return sself->GetActualMemorySize(); }
extern "C" vtkStaticCellLocator * vtkStaticCellLocator_new () {return vtkStaticCellLocator :: New () ;}
extern "C" void vtkStaticCellLocator_destructor (vtkStaticCellLocator * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_static_cell_locator_set_divisions(vtkStaticCellLocator* sself, int _arg1, int _arg2, int _arg3) { sself->SetDivisions(_arg1, _arg2, _arg3); }
extern "C" void vtk_static_cell_locator_free_search_structure(vtkStaticCellLocator* sself) { sself->FreeSearchStructure(); }
extern "C" void vtk_static_cell_locator_build_locator(vtkStaticCellLocator* sself) { sself->BuildLocator(); }
extern "C" void vtk_static_cell_locator_set_max_number_of_buckets(vtkStaticCellLocator* sself, long long _arg) { sself->SetMaxNumberOfBuckets(_arg); }
extern "C" long long vtk_static_cell_locator_get_max_number_of_buckets_min_value(vtkStaticCellLocator* sself) { return sself->GetMaxNumberOfBucketsMinValue(); }
extern "C" long long vtk_static_cell_locator_get_max_number_of_buckets_max_value(vtkStaticCellLocator* sself) { return sself->GetMaxNumberOfBucketsMaxValue(); }
extern "C" long long vtk_static_cell_locator_get_max_number_of_buckets(vtkStaticCellLocator* sself) { return sself->GetMaxNumberOfBuckets(); }
extern "C" bool vtk_static_cell_locator_get_large_ids(vtkStaticCellLocator* sself) { return sself->GetLargeIds(); }
extern "C" void vtk_static_cell_locator_set_use_diagonal_length_tolerance(vtkStaticCellLocator* sself, bool _arg) { sself->SetUseDiagonalLengthTolerance(_arg); }
extern "C" bool vtk_static_cell_locator_get_use_diagonal_length_tolerance(vtkStaticCellLocator* sself) { return sself->GetUseDiagonalLengthTolerance(); }
extern "C" void vtk_static_cell_locator_use_diagonal_length_tolerance_on(vtkStaticCellLocator* sself) { sself->UseDiagonalLengthToleranceOn(); }
extern "C" void vtk_static_cell_locator_use_diagonal_length_tolerance_off(vtkStaticCellLocator* sself) { sself->UseDiagonalLengthToleranceOff(); }
extern "C" vtkStaticPointLocator * vtkStaticPointLocator_new () {return vtkStaticPointLocator :: New () ;}
extern "C" void vtkStaticPointLocator_destructor (vtkStaticPointLocator * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_static_point_locator_set_number_of_points_per_bucket(vtkStaticPointLocator* sself, int _arg) { sself->SetNumberOfPointsPerBucket(_arg); }
extern "C" int vtk_static_point_locator_get_number_of_points_per_bucket_min_value(vtkStaticPointLocator* sself) { return sself->GetNumberOfPointsPerBucketMinValue(); }
extern "C" int vtk_static_point_locator_get_number_of_points_per_bucket_max_value(vtkStaticPointLocator* sself) { return sself->GetNumberOfPointsPerBucketMaxValue(); }
extern "C" int vtk_static_point_locator_get_number_of_points_per_bucket(vtkStaticPointLocator* sself) { return sself->GetNumberOfPointsPerBucket(); }
extern "C" void vtk_static_point_locator_set_divisions(vtkStaticPointLocator* sself, int _arg1, int _arg2, int _arg3) { sself->SetDivisions(_arg1, _arg2, _arg3); }
extern "C" void vtk_static_point_locator_initialize(vtkStaticPointLocator* sself) { sself->Initialize(); }
extern "C" void vtk_static_point_locator_free_search_structure(vtkStaticPointLocator* sself) { sself->FreeSearchStructure(); }
extern "C" void vtk_static_point_locator_build_locator(vtkStaticPointLocator* sself) { sself->BuildLocator(); }
extern "C" long long vtk_static_point_locator_get_number_of_points_in_bucket(vtkStaticPointLocator* sself, long long bNum) { return sself->GetNumberOfPointsInBucket(bNum); }
extern "C" void vtk_static_point_locator_set_max_number_of_buckets(vtkStaticPointLocator* sself, long long _arg) { sself->SetMaxNumberOfBuckets(_arg); }
extern "C" long long vtk_static_point_locator_get_max_number_of_buckets_min_value(vtkStaticPointLocator* sself) { return sself->GetMaxNumberOfBucketsMinValue(); }
extern "C" long long vtk_static_point_locator_get_max_number_of_buckets_max_value(vtkStaticPointLocator* sself) { return sself->GetMaxNumberOfBucketsMaxValue(); }
extern "C" long long vtk_static_point_locator_get_max_number_of_buckets(vtkStaticPointLocator* sself) { return sself->GetMaxNumberOfBuckets(); }
extern "C" bool vtk_static_point_locator_get_large_ids(vtkStaticPointLocator* sself) { return sself->GetLargeIds(); }
extern "C" vtkStaticPointLocator2D * vtkStaticPointLocator2D_new () {return vtkStaticPointLocator2D :: New () ;}
extern "C" void vtkStaticPointLocator2D_destructor (vtkStaticPointLocator2D * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_static_point_locator_2_d_set_number_of_points_per_bucket(vtkStaticPointLocator2D* sself, int _arg) { sself->SetNumberOfPointsPerBucket(_arg); }
extern "C" int vtk_static_point_locator_2_d_get_number_of_points_per_bucket_min_value(vtkStaticPointLocator2D* sself) { return sself->GetNumberOfPointsPerBucketMinValue(); }
extern "C" int vtk_static_point_locator_2_d_get_number_of_points_per_bucket_max_value(vtkStaticPointLocator2D* sself) { return sself->GetNumberOfPointsPerBucketMaxValue(); }
extern "C" int vtk_static_point_locator_2_d_get_number_of_points_per_bucket(vtkStaticPointLocator2D* sself) { return sself->GetNumberOfPointsPerBucket(); }
extern "C" void vtk_static_point_locator_2_d_set_divisions(vtkStaticPointLocator2D* sself, int _arg1, int _arg2) { sself->SetDivisions(_arg1, _arg2); }
extern "C" void vtk_static_point_locator_2_d_initialize(vtkStaticPointLocator2D* sself) { sself->Initialize(); }
extern "C" void vtk_static_point_locator_2_d_free_search_structure(vtkStaticPointLocator2D* sself) { sself->FreeSearchStructure(); }
extern "C" void vtk_static_point_locator_2_d_build_locator(vtkStaticPointLocator2D* sself) { sself->BuildLocator(); }
extern "C" long long vtk_static_point_locator_2_d_get_number_of_points_in_bucket(vtkStaticPointLocator2D* sself, long long bNum) { return sself->GetNumberOfPointsInBucket(bNum); }
extern "C" void vtk_static_point_locator_2_d_set_max_number_of_buckets(vtkStaticPointLocator2D* sself, long long _arg) { sself->SetMaxNumberOfBuckets(_arg); }
extern "C" long long vtk_static_point_locator_2_d_get_max_number_of_buckets_min_value(vtkStaticPointLocator2D* sself) { return sself->GetMaxNumberOfBucketsMinValue(); }
extern "C" long long vtk_static_point_locator_2_d_get_max_number_of_buckets_max_value(vtkStaticPointLocator2D* sself) { return sself->GetMaxNumberOfBucketsMaxValue(); }
extern "C" long long vtk_static_point_locator_2_d_get_max_number_of_buckets(vtkStaticPointLocator2D* sself) { return sself->GetMaxNumberOfBuckets(); }
extern "C" bool vtk_static_point_locator_2_d_get_large_ids(vtkStaticPointLocator2D* sself) { return sself->GetLargeIds(); }
extern "C" vtkStructuredExtent * vtkStructuredExtent_new () {return vtkStructuredExtent :: New () ;}
extern "C" void vtkStructuredExtent_destructor (vtkStructuredExtent * sself) {sself -> Delete () ; return ;}
extern "C" vtkStructuredGrid * vtkStructuredGrid_new () {return vtkStructuredGrid :: New () ;}
extern "C" void vtkStructuredGrid_destructor (vtkStructuredGrid * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_structured_grid_get_data_object_type(vtkStructuredGrid* sself) { return sself->GetDataObjectType(); }
extern "C" long long vtk_structured_grid_get_number_of_points(vtkStructuredGrid* sself) { return sself->GetNumberOfPoints(); }
extern "C" int vtk_structured_grid_get_cell_type(vtkStructuredGrid* sself, long long cellId) { return sself->GetCellType(cellId); }
extern "C" void vtk_structured_grid_set_dimensions(vtkStructuredGrid* sself, int i, int j, int k) { sself->SetDimensions(i, j, k); }
extern "C" int vtk_structured_grid_get_data_dimension(vtkStructuredGrid* sself) { return sself->GetDataDimension(); }
extern "C" void vtk_structured_grid_set_extent(vtkStructuredGrid* sself, int xMin, int xMax, int yMin, int yMax, int zMin, int zMax) { sself->SetExtent(xMin, xMax, yMin, yMax, zMin, zMax); }
extern "C" int vtk_structured_grid_get_extent_type(vtkStructuredGrid* sself) { return sself->GetExtentType(); }
extern "C" void vtk_structured_grid_blank_point(vtkStructuredGrid* sself, long long ptId) { sself->BlankPoint(ptId); }
extern "C" void vtk_structured_grid_un_blank_point(vtkStructuredGrid* sself, long long ptId) { sself->UnBlankPoint(ptId); }
extern "C" void vtk_structured_grid_blank_cell(vtkStructuredGrid* sself, long long ptId) { sself->BlankCell(ptId); }
extern "C" void vtk_structured_grid_un_blank_cell(vtkStructuredGrid* sself, long long ptId) { sself->UnBlankCell(ptId); }
extern "C" unsigned char vtk_structured_grid_is_point_visible(vtkStructuredGrid* sself, long long ptId) { return sself->IsPointVisible(ptId); }
extern "C" unsigned char vtk_structured_grid_is_cell_visible(vtkStructuredGrid* sself, long long cellId) { return sself->IsCellVisible(cellId); }
extern "C" bool vtk_structured_grid_has_any_blank_points(vtkStructuredGrid* sself) { return sself->HasAnyBlankPoints(); }
extern "C" bool vtk_structured_grid_has_any_blank_cells(vtkStructuredGrid* sself) { return sself->HasAnyBlankCells(); }
extern "C" vtkStructuredPoints * vtkStructuredPoints_new () {return vtkStructuredPoints :: New () ;}
extern "C" void vtkStructuredPoints_destructor (vtkStructuredPoints * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_structured_points_get_data_object_type(vtkStructuredPoints* sself) { return sself->GetDataObjectType(); }
extern "C" vtkStructuredPointsCollection * vtkStructuredPointsCollection_new () {return vtkStructuredPointsCollection :: New () ;}
extern "C" void vtkStructuredPointsCollection_destructor (vtkStructuredPointsCollection * sself) {sself -> Delete () ; return ;}
extern "C" vtkSuperquadric * vtkSuperquadric_new () {return vtkSuperquadric :: New () ;}
extern "C" void vtkSuperquadric_destructor (vtkSuperquadric * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_superquadric_set_center(vtkSuperquadric* sself, double _arg1, double _arg2, double _arg3) { sself->SetCenter(_arg1, _arg2, _arg3); }
extern "C" void vtk_superquadric_set_scale(vtkSuperquadric* sself, double _arg1, double _arg2, double _arg3) { sself->SetScale(_arg1, _arg2, _arg3); }
extern "C" double vtk_superquadric_get_thickness(vtkSuperquadric* sself) { return sself->GetThickness(); }
extern "C" void vtk_superquadric_set_thickness(vtkSuperquadric* sself, double _arg) { sself->SetThickness(_arg); }
extern "C" double vtk_superquadric_get_thickness_min_value(vtkSuperquadric* sself) { return sself->GetThicknessMinValue(); }
extern "C" double vtk_superquadric_get_thickness_max_value(vtkSuperquadric* sself) { return sself->GetThicknessMaxValue(); }
extern "C" double vtk_superquadric_get_phi_roundness(vtkSuperquadric* sself) { return sself->GetPhiRoundness(); }
extern "C" void vtk_superquadric_set_phi_roundness(vtkSuperquadric* sself, double e) { sself->SetPhiRoundness(e); }
extern "C" double vtk_superquadric_get_theta_roundness(vtkSuperquadric* sself) { return sself->GetThetaRoundness(); }
extern "C" void vtk_superquadric_set_theta_roundness(vtkSuperquadric* sself, double e) { sself->SetThetaRoundness(e); }
extern "C" void vtk_superquadric_set_size(vtkSuperquadric* sself, double _arg) { sself->SetSize(_arg); }
extern "C" double vtk_superquadric_get_size(vtkSuperquadric* sself) { return sself->GetSize(); }
extern "C" void vtk_superquadric_toroidal_on(vtkSuperquadric* sself) { sself->ToroidalOn(); }
extern "C" void vtk_superquadric_toroidal_off(vtkSuperquadric* sself) { sself->ToroidalOff(); }
extern "C" int vtk_superquadric_get_toroidal(vtkSuperquadric* sself) { return sself->GetToroidal(); }
extern "C" void vtk_superquadric_set_toroidal(vtkSuperquadric* sself, int _arg) { sself->SetToroidal(_arg); }
extern "C" vtkTable * vtkTable_new () {return vtkTable :: New () ;}
extern "C" void vtkTable_destructor (vtkTable * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_table_dump(vtkTable* sself, unsigned int colWidth, int rowLimit) { sself->Dump(colWidth, rowLimit); }
extern "C" int vtk_table_get_data_object_type(vtkTable* sself) { return sself->GetDataObjectType(); }
extern "C" long long vtk_table_get_number_of_rows(vtkTable* sself) { return sself->GetNumberOfRows(); }
extern "C" void vtk_table_set_number_of_rows(vtkTable* sself, const long long p0) { sself->SetNumberOfRows(p0); }
extern "C" long long vtk_table_insert_next_blank_row(vtkTable* sself, double default_num_val) { return sself->InsertNextBlankRow(default_num_val); }
extern "C" void vtk_table_remove_row(vtkTable* sself, long long row) { sself->RemoveRow(row); }
extern "C" long long vtk_table_get_number_of_columns(vtkTable* sself) { return sself->GetNumberOfColumns(); }
extern "C" const char* vtk_table_get_column_name(vtkTable* sself, long long col) { return sself->GetColumnName(col); }
extern "C" void vtk_table_remove_column_by_name(vtkTable* sself, const char* name) { sself->RemoveColumnByName(name); }
extern "C" void vtk_table_remove_column(vtkTable* sself, long long col) { sself->RemoveColumn(col); }
extern "C" void vtk_table_initialize(vtkTable* sself) { sself->Initialize(); }
extern "C" long long vtk_table_get_number_of_elements(vtkTable* sself, int type) { return sself->GetNumberOfElements(type); }
extern "C" vtkTetra * vtkTetra_new () {return vtkTetra :: New () ;}
extern "C" void vtkTetra_destructor (vtkTetra * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_tetra_get_cell_type(vtkTetra* sself) { return sself->GetCellType(); }
extern "C" int vtk_tetra_get_number_of_edges(vtkTetra* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_tetra_get_number_of_faces(vtkTetra* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkTree * vtkTree_new () {return vtkTree :: New () ;}
extern "C" void vtkTree_destructor (vtkTree * sself) {sself -> Delete () ; return ;}
extern "C" long long vtk_tree_get_root(vtkTree* sself) { return sself->GetRoot(); }
extern "C" long long vtk_tree_get_number_of_children(vtkTree* sself, long long v) { return sself->GetNumberOfChildren(v); }
extern "C" long long vtk_tree_get_child(vtkTree* sself, long long v, long long i) { return sself->GetChild(v, i); }
extern "C" long long vtk_tree_get_parent(vtkTree* sself, long long v) { return sself->GetParent(v); }
extern "C" long long vtk_tree_get_level(vtkTree* sself, long long v) { return sself->GetLevel(v); }
extern "C" bool vtk_tree_is_leaf(vtkTree* sself, long long vertex) { return sself->IsLeaf(vertex); }
extern "C" vtkTreeBFSIterator * vtkTreeBFSIterator_new () {return vtkTreeBFSIterator :: New () ;}
extern "C" void vtkTreeBFSIterator_destructor (vtkTreeBFSIterator * sself) {sself -> Delete () ; return ;}
extern "C" vtkTreeDFSIterator * vtkTreeDFSIterator_new () {return vtkTreeDFSIterator :: New () ;}
extern "C" void vtkTreeDFSIterator_destructor (vtkTreeDFSIterator * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_tree_dfs_iterator_set_mode(vtkTreeDFSIterator* sself, int mode) { sself->SetMode(mode); }
extern "C" int vtk_tree_dfs_iterator_get_mode(vtkTreeDFSIterator* sself) { return sself->GetMode(); }
extern "C" vtkTriQuadraticHexahedron * vtkTriQuadraticHexahedron_new () {return vtkTriQuadraticHexahedron :: New () ;}
extern "C" void vtkTriQuadraticHexahedron_destructor (vtkTriQuadraticHexahedron * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_tri_quadratic_hexahedron_get_cell_type(vtkTriQuadraticHexahedron* sself) { return sself->GetCellType(); }
extern "C" int vtk_tri_quadratic_hexahedron_get_cell_dimension(vtkTriQuadraticHexahedron* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_tri_quadratic_hexahedron_get_number_of_edges(vtkTriQuadraticHexahedron* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_tri_quadratic_hexahedron_get_number_of_faces(vtkTriQuadraticHexahedron* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkTriQuadraticPyramid * vtkTriQuadraticPyramid_new () {return vtkTriQuadraticPyramid :: New () ;}
extern "C" void vtkTriQuadraticPyramid_destructor (vtkTriQuadraticPyramid * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_tri_quadratic_pyramid_get_cell_type(vtkTriQuadraticPyramid* sself) { return sself->GetCellType(); }
extern "C" int vtk_tri_quadratic_pyramid_get_cell_dimension(vtkTriQuadraticPyramid* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_tri_quadratic_pyramid_get_number_of_edges(vtkTriQuadraticPyramid* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_tri_quadratic_pyramid_get_number_of_faces(vtkTriQuadraticPyramid* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkTriangle * vtkTriangle_new () {return vtkTriangle :: New () ;}
extern "C" void vtkTriangle_destructor (vtkTriangle * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_triangle_get_cell_type(vtkTriangle* sself) { return sself->GetCellType(); }
extern "C" int vtk_triangle_get_cell_dimension(vtkTriangle* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_triangle_get_number_of_edges(vtkTriangle* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_triangle_get_number_of_faces(vtkTriangle* sself) { return sself->GetNumberOfFaces(); }
extern "C" double vtk_triangle_compute_area(vtkTriangle* sself) { return sself->ComputeArea(); }
extern "C" vtkTriangleStrip * vtkTriangleStrip_new () {return vtkTriangleStrip :: New () ;}
extern "C" void vtkTriangleStrip_destructor (vtkTriangleStrip * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_triangle_strip_get_cell_type(vtkTriangleStrip* sself) { return sself->GetCellType(); }
extern "C" int vtk_triangle_strip_get_cell_dimension(vtkTriangleStrip* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_triangle_strip_get_number_of_edges(vtkTriangleStrip* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_triangle_strip_get_number_of_faces(vtkTriangleStrip* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkUndirectedGraph * vtkUndirectedGraph_new () {return vtkUndirectedGraph :: New () ;}
extern "C" void vtkUndirectedGraph_destructor (vtkUndirectedGraph * sself) {sself -> Delete () ; return ;}
extern "C" long long vtk_undirected_graph_get_in_degree(vtkUndirectedGraph* sself, long long v) { return sself->GetInDegree(v); }
extern "C" vtkUniformGrid * vtkUniformGrid_new () {return vtkUniformGrid :: New () ;}
extern "C" void vtkUniformGrid_destructor (vtkUniformGrid * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_uniform_grid_get_grid_description(vtkUniformGrid* sself) { return sself->GetGridDescription(); }
extern "C" void vtk_uniform_grid_blank_point(vtkUniformGrid* sself, long long ptId) { sself->BlankPoint(ptId); }
extern "C" void vtk_uniform_grid_un_blank_point(vtkUniformGrid* sself, long long ptId) { sself->UnBlankPoint(ptId); }
extern "C" void vtk_uniform_grid_blank_cell(vtkUniformGrid* sself, long long ptId) { sself->BlankCell(ptId); }
extern "C" void vtk_uniform_grid_un_blank_cell(vtkUniformGrid* sself, long long ptId) { sself->UnBlankCell(ptId); }
extern "C" unsigned char vtk_uniform_grid_is_point_visible(vtkUniformGrid* sself, long long pointId) { return sself->IsPointVisible(pointId); }
extern "C" unsigned char vtk_uniform_grid_is_cell_visible(vtkUniformGrid* sself, long long cellId) { return sself->IsCellVisible(cellId); }
extern "C" vtkUniformGridAMR * vtkUniformGridAMR_new () {return vtkUniformGridAMR :: New () ;}
extern "C" void vtkUniformGridAMR_destructor (vtkUniformGridAMR * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_uniform_grid_amr_get_data_object_type(vtkUniformGridAMR* sself) { return sself->GetDataObjectType(); }
extern "C" void vtk_uniform_grid_amr_initialize(vtkUniformGridAMR* sself) { sself->Initialize(); }
extern "C" void vtk_uniform_grid_amr_set_grid_description(vtkUniformGridAMR* sself, int gridDescription) { sself->SetGridDescription(gridDescription); }
extern "C" int vtk_uniform_grid_amr_get_grid_description(vtkUniformGridAMR* sself) { return sself->GetGridDescription(); }
extern "C" unsigned int vtk_uniform_grid_amr_get_number_of_levels(vtkUniformGridAMR* sself) { return sself->GetNumberOfLevels(); }
extern "C" unsigned int vtk_uniform_grid_amr_get_total_number_of_blocks(vtkUniformGridAMR* sself) { return sself->GetTotalNumberOfBlocks(); }
extern "C" unsigned int vtk_uniform_grid_amr_get_number_of_data_sets(vtkUniformGridAMR* sself, const unsigned int level) { return sself->GetNumberOfDataSets(level); }
extern "C" int vtk_uniform_grid_amr_get_composite_index(vtkUniformGridAMR* sself, const unsigned int level, const unsigned int index) { return sself->GetCompositeIndex(level, index); }
extern "C" void vtk_uniform_grid_amr_get_level_and_index(vtkUniformGridAMR* sself, const unsigned int compositeIdx, unsigned int& level, unsigned int& idx) { sself->GetLevelAndIndex(compositeIdx, level, idx); }
extern "C" vtkUniformGridAMRDataIterator * vtkUniformGridAMRDataIterator_new () {return vtkUniformGridAMRDataIterator :: New () ;}
extern "C" void vtkUniformGridAMRDataIterator_destructor (vtkUniformGridAMRDataIterator * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_uniform_grid_amr_data_iterator_has_current_meta_data(vtkUniformGridAMRDataIterator* sself) { return sself->HasCurrentMetaData(); }
extern "C" unsigned int vtk_uniform_grid_amr_data_iterator_get_current_flat_index(vtkUniformGridAMRDataIterator* sself) { return sself->GetCurrentFlatIndex(); }
extern "C" unsigned int vtk_uniform_grid_amr_data_iterator_get_current_level(vtkUniformGridAMRDataIterator* sself) { return sself->GetCurrentLevel(); }
extern "C" unsigned int vtk_uniform_grid_amr_data_iterator_get_current_index(vtkUniformGridAMRDataIterator* sself) { return sself->GetCurrentIndex(); }
extern "C" void vtk_uniform_grid_amr_data_iterator_go_to_first_item(vtkUniformGridAMRDataIterator* sself) { sself->GoToFirstItem(); }
extern "C" void vtk_uniform_grid_amr_data_iterator_go_to_next_item(vtkUniformGridAMRDataIterator* sself) { sself->GoToNextItem(); }
extern "C" int vtk_uniform_grid_amr_data_iterator_is_done_with_traversal(vtkUniformGridAMRDataIterator* sself) { return sself->IsDoneWithTraversal(); }
extern "C" vtkUniformHyperTreeGrid * vtkUniformHyperTreeGrid_new () {return vtkUniformHyperTreeGrid :: New () ;}
extern "C" void vtkUniformHyperTreeGrid_destructor (vtkUniformHyperTreeGrid * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_uniform_hyper_tree_grid_set_origin(vtkUniformHyperTreeGrid* sself, double _arg1, double _arg2, double _arg3) { sself->SetOrigin(_arg1, _arg2, _arg3); }
extern "C" void vtk_uniform_hyper_tree_grid_set_grid_scale(vtkUniformHyperTreeGrid* sself, double p0, double p1, double p2) { sself->SetGridScale(p0, p1, p2); }
extern "C" unsigned long vtk_uniform_hyper_tree_grid_get_actual_memory_size_bytes(vtkUniformHyperTreeGrid* sself) { return sself->GetActualMemorySizeBytes(); }
extern "C" vtkUnstructuredGrid * vtkUnstructuredGrid_new () {return vtkUnstructuredGrid :: New () ;}
extern "C" void vtkUnstructuredGrid_destructor (vtkUnstructuredGrid * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_unstructured_grid_get_data_object_type(vtkUnstructuredGrid* sself) { return sself->GetDataObjectType(); }
extern "C" bool vtk_unstructured_grid_allocate_estimate(vtkUnstructuredGrid* sself, long long numCells, long long maxCellSize) { return sself->AllocateEstimate(numCells, maxCellSize); }
extern "C" bool vtk_unstructured_grid_allocate_exact(vtkUnstructuredGrid* sself, long long numCells, long long connectivitySize) { return sself->AllocateExact(numCells, connectivitySize); }
extern "C" void vtk_unstructured_grid_allocate(vtkUnstructuredGrid* sself, long long numCells, int extSize) { sself->Allocate(numCells, extSize); }
extern "C" void vtk_unstructured_grid_reset(vtkUnstructuredGrid* sself) { sself->Reset(); }
extern "C" int vtk_unstructured_grid_get_cell_type(vtkUnstructuredGrid* sself, long long cellId) { return sself->GetCellType(cellId); }
extern "C" void vtk_unstructured_grid_squeeze(vtkUnstructuredGrid* sself) { sself->Squeeze(); }
extern "C" void vtk_unstructured_grid_initialize(vtkUnstructuredGrid* sself) { sself->Initialize(); }
extern "C" int vtk_unstructured_grid_get_max_cell_size(vtkUnstructuredGrid* sself) { return sself->GetMaxCellSize(); }
extern "C" void vtk_unstructured_grid_build_links(vtkUnstructuredGrid* sself) { sself->BuildLinks(); }
extern "C" void vtk_unstructured_grid_remove_reference_to_cell(vtkUnstructuredGrid* sself, long long ptId, long long cellId) { sself->RemoveReferenceToCell(ptId, cellId); }
extern "C" void vtk_unstructured_grid_add_reference_to_cell(vtkUnstructuredGrid* sself, long long ptId, long long cellId) { sself->AddReferenceToCell(ptId, cellId); }
extern "C" void vtk_unstructured_grid_resize_cell_list(vtkUnstructuredGrid* sself, long long ptId, int size) { sself->ResizeCellList(ptId, size); }
extern "C" int vtk_unstructured_grid_get_piece(vtkUnstructuredGrid* sself) { return sself->GetPiece(); }
extern "C" int vtk_unstructured_grid_get_number_of_pieces(vtkUnstructuredGrid* sself) { return sself->GetNumberOfPieces(); }
extern "C" int vtk_unstructured_grid_get_ghost_level(vtkUnstructuredGrid* sself) { return sself->GetGhostLevel(); }
extern "C" int vtk_unstructured_grid_is_homogeneous(vtkUnstructuredGrid* sself) { return sself->IsHomogeneous(); }
extern "C" void vtk_unstructured_grid_remove_ghost_cells(vtkUnstructuredGrid* sself) { sself->RemoveGhostCells(); }
extern "C" int vtk_unstructured_grid_initialize_faces_representation(vtkUnstructuredGrid* sself, long long numPrevCells) { return sself->InitializeFacesRepresentation(numPrevCells); }
extern "C" unsigned long vtk_unstructured_grid_get_mesh_m_time(vtkUnstructuredGrid* sself) { return sself->GetMeshMTime(); }
extern "C" vtkUnstructuredGridCellIterator * vtkUnstructuredGridCellIterator_new () {return vtkUnstructuredGridCellIterator :: New () ;}
extern "C" void vtkUnstructuredGridCellIterator_destructor (vtkUnstructuredGridCellIterator * sself) {sself -> Delete () ; return ;}
extern "C" bool vtk_unstructured_grid_cell_iterator_is_done_with_traversal(vtkUnstructuredGridCellIterator* sself) { return sself->IsDoneWithTraversal(); }
extern "C" long long vtk_unstructured_grid_cell_iterator_get_cell_id(vtkUnstructuredGridCellIterator* sself) { return sself->GetCellId(); }
extern "C" void vtk_unstructured_grid_cell_iterator_go_to_cell(vtkUnstructuredGridCellIterator* sself, long long cellId) { sself->GoToCell(cellId); }
extern "C" vtkVertex * vtkVertex_new () {return vtkVertex :: New () ;}
extern "C" void vtkVertex_destructor (vtkVertex * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_vertex_get_cell_type(vtkVertex* sself) { return sself->GetCellType(); }
extern "C" int vtk_vertex_get_cell_dimension(vtkVertex* sself) { return sself->GetCellDimension(); }
extern "C" int vtk_vertex_get_number_of_edges(vtkVertex* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_vertex_get_number_of_faces(vtkVertex* sself) { return sself->GetNumberOfFaces(); }
extern "C" int vtk_vertex_inflate(vtkVertex* sself, double p0) { return sself->Inflate(p0); }
extern "C" vtkVertexListIterator * vtkVertexListIterator_new () {return vtkVertexListIterator :: New () ;}
extern "C" void vtkVertexListIterator_destructor (vtkVertexListIterator * sself) {sself -> Delete () ; return ;}
extern "C" long long vtk_vertex_list_iterator_next(vtkVertexListIterator* sself) { return sself->Next(); }
extern "C" bool vtk_vertex_list_iterator_has_next(vtkVertexListIterator* sself) { return sself->HasNext(); }
extern "C" vtkVoxel * vtkVoxel_new () {return vtkVoxel :: New () ;}
extern "C" void vtkVoxel_destructor (vtkVoxel * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_voxel_get_cell_type(vtkVoxel* sself) { return sself->GetCellType(); }
extern "C" int vtk_voxel_get_number_of_edges(vtkVoxel* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_voxel_get_number_of_faces(vtkVoxel* sself) { return sself->GetNumberOfFaces(); }
extern "C" int vtk_voxel_inflate(vtkVoxel* sself, double dist) { return sself->Inflate(dist); }
extern "C" vtkWedge * vtkWedge_new () {return vtkWedge :: New () ;}
extern "C" void vtkWedge_destructor (vtkWedge * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_wedge_get_cell_type(vtkWedge* sself) { return sself->GetCellType(); }
extern "C" int vtk_wedge_get_number_of_edges(vtkWedge* sself) { return sself->GetNumberOfEdges(); }
extern "C" int vtk_wedge_get_number_of_faces(vtkWedge* sself) { return sself->GetNumberOfFaces(); }
extern "C" vtkXMLDataElement * vtkXMLDataElement_new () {return vtkXMLDataElement :: New () ;}
extern "C" void vtkXMLDataElement_destructor (vtkXMLDataElement * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_xml_data_element_set_name(vtkXMLDataElement* sself, const char* _arg) { sself->SetName(_arg); }
extern "C" void vtk_xml_data_element_set_id(vtkXMLDataElement* sself, const char* _arg) { sself->SetId(_arg); }
extern "C" const char* vtk_xml_data_element_get_attribute(vtkXMLDataElement* sself, const char* name) { return sself->GetAttribute(name); }
extern "C" void vtk_xml_data_element_set_attribute(vtkXMLDataElement* sself, const char* name, const char* value) { sself->SetAttribute(name, value); }
extern "C" void vtk_xml_data_element_set_character_data(vtkXMLDataElement* sself, const char* data, int length) { sself->SetCharacterData(data, length); }
extern "C" void vtk_xml_data_element_add_character_data(vtkXMLDataElement* sself, const char* c, size_t length) { sself->AddCharacterData(c, length); }
extern "C" int vtk_xml_data_element_get_scalar_attribute(vtkXMLDataElement* sself, const char* name, int& value) { return sself->GetScalarAttribute(name, value); }
extern "C" void vtk_xml_data_element_set_int_attribute(vtkXMLDataElement* sself, const char* name, int value) { sself->SetIntAttribute(name, value); }
extern "C" void vtk_xml_data_element_set_float_attribute(vtkXMLDataElement* sself, const char* name, float value) { sself->SetFloatAttribute(name, value); }
extern "C" void vtk_xml_data_element_set_double_attribute(vtkXMLDataElement* sself, const char* name, double value) { sself->SetDoubleAttribute(name, value); }
extern "C" void vtk_xml_data_element_set_unsigned_long_attribute(vtkXMLDataElement* sself, const char* name, unsigned long value) { sself->SetUnsignedLongAttribute(name, value); }
extern "C" int vtk_xml_data_element_get_word_type_attribute(vtkXMLDataElement* sself, const char* name, int& value) { return sself->GetWordTypeAttribute(name, value); }
extern "C" int vtk_xml_data_element_get_number_of_attributes(vtkXMLDataElement* sself) { return sself->GetNumberOfAttributes(); }
extern "C" const char* vtk_xml_data_element_get_attribute_name(vtkXMLDataElement* sself, int idx) { return sself->GetAttributeName(idx); }
extern "C" const char* vtk_xml_data_element_get_attribute_value(vtkXMLDataElement* sself, int idx) { return sself->GetAttributeValue(idx); }
extern "C" void vtk_xml_data_element_remove_attribute(vtkXMLDataElement* sself, const char* name) { sself->RemoveAttribute(name); }
extern "C" void vtk_xml_data_element_remove_all_attributes(vtkXMLDataElement* sself) { sself->RemoveAllAttributes(); }
extern "C" int vtk_xml_data_element_get_number_of_nested_elements(vtkXMLDataElement* sself) { return sself->GetNumberOfNestedElements(); }
extern "C" void vtk_xml_data_element_remove_all_nested_elements(vtkXMLDataElement* sself) { sself->RemoveAllNestedElements(); }
extern "C" long long vtk_xml_data_element_get_xml_byte_index(vtkXMLDataElement* sself) { return sself->GetXMLByteIndex(); }
extern "C" void vtk_xml_data_element_set_xml_byte_index(vtkXMLDataElement* sself, long long _arg) { sself->SetXMLByteIndex(_arg); }
extern "C" void vtk_xml_data_element_set_attribute_encoding(vtkXMLDataElement* sself, int _arg) { sself->SetAttributeEncoding(_arg); }
extern "C" int vtk_xml_data_element_get_attribute_encoding_min_value(vtkXMLDataElement* sself) { return sself->GetAttributeEncodingMinValue(); }
extern "C" int vtk_xml_data_element_get_attribute_encoding_max_value(vtkXMLDataElement* sself) { return sself->GetAttributeEncodingMaxValue(); }
extern "C" int vtk_xml_data_element_get_attribute_encoding(vtkXMLDataElement* sself) { return sself->GetAttributeEncoding(); }
extern "C" void vtk_xml_data_element_print_xml(vtkXMLDataElement* sself, const char* fname) { sself->PrintXML(fname); }
extern "C" int vtk_xml_data_element_get_character_data_width(vtkXMLDataElement* sself) { return sself->GetCharacterDataWidth(); }
extern "C" void vtk_xml_data_element_set_character_data_width(vtkXMLDataElement* sself, int _arg) { sself->SetCharacterDataWidth(_arg); }
