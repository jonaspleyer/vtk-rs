// Include header file
#include <vtk_common_data_model.h>

// Default include in all modules
#include <vtkNew.h>
#include <vtkObjectBase.h>

// Include objects of this module
#include <vtkAMRBox.h>
#include <vtkAMRDataInternals.h>
#include <vtkAMRDataObject.h>
#include <vtkAMRInformation.h>
#include <vtkAMRMetaData.h>
#include <vtkAMRUtilities.h>
#include <vtkAbstractCellArray.h>
#include <vtkAbstractCellLinks.h>
#include <vtkAbstractCellLocator.h>
#include <vtkAbstractElectronicData.h>
#include <vtkAbstractPointLocator.h>
#include <vtkAdjacentVertexIterator.h>
#include <vtkAngularPeriodicDataArray.h>
#include <vtkAnimationScene.h>
#include <vtkAnnotation.h>
#include <vtkAnnotationLayers.h>
#include <vtkAnnulus.h>
#include <vtkArrayData.h>
#include <vtkAtom.h>
#include <vtkAttributesErrorMetric.h>
#include <vtkBSPCuts.h>
#include <vtkBSPIntersections.h>
#include <vtkBezierCurve.h>
#include <vtkBezierHexahedron.h>
#include <vtkBezierInterpolation.h>
#include <vtkBezierQuadrilateral.h>
#include <vtkBezierTetra.h>
#include <vtkBezierTriangle.h>
#include <vtkBezierWedge.h>
#include <vtkBiQuadraticQuad.h>
#include <vtkBiQuadraticQuadraticHexahedron.h>
#include <vtkBiQuadraticQuadraticWedge.h>
#include <vtkBiQuadraticTriangle.h>
#include <vtkBond.h>
#include <vtkBoundingBox.h>
#include <vtkBox.h>
#include <vtkCartesianGrid.h>
#include <vtkCell.h>
#include <vtkCell3D.h>
#include <vtkCellArray.h>
#include <vtkCellArrayIterator.h>
#include <vtkCellAttribute.h>
#include <vtkCellAttributeCalculator.h>
#include <vtkCellData.h>
#include <vtkCellGrid.h>
#include <vtkCellGridBoundsQuery.h>
#include <vtkCellGridCopyQuery.h>
#include <vtkCellGridEvaluator.h>
#include <vtkCellGridQuery.h>
#include <vtkCellGridRangeQuery.h>
#include <vtkCellGridResponder.h>
#include <vtkCellGridResponderBase.h>
#include <vtkCellGridResponders.h>
#include <vtkCellGridSidesCache.h>
#include <vtkCellGridSidesQuery.h>
#include <vtkCellIterator.h>
#include <vtkCellLinks.h>
#include <vtkCellLocator.h>
#include <vtkCellLocatorStrategy.h>
#include <vtkCellMetadata.h>
#include <vtkCellTreeLocator.h>
#include <vtkCellTypeUtilities.h>
#include <vtkCellTypes.h>
#include <vtkClosestNPointsStrategy.h>
#include <vtkClosestPointStrategy.h>
#include <vtkColor.h>
#include <vtkCompositeDataIterator.h>
#include <vtkCompositeDataSet.h>
#include <vtkCone.h>
#include <vtkConvexPointSet.h>
#include <vtkCoordinateFrame.h>
#include <vtkCubicLine.h>
#include <vtkCylinder.h>
#include <vtkDataAssembly.h>
#include <vtkDataAssemblyUtilities.h>
#include <vtkDataAssemblyVisitor.h>
#include <vtkDataObject.h>
#include <vtkDataObjectCollection.h>
#include <vtkDataObjectTree.h>
#include <vtkDataObjectTreeInternals.h>
#include <vtkDataObjectTreeIterator.h>
#include <vtkDataObjectTypes.h>
#include <vtkDataSet.h>
#include <vtkDataSetAttributes.h>
#include <vtkDataSetAttributesFieldList.h>
#include <vtkDataSetCellIterator.h>
#include <vtkDataSetCollection.h>
#include <vtkDirectedAcyclicGraph.h>
#include <vtkDirectedGraph.h>
#include <vtkDistributedGraphHelper.h>
#include <vtkEdgeListIterator.h>
#include <vtkEdgeTable.h>
#include <vtkEmptyCell.h>
#include <vtkExplicitStructuredGrid.h>
#include <vtkExtractStructuredGridHelper.h>
#include <vtkFieldData.h>
#include <vtkFindCellStrategy.h>
#include <vtkFrustum.h>
#include <vtkGenericAdaptorCell.h>
#include <vtkGenericAttribute.h>
#include <vtkGenericAttributeCollection.h>
#include <vtkGenericCell.h>
#include <vtkGenericCellIterator.h>
#include <vtkGenericCellTessellator.h>
#include <vtkGenericDataSet.h>
#include <vtkGenericEdgeTable.h>
#include <vtkGenericInterpolatedVelocityField.h>
#include <vtkGenericPointIterator.h>
#include <vtkGenericSubdivisionErrorMetric.h>
#include <vtkGeometricErrorMetric.h>
#include <vtkGraph.h>
#include <vtkGraphEdge.h>
#include <vtkGraphInternals.h>
#include <vtkHexagonalPrism.h>
#include <vtkHexahedron.h>
#include <vtkHierarchicalBoxDataSet.h>
#include <vtkHigherOrderCurve.h>
#include <vtkHigherOrderHexahedron.h>
#include <vtkHigherOrderInterpolation.h>
#include <vtkHigherOrderQuadrilateral.h>
#include <vtkHigherOrderTetra.h>
#include <vtkHigherOrderTriangle.h>
#include <vtkHigherOrderWedge.h>
#include <vtkHyperTree.h>
#include <vtkHyperTreeGrid.h>
#include <vtkHyperTreeGridGeometricLocator.h>
#include <vtkHyperTreeGridLocator.h>
#include <vtkHyperTreeGridNonOrientedCursor.h>
#include <vtkHyperTreeGridNonOrientedGeometryCursor.h>
#include <vtkHyperTreeGridNonOrientedMooreSuperCursor.h>
#include <vtkHyperTreeGridNonOrientedMooreSuperCursorLight.h>
#include <vtkHyperTreeGridNonOrientedSuperCursor.h>
#include <vtkHyperTreeGridNonOrientedSuperCursorLight.h>
#include <vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor.h>
#include <vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor.h>
#include <vtkHyperTreeGridNonOrientedUnlimitedSuperCursor.h>
#include <vtkHyperTreeGridNonOrientedVonNeumannSuperCursor.h>
#include <vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight.h>
#include <vtkHyperTreeGridOrientedCursor.h>
#include <vtkHyperTreeGridOrientedGeometryCursor.h>
#include <vtkHyperTreeGridScales.h>
#include <vtkImageData.h>
#include <vtkImageIterator.h>
#include <vtkImageTransform.h>
#include <vtkImplicitBoolean.h>
#include <vtkImplicitDataSet.h>
#include <vtkImplicitFunction.h>
#include <vtkImplicitFunctionCollection.h>
#include <vtkImplicitHalo.h>
#include <vtkImplicitSelectionLoop.h>
#include <vtkImplicitSum.h>
#include <vtkImplicitVolume.h>
#include <vtkImplicitWindowFunction.h>
#include <vtkInEdgeIterator.h>
#include <vtkIncrementalOctreeNode.h>
#include <vtkIncrementalOctreePointLocator.h>
#include <vtkIncrementalPointLocator.h>
#include <vtkInformationQuadratureSchemeDefinitionVectorKey.h>
#include <vtkIntersectionCounter.h>
#include <vtkIterativeClosestPointTransform.h>
#include <vtkKdNode.h>
#include <vtkKdTree.h>
#include <vtkKdTreePointLocator.h>
#include <vtkLagrangeCurve.h>
#include <vtkLagrangeHexahedron.h>
#include <vtkLagrangeInterpolation.h>
#include <vtkLagrangeQuadrilateral.h>
#include <vtkLagrangeTetra.h>
#include <vtkLagrangeTriangle.h>
#include <vtkLagrangeWedge.h>
#include <vtkLine.h>
#include <vtkLocator.h>
#include <vtkMappedUnstructuredGrid.h>
#include <vtkMappedUnstructuredGridCellIterator.h>
#include <vtkMeanValueCoordinatesInterpolator.h>
#include <vtkMergePoints.h>
#include <vtkMolecule.h>
#include <vtkMultiBlockDataSet.h>
#include <vtkMultiPieceDataSet.h>
#include <vtkMutableDirectedGraph.h>
#include <vtkMutableUndirectedGraph.h>
#include <vtkNonLinearCell.h>
#include <vtkNonMergingPointLocator.h>
#include <vtkNonOverlappingAMR.h>
#include <vtkOctreePointLocator.h>
#include <vtkOctreePointLocatorNode.h>
#include <vtkOrderedTriangulator.h>
#include <vtkOutEdgeIterator.h>
#include <vtkOverlappingAMR.h>
#include <vtkOverlappingAMRMetaData.h>
#include <vtkPartitionedDataSet.h>
#include <vtkPartitionedDataSetCollection.h>
#include <vtkPath.h>
#include <vtkPentagonalPrism.h>
#include <vtkPeriodicDataArray.h>
#include <vtkPerlinNoise.h>
#include <vtkPiecewiseFunction.h>
#include <vtkPixel.h>
#include <vtkPixelExtent.h>
#include <vtkPixelTransfer.h>
#include <vtkPlane.h>
#include <vtkPlaneCollection.h>
#include <vtkPlanes.h>
#include <vtkPlanesIntersection.h>
#include <vtkPointData.h>
#include <vtkPointLocator.h>
#include <vtkPointSet.h>
#include <vtkPointSetCellIterator.h>
#include <vtkPointsProjectedHull.h>
#include <vtkPolyData.h>
#include <vtkPolyDataCollection.h>
#include <vtkPolyLine.h>
#include <vtkPolyPlane.h>
#include <vtkPolyVertex.h>
#include <vtkPolygon.h>
#include <vtkPolyhedron.h>
#include <vtkPolyhedronUtilities.h>
#include <vtkPyramid.h>
#include <vtkQuad.h>
#include <vtkQuadraticEdge.h>
#include <vtkQuadraticHexahedron.h>
#include <vtkQuadraticLinearQuad.h>
#include <vtkQuadraticLinearWedge.h>
#include <vtkQuadraticPolygon.h>
#include <vtkQuadraticPyramid.h>
#include <vtkQuadraticQuad.h>
#include <vtkQuadraticTetra.h>
#include <vtkQuadraticTriangle.h>
#include <vtkQuadraticWedge.h>
#include <vtkQuadratureSchemeDefinition.h>
#include <vtkQuadric.h>
#include <vtkRect.h>
#include <vtkRectilinearGrid.h>
#include <vtkReebGraph.h>
#include <vtkReebGraphSimplificationMetric.h>
#include <vtkSelection.h>
#include <vtkSelectionNode.h>
#include <vtkSimpleCellTessellator.h>
#include <vtkSmoothErrorMetric.h>
#include <vtkSortFieldData.h>
#include <vtkSphere.h>
#include <vtkSpheres.h>
#include <vtkSphericalPointIterator.h>
#include <vtkSpline.h>
#include <vtkStaticCellLinks.h>
#include <vtkStaticCellLinksTemplate.h>
#include <vtkStaticCellLocator.h>
#include <vtkStaticEdgeLocatorTemplate.h>
#include <vtkStaticFaceHashLinksTemplate.h>
#include <vtkStaticPointLocator.h>
#include <vtkStaticPointLocator2D.h>
#include <vtkStatisticalModel.h>
#include <vtkStructuredCellArray.h>
#include <vtkStructuredData.h>
#include <vtkStructuredExtent.h>
#include <vtkStructuredGrid.h>
#include <vtkStructuredPoints.h>
#include <vtkStructuredPointsCollection.h>
#include <vtkSuperquadric.h>
#include <vtkTable.h>
#include <vtkTetra.h>
#include <vtkTree.h>
#include <vtkTreeBFSIterator.h>
#include <vtkTreeDFSIterator.h>
#include <vtkTreeIterator.h>
#include <vtkTriQuadraticHexahedron.h>
#include <vtkTriQuadraticPyramid.h>
#include <vtkTriangle.h>
#include <vtkTriangleStrip.h>
#include <vtkUndirectedGraph.h>
#include <vtkUniformGrid.h>
#include <vtkUniformGridAMR.h>
#include <vtkUniformGridAMRDataIterator.h>
#include <vtkUniformGridAMRIterator.h>
#include <vtkUniformHyperTreeGrid.h>
#include <vtkUnstructuredGrid.h>
#include <vtkUnstructuredGridBase.h>
#include <vtkUnstructuredGridCellIterator.h>
#include <vtkVector.h>
#include <vtkVertex.h>
#include <vtkVertexListIterator.h>
#include <vtkVoxel.h>
#include <vtkWedge.h>
#include <vtkXMLDataElement.h>

// Implement declared functions
extern "C" vtkNew<vtkAMRDataInternals> vtkAMRDataInternals_new() {
    return vtkNew<vtkAMRDataInternals>();
}
extern "C" void vtkAMRDataInternals_destructor(vtkNew<vtkAMRDataInternals> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAMRDataInternals_get_ptr(vtkNew<vtkAMRDataInternals> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_amr_data_internals_initialize(vtkNew<vtkAMRDataInternals> self) {
    return self->Initialize();
}
extern "C" bool vtk_amr_data_internals_empty(vtkNew<vtkAMRDataInternals> self) {
    return self->Empty();
}
extern "C" unsigned int vtk_amr_data_internals_get_number_of_blocks(
    vtkNew<vtkAMRDataInternals> self
) {
    return self->GetNumberOfBlocks();
}
extern "C" vtkNew<vtkAMRDataObject> vtkAMRDataObject_new() {
    return vtkNew<vtkAMRDataObject>();
}
extern "C" void vtkAMRDataObject_destructor(vtkNew<vtkAMRDataObject> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAMRDataObject_get_ptr(vtkNew<vtkAMRDataObject> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_amr_data_object_get_data_object_type(vtkNew<vtkAMRDataObject> self) {
    return self->GetDataObjectType();
}
extern "C" void vtk_amr_data_object_initialize(vtkNew<vtkAMRDataObject> self) {
    return self->Initialize();
}
extern "C" void vtk_amr_data_object_set_grid_description(
    vtkNew<vtkAMRDataObject> self, int gridDescription
) {
    return self->SetGridDescription(gridDescription);
}
extern "C" int vtk_amr_data_object_get_grid_description(vtkNew<vtkAMRDataObject> self) {
    return self->GetGridDescription();
}
extern "C" unsigned int vtk_amr_data_object_get_number_of_levels(vtkNew<vtkAMRDataObject> self) {
    return self->GetNumberOfLevels();
}
extern "C" unsigned int vtk_amr_data_object_get_number_of_blocks(vtkNew<vtkAMRDataObject> self) {
    return self->GetNumberOfBlocks();
}
extern "C" unsigned int vtk_amr_data_object_get_total_number_of_blocks(
    vtkNew<vtkAMRDataObject> self
) {
    return self->GetTotalNumberOfBlocks();
}
extern "C" unsigned int vtk_amr_data_object_get_number_of_blocks_c_uint(
    vtkNew<vtkAMRDataObject> self, unsigned int level
) {
    return self->GetNumberOfBlocks(level);
}
extern "C" unsigned int vtk_amr_data_object_get_number_of_data_sets(
    vtkNew<vtkAMRDataObject> self, unsigned int level
) {
    return self->GetNumberOfDataSets(level);
}
extern "C" void vtk_amr_data_object_get_bounds_double_6(
    vtkNew<vtkAMRDataObject> self, double* bounds
) {
    return self->GetBounds(bounds);
}
extern "C" void vtk_amr_data_object_get_min(vtkNew<vtkAMRDataObject> self, double* min) {
    return self->GetMin(min);
}
extern "C" void vtk_amr_data_object_get_max(vtkNew<vtkAMRDataObject> self, double* max) {
    return self->GetMax(max);
}
extern "C" int vtk_amr_data_object_get_absolute_block_index(
    vtkNew<vtkAMRDataObject> self, unsigned int level, unsigned int index
) {
    return self->GetAbsoluteBlockIndex(level, index);
}
extern "C" int vtk_amr_data_object_get_composite_index(
    vtkNew<vtkAMRDataObject> self, unsigned int level, unsigned int index
) {
    return self->GetCompositeIndex(level, index);
}
extern "C" vtkNew<vtkAMRMetaData> vtkAMRMetaData_new() {
    return vtkNew<vtkAMRMetaData>();
}
extern "C" void vtkAMRMetaData_destructor(vtkNew<vtkAMRMetaData> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAMRMetaData_get_ptr(vtkNew<vtkAMRMetaData> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_amr_meta_data_get_grid_description(vtkNew<vtkAMRMetaData> self) {
    return self->GetGridDescription();
}
extern "C" void vtk_amr_meta_data_set_grid_description(
    vtkNew<vtkAMRMetaData> self, int description
) {
    return self->SetGridDescription(description);
}
extern "C" unsigned int vtk_amr_meta_data_get_number_of_levels(vtkNew<vtkAMRMetaData> self) {
    return self->GetNumberOfLevels();
}
extern "C" unsigned int vtk_amr_meta_data_get_number_of_blocks_c_uint(
    vtkNew<vtkAMRMetaData> self, unsigned int level
) {
    return self->GetNumberOfBlocks(level);
}
extern "C" unsigned int vtk_amr_meta_data_get_number_of_data_sets(
    vtkNew<vtkAMRMetaData> self, unsigned int level
) {
    return self->GetNumberOfDataSets(level);
}
extern "C" unsigned int vtk_amr_meta_data_get_number_of_blocks(vtkNew<vtkAMRMetaData> self) {
    return self->GetNumberOfBlocks();
}
extern "C" unsigned int vtk_amr_meta_data_get_total_number_of_blocks(vtkNew<vtkAMRMetaData> self) {
    return self->GetTotalNumberOfBlocks();
}
extern "C" int vtk_amr_meta_data_get_absolute_block_index(
    vtkNew<vtkAMRMetaData> self, unsigned int level, unsigned int relativeBlockIdx
) {
    return self->GetAbsoluteBlockIndex(level, relativeBlockIdx);
}
extern "C" int vtk_amr_meta_data_get_index(
    vtkNew<vtkAMRMetaData> self, unsigned int level, unsigned int id
) {
    return self->GetIndex(level, id);
}
extern "C" vtkNew<vtkAdjacentVertexIterator> vtkAdjacentVertexIterator_new() {
    return vtkNew<vtkAdjacentVertexIterator>();
}
extern "C" void vtkAdjacentVertexIterator_destructor(vtkNew<vtkAdjacentVertexIterator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAdjacentVertexIterator_get_ptr(vtkNew<vtkAdjacentVertexIterator> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_adjacent_vertex_iterator_get_vertex(
    vtkNew<vtkAdjacentVertexIterator> self
) {
    return self->GetVertex();
}
extern "C" long long vtk_adjacent_vertex_iterator_next(vtkNew<vtkAdjacentVertexIterator> self) {
    return self->Next();
}
extern "C" bool vtk_adjacent_vertex_iterator_has_next(vtkNew<vtkAdjacentVertexIterator> self) {
    return self->HasNext();
}
extern "C" vtkNew<vtkAnimationScene> vtkAnimationScene_new() {
    return vtkNew<vtkAnimationScene>();
}
extern "C" void vtkAnimationScene_destructor(vtkNew<vtkAnimationScene> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAnimationScene_get_ptr(vtkNew<vtkAnimationScene> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_animation_scene_set_play_mode(vtkNew<vtkAnimationScene> self, int _arg) {
    return self->SetPlayMode(_arg);
}
extern "C" void vtk_animation_scene_set_mode_to_sequence(vtkNew<vtkAnimationScene> self) {
    return self->SetModeToSequence();
}
extern "C" void vtk_animation_scene_set_mode_to_real_time(vtkNew<vtkAnimationScene> self) {
    return self->SetModeToRealTime();
}
extern "C" int vtk_animation_scene_get_play_mode(vtkNew<vtkAnimationScene> self) {
    return self->GetPlayMode();
}
extern "C" void vtk_animation_scene_set_frame_rate(vtkNew<vtkAnimationScene> self, double _arg) {
    return self->SetFrameRate(_arg);
}
extern "C" double vtk_animation_scene_get_frame_rate(vtkNew<vtkAnimationScene> self) {
    return self->GetFrameRate();
}
extern "C" void vtk_animation_scene_remove_all_cues(vtkNew<vtkAnimationScene> self) {
    return self->RemoveAllCues();
}
extern "C" int vtk_animation_scene_get_number_of_cues(vtkNew<vtkAnimationScene> self) {
    return self->GetNumberOfCues();
}
extern "C" void vtk_animation_scene_play(vtkNew<vtkAnimationScene> self) {
    return self->Play();
}
extern "C" void vtk_animation_scene_stop(vtkNew<vtkAnimationScene> self) {
    return self->Stop();
}
extern "C" void vtk_animation_scene_set_loop(vtkNew<vtkAnimationScene> self, int _arg) {
    return self->SetLoop(_arg);
}
extern "C" int vtk_animation_scene_get_loop(vtkNew<vtkAnimationScene> self) {
    return self->GetLoop();
}
extern "C" void vtk_animation_scene_set_animation_time(
    vtkNew<vtkAnimationScene> self, double time
) {
    return self->SetAnimationTime(time);
}
extern "C" void vtk_animation_scene_set_time_mode(vtkNew<vtkAnimationScene> self, int mode) {
    return self->SetTimeMode(mode);
}
extern "C" int vtk_animation_scene_is_in_play(vtkNew<vtkAnimationScene> self) {
    return self->IsInPlay();
}
extern "C" vtkNew<vtkAnnotation> vtkAnnotation_new() {
    return vtkNew<vtkAnnotation>();
}
extern "C" void vtkAnnotation_destructor(vtkNew<vtkAnnotation> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAnnotation_get_ptr(vtkNew<vtkAnnotation> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_annotation_get_data_object_type(vtkNew<vtkAnnotation> self) {
    return self->GetDataObjectType();
}
extern "C" void vtk_annotation_initialize(vtkNew<vtkAnnotation> self) {
    return self->Initialize();
}
extern "C" unsigned long vtk_annotation_get_m_time(vtkNew<vtkAnnotation> self) {
    return self->GetMTime();
}
extern "C" vtkNew<vtkAnnotationLayers> vtkAnnotationLayers_new() {
    return vtkNew<vtkAnnotationLayers>();
}
extern "C" void vtkAnnotationLayers_destructor(vtkNew<vtkAnnotationLayers> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAnnotationLayers_get_ptr(vtkNew<vtkAnnotationLayers> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_annotation_layers_get_data_object_type(vtkNew<vtkAnnotationLayers> self) {
    return self->GetDataObjectType();
}
extern "C" unsigned int vtk_annotation_layers_get_number_of_annotations(
    vtkNew<vtkAnnotationLayers> self
) {
    return self->GetNumberOfAnnotations();
}
extern "C" void vtk_annotation_layers_initialize(vtkNew<vtkAnnotationLayers> self) {
    return self->Initialize();
}
extern "C" unsigned long vtk_annotation_layers_get_m_time(vtkNew<vtkAnnotationLayers> self) {
    return self->GetMTime();
}
extern "C" vtkNew<vtkAnnulus> vtkAnnulus_new() {
    return vtkNew<vtkAnnulus>();
}
extern "C" void vtkAnnulus_destructor(vtkNew<vtkAnnulus> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAnnulus_get_ptr(vtkNew<vtkAnnulus> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_annulus_evaluate_function(vtkNew<vtkAnnulus> self, double* x) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_annulus_evaluate_gradient(vtkNew<vtkAnnulus> self, double* x, double* g) {
    return self->EvaluateGradient(x, g);
}
extern "C" void vtk_annulus_set_inner_radius(vtkNew<vtkAnnulus> self, double radius) {
    return self->SetInnerRadius(radius);
}
extern "C" double vtk_annulus_get_inner_radius(vtkNew<vtkAnnulus> self) {
    return self->GetInnerRadius();
}
extern "C" void vtk_annulus_set_outer_radius(vtkNew<vtkAnnulus> self, double radius) {
    return self->SetOuterRadius(radius);
}
extern "C" double vtk_annulus_get_outer_radius(vtkNew<vtkAnnulus> self) {
    return self->GetOuterRadius();
}
extern "C" void vtk_annulus_set_center_double_double_double(
    vtkNew<vtkAnnulus> self, double x, double y, double z
) {
    return self->SetCenter(x, y, z);
}
extern "C" void vtk_annulus_set_center_const_double_3(vtkNew<vtkAnnulus> self, const double* xyz) {
    return self->SetCenter(xyz);
}
extern "C" void vtk_annulus_get_center_double_3(vtkNew<vtkAnnulus> self, double* xyz) {
    return self->GetCenter(xyz);
}
extern "C" void vtk_annulus_set_axis_double_double_double(
    vtkNew<vtkAnnulus> self, double x, double y, double z
) {
    return self->SetAxis(x, y, z);
}
extern "C" void vtk_annulus_set_axis_double_3(vtkNew<vtkAnnulus> self, double* axis) {
    return self->SetAxis(axis);
}
extern "C" void vtk_annulus_get_axis_double_3(vtkNew<vtkAnnulus> self, double* xyz) {
    return self->GetAxis(xyz);
}
extern "C" vtkNew<vtkArrayData> vtkArrayData_new() {
    return vtkNew<vtkArrayData>();
}
extern "C" void vtkArrayData_destructor(vtkNew<vtkArrayData> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkArrayData_get_ptr(vtkNew<vtkArrayData> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_array_data_clear_arrays(vtkNew<vtkArrayData> self) {
    return self->ClearArrays();
}
extern "C" long long vtk_array_data_get_number_of_arrays(vtkNew<vtkArrayData> self) {
    return self->GetNumberOfArrays();
}
extern "C" int vtk_array_data_get_data_object_type(vtkNew<vtkArrayData> self) {
    return self->GetDataObjectType();
}
extern "C" vtkNew<vtkAttributesErrorMetric> vtkAttributesErrorMetric_new() {
    return vtkNew<vtkAttributesErrorMetric>();
}
extern "C" void vtkAttributesErrorMetric_destructor(vtkNew<vtkAttributesErrorMetric> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAttributesErrorMetric_get_ptr(vtkNew<vtkAttributesErrorMetric> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_attributes_error_metric_get_absolute_attribute_tolerance(
    vtkNew<vtkAttributesErrorMetric> self
) {
    return self->GetAbsoluteAttributeTolerance();
}
extern "C" void vtk_attributes_error_metric_set_absolute_attribute_tolerance(
    vtkNew<vtkAttributesErrorMetric> self, double value
) {
    return self->SetAbsoluteAttributeTolerance(value);
}
extern "C" double vtk_attributes_error_metric_get_attribute_tolerance(
    vtkNew<vtkAttributesErrorMetric> self
) {
    return self->GetAttributeTolerance();
}
extern "C" void vtk_attributes_error_metric_set_attribute_tolerance(
    vtkNew<vtkAttributesErrorMetric> self, double value
) {
    return self->SetAttributeTolerance(value);
}
extern "C" vtkNew<vtkBSPCuts> vtkBSPCuts_new() {
    return vtkNew<vtkBSPCuts>();
}
extern "C" void vtkBSPCuts_destructor(vtkNew<vtkBSPCuts> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkBSPCuts_get_ptr(vtkNew<vtkBSPCuts> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_bsp_cuts_get_data_object_type(vtkNew<vtkBSPCuts> self) {
    return self->GetDataObjectType();
}
extern "C" int vtk_bsp_cuts_get_number_of_cuts(vtkNew<vtkBSPCuts> self) {
    return self->GetNumberOfCuts();
}
extern "C" void vtk_bsp_cuts_print_tree(vtkNew<vtkBSPCuts> self) {
    return self->PrintTree();
}
extern "C" void vtk_bsp_cuts_print_arrays(vtkNew<vtkBSPCuts> self) {
    return self->PrintArrays();
}
extern "C" vtkNew<vtkBSPIntersections> vtkBSPIntersections_new() {
    return vtkNew<vtkBSPIntersections>();
}
extern "C" void vtkBSPIntersections_destructor(vtkNew<vtkBSPIntersections> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkBSPIntersections_get_ptr(vtkNew<vtkBSPIntersections> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_bsp_intersections_get_number_of_regions(vtkNew<vtkBSPIntersections> self) {
    return self->GetNumberOfRegions();
}
extern "C" int vtk_bsp_intersections_get_region_bounds(
    vtkNew<vtkBSPIntersections> self, int regionID, double* bounds
) {
    return self->GetRegionBounds(regionID, bounds);
}
extern "C" int vtk_bsp_intersections_get_region_data_bounds(
    vtkNew<vtkBSPIntersections> self, int regionID, double* bounds
) {
    return self->GetRegionDataBounds(regionID, bounds);
}
extern "C" int vtk_bsp_intersections_intersects_box_c_int_double_double_double_double_double_double(
    vtkNew<vtkBSPIntersections> self,
    int regionId,
    double x0,
    double x1,
    double y0,
    double y1,
    double z0,
    double z1
) {
    return self->IntersectsBox(regionId, x0, x1, y0, y1, z0, z1);
}
extern "C" int vtk_bsp_intersections_intersects_sphere_2_c_int_double_double_double_double(
    vtkNew<vtkBSPIntersections> self, int regionId, double x, double y, double z, double rSquared
) {
    return self->IntersectsSphere2(regionId, x, y, z, rSquared);
}
extern "C" int vtk_bsp_intersections_get_compute_intersections_using_data_bounds(
    vtkNew<vtkBSPIntersections> self
) {
    return self->GetComputeIntersectionsUsingDataBounds();
}
extern "C" void vtk_bsp_intersections_set_compute_intersections_using_data_bounds(
    vtkNew<vtkBSPIntersections> self, int c
) {
    return self->SetComputeIntersectionsUsingDataBounds(c);
}
extern "C" void vtk_bsp_intersections_compute_intersections_using_data_bounds_on(
    vtkNew<vtkBSPIntersections> self
) {
    return self->ComputeIntersectionsUsingDataBoundsOn();
}
extern "C" void vtk_bsp_intersections_compute_intersections_using_data_bounds_off(
    vtkNew<vtkBSPIntersections> self
) {
    return self->ComputeIntersectionsUsingDataBoundsOff();
}
extern "C" vtkNew<vtkBezierCurve> vtkBezierCurve_new() {
    return vtkNew<vtkBezierCurve>();
}
extern "C" void vtkBezierCurve_destructor(vtkNew<vtkBezierCurve> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkBezierCurve_get_ptr(vtkNew<vtkBezierCurve> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_bezier_curve_get_cell_type(vtkNew<vtkBezierCurve> self) {
    return self->GetCellType();
}
extern "C" vtkNew<vtkBezierHexahedron> vtkBezierHexahedron_new() {
    return vtkNew<vtkBezierHexahedron>();
}
extern "C" void vtkBezierHexahedron_destructor(vtkNew<vtkBezierHexahedron> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkBezierHexahedron_get_ptr(vtkNew<vtkBezierHexahedron> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_bezier_hexahedron_get_cell_type(vtkNew<vtkBezierHexahedron> self) {
    return self->GetCellType();
}
extern "C" vtkNew<vtkBezierInterpolation> vtkBezierInterpolation_new() {
    return vtkNew<vtkBezierInterpolation>();
}
extern "C" void vtkBezierInterpolation_destructor(vtkNew<vtkBezierInterpolation> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkBezierInterpolation_get_ptr(vtkNew<vtkBezierInterpolation> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkBezierQuadrilateral> vtkBezierQuadrilateral_new() {
    return vtkNew<vtkBezierQuadrilateral>();
}
extern "C" void vtkBezierQuadrilateral_destructor(vtkNew<vtkBezierQuadrilateral> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkBezierQuadrilateral_get_ptr(vtkNew<vtkBezierQuadrilateral> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_bezier_quadrilateral_get_cell_type(vtkNew<vtkBezierQuadrilateral> self) {
    return self->GetCellType();
}
extern "C" vtkNew<vtkBezierTetra> vtkBezierTetra_new() {
    return vtkNew<vtkBezierTetra>();
}
extern "C" void vtkBezierTetra_destructor(vtkNew<vtkBezierTetra> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkBezierTetra_get_ptr(vtkNew<vtkBezierTetra> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_bezier_tetra_get_cell_type(vtkNew<vtkBezierTetra> self) {
    return self->GetCellType();
}
extern "C" vtkNew<vtkBezierTriangle> vtkBezierTriangle_new() {
    return vtkNew<vtkBezierTriangle>();
}
extern "C" void vtkBezierTriangle_destructor(vtkNew<vtkBezierTriangle> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkBezierTriangle_get_ptr(vtkNew<vtkBezierTriangle> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_bezier_triangle_get_cell_type(vtkNew<vtkBezierTriangle> self) {
    return self->GetCellType();
}
extern "C" vtkNew<vtkBezierWedge> vtkBezierWedge_new() {
    return vtkNew<vtkBezierWedge>();
}
extern "C" void vtkBezierWedge_destructor(vtkNew<vtkBezierWedge> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkBezierWedge_get_ptr(vtkNew<vtkBezierWedge> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_bezier_wedge_get_cell_type(vtkNew<vtkBezierWedge> self) {
    return self->GetCellType();
}
extern "C" vtkNew<vtkBiQuadraticQuad> vtkBiQuadraticQuad_new() {
    return vtkNew<vtkBiQuadraticQuad>();
}
extern "C" void vtkBiQuadraticQuad_destructor(vtkNew<vtkBiQuadraticQuad> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkBiQuadraticQuad_get_ptr(vtkNew<vtkBiQuadraticQuad> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_bi_quadratic_quad_get_cell_type(vtkNew<vtkBiQuadraticQuad> self) {
    return self->GetCellType();
}
extern "C" int vtk_bi_quadratic_quad_get_cell_dimension(vtkNew<vtkBiQuadraticQuad> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_bi_quadratic_quad_get_number_of_edges(vtkNew<vtkBiQuadraticQuad> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_bi_quadratic_quad_get_number_of_faces(vtkNew<vtkBiQuadraticQuad> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_bi_quadratic_quad_get_parametric_center(
    vtkNew<vtkBiQuadraticQuad> self, double* pcoords
) {
    return self->GetParametricCenter(pcoords);
}
extern "C" void vtk_bi_quadratic_quad_interpolate_functions(
    vtkNew<vtkBiQuadraticQuad> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_bi_quadratic_quad_interpolate_derivs(
    vtkNew<vtkBiQuadraticQuad> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkBiQuadraticQuadraticHexahedron> vtkBiQuadraticQuadraticHexahedron_new() {
    return vtkNew<vtkBiQuadraticQuadraticHexahedron>();
}
extern "C" void vtkBiQuadraticQuadraticHexahedron_destructor(
    vtkNew<vtkBiQuadraticQuadraticHexahedron> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkBiQuadraticQuadraticHexahedron_get_ptr(
    vtkNew<vtkBiQuadraticQuadraticHexahedron> sself
) {
    return sself.GetPointer();
}
extern "C" int vtk_bi_quadratic_quadratic_hexahedron_get_cell_type(
    vtkNew<vtkBiQuadraticQuadraticHexahedron> self
) {
    return self->GetCellType();
}
extern "C" int vtk_bi_quadratic_quadratic_hexahedron_get_cell_dimension(
    vtkNew<vtkBiQuadraticQuadraticHexahedron> self
) {
    return self->GetCellDimension();
}
extern "C" int vtk_bi_quadratic_quadratic_hexahedron_get_number_of_edges(
    vtkNew<vtkBiQuadraticQuadraticHexahedron> self
) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_bi_quadratic_quadratic_hexahedron_get_number_of_faces(
    vtkNew<vtkBiQuadraticQuadraticHexahedron> self
) {
    return self->GetNumberOfFaces();
}
extern "C" void vtk_bi_quadratic_quadratic_hexahedron_interpolation_functions(
    vtkNew<vtkBiQuadraticQuadraticHexahedron> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_bi_quadratic_quadratic_hexahedron_interpolation_derivs(
    vtkNew<vtkBiQuadraticQuadraticHexahedron> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_bi_quadratic_quadratic_hexahedron_interpolate_functions(
    vtkNew<vtkBiQuadraticQuadraticHexahedron> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_bi_quadratic_quadratic_hexahedron_interpolate_derivs(
    vtkNew<vtkBiQuadraticQuadraticHexahedron> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkBiQuadraticQuadraticWedge> vtkBiQuadraticQuadraticWedge_new() {
    return vtkNew<vtkBiQuadraticQuadraticWedge>();
}
extern "C" void vtkBiQuadraticQuadraticWedge_destructor(
    vtkNew<vtkBiQuadraticQuadraticWedge> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkBiQuadraticQuadraticWedge_get_ptr(vtkNew<vtkBiQuadraticQuadraticWedge> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_bi_quadratic_quadratic_wedge_get_cell_type(
    vtkNew<vtkBiQuadraticQuadraticWedge> self
) {
    return self->GetCellType();
}
extern "C" int vtk_bi_quadratic_quadratic_wedge_get_cell_dimension(
    vtkNew<vtkBiQuadraticQuadraticWedge> self
) {
    return self->GetCellDimension();
}
extern "C" int vtk_bi_quadratic_quadratic_wedge_get_number_of_edges(
    vtkNew<vtkBiQuadraticQuadraticWedge> self
) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_bi_quadratic_quadratic_wedge_get_number_of_faces(
    vtkNew<vtkBiQuadraticQuadraticWedge> self
) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_bi_quadratic_quadratic_wedge_get_parametric_center(
    vtkNew<vtkBiQuadraticQuadraticWedge> self, double* pcoords
) {
    return self->GetParametricCenter(pcoords);
}
extern "C" void vtk_bi_quadratic_quadratic_wedge_interpolation_functions(
    vtkNew<vtkBiQuadraticQuadraticWedge> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_bi_quadratic_quadratic_wedge_interpolation_derivs(
    vtkNew<vtkBiQuadraticQuadraticWedge> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_bi_quadratic_quadratic_wedge_interpolate_functions(
    vtkNew<vtkBiQuadraticQuadraticWedge> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_bi_quadratic_quadratic_wedge_interpolate_derivs(
    vtkNew<vtkBiQuadraticQuadraticWedge> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkBiQuadraticTriangle> vtkBiQuadraticTriangle_new() {
    return vtkNew<vtkBiQuadraticTriangle>();
}
extern "C" void vtkBiQuadraticTriangle_destructor(vtkNew<vtkBiQuadraticTriangle> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkBiQuadraticTriangle_get_ptr(vtkNew<vtkBiQuadraticTriangle> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_bi_quadratic_triangle_get_cell_type(vtkNew<vtkBiQuadraticTriangle> self) {
    return self->GetCellType();
}
extern "C" int vtk_bi_quadratic_triangle_get_cell_dimension(vtkNew<vtkBiQuadraticTriangle> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_bi_quadratic_triangle_get_number_of_edges(vtkNew<vtkBiQuadraticTriangle> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_bi_quadratic_triangle_get_number_of_faces(vtkNew<vtkBiQuadraticTriangle> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_bi_quadratic_triangle_get_parametric_center(
    vtkNew<vtkBiQuadraticTriangle> self, double* pcoords
) {
    return self->GetParametricCenter(pcoords);
}
extern "C" double vtk_bi_quadratic_triangle_get_parametric_distance(
    vtkNew<vtkBiQuadraticTriangle> self, const double* pcoords
) {
    return self->GetParametricDistance(pcoords);
}
extern "C" void vtk_bi_quadratic_triangle_interpolation_functions(
    vtkNew<vtkBiQuadraticTriangle> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_bi_quadratic_triangle_interpolation_derivs(
    vtkNew<vtkBiQuadraticTriangle> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_bi_quadratic_triangle_interpolate_functions(
    vtkNew<vtkBiQuadraticTriangle> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_bi_quadratic_triangle_interpolate_derivs(
    vtkNew<vtkBiQuadraticTriangle> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkBox> vtkBox_new() {
    return vtkNew<vtkBox>();
}
extern "C" void vtkBox_destructor(vtkNew<vtkBox> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkBox_get_ptr(vtkNew<vtkBox> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_box_evaluate_function(vtkNew<vtkBox> self, double* x) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_box_evaluate_gradient(vtkNew<vtkBox> self, double* x, double* n) {
    return self->EvaluateGradient(x, n);
}
extern "C" void vtk_box_set_x_min_double_3(vtkNew<vtkBox> self, double* p) {
    return self->SetXMin(p);
}
extern "C" void vtk_box_set_x_min_double_double_double(
    vtkNew<vtkBox> self, double x, double y, double z
) {
    return self->SetXMin(x, y, z);
}
extern "C" void vtk_box_get_x_min_double_3(vtkNew<vtkBox> self, double* p) {
    return self->GetXMin(p);
}
extern "C" void vtk_box_set_x_max_double_3(vtkNew<vtkBox> self, double* p) {
    return self->SetXMax(p);
}
extern "C" void vtk_box_set_x_max_double_double_double(
    vtkNew<vtkBox> self, double x, double y, double z
) {
    return self->SetXMax(x, y, z);
}
extern "C" void vtk_box_get_x_max_double_3(vtkNew<vtkBox> self, double* p) {
    return self->GetXMax(p);
}
extern "C" void vtk_box_set_bounds_double_double_double_double_double_double(
    vtkNew<vtkBox> self,
    double xMin,
    double xMax,
    double yMin,
    double yMax,
    double zMin,
    double zMax
) {
    return self->SetBounds(xMin, xMax, yMin, yMax, zMin, zMax);
}
extern "C" void vtk_box_set_bounds_const_double_6(vtkNew<vtkBox> self, const double* bounds) {
    return self->SetBounds(bounds);
}
extern "C" void vtk_box_get_bounds_double_6(vtkNew<vtkBox> self, double* bounds) {
    return self->GetBounds(bounds);
}
extern "C" void vtk_box_add_bounds(vtkNew<vtkBox> self, const double* bounds) {
    return self->AddBounds(bounds);
}
extern "C" int vtk_box_intersect_with_plane_double_6_double_3_double_3(
    vtkNew<vtkBox> self, double* bounds, double* origin, double* normal
) {
    return self->IntersectWithPlane(bounds, origin, normal);
}
extern "C" int vtk_box_intersect_with_plane_double_6_double_3_double_3_double_18(
    vtkNew<vtkBox> self, double* bounds, double* origin, double* normal, double* xout
) {
    return self->IntersectWithPlane(bounds, origin, normal, xout);
}
extern "C" int vtk_box_is_box_in_frustum(vtkNew<vtkBox> self, double* planes, double* bounds) {
    return self->IsBoxInFrustum(planes, bounds);
}
extern "C" vtkNew<vtkCellArray> vtkCellArray_new() {
    return vtkNew<vtkCellArray>();
}
extern "C" void vtkCellArray_destructor(vtkNew<vtkCellArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellArray_get_ptr(vtkNew<vtkCellArray> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_cell_array_allocate(vtkNew<vtkCellArray> self, long long sz, long long ext) {
    return self->Allocate(sz, ext);
}
extern "C" bool vtk_cell_array_allocate_estimate(
    vtkNew<vtkCellArray> self, long long numCells, long long maxCellSize
) {
    return self->AllocateEstimate(numCells, maxCellSize);
}
extern "C" bool vtk_cell_array_allocate_exact(
    vtkNew<vtkCellArray> self, long long numCells, long long connectivitySize
) {
    return self->AllocateExact(numCells, connectivitySize);
}
extern "C" bool vtk_cell_array_resize_exact(
    vtkNew<vtkCellArray> self, long long numCells, long long connectivitySize
) {
    return self->ResizeExact(numCells, connectivitySize);
}
extern "C" void vtk_cell_array_initialize(vtkNew<vtkCellArray> self) {
    return self->Initialize();
}
extern "C" void vtk_cell_array_reset(vtkNew<vtkCellArray> self) {
    return self->Reset();
}
extern "C" void vtk_cell_array_squeeze(vtkNew<vtkCellArray> self) {
    return self->Squeeze();
}
extern "C" bool vtk_cell_array_is_valid(vtkNew<vtkCellArray> self) {
    return self->IsValid();
}
extern "C" long long vtk_cell_array_get_number_of_cells(vtkNew<vtkCellArray> self) {
    return self->GetNumberOfCells();
}
extern "C" long long vtk_cell_array_get_number_of_offsets(vtkNew<vtkCellArray> self) {
    return self->GetNumberOfOffsets();
}
extern "C" long long vtk_cell_array_get_offset(vtkNew<vtkCellArray> self, long long cellId) {
    return self->GetOffset(cellId);
}
extern "C" void vtk_cell_array_set_offset(
    vtkNew<vtkCellArray> self, long long cellId, long long offset
) {
    return self->SetOffset(cellId, offset);
}
extern "C" long long vtk_cell_array_get_number_of_connectivity_ids(vtkNew<vtkCellArray> self) {
    return self->GetNumberOfConnectivityIds();
}
extern "C" bool vtk_cell_array_is_storage_64_bit(vtkNew<vtkCellArray> self) {
    return self->IsStorage64Bit();
}
extern "C" bool vtk_cell_array_is_storage_32_bit(vtkNew<vtkCellArray> self) {
    return self->IsStorage32Bit();
}
extern "C" bool vtk_cell_array_is_storage_fixed_size_64_bit(vtkNew<vtkCellArray> self) {
    return self->IsStorageFixedSize64Bit();
}
extern "C" bool vtk_cell_array_is_storage_fixed_size_32_bit(vtkNew<vtkCellArray> self) {
    return self->IsStorageFixedSize32Bit();
}
extern "C" bool vtk_cell_array_is_storage_fixed_size(vtkNew<vtkCellArray> self) {
    return self->IsStorageFixedSize();
}
extern "C" bool vtk_cell_array_is_storage_generic(vtkNew<vtkCellArray> self) {
    return self->IsStorageGeneric();
}
extern "C" bool vtk_cell_array_is_storage_shareable(vtkNew<vtkCellArray> self) {
    return self->IsStorageShareable();
}
extern "C" void vtk_cell_array_use_32_bit_storage(vtkNew<vtkCellArray> self) {
    return self->Use32BitStorage();
}
extern "C" void vtk_cell_array_use_64_bit_storage(vtkNew<vtkCellArray> self) {
    return self->Use64BitStorage();
}
extern "C" void vtk_cell_array_use_default_storage(vtkNew<vtkCellArray> self) {
    return self->UseDefaultStorage();
}
extern "C" void vtk_cell_array_use_fixed_size_32_bit_storage(
    vtkNew<vtkCellArray> self, long long cellSize
) {
    return self->UseFixedSize32BitStorage(cellSize);
}
extern "C" void vtk_cell_array_use_fixed_size_64_bit_storage(
    vtkNew<vtkCellArray> self, long long cellSize
) {
    return self->UseFixedSize64BitStorage(cellSize);
}
extern "C" void vtk_cell_array_use_fixed_size_default_storage(
    vtkNew<vtkCellArray> self, long long cellSize
) {
    return self->UseFixedSizeDefaultStorage(cellSize);
}
extern "C" bool vtk_cell_array_can_convert_to_32_bit_storage(vtkNew<vtkCellArray> self) {
    return self->CanConvertTo32BitStorage();
}
extern "C" bool vtk_cell_array_can_convert_to_64_bit_storage(vtkNew<vtkCellArray> self) {
    return self->CanConvertTo64BitStorage();
}
extern "C" bool vtk_cell_array_can_convert_to_default_storage(vtkNew<vtkCellArray> self) {
    return self->CanConvertToDefaultStorage();
}
extern "C" bool vtk_cell_array_can_convert_to_fixed_size_32_bit_storage(vtkNew<vtkCellArray> self) {
    return self->CanConvertToFixedSize32BitStorage();
}
extern "C" bool vtk_cell_array_can_convert_to_fixed_size_64_bit_storage(vtkNew<vtkCellArray> self) {
    return self->CanConvertToFixedSize64BitStorage();
}
extern "C" bool vtk_cell_array_can_convert_to_fixed_size_default_storage(
    vtkNew<vtkCellArray> self
) {
    return self->CanConvertToFixedSizeDefaultStorage();
}
extern "C" bool vtk_cell_array_convert_to_32_bit_storage(vtkNew<vtkCellArray> self) {
    return self->ConvertTo32BitStorage();
}
extern "C" bool vtk_cell_array_convert_to_64_bit_storage(vtkNew<vtkCellArray> self) {
    return self->ConvertTo64BitStorage();
}
extern "C" bool vtk_cell_array_convert_to_default_storage(vtkNew<vtkCellArray> self) {
    return self->ConvertToDefaultStorage();
}
extern "C" bool vtk_cell_array_convert_to_fixed_size_32_bit_storage(vtkNew<vtkCellArray> self) {
    return self->ConvertToFixedSize32BitStorage();
}
extern "C" bool vtk_cell_array_convert_to_fixed_size_64_bit_storage(vtkNew<vtkCellArray> self) {
    return self->ConvertToFixedSize64BitStorage();
}
extern "C" bool vtk_cell_array_convert_to_fixed_size_default_storage(vtkNew<vtkCellArray> self) {
    return self->ConvertToFixedSizeDefaultStorage();
}
extern "C" bool vtk_cell_array_convert_to_smallest_storage(vtkNew<vtkCellArray> self) {
    return self->ConvertToSmallestStorage();
}
extern "C" long long vtk_cell_array_is_homogeneous(vtkNew<vtkCellArray> self) {
    return self->IsHomogeneous();
}
extern "C" void vtk_cell_array_init_traversal(vtkNew<vtkCellArray> self) {
    return self->InitTraversal();
}
extern "C" long long vtk_cell_array_get_cell_point_at_id(
    vtkNew<vtkCellArray> self, long long cellId, long long cellPointIndex
) {
    return self->GetCellPointAtId(cellId, cellPointIndex);
}
extern "C" long long vtk_cell_array_get_cell_size(vtkNew<vtkCellArray> self, long long cellId) {
    return self->GetCellSize(cellId);
}
extern "C" long long vtk_cell_array_insert_next_cell_c_int(vtkNew<vtkCellArray> self, int npts) {
    return self->InsertNextCell(npts);
}
extern "C" void vtk_cell_array_insert_cell_point(vtkNew<vtkCellArray> self, long long id) {
    return self->InsertCellPoint(id);
}
extern "C" void vtk_cell_array_update_cell_count(vtkNew<vtkCellArray> self, int npts) {
    return self->UpdateCellCount(npts);
}
extern "C" long long vtk_cell_array_get_traversal_cell_id(vtkNew<vtkCellArray> self) {
    return self->GetTraversalCellId();
}
extern "C" void vtk_cell_array_set_traversal_cell_id(vtkNew<vtkCellArray> self, long long cellId) {
    return self->SetTraversalCellId(cellId);
}
extern "C" void vtk_cell_array_reverse_cell_at_id(vtkNew<vtkCellArray> self, long long cellId) {
    return self->ReverseCellAtId(cellId);
}
extern "C" void vtk_cell_array_replace_cell_point_at_id(
    vtkNew<vtkCellArray> self, long long cellId, long long cellPointIndex, long long newPointId
) {
    return self->ReplaceCellPointAtId(cellId, cellPointIndex, newPointId);
}
extern "C" int vtk_cell_array_get_max_cell_size(vtkNew<vtkCellArray> self) {
    return self->GetMaxCellSize();
}
extern "C" unsigned long vtk_cell_array_get_actual_memory_size(vtkNew<vtkCellArray> self) {
    return self->GetActualMemorySize();
}
extern "C" bool vtk_cell_array_get_default_storage_is_64_bit(vtkNew<vtkCellArray> self) {
    return self->GetDefaultStorageIs64Bit();
}
extern "C" void vtk_cell_array_set_default_storage_is_64_bit(vtkNew<vtkCellArray> self, bool val) {
    return self->SetDefaultStorageIs64Bit(val);
}
extern "C" void vtk_cell_array_set_number_of_cells(vtkNew<vtkCellArray> self, long long p0) {
    return self->SetNumberOfCells(p0);
}
extern "C" long long vtk_cell_array_estimate_size(
    vtkNew<vtkCellArray> self, long long numCells, int maxPtsPerCell
) {
    return self->EstimateSize(numCells, maxPtsPerCell);
}
extern "C" long long vtk_cell_array_get_size(vtkNew<vtkCellArray> self) {
    return self->GetSize();
}
extern "C" long long vtk_cell_array_get_number_of_connectivity_entries(vtkNew<vtkCellArray> self) {
    return self->GetNumberOfConnectivityEntries();
}
extern "C" long long vtk_cell_array_get_insert_location(vtkNew<vtkCellArray> self, int npts) {
    return self->GetInsertLocation(npts);
}
extern "C" long long vtk_cell_array_get_traversal_location(vtkNew<vtkCellArray> self) {
    return self->GetTraversalLocation();
}
extern "C" long long vtk_cell_array_get_traversal_location_c_longlong(
    vtkNew<vtkCellArray> self, long long npts
) {
    return self->GetTraversalLocation(npts);
}
extern "C" void vtk_cell_array_set_traversal_location(vtkNew<vtkCellArray> self, long long loc) {
    return self->SetTraversalLocation(loc);
}
extern "C" void vtk_cell_array_reverse_cell(vtkNew<vtkCellArray> self, long long loc) {
    return self->ReverseCell(loc);
}
extern "C" vtkNew<vtkCellArrayIterator> vtkCellArrayIterator_new() {
    return vtkNew<vtkCellArrayIterator>();
}
extern "C" void vtkCellArrayIterator_destructor(vtkNew<vtkCellArrayIterator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellArrayIterator_get_ptr(vtkNew<vtkCellArrayIterator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_cell_array_iterator_go_to_cell(
    vtkNew<vtkCellArrayIterator> self, long long cellId
) {
    return self->GoToCell(cellId);
}
extern "C" void vtk_cell_array_iterator_go_to_first_cell(vtkNew<vtkCellArrayIterator> self) {
    return self->GoToFirstCell();
}
extern "C" void vtk_cell_array_iterator_go_to_next_cell(vtkNew<vtkCellArrayIterator> self) {
    return self->GoToNextCell();
}
extern "C" bool vtk_cell_array_iterator_is_done_with_traversal(vtkNew<vtkCellArrayIterator> self) {
    return self->IsDoneWithTraversal();
}
extern "C" long long vtk_cell_array_iterator_get_current_cell_id(
    vtkNew<vtkCellArrayIterator> self
) {
    return self->GetCurrentCellId();
}
extern "C" void vtk_cell_array_iterator_reverse_current_cell(vtkNew<vtkCellArrayIterator> self) {
    return self->ReverseCurrentCell();
}
extern "C" vtkNew<vtkCellAttribute> vtkCellAttribute_new() {
    return vtkNew<vtkCellAttribute>();
}
extern "C" void vtkCellAttribute_destructor(vtkNew<vtkCellAttribute> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellAttribute_get_ptr(vtkNew<vtkCellAttribute> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_cell_attribute_get_id(vtkNew<vtkCellAttribute> self) {
    return self->GetId();
}
extern "C" void vtk_cell_attribute_set_id(vtkNew<vtkCellAttribute> self, int _arg) {
    return self->SetId(_arg);
}
extern "C" int vtk_cell_attribute_get_number_of_components(vtkNew<vtkCellAttribute> self) {
    return self->GetNumberOfComponents();
}
extern "C" vtkNew<vtkCellAttributeCalculator> vtkCellAttributeCalculator_new() {
    return vtkNew<vtkCellAttributeCalculator>();
}
extern "C" void vtkCellAttributeCalculator_destructor(vtkNew<vtkCellAttributeCalculator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellAttributeCalculator_get_ptr(vtkNew<vtkCellAttributeCalculator> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkCellData> vtkCellData_new() {
    return vtkNew<vtkCellData>();
}
extern "C" void vtkCellData_destructor(vtkNew<vtkCellData> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellData_get_ptr(vtkNew<vtkCellData> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkCellGrid> vtkCellGrid_new() {
    return vtkNew<vtkCellGrid>();
}
extern "C" void vtkCellGrid_destructor(vtkNew<vtkCellGrid> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellGrid_get_ptr(vtkNew<vtkCellGrid> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_cell_grid_get_data_object_type(vtkNew<vtkCellGrid> self) {
    return self->GetDataObjectType();
}
extern "C" bool vtk_cell_grid_supports_ghost_array(vtkNew<vtkCellGrid> self, int type) {
    return self->SupportsGhostArray(type);
}
extern "C" long long vtk_cell_grid_get_number_of_cells(vtkNew<vtkCellGrid> self) {
    return self->GetNumberOfCells();
}
extern "C" void vtk_cell_grid_get_bounds(vtkNew<vtkCellGrid> self, double* bounds) {
    return self->GetBounds(bounds);
}
extern "C" int vtk_cell_grid_add_all_cell_metadata(vtkNew<vtkCellGrid> self) {
    return self->AddAllCellMetadata();
}
extern "C" int vtk_cell_grid_remove_unused_cell_metadata(vtkNew<vtkCellGrid> self) {
    return self->RemoveUnusedCellMetadata();
}
extern "C" unsigned int vtk_cell_grid_get_schema_version(vtkNew<vtkCellGrid> self) {
    return self->GetSchemaVersion();
}
extern "C" void vtk_cell_grid_set_content_version(vtkNew<vtkCellGrid> self, unsigned int _arg) {
    return self->SetContentVersion(_arg);
}
extern "C" unsigned int vtk_cell_grid_get_content_version(vtkNew<vtkCellGrid> self) {
    return self->GetContentVersion();
}
extern "C" vtkNew<vtkCellGridBoundsQuery> vtkCellGridBoundsQuery_new() {
    return vtkNew<vtkCellGridBoundsQuery>();
}
extern "C" void vtkCellGridBoundsQuery_destructor(vtkNew<vtkCellGridBoundsQuery> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellGridBoundsQuery_get_ptr(vtkNew<vtkCellGridBoundsQuery> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkCellGridCopyQuery> vtkCellGridCopyQuery_new() {
    return vtkNew<vtkCellGridCopyQuery>();
}
extern "C" void vtkCellGridCopyQuery_destructor(vtkNew<vtkCellGridCopyQuery> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellGridCopyQuery_get_ptr(vtkNew<vtkCellGridCopyQuery> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_cell_grid_copy_query_initialize(vtkNew<vtkCellGridCopyQuery> self) {
    return self->Initialize();
}
extern "C" bool vtk_cell_grid_copy_query_finalize(vtkNew<vtkCellGridCopyQuery> self) {
    return self->Finalize();
}
extern "C" int vtk_cell_grid_copy_query_get_copy_cell_types(vtkNew<vtkCellGridCopyQuery> self) {
    return self->GetCopyCellTypes();
}
extern "C" void vtk_cell_grid_copy_query_set_copy_cell_types(
    vtkNew<vtkCellGridCopyQuery> self, int _arg
) {
    return self->SetCopyCellTypes(_arg);
}
extern "C" void vtk_cell_grid_copy_query_copy_cell_types_on(vtkNew<vtkCellGridCopyQuery> self) {
    return self->CopyCellTypesOn();
}
extern "C" void vtk_cell_grid_copy_query_copy_cell_types_off(vtkNew<vtkCellGridCopyQuery> self) {
    return self->CopyCellTypesOff();
}
extern "C" int vtk_cell_grid_copy_query_get_copy_cells(vtkNew<vtkCellGridCopyQuery> self) {
    return self->GetCopyCells();
}
extern "C" void vtk_cell_grid_copy_query_set_copy_cells(
    vtkNew<vtkCellGridCopyQuery> self, int _arg
) {
    return self->SetCopyCells(_arg);
}
extern "C" void vtk_cell_grid_copy_query_copy_cells_on(vtkNew<vtkCellGridCopyQuery> self) {
    return self->CopyCellsOn();
}
extern "C" void vtk_cell_grid_copy_query_copy_cells_off(vtkNew<vtkCellGridCopyQuery> self) {
    return self->CopyCellsOff();
}
extern "C" int vtk_cell_grid_copy_query_get_copy_only_shape(vtkNew<vtkCellGridCopyQuery> self) {
    return self->GetCopyOnlyShape();
}
extern "C" void vtk_cell_grid_copy_query_set_copy_only_shape(
    vtkNew<vtkCellGridCopyQuery> self, int _arg
) {
    return self->SetCopyOnlyShape(_arg);
}
extern "C" void vtk_cell_grid_copy_query_copy_only_shape_on(vtkNew<vtkCellGridCopyQuery> self) {
    return self->CopyOnlyShapeOn();
}
extern "C" void vtk_cell_grid_copy_query_copy_only_shape_off(vtkNew<vtkCellGridCopyQuery> self) {
    return self->CopyOnlyShapeOff();
}
extern "C" int vtk_cell_grid_copy_query_get_copy_arrays(vtkNew<vtkCellGridCopyQuery> self) {
    return self->GetCopyArrays();
}
extern "C" void vtk_cell_grid_copy_query_set_copy_arrays(
    vtkNew<vtkCellGridCopyQuery> self, int _arg
) {
    return self->SetCopyArrays(_arg);
}
extern "C" void vtk_cell_grid_copy_query_copy_arrays_on(vtkNew<vtkCellGridCopyQuery> self) {
    return self->CopyArraysOn();
}
extern "C" void vtk_cell_grid_copy_query_copy_arrays_off(vtkNew<vtkCellGridCopyQuery> self) {
    return self->CopyArraysOff();
}
extern "C" int vtk_cell_grid_copy_query_get_copy_array_values(vtkNew<vtkCellGridCopyQuery> self) {
    return self->GetCopyArrayValues();
}
extern "C" void vtk_cell_grid_copy_query_set_copy_array_values(
    vtkNew<vtkCellGridCopyQuery> self, int _arg
) {
    return self->SetCopyArrayValues(_arg);
}
extern "C" void vtk_cell_grid_copy_query_copy_array_values_on(vtkNew<vtkCellGridCopyQuery> self) {
    return self->CopyArrayValuesOn();
}
extern "C" void vtk_cell_grid_copy_query_copy_array_values_off(vtkNew<vtkCellGridCopyQuery> self) {
    return self->CopyArrayValuesOff();
}
extern "C" int vtk_cell_grid_copy_query_get_deep_copy_arrays(vtkNew<vtkCellGridCopyQuery> self) {
    return self->GetDeepCopyArrays();
}
extern "C" void vtk_cell_grid_copy_query_set_deep_copy_arrays(
    vtkNew<vtkCellGridCopyQuery> self, int _arg
) {
    return self->SetDeepCopyArrays(_arg);
}
extern "C" void vtk_cell_grid_copy_query_deep_copy_arrays_on(vtkNew<vtkCellGridCopyQuery> self) {
    return self->DeepCopyArraysOn();
}
extern "C" void vtk_cell_grid_copy_query_deep_copy_arrays_off(vtkNew<vtkCellGridCopyQuery> self) {
    return self->DeepCopyArraysOff();
}
extern "C" int vtk_cell_grid_copy_query_get_copy_schema(vtkNew<vtkCellGridCopyQuery> self) {
    return self->GetCopySchema();
}
extern "C" void vtk_cell_grid_copy_query_set_copy_schema(
    vtkNew<vtkCellGridCopyQuery> self, int _arg
) {
    return self->SetCopySchema(_arg);
}
extern "C" void vtk_cell_grid_copy_query_copy_schema_on(vtkNew<vtkCellGridCopyQuery> self) {
    return self->CopySchemaOn();
}
extern "C" void vtk_cell_grid_copy_query_copy_schema_off(vtkNew<vtkCellGridCopyQuery> self) {
    return self->CopySchemaOff();
}
extern "C" bool vtk_cell_grid_copy_query_add_source_cell_attribute_id(
    vtkNew<vtkCellGridCopyQuery> self, int attributeId
) {
    return self->AddSourceCellAttributeId(attributeId);
}
extern "C" bool vtk_cell_grid_copy_query_remove_source_cell_attribute_id(
    vtkNew<vtkCellGridCopyQuery> self, int attributeId
) {
    return self->RemoveSourceCellAttributeId(attributeId);
}
extern "C" bool vtk_cell_grid_copy_query_add_all_source_cell_attribute_ids(
    vtkNew<vtkCellGridCopyQuery> self
) {
    return self->AddAllSourceCellAttributeIds();
}
extern "C" void vtk_cell_grid_copy_query_reset_cell_attribute_ids(
    vtkNew<vtkCellGridCopyQuery> self
) {
    return self->ResetCellAttributeIds();
}
extern "C" vtkNew<vtkCellGridEvaluator> vtkCellGridEvaluator_new() {
    return vtkNew<vtkCellGridEvaluator>();
}
extern "C" void vtkCellGridEvaluator_destructor(vtkNew<vtkCellGridEvaluator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellGridEvaluator_get_ptr(vtkNew<vtkCellGridEvaluator> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_cell_grid_evaluator_initialize(vtkNew<vtkCellGridEvaluator> self) {
    return self->Initialize();
}
extern "C" void vtk_cell_grid_evaluator_start_pass(vtkNew<vtkCellGridEvaluator> self) {
    return self->StartPass();
}
extern "C" bool vtk_cell_grid_evaluator_is_another_pass_required(
    vtkNew<vtkCellGridEvaluator> self
) {
    return self->IsAnotherPassRequired();
}
extern "C" bool vtk_cell_grid_evaluator_finalize(vtkNew<vtkCellGridEvaluator> self) {
    return self->Finalize();
}
extern "C" vtkNew<vtkCellGridRangeQuery> vtkCellGridRangeQuery_new() {
    return vtkNew<vtkCellGridRangeQuery>();
}
extern "C" void vtkCellGridRangeQuery_destructor(vtkNew<vtkCellGridRangeQuery> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellGridRangeQuery_get_ptr(vtkNew<vtkCellGridRangeQuery> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_cell_grid_range_query_set_component(
    vtkNew<vtkCellGridRangeQuery> self, int _arg
) {
    return self->SetComponent(_arg);
}
extern "C" int vtk_cell_grid_range_query_get_component(vtkNew<vtkCellGridRangeQuery> self) {
    return self->GetComponent();
}
extern "C" void vtk_cell_grid_range_query_set_finite_range(
    vtkNew<vtkCellGridRangeQuery> self, int _arg
) {
    return self->SetFiniteRange(_arg);
}
extern "C" int vtk_cell_grid_range_query_get_finite_range(vtkNew<vtkCellGridRangeQuery> self) {
    return self->GetFiniteRange();
}
extern "C" bool vtk_cell_grid_range_query_initialize(vtkNew<vtkCellGridRangeQuery> self) {
    return self->Initialize();
}
extern "C" bool vtk_cell_grid_range_query_finalize(vtkNew<vtkCellGridRangeQuery> self) {
    return self->Finalize();
}
extern "C" vtkNew<vtkCellGridResponders> vtkCellGridResponders_new() {
    return vtkNew<vtkCellGridResponders>();
}
extern "C" void vtkCellGridResponders_destructor(vtkNew<vtkCellGridResponders> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellGridResponders_get_ptr(vtkNew<vtkCellGridResponders> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkCellGridSidesCache> vtkCellGridSidesCache_new() {
    return vtkNew<vtkCellGridSidesCache>();
}
extern "C" void vtkCellGridSidesCache_destructor(vtkNew<vtkCellGridSidesCache> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellGridSidesCache_get_ptr(vtkNew<vtkCellGridSidesCache> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_cell_grid_sides_cache_initialize(vtkNew<vtkCellGridSidesCache> self) {
    return self->Initialize();
}
extern "C" vtkNew<vtkCellGridSidesQuery> vtkCellGridSidesQuery_new() {
    return vtkNew<vtkCellGridSidesQuery>();
}
extern "C" void vtkCellGridSidesQuery_destructor(vtkNew<vtkCellGridSidesQuery> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellGridSidesQuery_get_ptr(vtkNew<vtkCellGridSidesQuery> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_cell_grid_sides_query_set_preserve_renderable_inputs(
    vtkNew<vtkCellGridSidesQuery> self, int _arg
) {
    return self->SetPreserveRenderableInputs(_arg);
}
extern "C" int vtk_cell_grid_sides_query_get_preserve_renderable_inputs(
    vtkNew<vtkCellGridSidesQuery> self
) {
    return self->GetPreserveRenderableInputs();
}
extern "C" void vtk_cell_grid_sides_query_preserve_renderable_inputs_on(
    vtkNew<vtkCellGridSidesQuery> self
) {
    return self->PreserveRenderableInputsOn();
}
extern "C" void vtk_cell_grid_sides_query_preserve_renderable_inputs_off(
    vtkNew<vtkCellGridSidesQuery> self
) {
    return self->PreserveRenderableInputsOff();
}
extern "C" void vtk_cell_grid_sides_query_set_omit_sides_for_renderable_inputs(
    vtkNew<vtkCellGridSidesQuery> self, int _arg
) {
    return self->SetOmitSidesForRenderableInputs(_arg);
}
extern "C" int vtk_cell_grid_sides_query_get_omit_sides_for_renderable_inputs(
    vtkNew<vtkCellGridSidesQuery> self
) {
    return self->GetOmitSidesForRenderableInputs();
}
extern "C" void vtk_cell_grid_sides_query_omit_sides_for_renderable_inputs_on(
    vtkNew<vtkCellGridSidesQuery> self
) {
    return self->OmitSidesForRenderableInputsOn();
}
extern "C" void vtk_cell_grid_sides_query_omit_sides_for_renderable_inputs_off(
    vtkNew<vtkCellGridSidesQuery> self
) {
    return self->OmitSidesForRenderableInputsOff();
}
extern "C" void vtk_cell_grid_sides_query_set_output_dimension_control(
    vtkNew<vtkCellGridSidesQuery> self, int _arg
) {
    return self->SetOutputDimensionControl(_arg);
}
extern "C" int vtk_cell_grid_sides_query_get_output_dimension_control(
    vtkNew<vtkCellGridSidesQuery> self
) {
    return self->GetOutputDimensionControl();
}
extern "C" void vtk_cell_grid_sides_query_output_dimension_control_on(
    vtkNew<vtkCellGridSidesQuery> self
) {
    return self->OutputDimensionControlOn();
}
extern "C" void vtk_cell_grid_sides_query_output_dimension_control_off(
    vtkNew<vtkCellGridSidesQuery> self
) {
    return self->OutputDimensionControlOff();
}
extern "C" void vtk_cell_grid_sides_query_set_strategy_to_winding(
    vtkNew<vtkCellGridSidesQuery> self
) {
    return self->SetStrategyToWinding();
}
extern "C" void vtk_cell_grid_sides_query_set_strategy_to_any_occurrence(
    vtkNew<vtkCellGridSidesQuery> self
) {
    return self->SetStrategyToAnyOccurrence();
}
extern "C" void vtk_cell_grid_sides_query_set_strategy_to_boundary(
    vtkNew<vtkCellGridSidesQuery> self
) {
    return self->SetStrategyToBoundary();
}
extern "C" void vtk_cell_grid_sides_query_set_strategy_c_int(
    vtkNew<vtkCellGridSidesQuery> self, int strategy
) {
    return self->SetStrategy(strategy);
}
extern "C" void vtk_cell_grid_sides_query_set_selection_type_c_int(
    vtkNew<vtkCellGridSidesQuery> self, int selnType
) {
    return self->SetSelectionType(selnType);
}
extern "C" vtkNew<vtkCellLinks> vtkCellLinks_new() {
    return vtkNew<vtkCellLinks>();
}
extern "C" void vtkCellLinks_destructor(vtkNew<vtkCellLinks> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellLinks_get_ptr(vtkNew<vtkCellLinks> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_cell_links_build_links(vtkNew<vtkCellLinks> self) {
    return self->BuildLinks();
}
extern "C" void vtk_cell_links_allocate(
    vtkNew<vtkCellLinks> self, long long numLinks, long long ext
) {
    return self->Allocate(numLinks, ext);
}
extern "C" void vtk_cell_links_initialize(vtkNew<vtkCellLinks> self) {
    return self->Initialize();
}
extern "C" long long vtk_cell_links_get_ncells(vtkNew<vtkCellLinks> self, long long ptId) {
    return self->GetNcells(ptId);
}
extern "C" long long vtk_cell_links_insert_next_point(vtkNew<vtkCellLinks> self, int numLinks) {
    return self->InsertNextPoint(numLinks);
}
extern "C" void vtk_cell_links_insert_next_cell_reference(
    vtkNew<vtkCellLinks> self, long long ptId, long long cellId
) {
    return self->InsertNextCellReference(ptId, cellId);
}
extern "C" void vtk_cell_links_delete_point(vtkNew<vtkCellLinks> self, long long ptId) {
    return self->DeletePoint(ptId);
}
extern "C" void vtk_cell_links_remove_cell_reference(
    vtkNew<vtkCellLinks> self, long long cellId, long long ptId
) {
    return self->RemoveCellReference(cellId, ptId);
}
extern "C" void vtk_cell_links_add_cell_reference(
    vtkNew<vtkCellLinks> self, long long cellId, long long ptId
) {
    return self->AddCellReference(cellId, ptId);
}
extern "C" void vtk_cell_links_resize_cell_list(
    vtkNew<vtkCellLinks> self, long long ptId, int size
) {
    return self->ResizeCellList(ptId, size);
}
extern "C" void vtk_cell_links_squeeze(vtkNew<vtkCellLinks> self) {
    return self->Squeeze();
}
extern "C" void vtk_cell_links_reset(vtkNew<vtkCellLinks> self) {
    return self->Reset();
}
extern "C" unsigned long vtk_cell_links_get_actual_memory_size(vtkNew<vtkCellLinks> self) {
    return self->GetActualMemorySize();
}
extern "C" vtkNew<vtkCellLocator> vtkCellLocator_new() {
    return vtkNew<vtkCellLocator>();
}
extern "C" void vtkCellLocator_destructor(vtkNew<vtkCellLocator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellLocator_get_ptr(vtkNew<vtkCellLocator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_cell_locator_set_number_of_cells_per_bucket(
    vtkNew<vtkCellLocator> self, int N
) {
    return self->SetNumberOfCellsPerBucket(N);
}
extern "C" int vtk_cell_locator_get_number_of_cells_per_bucket(vtkNew<vtkCellLocator> self) {
    return self->GetNumberOfCellsPerBucket();
}
extern "C" void vtk_cell_locator_free_search_structure(vtkNew<vtkCellLocator> self) {
    return self->FreeSearchStructure();
}
extern "C" void vtk_cell_locator_build_locator(vtkNew<vtkCellLocator> self) {
    return self->BuildLocator();
}
extern "C" int vtk_cell_locator_get_number_of_buckets(vtkNew<vtkCellLocator> self) {
    return self->GetNumberOfBuckets();
}
extern "C" vtkNew<vtkCellLocatorStrategy> vtkCellLocatorStrategy_new() {
    return vtkNew<vtkCellLocatorStrategy>();
}
extern "C" void vtkCellLocatorStrategy_destructor(vtkNew<vtkCellLocatorStrategy> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellLocatorStrategy_get_ptr(vtkNew<vtkCellLocatorStrategy> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_cell_locator_strategy_inside_cell_bounds(
    vtkNew<vtkCellLocatorStrategy> self, double* x, long long cellId
) {
    return self->InsideCellBounds(x, cellId);
}
extern "C" vtkNew<vtkCellTreeLocator> vtkCellTreeLocator_new() {
    return vtkNew<vtkCellTreeLocator>();
}
extern "C" void vtkCellTreeLocator_destructor(vtkNew<vtkCellTreeLocator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellTreeLocator_get_ptr(vtkNew<vtkCellTreeLocator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_cell_tree_locator_set_number_of_buckets(
    vtkNew<vtkCellTreeLocator> self, int _arg
) {
    return self->SetNumberOfBuckets(_arg);
}
extern "C" int vtk_cell_tree_locator_get_number_of_buckets(vtkNew<vtkCellTreeLocator> self) {
    return self->GetNumberOfBuckets();
}
extern "C" bool vtk_cell_tree_locator_get_large_ids(vtkNew<vtkCellTreeLocator> self) {
    return self->GetLargeIds();
}
extern "C" void vtk_cell_tree_locator_free_search_structure(vtkNew<vtkCellTreeLocator> self) {
    return self->FreeSearchStructure();
}
extern "C" void vtk_cell_tree_locator_build_locator(vtkNew<vtkCellTreeLocator> self) {
    return self->BuildLocator();
}
extern "C" vtkNew<vtkCellTypeUtilities> vtkCellTypeUtilities_new() {
    return vtkNew<vtkCellTypeUtilities>();
}
extern "C" void vtkCellTypeUtilities_destructor(vtkNew<vtkCellTypeUtilities> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellTypeUtilities_get_ptr(vtkNew<vtkCellTypeUtilities> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_cell_type_utilities_is_linear(
    vtkNew<vtkCellTypeUtilities> self, unsigned char type
) {
    return self->IsLinear(type);
}
extern "C" int vtk_cell_type_utilities_get_dimension(
    vtkNew<vtkCellTypeUtilities> self, unsigned char type
) {
    return self->GetDimension(type);
}
extern "C" vtkNew<vtkCellTypes> vtkCellTypes_new() {
    return vtkNew<vtkCellTypes>();
}
extern "C" void vtkCellTypes_destructor(vtkNew<vtkCellTypes> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellTypes_get_ptr(vtkNew<vtkCellTypes> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_cell_types_allocate(vtkNew<vtkCellTypes> self, long long sz, long long ext) {
    return self->Allocate(sz, ext);
}
extern "C" void vtk_cell_types_insert_cell_c_longlong_c_uchar_c_longlong(
    vtkNew<vtkCellTypes> self, long long id, unsigned char type, long long p2
) {
    return self->InsertCell(id, type, p2);
}
extern "C" void vtk_cell_types_insert_cell_c_longlong_c_uchar(
    vtkNew<vtkCellTypes> self, long long id, unsigned char type
) {
    return self->InsertCell(id, type);
}
extern "C" long long vtk_cell_types_insert_next_cell_c_uchar_c_longlong(
    vtkNew<vtkCellTypes> self, unsigned char type, long long p1
) {
    return self->InsertNextCell(type, p1);
}
extern "C" long long vtk_cell_types_insert_next_cell_c_uchar(
    vtkNew<vtkCellTypes> self, unsigned char type
) {
    return self->InsertNextCell(type);
}
extern "C" void vtk_cell_types_delete_cell(vtkNew<vtkCellTypes> self, long long cellId) {
    return self->DeleteCell(cellId);
}
extern "C" long long vtk_cell_types_get_number_of_types(vtkNew<vtkCellTypes> self) {
    return self->GetNumberOfTypes();
}
extern "C" int vtk_cell_types_is_type(vtkNew<vtkCellTypes> self, unsigned char type) {
    return self->IsType(type);
}
extern "C" long long vtk_cell_types_insert_next_type(
    vtkNew<vtkCellTypes> self, unsigned char type
) {
    return self->InsertNextType(type);
}
extern "C" unsigned char vtk_cell_types_get_cell_type(vtkNew<vtkCellTypes> self, long long cellId) {
    return self->GetCellType(cellId);
}
extern "C" void vtk_cell_types_squeeze(vtkNew<vtkCellTypes> self) {
    return self->Squeeze();
}
extern "C" void vtk_cell_types_reset(vtkNew<vtkCellTypes> self) {
    return self->Reset();
}
extern "C" unsigned long vtk_cell_types_get_actual_memory_size(vtkNew<vtkCellTypes> self) {
    return self->GetActualMemorySize();
}
extern "C" int vtk_cell_types_is_linear(vtkNew<vtkCellTypes> self, unsigned char type) {
    return self->IsLinear(type);
}
extern "C" int vtk_cell_types_get_dimension(vtkNew<vtkCellTypes> self, unsigned char type) {
    return self->GetDimension(type);
}
extern "C" vtkNew<vtkClosestNPointsStrategy> vtkClosestNPointsStrategy_new() {
    return vtkNew<vtkClosestNPointsStrategy>();
}
extern "C" void vtkClosestNPointsStrategy_destructor(vtkNew<vtkClosestNPointsStrategy> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkClosestNPointsStrategy_get_ptr(vtkNew<vtkClosestNPointsStrategy> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_closest_n_points_strategy_set_closest_n_points(
    vtkNew<vtkClosestNPointsStrategy> self, int _arg
) {
    return self->SetClosestNPoints(_arg);
}
extern "C" int vtk_closest_n_points_strategy_get_closest_n_points_min_value(
    vtkNew<vtkClosestNPointsStrategy> self
) {
    return self->GetClosestNPointsMinValue();
}
extern "C" int vtk_closest_n_points_strategy_get_closest_n_points_max_value(
    vtkNew<vtkClosestNPointsStrategy> self
) {
    return self->GetClosestNPointsMaxValue();
}
extern "C" int vtk_closest_n_points_strategy_get_closest_n_points(
    vtkNew<vtkClosestNPointsStrategy> self
) {
    return self->GetClosestNPoints();
}
extern "C" vtkNew<vtkClosestPointStrategy> vtkClosestPointStrategy_new() {
    return vtkNew<vtkClosestPointStrategy>();
}
extern "C" void vtkClosestPointStrategy_destructor(vtkNew<vtkClosestPointStrategy> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkClosestPointStrategy_get_ptr(vtkNew<vtkClosestPointStrategy> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_closest_point_strategy_inside_cell_bounds(
    vtkNew<vtkClosestPointStrategy> self, double* x, long long cellId
) {
    return self->InsideCellBounds(x, cellId);
}
extern "C" vtkNew<vtkCone> vtkCone_new() {
    return vtkNew<vtkCone>();
}
extern "C" void vtkCone_destructor(vtkNew<vtkCone> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCone_get_ptr(vtkNew<vtkCone> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_cone_evaluate_function(vtkNew<vtkCone> self, double* x) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_cone_evaluate_gradient(vtkNew<vtkCone> self, double* x, double* g) {
    return self->EvaluateGradient(x, g);
}
extern "C" void vtk_cone_set_angle(vtkNew<vtkCone> self, double _arg) {
    return self->SetAngle(_arg);
}
extern "C" double vtk_cone_get_angle_min_value(vtkNew<vtkCone> self) {
    return self->GetAngleMinValue();
}
extern "C" double vtk_cone_get_angle_max_value(vtkNew<vtkCone> self) {
    return self->GetAngleMaxValue();
}
extern "C" double vtk_cone_get_angle(vtkNew<vtkCone> self) {
    return self->GetAngle();
}
extern "C" void vtk_cone_set_origin_double_double_double(
    vtkNew<vtkCone> self, double x, double y, double z
) {
    return self->SetOrigin(x, y, z);
}
extern "C" void vtk_cone_set_origin_const_double_3(vtkNew<vtkCone> self, const double* xyz) {
    return self->SetOrigin(xyz);
}
extern "C" void vtk_cone_get_origin_double_3(vtkNew<vtkCone> self, double* _arg) {
    return self->GetOrigin(_arg);
}
extern "C" void vtk_cone_set_axis_double_double_double(
    vtkNew<vtkCone> self, double x, double y, double z
) {
    return self->SetAxis(x, y, z);
}
extern "C" void vtk_cone_set_axis_double_3(vtkNew<vtkCone> self, double* axis) {
    return self->SetAxis(axis);
}
extern "C" void vtk_cone_get_axis_double_3(vtkNew<vtkCone> self, double* _arg) {
    return self->GetAxis(_arg);
}
extern "C" void vtk_cone_set_is_double_cone(vtkNew<vtkCone> self, bool _arg) {
    return self->SetIsDoubleCone(_arg);
}
extern "C" bool vtk_cone_get_is_double_cone(vtkNew<vtkCone> self) {
    return self->GetIsDoubleCone();
}
extern "C" void vtk_cone_is_double_cone_on(vtkNew<vtkCone> self) {
    return self->IsDoubleConeOn();
}
extern "C" void vtk_cone_is_double_cone_off(vtkNew<vtkCone> self) {
    return self->IsDoubleConeOff();
}
extern "C" vtkNew<vtkConvexPointSet> vtkConvexPointSet_new() {
    return vtkNew<vtkConvexPointSet>();
}
extern "C" void vtkConvexPointSet_destructor(vtkNew<vtkConvexPointSet> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConvexPointSet_get_ptr(vtkNew<vtkConvexPointSet> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_convex_point_set_has_fixed_topology(vtkNew<vtkConvexPointSet> self) {
    return self->HasFixedTopology();
}
extern "C" bool vtk_convex_point_set_get_centroid(
    vtkNew<vtkConvexPointSet> self, double* centroid
) {
    return self->GetCentroid(centroid);
}
extern "C" int vtk_convex_point_set_get_cell_type(vtkNew<vtkConvexPointSet> self) {
    return self->GetCellType();
}
extern "C" int vtk_convex_point_set_requires_initialization(vtkNew<vtkConvexPointSet> self) {
    return self->RequiresInitialization();
}
extern "C" int vtk_convex_point_set_get_number_of_edges(vtkNew<vtkConvexPointSet> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_convex_point_set_get_number_of_faces(vtkNew<vtkConvexPointSet> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_convex_point_set_get_parametric_center(
    vtkNew<vtkConvexPointSet> self, double* pcoords
) {
    return self->GetParametricCenter(pcoords);
}
extern "C" int vtk_convex_point_set_is_primary_cell(vtkNew<vtkConvexPointSet> self) {
    return self->IsPrimaryCell();
}
extern "C" vtkNew<vtkCoordinateFrame> vtkCoordinateFrame_new() {
    return vtkNew<vtkCoordinateFrame>();
}
extern "C" void vtkCoordinateFrame_destructor(vtkNew<vtkCoordinateFrame> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCoordinateFrame_get_ptr(vtkNew<vtkCoordinateFrame> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_coordinate_frame_evaluate_function(
    vtkNew<vtkCoordinateFrame> self, double* x
) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_coordinate_frame_evaluate_gradient(
    vtkNew<vtkCoordinateFrame> self, double* x, double* n
) {
    return self->EvaluateGradient(x, n);
}
extern "C" void vtk_coordinate_frame_set_origin_double_double_double(
    vtkNew<vtkCoordinateFrame> self, double _arg1, double _arg2, double _arg3
) {
    return self->SetOrigin(_arg1, _arg2, _arg3);
}
extern "C" void vtk_coordinate_frame_set_origin_const_double_3(
    vtkNew<vtkCoordinateFrame> self, const double* _arg
) {
    return self->SetOrigin(_arg);
}
extern "C" void vtk_coordinate_frame_get_origin_double_3(
    vtkNew<vtkCoordinateFrame> self, double* _arg
) {
    return self->GetOrigin(_arg);
}
extern "C" void vtk_coordinate_frame_set_x_axis_double_double_double(
    vtkNew<vtkCoordinateFrame> self, double _arg1, double _arg2, double _arg3
) {
    return self->SetXAxis(_arg1, _arg2, _arg3);
}
extern "C" void vtk_coordinate_frame_set_x_axis_const_double_3(
    vtkNew<vtkCoordinateFrame> self, const double* _arg
) {
    return self->SetXAxis(_arg);
}
extern "C" void vtk_coordinate_frame_get_x_axis_double_3(
    vtkNew<vtkCoordinateFrame> self, double* _arg
) {
    return self->GetXAxis(_arg);
}
extern "C" void vtk_coordinate_frame_set_y_axis_double_double_double(
    vtkNew<vtkCoordinateFrame> self, double _arg1, double _arg2, double _arg3
) {
    return self->SetYAxis(_arg1, _arg2, _arg3);
}
extern "C" void vtk_coordinate_frame_set_y_axis_const_double_3(
    vtkNew<vtkCoordinateFrame> self, const double* _arg
) {
    return self->SetYAxis(_arg);
}
extern "C" void vtk_coordinate_frame_get_y_axis_double_3(
    vtkNew<vtkCoordinateFrame> self, double* _arg
) {
    return self->GetYAxis(_arg);
}
extern "C" void vtk_coordinate_frame_set_z_axis_double_double_double(
    vtkNew<vtkCoordinateFrame> self, double _arg1, double _arg2, double _arg3
) {
    return self->SetZAxis(_arg1, _arg2, _arg3);
}
extern "C" void vtk_coordinate_frame_set_z_axis_const_double_3(
    vtkNew<vtkCoordinateFrame> self, const double* _arg
) {
    return self->SetZAxis(_arg);
}
extern "C" void vtk_coordinate_frame_get_z_axis_double_3(
    vtkNew<vtkCoordinateFrame> self, double* _arg
) {
    return self->GetZAxis(_arg);
}
extern "C" vtkNew<vtkCubicLine> vtkCubicLine_new() {
    return vtkNew<vtkCubicLine>();
}
extern "C" void vtkCubicLine_destructor(vtkNew<vtkCubicLine> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCubicLine_get_ptr(vtkNew<vtkCubicLine> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_cubic_line_get_cell_type(vtkNew<vtkCubicLine> self) {
    return self->GetCellType();
}
extern "C" int vtk_cubic_line_get_cell_dimension(vtkNew<vtkCubicLine> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_cubic_line_get_number_of_edges(vtkNew<vtkCubicLine> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_cubic_line_get_number_of_faces(vtkNew<vtkCubicLine> self) {
    return self->GetNumberOfFaces();
}
extern "C" double vtk_cubic_line_get_parametric_distance(
    vtkNew<vtkCubicLine> self, const double* pcoords
) {
    return self->GetParametricDistance(pcoords);
}
extern "C" int vtk_cubic_line_get_parametric_center(vtkNew<vtkCubicLine> self, double* pcoords) {
    return self->GetParametricCenter(pcoords);
}
extern "C" void vtk_cubic_line_interpolation_functions(
    vtkNew<vtkCubicLine> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_cubic_line_interpolation_derivs(
    vtkNew<vtkCubicLine> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_cubic_line_interpolate_functions(
    vtkNew<vtkCubicLine> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_cubic_line_interpolate_derivs(
    vtkNew<vtkCubicLine> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkCylinder> vtkCylinder_new() {
    return vtkNew<vtkCylinder>();
}
extern "C" void vtkCylinder_destructor(vtkNew<vtkCylinder> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCylinder_get_ptr(vtkNew<vtkCylinder> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_cylinder_evaluate_function(vtkNew<vtkCylinder> self, double* x) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_cylinder_evaluate_gradient(vtkNew<vtkCylinder> self, double* x, double* g) {
    return self->EvaluateGradient(x, g);
}
extern "C" void vtk_cylinder_set_radius(vtkNew<vtkCylinder> self, double _arg) {
    return self->SetRadius(_arg);
}
extern "C" double vtk_cylinder_get_radius_min_value(vtkNew<vtkCylinder> self) {
    return self->GetRadiusMinValue();
}
extern "C" double vtk_cylinder_get_radius_max_value(vtkNew<vtkCylinder> self) {
    return self->GetRadiusMaxValue();
}
extern "C" double vtk_cylinder_get_radius(vtkNew<vtkCylinder> self) {
    return self->GetRadius();
}
extern "C" void vtk_cylinder_set_center_double_double_double(
    vtkNew<vtkCylinder> self, double _arg1, double _arg2, double _arg3
) {
    return self->SetCenter(_arg1, _arg2, _arg3);
}
extern "C" void vtk_cylinder_set_center_const_double_3(
    vtkNew<vtkCylinder> self, const double* _arg
) {
    return self->SetCenter(_arg);
}
extern "C" void vtk_cylinder_get_center_double_3(vtkNew<vtkCylinder> self, double* _arg) {
    return self->GetCenter(_arg);
}
extern "C" void vtk_cylinder_set_axis_double_double_double(
    vtkNew<vtkCylinder> self, double ax, double ay, double az
) {
    return self->SetAxis(ax, ay, az);
}
extern "C" void vtk_cylinder_set_axis_double_3(vtkNew<vtkCylinder> self, double* a) {
    return self->SetAxis(a);
}
extern "C" void vtk_cylinder_get_axis_double_3(vtkNew<vtkCylinder> self, double* _arg) {
    return self->GetAxis(_arg);
}
extern "C" vtkNew<vtkDataAssembly> vtkDataAssembly_new() {
    return vtkNew<vtkDataAssembly>();
}
extern "C" void vtkDataAssembly_destructor(vtkNew<vtkDataAssembly> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDataAssembly_get_ptr(vtkNew<vtkDataAssembly> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_data_assembly_initialize(vtkNew<vtkDataAssembly> self) {
    return self->Initialize();
}
extern "C" int vtk_data_assembly_get_root_node(vtkNew<vtkDataAssembly> self) {
    return self->GetRootNode();
}
extern "C" bool vtk_data_assembly_remove_node(vtkNew<vtkDataAssembly> self, int id) {
    return self->RemoveNode(id);
}
extern "C" bool vtk_data_assembly_add_data_set_index(
    vtkNew<vtkDataAssembly> self, int id, unsigned int dataset_index
) {
    return self->AddDataSetIndex(id, dataset_index);
}
extern "C" bool vtk_data_assembly_add_data_set_index_range(
    vtkNew<vtkDataAssembly> self, int id, unsigned int index_start, int count
) {
    return self->AddDataSetIndexRange(id, index_start, count);
}
extern "C" bool vtk_data_assembly_remove_data_set_index(
    vtkNew<vtkDataAssembly> self, int id, unsigned int dataset_index
) {
    return self->RemoveDataSetIndex(id, dataset_index);
}
extern "C" bool vtk_data_assembly_remove_all_data_set_indices(
    vtkNew<vtkDataAssembly> self, int id, bool traverse_subtree
) {
    return self->RemoveAllDataSetIndices(id, traverse_subtree);
}
extern "C" int vtk_data_assembly_get_number_of_children(vtkNew<vtkDataAssembly> self, int parent) {
    return self->GetNumberOfChildren(parent);
}
extern "C" int vtk_data_assembly_get_child(vtkNew<vtkDataAssembly> self, int parent, int index) {
    return self->GetChild(parent, index);
}
extern "C" int vtk_data_assembly_get_child_index(
    vtkNew<vtkDataAssembly> self, int parent, int child
) {
    return self->GetChildIndex(parent, child);
}
extern "C" int vtk_data_assembly_get_parent(vtkNew<vtkDataAssembly> self, int id) {
    return self->GetParent(id);
}
extern "C" vtkNew<vtkDataAssemblyUtilities> vtkDataAssemblyUtilities_new() {
    return vtkNew<vtkDataAssemblyUtilities>();
}
extern "C" void vtkDataAssemblyUtilities_destructor(vtkNew<vtkDataAssemblyUtilities> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDataAssemblyUtilities_get_ptr(vtkNew<vtkDataAssemblyUtilities> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkDataObject> vtkDataObject_new() {
    return vtkNew<vtkDataObject>();
}
extern "C" void vtkDataObject_destructor(vtkNew<vtkDataObject> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDataObject_get_ptr(vtkNew<vtkDataObject> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long vtk_data_object_get_m_time(vtkNew<vtkDataObject> self) {
    return self->GetMTime();
}
extern "C" void vtk_data_object_initialize(vtkNew<vtkDataObject> self) {
    return self->Initialize();
}
extern "C" void vtk_data_object_release_data(vtkNew<vtkDataObject> self) {
    return self->ReleaseData();
}
extern "C" int vtk_data_object_get_data_released(vtkNew<vtkDataObject> self) {
    return self->GetDataReleased();
}
extern "C" void vtk_data_object_set_global_release_data_flag(vtkNew<vtkDataObject> self, int val) {
    return self->SetGlobalReleaseDataFlag(val);
}
extern "C" void vtk_data_object_global_release_data_flag_on(vtkNew<vtkDataObject> self) {
    return self->GlobalReleaseDataFlagOn();
}
extern "C" void vtk_data_object_global_release_data_flag_off(vtkNew<vtkDataObject> self) {
    return self->GlobalReleaseDataFlagOff();
}
extern "C" int vtk_data_object_get_global_release_data_flag(vtkNew<vtkDataObject> self) {
    return self->GetGlobalReleaseDataFlag();
}
extern "C" int vtk_data_object_get_data_object_type(vtkNew<vtkDataObject> self) {
    return self->GetDataObjectType();
}
extern "C" unsigned long vtk_data_object_get_update_time(vtkNew<vtkDataObject> self) {
    return self->GetUpdateTime();
}
extern "C" unsigned long vtk_data_object_get_actual_memory_size(vtkNew<vtkDataObject> self) {
    return self->GetActualMemorySize();
}
extern "C" void vtk_data_object_data_has_been_generated(vtkNew<vtkDataObject> self) {
    return self->DataHasBeenGenerated();
}
extern "C" void vtk_data_object_prepare_for_new_data(vtkNew<vtkDataObject> self) {
    return self->PrepareForNewData();
}
extern "C" int vtk_data_object_get_extent_type(vtkNew<vtkDataObject> self) {
    return self->GetExtentType();
}
extern "C" bool vtk_data_object_supports_ghost_array(vtkNew<vtkDataObject> self, int type) {
    return self->SupportsGhostArray(type);
}
extern "C" long long vtk_data_object_get_number_of_elements(vtkNew<vtkDataObject> self, int type) {
    return self->GetNumberOfElements(type);
}
extern "C" vtkNew<vtkDataObjectCollection> vtkDataObjectCollection_new() {
    return vtkNew<vtkDataObjectCollection>();
}
extern "C" void vtkDataObjectCollection_destructor(vtkNew<vtkDataObjectCollection> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDataObjectCollection_get_ptr(vtkNew<vtkDataObjectCollection> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_data_object_collection_get_number_of_items(
    vtkNew<vtkDataObjectCollection> self
) {
    return self->GetNumberOfItems();
}
extern "C" vtkNew<vtkDataObjectTreeIterator> vtkDataObjectTreeIterator_new() {
    return vtkNew<vtkDataObjectTreeIterator>();
}
extern "C" void vtkDataObjectTreeIterator_destructor(vtkNew<vtkDataObjectTreeIterator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDataObjectTreeIterator_get_ptr(vtkNew<vtkDataObjectTreeIterator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_data_object_tree_iterator_go_to_first_item(
    vtkNew<vtkDataObjectTreeIterator> self
) {
    return self->GoToFirstItem();
}
extern "C" void vtk_data_object_tree_iterator_go_to_next_item(
    vtkNew<vtkDataObjectTreeIterator> self
) {
    return self->GoToNextItem();
}
extern "C" int vtk_data_object_tree_iterator_is_done_with_traversal(
    vtkNew<vtkDataObjectTreeIterator> self
) {
    return self->IsDoneWithTraversal();
}
extern "C" int vtk_data_object_tree_iterator_has_current_meta_data(
    vtkNew<vtkDataObjectTreeIterator> self
) {
    return self->HasCurrentMetaData();
}
extern "C" unsigned int vtk_data_object_tree_iterator_get_current_flat_index(
    vtkNew<vtkDataObjectTreeIterator> self
) {
    return self->GetCurrentFlatIndex();
}
extern "C" void vtk_data_object_tree_iterator_set_visit_only_leaves(
    vtkNew<vtkDataObjectTreeIterator> self, int _arg
) {
    return self->SetVisitOnlyLeaves(_arg);
}
extern "C" int vtk_data_object_tree_iterator_get_visit_only_leaves(
    vtkNew<vtkDataObjectTreeIterator> self
) {
    return self->GetVisitOnlyLeaves();
}
extern "C" void vtk_data_object_tree_iterator_visit_only_leaves_on(
    vtkNew<vtkDataObjectTreeIterator> self
) {
    return self->VisitOnlyLeavesOn();
}
extern "C" void vtk_data_object_tree_iterator_visit_only_leaves_off(
    vtkNew<vtkDataObjectTreeIterator> self
) {
    return self->VisitOnlyLeavesOff();
}
extern "C" void vtk_data_object_tree_iterator_set_traverse_sub_tree(
    vtkNew<vtkDataObjectTreeIterator> self, int _arg
) {
    return self->SetTraverseSubTree(_arg);
}
extern "C" int vtk_data_object_tree_iterator_get_traverse_sub_tree(
    vtkNew<vtkDataObjectTreeIterator> self
) {
    return self->GetTraverseSubTree();
}
extern "C" void vtk_data_object_tree_iterator_traverse_sub_tree_on(
    vtkNew<vtkDataObjectTreeIterator> self
) {
    return self->TraverseSubTreeOn();
}
extern "C" void vtk_data_object_tree_iterator_traverse_sub_tree_off(
    vtkNew<vtkDataObjectTreeIterator> self
) {
    return self->TraverseSubTreeOff();
}
extern "C" vtkNew<vtkDataObjectTypes> vtkDataObjectTypes_new() {
    return vtkNew<vtkDataObjectTypes>();
}
extern "C" void vtkDataObjectTypes_destructor(vtkNew<vtkDataObjectTypes> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDataObjectTypes_get_ptr(vtkNew<vtkDataObjectTypes> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_data_object_types_type_id_is_a(
    vtkNew<vtkDataObjectTypes> self, int typeId, int targetTypeId
) {
    return self->TypeIdIsA(typeId, targetTypeId);
}
extern "C" int vtk_data_object_types_get_common_base_type_id(
    vtkNew<vtkDataObjectTypes> self, int typeA, int typeB
) {
    return self->GetCommonBaseTypeId(typeA, typeB);
}
extern "C" vtkNew<vtkDataSetAttributes> vtkDataSetAttributes_new() {
    return vtkNew<vtkDataSetAttributes>();
}
extern "C" void vtkDataSetAttributes_destructor(vtkNew<vtkDataSetAttributes> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDataSetAttributes_get_ptr(vtkNew<vtkDataSetAttributes> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_data_set_attributes_initialize(vtkNew<vtkDataSetAttributes> self) {
    return self->Initialize();
}
extern "C" void vtk_data_set_attributes_update(vtkNew<vtkDataSetAttributes> self) {
    return self->Update();
}
extern "C" int vtk_data_set_attributes_set_active_attribute_c_int_c_int(
    vtkNew<vtkDataSetAttributes> self, int index, int attributeType
) {
    return self->SetActiveAttribute(index, attributeType);
}
extern "C" int vtk_data_set_attributes_is_array_an_attribute(
    vtkNew<vtkDataSetAttributes> self, int idx
) {
    return self->IsArrayAnAttribute(idx);
}
extern "C" void vtk_data_set_attributes_set_copy_attribute(
    vtkNew<vtkDataSetAttributes> self, int index, int value, int ctype
) {
    return self->SetCopyAttribute(index, value, ctype);
}
extern "C" int vtk_data_set_attributes_get_copy_attribute(
    vtkNew<vtkDataSetAttributes> self, int index, int ctype
) {
    return self->GetCopyAttribute(index, ctype);
}
extern "C" void vtk_data_set_attributes_set_copy_scalars(
    vtkNew<vtkDataSetAttributes> self, int i, int ctype
) {
    return self->SetCopyScalars(i, ctype);
}
extern "C" int vtk_data_set_attributes_get_copy_scalars(
    vtkNew<vtkDataSetAttributes> self, int ctype
) {
    return self->GetCopyScalars(ctype);
}
extern "C" void vtk_data_set_attributes_copy_scalars_on(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyScalarsOn();
}
extern "C" void vtk_data_set_attributes_copy_scalars_off(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyScalarsOff();
}
extern "C" void vtk_data_set_attributes_set_copy_vectors(
    vtkNew<vtkDataSetAttributes> self, int i, int ctype
) {
    return self->SetCopyVectors(i, ctype);
}
extern "C" int vtk_data_set_attributes_get_copy_vectors(
    vtkNew<vtkDataSetAttributes> self, int ctype
) {
    return self->GetCopyVectors(ctype);
}
extern "C" void vtk_data_set_attributes_copy_vectors_on(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyVectorsOn();
}
extern "C" void vtk_data_set_attributes_copy_vectors_off(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyVectorsOff();
}
extern "C" void vtk_data_set_attributes_set_copy_normals(
    vtkNew<vtkDataSetAttributes> self, int i, int ctype
) {
    return self->SetCopyNormals(i, ctype);
}
extern "C" int vtk_data_set_attributes_get_copy_normals(
    vtkNew<vtkDataSetAttributes> self, int ctype
) {
    return self->GetCopyNormals(ctype);
}
extern "C" void vtk_data_set_attributes_copy_normals_on(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyNormalsOn();
}
extern "C" void vtk_data_set_attributes_copy_normals_off(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyNormalsOff();
}
extern "C" void vtk_data_set_attributes_set_copy_tangents(
    vtkNew<vtkDataSetAttributes> self, int i, int ctype
) {
    return self->SetCopyTangents(i, ctype);
}
extern "C" int vtk_data_set_attributes_get_copy_tangents(
    vtkNew<vtkDataSetAttributes> self, int ctype
) {
    return self->GetCopyTangents(ctype);
}
extern "C" void vtk_data_set_attributes_copy_tangents_on(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyTangentsOn();
}
extern "C" void vtk_data_set_attributes_copy_tangents_off(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyTangentsOff();
}
extern "C" void vtk_data_set_attributes_set_copy_t_coords(
    vtkNew<vtkDataSetAttributes> self, int i, int ctype
) {
    return self->SetCopyTCoords(i, ctype);
}
extern "C" int vtk_data_set_attributes_get_copy_t_coords(
    vtkNew<vtkDataSetAttributes> self, int ctype
) {
    return self->GetCopyTCoords(ctype);
}
extern "C" void vtk_data_set_attributes_copy_t_coords_on(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyTCoordsOn();
}
extern "C" void vtk_data_set_attributes_copy_t_coords_off(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyTCoordsOff();
}
extern "C" void vtk_data_set_attributes_set_copy_tensors(
    vtkNew<vtkDataSetAttributes> self, int i, int ctype
) {
    return self->SetCopyTensors(i, ctype);
}
extern "C" int vtk_data_set_attributes_get_copy_tensors(
    vtkNew<vtkDataSetAttributes> self, int ctype
) {
    return self->GetCopyTensors(ctype);
}
extern "C" void vtk_data_set_attributes_copy_tensors_on(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyTensorsOn();
}
extern "C" void vtk_data_set_attributes_copy_tensors_off(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyTensorsOff();
}
extern "C" void vtk_data_set_attributes_set_copy_global_ids(
    vtkNew<vtkDataSetAttributes> self, int i, int ctype
) {
    return self->SetCopyGlobalIds(i, ctype);
}
extern "C" int vtk_data_set_attributes_get_copy_global_ids(
    vtkNew<vtkDataSetAttributes> self, int ctype
) {
    return self->GetCopyGlobalIds(ctype);
}
extern "C" void vtk_data_set_attributes_copy_global_ids_on(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyGlobalIdsOn();
}
extern "C" void vtk_data_set_attributes_copy_global_ids_off(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyGlobalIdsOff();
}
extern "C" void vtk_data_set_attributes_set_copy_pedigree_ids(
    vtkNew<vtkDataSetAttributes> self, int i, int ctype
) {
    return self->SetCopyPedigreeIds(i, ctype);
}
extern "C" int vtk_data_set_attributes_get_copy_pedigree_ids(
    vtkNew<vtkDataSetAttributes> self, int ctype
) {
    return self->GetCopyPedigreeIds(ctype);
}
extern "C" void vtk_data_set_attributes_copy_pedigree_ids_on(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyPedigreeIdsOn();
}
extern "C" void vtk_data_set_attributes_copy_pedigree_ids_off(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyPedigreeIdsOff();
}
extern "C" void vtk_data_set_attributes_set_copy_rational_weights(
    vtkNew<vtkDataSetAttributes> self, int i, int ctype
) {
    return self->SetCopyRationalWeights(i, ctype);
}
extern "C" int vtk_data_set_attributes_get_copy_rational_weights(
    vtkNew<vtkDataSetAttributes> self, int ctype
) {
    return self->GetCopyRationalWeights(ctype);
}
extern "C" void vtk_data_set_attributes_copy_rational_weights_on(
    vtkNew<vtkDataSetAttributes> self
) {
    return self->CopyRationalWeightsOn();
}
extern "C" void vtk_data_set_attributes_copy_rational_weights_off(
    vtkNew<vtkDataSetAttributes> self
) {
    return self->CopyRationalWeightsOff();
}
extern "C" void vtk_data_set_attributes_set_copy_higher_order_degrees(
    vtkNew<vtkDataSetAttributes> self, int i, int ctype
) {
    return self->SetCopyHigherOrderDegrees(i, ctype);
}
extern "C" int vtk_data_set_attributes_get_copy_higher_order_degrees(
    vtkNew<vtkDataSetAttributes> self, int ctype
) {
    return self->GetCopyHigherOrderDegrees(ctype);
}
extern "C" void vtk_data_set_attributes_copy_higher_order_degrees_on(
    vtkNew<vtkDataSetAttributes> self
) {
    return self->CopyHigherOrderDegreesOn();
}
extern "C" void vtk_data_set_attributes_copy_higher_order_degrees_off(
    vtkNew<vtkDataSetAttributes> self
) {
    return self->CopyHigherOrderDegreesOff();
}
extern "C" void vtk_data_set_attributes_set_copy_process_ids(
    vtkNew<vtkDataSetAttributes> self, int i, int ctype
) {
    return self->SetCopyProcessIds(i, ctype);
}
extern "C" int vtk_data_set_attributes_get_copy_process_ids(
    vtkNew<vtkDataSetAttributes> self, int ctype
) {
    return self->GetCopyProcessIds(ctype);
}
extern "C" void vtk_data_set_attributes_copy_process_ids_on(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyProcessIdsOn();
}
extern "C" void vtk_data_set_attributes_copy_process_ids_off(vtkNew<vtkDataSetAttributes> self) {
    return self->CopyProcessIdsOff();
}
extern "C" void vtk_data_set_attributes_copy_all_on(vtkNew<vtkDataSetAttributes> self, int ctype) {
    return self->CopyAllOn(ctype);
}
extern "C" void vtk_data_set_attributes_copy_all_off(vtkNew<vtkDataSetAttributes> self, int ctype) {
    return self->CopyAllOff(ctype);
}
extern "C" vtkNew<vtkDataSetCellIterator> vtkDataSetCellIterator_new() {
    return vtkNew<vtkDataSetCellIterator>();
}
extern "C" void vtkDataSetCellIterator_destructor(vtkNew<vtkDataSetCellIterator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDataSetCellIterator_get_ptr(vtkNew<vtkDataSetCellIterator> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_data_set_cell_iterator_is_done_with_traversal(
    vtkNew<vtkDataSetCellIterator> self
) {
    return self->IsDoneWithTraversal();
}
extern "C" long long vtk_data_set_cell_iterator_get_cell_id(vtkNew<vtkDataSetCellIterator> self) {
    return self->GetCellId();
}
extern "C" vtkNew<vtkDataSetCollection> vtkDataSetCollection_new() {
    return vtkNew<vtkDataSetCollection>();
}
extern "C" void vtkDataSetCollection_destructor(vtkNew<vtkDataSetCollection> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDataSetCollection_get_ptr(vtkNew<vtkDataSetCollection> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_data_set_collection_get_number_of_items(vtkNew<vtkDataSetCollection> self) {
    return self->GetNumberOfItems();
}
extern "C" vtkNew<vtkDirectedAcyclicGraph> vtkDirectedAcyclicGraph_new() {
    return vtkNew<vtkDirectedAcyclicGraph>();
}
extern "C" void vtkDirectedAcyclicGraph_destructor(vtkNew<vtkDirectedAcyclicGraph> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDirectedAcyclicGraph_get_ptr(vtkNew<vtkDirectedAcyclicGraph> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkDirectedGraph> vtkDirectedGraph_new() {
    return vtkNew<vtkDirectedGraph>();
}
extern "C" void vtkDirectedGraph_destructor(vtkNew<vtkDirectedGraph> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDirectedGraph_get_ptr(vtkNew<vtkDirectedGraph> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkEdgeListIterator> vtkEdgeListIterator_new() {
    return vtkNew<vtkEdgeListIterator>();
}
extern "C" void vtkEdgeListIterator_destructor(vtkNew<vtkEdgeListIterator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkEdgeListIterator_get_ptr(vtkNew<vtkEdgeListIterator> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_edge_list_iterator_has_next(vtkNew<vtkEdgeListIterator> self) {
    return self->HasNext();
}
extern "C" vtkNew<vtkEdgeTable> vtkEdgeTable_new() {
    return vtkNew<vtkEdgeTable>();
}
extern "C" void vtkEdgeTable_destructor(vtkNew<vtkEdgeTable> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkEdgeTable_get_ptr(vtkNew<vtkEdgeTable> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_edge_table_initialize(vtkNew<vtkEdgeTable> self) {
    return self->Initialize();
}
extern "C" int vtk_edge_table_init_edge_insertion(
    vtkNew<vtkEdgeTable> self, long long numPoints, int storeAttributes
) {
    return self->InitEdgeInsertion(numPoints, storeAttributes);
}
extern "C" long long vtk_edge_table_insert_edge_c_longlong_c_longlong(
    vtkNew<vtkEdgeTable> self, long long p1, long long p2
) {
    return self->InsertEdge(p1, p2);
}
extern "C" void vtk_edge_table_insert_edge_c_longlong_c_longlong_c_longlong(
    vtkNew<vtkEdgeTable> self, long long p1, long long p2, long long attributeId
) {
    return self->InsertEdge(p1, p2, attributeId);
}
extern "C" long long vtk_edge_table_is_edge_c_longlong_c_longlong(
    vtkNew<vtkEdgeTable> self, long long p1, long long p2
) {
    return self->IsEdge(p1, p2);
}
extern "C" long long vtk_edge_table_get_number_of_edges(vtkNew<vtkEdgeTable> self) {
    return self->GetNumberOfEdges();
}
extern "C" void vtk_edge_table_init_traversal(vtkNew<vtkEdgeTable> self) {
    return self->InitTraversal();
}
extern "C" void vtk_edge_table_reset(vtkNew<vtkEdgeTable> self) {
    return self->Reset();
}
extern "C" vtkNew<vtkEmptyCell> vtkEmptyCell_new() {
    return vtkNew<vtkEmptyCell>();
}
extern "C" void vtkEmptyCell_destructor(vtkNew<vtkEmptyCell> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkEmptyCell_get_ptr(vtkNew<vtkEmptyCell> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_empty_cell_get_cell_type(vtkNew<vtkEmptyCell> self) {
    return self->GetCellType();
}
extern "C" int vtk_empty_cell_get_cell_dimension(vtkNew<vtkEmptyCell> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_empty_cell_get_number_of_edges(vtkNew<vtkEmptyCell> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_empty_cell_get_number_of_faces(vtkNew<vtkEmptyCell> self) {
    return self->GetNumberOfFaces();
}
extern "C" vtkNew<vtkExplicitStructuredGrid> vtkExplicitStructuredGrid_new() {
    return vtkNew<vtkExplicitStructuredGrid>();
}
extern "C" void vtkExplicitStructuredGrid_destructor(vtkNew<vtkExplicitStructuredGrid> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkExplicitStructuredGrid_get_ptr(vtkNew<vtkExplicitStructuredGrid> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_explicit_structured_grid_get_data_object_type(
    vtkNew<vtkExplicitStructuredGrid> self
) {
    return self->GetDataObjectType();
}
extern "C" void vtk_explicit_structured_grid_initialize(vtkNew<vtkExplicitStructuredGrid> self) {
    return self->Initialize();
}
extern "C" int vtk_explicit_structured_grid_get_cell_type(
    vtkNew<vtkExplicitStructuredGrid> self, long long cellId
) {
    return self->GetCellType(cellId);
}
extern "C" long long vtk_explicit_structured_grid_get_cell_size(
    vtkNew<vtkExplicitStructuredGrid> self, long long cellId
) {
    return self->GetCellSize(cellId);
}
extern "C" int vtk_explicit_structured_grid_get_data_dimension(
    vtkNew<vtkExplicitStructuredGrid> self
) {
    return self->GetDataDimension();
}
extern "C" void vtk_explicit_structured_grid_set_dimensions_c_int_c_int_c_int(
    vtkNew<vtkExplicitStructuredGrid> self, int i, int j, int k
) {
    return self->SetDimensions(i, j, k);
}
extern "C" void vtk_explicit_structured_grid_set_dimensions_c_int_3(
    vtkNew<vtkExplicitStructuredGrid> self, int* dim
) {
    return self->SetDimensions(dim);
}
extern "C" void vtk_explicit_structured_grid_get_dimensions(
    vtkNew<vtkExplicitStructuredGrid> self, int* dim
) {
    return self->GetDimensions(dim);
}
extern "C" void vtk_explicit_structured_grid_get_cell_dims(
    vtkNew<vtkExplicitStructuredGrid> self, int* cellDims
) {
    return self->GetCellDims(cellDims);
}
extern "C" int vtk_explicit_structured_grid_get_extent_type(
    vtkNew<vtkExplicitStructuredGrid> self
) {
    return self->GetExtentType();
}
extern "C" void vtk_explicit_structured_grid_set_extent_c_int_c_int_c_int_c_int_c_int_c_int(
    vtkNew<vtkExplicitStructuredGrid> self, int x0, int x1, int y0, int y1, int z0, int z1
) {
    return self->SetExtent(x0, x1, y0, y1, z0, z1);
}
extern "C" void vtk_explicit_structured_grid_set_extent_c_int_6(
    vtkNew<vtkExplicitStructuredGrid> self, int* extent
) {
    return self->SetExtent(extent);
}
extern "C" void vtk_explicit_structured_grid_get_extent_c_int_6(
    vtkNew<vtkExplicitStructuredGrid> self, int* _arg
) {
    return self->GetExtent(_arg);
}
extern "C" void vtk_explicit_structured_grid_build_links(vtkNew<vtkExplicitStructuredGrid> self) {
    return self->BuildLinks();
}
extern "C" long long vtk_explicit_structured_grid_compute_cell_id(
    vtkNew<vtkExplicitStructuredGrid> self, int i, int j, int k, bool adjustForExtent
) {
    return self->ComputeCellId(i, j, k, adjustForExtent);
}
extern "C" void vtk_explicit_structured_grid_compute_faces_connectivity_flags_array(
    vtkNew<vtkExplicitStructuredGrid> self
) {
    return self->ComputeFacesConnectivityFlagsArray();
}
extern "C" void vtk_explicit_structured_grid_blank_cell(
    vtkNew<vtkExplicitStructuredGrid> self, long long cellId
) {
    return self->BlankCell(cellId);
}
extern "C" void vtk_explicit_structured_grid_un_blank_cell(
    vtkNew<vtkExplicitStructuredGrid> self, long long cellId
) {
    return self->UnBlankCell(cellId);
}
extern "C" bool vtk_explicit_structured_grid_has_any_blank_cells(
    vtkNew<vtkExplicitStructuredGrid> self
) {
    return self->HasAnyBlankCells();
}
extern "C" unsigned char vtk_explicit_structured_grid_is_cell_visible(
    vtkNew<vtkExplicitStructuredGrid> self, long long cellId
) {
    return self->IsCellVisible(cellId);
}
extern "C" unsigned char vtk_explicit_structured_grid_is_cell_ghost(
    vtkNew<vtkExplicitStructuredGrid> self, long long cellId
) {
    return self->IsCellGhost(cellId);
}
extern "C" bool vtk_explicit_structured_grid_has_any_ghost_cells(
    vtkNew<vtkExplicitStructuredGrid> self
) {
    return self->HasAnyGhostCells();
}
extern "C" unsigned long vtk_explicit_structured_grid_get_actual_memory_size(
    vtkNew<vtkExplicitStructuredGrid> self
) {
    return self->GetActualMemorySize();
}
extern "C" void vtk_explicit_structured_grid_check_and_reorder_faces(
    vtkNew<vtkExplicitStructuredGrid> self
) {
    return self->CheckAndReorderFaces();
}
extern "C" vtkNew<vtkExtractStructuredGridHelper> vtkExtractStructuredGridHelper_new() {
    return vtkNew<vtkExtractStructuredGridHelper>();
}
extern "C" void vtkExtractStructuredGridHelper_destructor(
    vtkNew<vtkExtractStructuredGridHelper> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkExtractStructuredGridHelper_get_ptr(
    vtkNew<vtkExtractStructuredGridHelper> sself
) {
    return sself.GetPointer();
}
extern "C" void vtk_extract_structured_grid_helper_get_output_whole_extent_c_int_6(
    vtkNew<vtkExtractStructuredGridHelper> self, int* _arg
) {
    return self->GetOutputWholeExtent(_arg);
}
extern "C" void vtk_extract_structured_grid_helper_initialize(
    vtkNew<vtkExtractStructuredGridHelper> self,
    int* voi,
    int* wholeExt,
    int* sampleRate,
    bool includeBoundary
) {
    return self->Initialize(voi, wholeExt, sampleRate, includeBoundary);
}
extern "C" bool vtk_extract_structured_grid_helper_is_valid(
    vtkNew<vtkExtractStructuredGridHelper> self
) {
    return self->IsValid();
}
extern "C" int vtk_extract_structured_grid_helper_get_size(
    vtkNew<vtkExtractStructuredGridHelper> self, int dim
) {
    return self->GetSize(dim);
}
extern "C" int vtk_extract_structured_grid_helper_get_mapped_index(
    vtkNew<vtkExtractStructuredGridHelper> self, int dim, int outIdx
) {
    return self->GetMappedIndex(dim, outIdx);
}
extern "C" int vtk_extract_structured_grid_helper_get_mapped_index_from_extent_value(
    vtkNew<vtkExtractStructuredGridHelper> self, int dim, int outExtVal
) {
    return self->GetMappedIndexFromExtentValue(dim, outExtVal);
}
extern "C" int vtk_extract_structured_grid_helper_get_mapped_extent_value(
    vtkNew<vtkExtractStructuredGridHelper> self, int dim, int outExtVal
) {
    return self->GetMappedExtentValue(dim, outExtVal);
}
extern "C" int vtk_extract_structured_grid_helper_get_mapped_extent_value_from_index(
    vtkNew<vtkExtractStructuredGridHelper> self, int dim, int outIdx
) {
    return self->GetMappedExtentValueFromIndex(dim, outIdx);
}
extern "C" void vtk_extract_structured_grid_helper_compute_begin_and_end(
    vtkNew<vtkExtractStructuredGridHelper> self, int* inExt, int* voi, int* begin, int* end
) {
    return self->ComputeBeginAndEnd(inExt, voi, begin, end);
}
extern "C" void vtk_extract_structured_grid_helper_get_partitioned_voi(
    vtkNew<vtkExtractStructuredGridHelper> self,
    const int* globalVOI,
    const int* partitionedExtent,
    const int* sampleRate,
    bool includeBoundary,
    int* partitionedVOI
) {
    return self->GetPartitionedVOI(
        globalVOI, partitionedExtent, sampleRate, includeBoundary, partitionedVOI
    );
}
extern "C" void vtk_extract_structured_grid_helper_get_partitioned_output_extent(
    vtkNew<vtkExtractStructuredGridHelper> self,
    const int* globalVOI,
    const int* partitionedVOI,
    const int* outputWholeExtent,
    const int* sampleRate,
    bool includeBoundary,
    int* partitionedOutputExtent
) {
    return self->GetPartitionedOutputExtent(
        globalVOI, partitionedVOI, outputWholeExtent, sampleRate, includeBoundary,
        partitionedOutputExtent
    );
}
extern "C" vtkNew<vtkFieldData> vtkFieldData_new() {
    return vtkNew<vtkFieldData>();
}
extern "C" void vtkFieldData_destructor(vtkNew<vtkFieldData> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkFieldData_get_ptr(vtkNew<vtkFieldData> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_field_data_initialize(vtkNew<vtkFieldData> self) {
    return self->Initialize();
}
extern "C" int vtk_field_data_allocate(vtkNew<vtkFieldData> self, long long sz, long long ext) {
    return self->Allocate(sz, ext);
}
extern "C" void vtk_field_data_allocate_arrays(vtkNew<vtkFieldData> self, int num) {
    return self->AllocateArrays(num);
}
extern "C" int vtk_field_data_get_number_of_arrays(vtkNew<vtkFieldData> self) {
    return self->GetNumberOfArrays();
}
extern "C" void vtk_field_data_null_data(vtkNew<vtkFieldData> self, long long id) {
    return self->NullData(id);
}
extern "C" void vtk_field_data_remove_array_c_int(vtkNew<vtkFieldData> self, int index) {
    return self->RemoveArray(index);
}
extern "C" void vtk_field_data_copy_all_on(vtkNew<vtkFieldData> self, int unused) {
    return self->CopyAllOn(unused);
}
extern "C" void vtk_field_data_copy_all_off(vtkNew<vtkFieldData> self, int unused) {
    return self->CopyAllOff(unused);
}
extern "C" void vtk_field_data_squeeze(vtkNew<vtkFieldData> self) {
    return self->Squeeze();
}
extern "C" void vtk_field_data_reset(vtkNew<vtkFieldData> self) {
    return self->Reset();
}
extern "C" unsigned long vtk_field_data_get_actual_memory_size(vtkNew<vtkFieldData> self) {
    return self->GetActualMemorySize();
}
extern "C" unsigned long vtk_field_data_get_m_time(vtkNew<vtkFieldData> self) {
    return self->GetMTime();
}
extern "C" int vtk_field_data_get_number_of_components(vtkNew<vtkFieldData> self) {
    return self->GetNumberOfComponents();
}
extern "C" long long vtk_field_data_get_number_of_tuples(vtkNew<vtkFieldData> self) {
    return self->GetNumberOfTuples();
}
extern "C" void vtk_field_data_set_number_of_tuples(vtkNew<vtkFieldData> self, long long number) {
    return self->SetNumberOfTuples(number);
}
extern "C" bool vtk_field_data_get_range_c_int_double_2_c_int(
    vtkNew<vtkFieldData> self, int index, double* range, int comp
) {
    return self->GetRange(index, range, comp);
}
extern "C" bool vtk_field_data_get_finite_range_c_int_double_2_c_int(
    vtkNew<vtkFieldData> self, int index, double* range, int comp
) {
    return self->GetFiniteRange(index, range, comp);
}
extern "C" unsigned char vtk_field_data_get_ghosts_to_skip(vtkNew<vtkFieldData> self) {
    return self->GetGhostsToSkip();
}
extern "C" void vtk_field_data_set_ghosts_to_skip(vtkNew<vtkFieldData> self, unsigned char p0) {
    return self->SetGhostsToSkip(p0);
}
extern "C" bool vtk_field_data_has_any_ghost_bit_set(vtkNew<vtkFieldData> self, int bitFlag) {
    return self->HasAnyGhostBitSet(bitFlag);
}
extern "C" vtkNew<vtkFrustum> vtkFrustum_new() {
    return vtkNew<vtkFrustum>();
}
extern "C" void vtkFrustum_destructor(vtkNew<vtkFrustum> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkFrustum_get_ptr(vtkNew<vtkFrustum> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_frustum_evaluate_function(vtkNew<vtkFrustum> self, double* x) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_frustum_evaluate_gradient(vtkNew<vtkFrustum> self, double* x, double* g) {
    return self->EvaluateGradient(x, g);
}
extern "C" double vtk_frustum_get_near_plane_distance(vtkNew<vtkFrustum> self) {
    return self->GetNearPlaneDistance();
}
extern "C" void vtk_frustum_set_near_plane_distance(vtkNew<vtkFrustum> self, double distance) {
    return self->SetNearPlaneDistance(distance);
}
extern "C" double vtk_frustum_get_horizontal_angle(vtkNew<vtkFrustum> self) {
    return self->GetHorizontalAngle();
}
extern "C" void vtk_frustum_set_horizontal_angle(vtkNew<vtkFrustum> self, double angleInDegrees) {
    return self->SetHorizontalAngle(angleInDegrees);
}
extern "C" double vtk_frustum_get_vertical_angle(vtkNew<vtkFrustum> self) {
    return self->GetVerticalAngle();
}
extern "C" void vtk_frustum_set_vertical_angle(vtkNew<vtkFrustum> self, double angleInDegrees) {
    return self->SetVerticalAngle(angleInDegrees);
}
extern "C" vtkNew<vtkGenericAttributeCollection> vtkGenericAttributeCollection_new() {
    return vtkNew<vtkGenericAttributeCollection>();
}
extern "C" void vtkGenericAttributeCollection_destructor(
    vtkNew<vtkGenericAttributeCollection> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkGenericAttributeCollection_get_ptr(
    vtkNew<vtkGenericAttributeCollection> sself
) {
    return sself.GetPointer();
}
extern "C" int vtk_generic_attribute_collection_get_number_of_attributes(
    vtkNew<vtkGenericAttributeCollection> self
) {
    return self->GetNumberOfAttributes();
}
extern "C" int vtk_generic_attribute_collection_get_number_of_components(
    vtkNew<vtkGenericAttributeCollection> self
) {
    return self->GetNumberOfComponents();
}
extern "C" int vtk_generic_attribute_collection_get_number_of_point_centered_components(
    vtkNew<vtkGenericAttributeCollection> self
) {
    return self->GetNumberOfPointCenteredComponents();
}
extern "C" int vtk_generic_attribute_collection_get_max_number_of_components(
    vtkNew<vtkGenericAttributeCollection> self
) {
    return self->GetMaxNumberOfComponents();
}
extern "C" unsigned long vtk_generic_attribute_collection_get_actual_memory_size(
    vtkNew<vtkGenericAttributeCollection> self
) {
    return self->GetActualMemorySize();
}
extern "C" int vtk_generic_attribute_collection_is_empty(
    vtkNew<vtkGenericAttributeCollection> self
) {
    return self->IsEmpty();
}
extern "C" int vtk_generic_attribute_collection_get_attribute_index(
    vtkNew<vtkGenericAttributeCollection> self, int i
) {
    return self->GetAttributeIndex(i);
}
extern "C" void vtk_generic_attribute_collection_remove_attribute(
    vtkNew<vtkGenericAttributeCollection> self, int i
) {
    return self->RemoveAttribute(i);
}
extern "C" void vtk_generic_attribute_collection_reset(vtkNew<vtkGenericAttributeCollection> self) {
    return self->Reset();
}
extern "C" unsigned long vtk_generic_attribute_collection_get_m_time(
    vtkNew<vtkGenericAttributeCollection> self
) {
    return self->GetMTime();
}
extern "C" int vtk_generic_attribute_collection_get_active_attribute(
    vtkNew<vtkGenericAttributeCollection> self
) {
    return self->GetActiveAttribute();
}
extern "C" int vtk_generic_attribute_collection_get_active_component(
    vtkNew<vtkGenericAttributeCollection> self
) {
    return self->GetActiveComponent();
}
extern "C" void vtk_generic_attribute_collection_set_active_attribute(
    vtkNew<vtkGenericAttributeCollection> self, int attribute, int component
) {
    return self->SetActiveAttribute(attribute, component);
}
extern "C" int vtk_generic_attribute_collection_get_number_of_attributes_to_interpolate(
    vtkNew<vtkGenericAttributeCollection> self
) {
    return self->GetNumberOfAttributesToInterpolate();
}
extern "C" void vtk_generic_attribute_collection_set_attributes_to_interpolate_to_all(
    vtkNew<vtkGenericAttributeCollection> self
) {
    return self->SetAttributesToInterpolateToAll();
}
extern "C" vtkNew<vtkGenericCell> vtkGenericCell_new() {
    return vtkNew<vtkGenericCell>();
}
extern "C" void vtkGenericCell_destructor(vtkNew<vtkGenericCell> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkGenericCell_get_ptr(vtkNew<vtkGenericCell> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_generic_cell_get_cell_type(vtkNew<vtkGenericCell> self) {
    return self->GetCellType();
}
extern "C" int vtk_generic_cell_get_cell_dimension(vtkNew<vtkGenericCell> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_generic_cell_get_number_of_edges(vtkNew<vtkGenericCell> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_generic_cell_get_number_of_faces(vtkNew<vtkGenericCell> self) {
    return self->GetNumberOfFaces();
}
extern "C" void vtk_generic_cell_set_cell_type(vtkNew<vtkGenericCell> self, int cellType) {
    return self->SetCellType(cellType);
}
extern "C" void vtk_generic_cell_set_cell_type_to_empty_cell(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToEmptyCell();
}
extern "C" void vtk_generic_cell_set_cell_type_to_vertex(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToVertex();
}
extern "C" void vtk_generic_cell_set_cell_type_to_poly_vertex(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToPolyVertex();
}
extern "C" void vtk_generic_cell_set_cell_type_to_line(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToLine();
}
extern "C" void vtk_generic_cell_set_cell_type_to_poly_line(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToPolyLine();
}
extern "C" void vtk_generic_cell_set_cell_type_to_triangle(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToTriangle();
}
extern "C" void vtk_generic_cell_set_cell_type_to_triangle_strip(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToTriangleStrip();
}
extern "C" void vtk_generic_cell_set_cell_type_to_polygon(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToPolygon();
}
extern "C" void vtk_generic_cell_set_cell_type_to_pixel(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToPixel();
}
extern "C" void vtk_generic_cell_set_cell_type_to_quad(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToQuad();
}
extern "C" void vtk_generic_cell_set_cell_type_to_tetra(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToTetra();
}
extern "C" void vtk_generic_cell_set_cell_type_to_voxel(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToVoxel();
}
extern "C" void vtk_generic_cell_set_cell_type_to_hexahedron(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToHexahedron();
}
extern "C" void vtk_generic_cell_set_cell_type_to_wedge(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToWedge();
}
extern "C" void vtk_generic_cell_set_cell_type_to_pyramid(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToPyramid();
}
extern "C" void vtk_generic_cell_set_cell_type_to_pentagonal_prism(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToPentagonalPrism();
}
extern "C" void vtk_generic_cell_set_cell_type_to_hexagonal_prism(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToHexagonalPrism();
}
extern "C" void vtk_generic_cell_set_cell_type_to_polyhedron(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToPolyhedron();
}
extern "C" void vtk_generic_cell_set_cell_type_to_convex_point_set(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToConvexPointSet();
}
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_edge(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToQuadraticEdge();
}
extern "C" void vtk_generic_cell_set_cell_type_to_cubic_line(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToCubicLine();
}
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_triangle(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToQuadraticTriangle();
}
extern "C" void vtk_generic_cell_set_cell_type_to_bi_quadratic_triangle(
    vtkNew<vtkGenericCell> self
) {
    return self->SetCellTypeToBiQuadraticTriangle();
}
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_quad(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToQuadraticQuad();
}
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_polygon(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToQuadraticPolygon();
}
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_tetra(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToQuadraticTetra();
}
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_hexahedron(
    vtkNew<vtkGenericCell> self
) {
    return self->SetCellTypeToQuadraticHexahedron();
}
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_wedge(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToQuadraticWedge();
}
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_pyramid(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToQuadraticPyramid();
}
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_linear_quad(
    vtkNew<vtkGenericCell> self
) {
    return self->SetCellTypeToQuadraticLinearQuad();
}
extern "C" void vtk_generic_cell_set_cell_type_to_bi_quadratic_quad(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToBiQuadraticQuad();
}
extern "C" void vtk_generic_cell_set_cell_type_to_quadratic_linear_wedge(
    vtkNew<vtkGenericCell> self
) {
    return self->SetCellTypeToQuadraticLinearWedge();
}
extern "C" void vtk_generic_cell_set_cell_type_to_bi_quadratic_quadratic_wedge(
    vtkNew<vtkGenericCell> self
) {
    return self->SetCellTypeToBiQuadraticQuadraticWedge();
}
extern "C" void vtk_generic_cell_set_cell_type_to_tri_quadratic_hexahedron(
    vtkNew<vtkGenericCell> self
) {
    return self->SetCellTypeToTriQuadraticHexahedron();
}
extern "C" void vtk_generic_cell_set_cell_type_to_tri_quadratic_pyramid(
    vtkNew<vtkGenericCell> self
) {
    return self->SetCellTypeToTriQuadraticPyramid();
}
extern "C" void vtk_generic_cell_set_cell_type_to_bi_quadratic_quadratic_hexahedron(
    vtkNew<vtkGenericCell> self
) {
    return self->SetCellTypeToBiQuadraticQuadraticHexahedron();
}
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_triangle(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToLagrangeTriangle();
}
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_tetra(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToLagrangeTetra();
}
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_curve(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToLagrangeCurve();
}
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_quadrilateral(
    vtkNew<vtkGenericCell> self
) {
    return self->SetCellTypeToLagrangeQuadrilateral();
}
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_hexahedron(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToLagrangeHexahedron();
}
extern "C" void vtk_generic_cell_set_cell_type_to_lagrange_wedge(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToLagrangeWedge();
}
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_triangle(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToBezierTriangle();
}
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_tetra(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToBezierTetra();
}
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_curve(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToBezierCurve();
}
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_quadrilateral(
    vtkNew<vtkGenericCell> self
) {
    return self->SetCellTypeToBezierQuadrilateral();
}
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_hexahedron(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToBezierHexahedron();
}
extern "C" void vtk_generic_cell_set_cell_type_to_bezier_wedge(vtkNew<vtkGenericCell> self) {
    return self->SetCellTypeToBezierWedge();
}
extern "C" vtkNew<vtkGenericEdgeTable> vtkGenericEdgeTable_new() {
    return vtkNew<vtkGenericEdgeTable>();
}
extern "C" void vtkGenericEdgeTable_destructor(vtkNew<vtkGenericEdgeTable> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkGenericEdgeTable_get_ptr(vtkNew<vtkGenericEdgeTable> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_generic_edge_table_insert_edge_c_longlong_c_longlong_c_longlong_c_int(
    vtkNew<vtkGenericEdgeTable> self, long long e1, long long e2, long long cellId, int ref
) {
    return self->InsertEdge(e1, e2, cellId, ref);
}
extern "C" int vtk_generic_edge_table_remove_edge(
    vtkNew<vtkGenericEdgeTable> self, long long e1, long long e2
) {
    return self->RemoveEdge(e1, e2);
}
extern "C" int vtk_generic_edge_table_increment_edge_reference_count(
    vtkNew<vtkGenericEdgeTable> self, long long e1, long long e2, long long cellId
) {
    return self->IncrementEdgeReferenceCount(e1, e2, cellId);
}
extern "C" int vtk_generic_edge_table_check_edge_reference_count(
    vtkNew<vtkGenericEdgeTable> self, long long e1, long long e2
) {
    return self->CheckEdgeReferenceCount(e1, e2);
}
extern "C" void vtk_generic_edge_table_initialize(
    vtkNew<vtkGenericEdgeTable> self, long long start
) {
    return self->Initialize(start);
}
extern "C" int vtk_generic_edge_table_get_number_of_components(vtkNew<vtkGenericEdgeTable> self) {
    return self->GetNumberOfComponents();
}
extern "C" void vtk_generic_edge_table_set_number_of_components(
    vtkNew<vtkGenericEdgeTable> self, int count
) {
    return self->SetNumberOfComponents(count);
}
extern "C" int vtk_generic_edge_table_check_point_c_longlong(
    vtkNew<vtkGenericEdgeTable> self, long long ptId
) {
    return self->CheckPoint(ptId);
}
extern "C" void vtk_generic_edge_table_insert_point(
    vtkNew<vtkGenericEdgeTable> self, long long ptId, double* point
) {
    return self->InsertPoint(ptId, point);
}
extern "C" void vtk_generic_edge_table_remove_point(
    vtkNew<vtkGenericEdgeTable> self, long long ptId
) {
    return self->RemovePoint(ptId);
}
extern "C" void vtk_generic_edge_table_increment_point_reference_count(
    vtkNew<vtkGenericEdgeTable> self, long long ptId
) {
    return self->IncrementPointReferenceCount(ptId);
}
extern "C" void vtk_generic_edge_table_dump_table(vtkNew<vtkGenericEdgeTable> self) {
    return self->DumpTable();
}
extern "C" void vtk_generic_edge_table_load_factor(vtkNew<vtkGenericEdgeTable> self) {
    return self->LoadFactor();
}
extern "C" vtkNew<vtkGenericInterpolatedVelocityField> vtkGenericInterpolatedVelocityField_new() {
    return vtkNew<vtkGenericInterpolatedVelocityField>();
}
extern "C" void vtkGenericInterpolatedVelocityField_destructor(
    vtkNew<vtkGenericInterpolatedVelocityField> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkGenericInterpolatedVelocityField_get_ptr(
    vtkNew<vtkGenericInterpolatedVelocityField> sself
) {
    return sself.GetPointer();
}
extern "C" void vtk_generic_interpolated_velocity_field_clear_last_cell(
    vtkNew<vtkGenericInterpolatedVelocityField> self
) {
    return self->ClearLastCell();
}
extern "C" int vtk_generic_interpolated_velocity_field_get_last_local_coordinates(
    vtkNew<vtkGenericInterpolatedVelocityField> self, double* pcoords
) {
    return self->GetLastLocalCoordinates(pcoords);
}
extern "C" int vtk_generic_interpolated_velocity_field_get_caching(
    vtkNew<vtkGenericInterpolatedVelocityField> self
) {
    return self->GetCaching();
}
extern "C" void vtk_generic_interpolated_velocity_field_set_caching(
    vtkNew<vtkGenericInterpolatedVelocityField> self, int _arg
) {
    return self->SetCaching(_arg);
}
extern "C" void vtk_generic_interpolated_velocity_field_caching_on(
    vtkNew<vtkGenericInterpolatedVelocityField> self
) {
    return self->CachingOn();
}
extern "C" void vtk_generic_interpolated_velocity_field_caching_off(
    vtkNew<vtkGenericInterpolatedVelocityField> self
) {
    return self->CachingOff();
}
extern "C" int vtk_generic_interpolated_velocity_field_get_cache_hit(
    vtkNew<vtkGenericInterpolatedVelocityField> self
) {
    return self->GetCacheHit();
}
extern "C" int vtk_generic_interpolated_velocity_field_get_cache_miss(
    vtkNew<vtkGenericInterpolatedVelocityField> self
) {
    return self->GetCacheMiss();
}
extern "C" vtkNew<vtkGeometricErrorMetric> vtkGeometricErrorMetric_new() {
    return vtkNew<vtkGeometricErrorMetric>();
}
extern "C" void vtkGeometricErrorMetric_destructor(vtkNew<vtkGeometricErrorMetric> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkGeometricErrorMetric_get_ptr(vtkNew<vtkGeometricErrorMetric> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_geometric_error_metric_get_absolute_geometric_tolerance(
    vtkNew<vtkGeometricErrorMetric> self
) {
    return self->GetAbsoluteGeometricTolerance();
}
extern "C" void vtk_geometric_error_metric_set_absolute_geometric_tolerance(
    vtkNew<vtkGeometricErrorMetric> self, double value
) {
    return self->SetAbsoluteGeometricTolerance(value);
}
extern "C" int vtk_geometric_error_metric_get_relative(vtkNew<vtkGeometricErrorMetric> self) {
    return self->GetRelative();
}
extern "C" vtkNew<vtkGraphEdge> vtkGraphEdge_new() {
    return vtkNew<vtkGraphEdge>();
}
extern "C" void vtkGraphEdge_destructor(vtkNew<vtkGraphEdge> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkGraphEdge_get_ptr(vtkNew<vtkGraphEdge> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_graph_edge_set_source(vtkNew<vtkGraphEdge> self, long long _arg) {
    return self->SetSource(_arg);
}
extern "C" long long vtk_graph_edge_get_source(vtkNew<vtkGraphEdge> self) {
    return self->GetSource();
}
extern "C" void vtk_graph_edge_set_target(vtkNew<vtkGraphEdge> self, long long _arg) {
    return self->SetTarget(_arg);
}
extern "C" long long vtk_graph_edge_get_target(vtkNew<vtkGraphEdge> self) {
    return self->GetTarget();
}
extern "C" void vtk_graph_edge_set_id(vtkNew<vtkGraphEdge> self, long long _arg) {
    return self->SetId(_arg);
}
extern "C" long long vtk_graph_edge_get_id(vtkNew<vtkGraphEdge> self) {
    return self->GetId();
}
extern "C" vtkNew<vtkGraphInternals> vtkGraphInternals_new() {
    return vtkNew<vtkGraphInternals>();
}
extern "C" void vtkGraphInternals_destructor(vtkNew<vtkGraphInternals> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkGraphInternals_get_ptr(vtkNew<vtkGraphInternals> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkHexagonalPrism> vtkHexagonalPrism_new() {
    return vtkNew<vtkHexagonalPrism>();
}
extern "C" void vtkHexagonalPrism_destructor(vtkNew<vtkHexagonalPrism> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkHexagonalPrism_get_ptr(vtkNew<vtkHexagonalPrism> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_hexagonal_prism_get_centroid(vtkNew<vtkHexagonalPrism> self, double* centroid) {
    return self->GetCentroid(centroid);
}
extern "C" int vtk_hexagonal_prism_get_cell_type(vtkNew<vtkHexagonalPrism> self) {
    return self->GetCellType();
}
extern "C" int vtk_hexagonal_prism_get_number_of_edges(vtkNew<vtkHexagonalPrism> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_hexagonal_prism_get_number_of_faces(vtkNew<vtkHexagonalPrism> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_hexagonal_prism_get_parametric_center(
    vtkNew<vtkHexagonalPrism> self, double* pcoords
) {
    return self->GetParametricCenter(pcoords);
}
extern "C" void vtk_hexagonal_prism_interpolation_functions(
    vtkNew<vtkHexagonalPrism> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_hexagonal_prism_interpolation_derivs(
    vtkNew<vtkHexagonalPrism> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_hexagonal_prism_interpolate_functions(
    vtkNew<vtkHexagonalPrism> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_hexagonal_prism_interpolate_derivs(
    vtkNew<vtkHexagonalPrism> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkHexahedron> vtkHexahedron_new() {
    return vtkNew<vtkHexahedron>();
}
extern "C" void vtkHexahedron_destructor(vtkNew<vtkHexahedron> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkHexahedron_get_ptr(vtkNew<vtkHexahedron> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_hexahedron_get_centroid(vtkNew<vtkHexahedron> self, double* centroid) {
    return self->GetCentroid(centroid);
}
extern "C" int vtk_hexahedron_get_cell_type(vtkNew<vtkHexahedron> self) {
    return self->GetCellType();
}
extern "C" int vtk_hexahedron_get_number_of_edges(vtkNew<vtkHexahedron> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_hexahedron_get_number_of_faces(vtkNew<vtkHexahedron> self) {
    return self->GetNumberOfFaces();
}
extern "C" void vtk_hexahedron_interpolation_functions(
    vtkNew<vtkHexahedron> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_hexahedron_interpolation_derivs(
    vtkNew<vtkHexahedron> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_hexahedron_interpolate_functions(
    vtkNew<vtkHexahedron> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_hexahedron_interpolate_derivs(
    vtkNew<vtkHexahedron> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkHierarchicalBoxDataSet> vtkHierarchicalBoxDataSet_new() {
    return vtkNew<vtkHierarchicalBoxDataSet>();
}
extern "C" void vtkHierarchicalBoxDataSet_destructor(vtkNew<vtkHierarchicalBoxDataSet> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkHierarchicalBoxDataSet_get_ptr(vtkNew<vtkHierarchicalBoxDataSet> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkHyperTree> vtkHyperTree_new() {
    return vtkNew<vtkHyperTree>();
}
extern "C" void vtkHyperTree_destructor(vtkNew<vtkHyperTree> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkHyperTree_get_ptr(vtkNew<vtkHyperTree> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_hyper_tree_initialize_c_uchar_c_uchar_c_uchar(
    vtkNew<vtkHyperTree> self,
    unsigned char branchFactor,
    unsigned char dimension,
    unsigned char numberOfChildren
) {
    return self->Initialize(branchFactor, dimension, numberOfChildren);
}
extern "C" bool vtk_hyper_tree_initialize_c_uchar_c_uchar(
    vtkNew<vtkHyperTree> self, unsigned char branchFactor, unsigned char dimension
) {
    return self->Initialize(branchFactor, dimension);
}
extern "C" void vtk_hyper_tree_set_tree_index(vtkNew<vtkHyperTree> self, long long treeIndex) {
    return self->SetTreeIndex(treeIndex);
}
extern "C" long long vtk_hyper_tree_get_tree_index(vtkNew<vtkHyperTree> self) {
    return self->GetTreeIndex();
}
extern "C" unsigned int vtk_hyper_tree_get_number_of_levels(vtkNew<vtkHyperTree> self) {
    return self->GetNumberOfLevels();
}
extern "C" long long vtk_hyper_tree_get_number_of_vertices(vtkNew<vtkHyperTree> self) {
    return self->GetNumberOfVertices();
}
extern "C" long long vtk_hyper_tree_get_number_of_nodes(vtkNew<vtkHyperTree> self) {
    return self->GetNumberOfNodes();
}
extern "C" long long vtk_hyper_tree_get_number_of_leaves(vtkNew<vtkHyperTree> self) {
    return self->GetNumberOfLeaves();
}
extern "C" int vtk_hyper_tree_get_branch_factor(vtkNew<vtkHyperTree> self) {
    return self->GetBranchFactor();
}
extern "C" int vtk_hyper_tree_get_dimension(vtkNew<vtkHyperTree> self) {
    return self->GetDimension();
}
extern "C" long long vtk_hyper_tree_get_number_of_children(vtkNew<vtkHyperTree> self) {
    return self->GetNumberOfChildren();
}
extern "C" void vtk_hyper_tree_get_scale_double_3(vtkNew<vtkHyperTree> self, double* s) {
    return self->GetScale(s);
}
extern "C" double vtk_hyper_tree_get_scale_c_uint(vtkNew<vtkHyperTree> self, unsigned int d) {
    return self->GetScale(d);
}
extern "C" unsigned long vtk_hyper_tree_get_actual_memory_size_bytes(vtkNew<vtkHyperTree> self) {
    return self->GetActualMemorySizeBytes();
}
extern "C" unsigned int vtk_hyper_tree_get_actual_memory_size(vtkNew<vtkHyperTree> self) {
    return self->GetActualMemorySize();
}
extern "C" bool vtk_hyper_tree_is_global_index_implicit(vtkNew<vtkHyperTree> self) {
    return self->IsGlobalIndexImplicit();
}
extern "C" void vtk_hyper_tree_set_global_index_start(vtkNew<vtkHyperTree> self, long long start) {
    return self->SetGlobalIndexStart(start);
}
extern "C" long long vtk_hyper_tree_get_global_index_start(vtkNew<vtkHyperTree> self) {
    return self->GetGlobalIndexStart();
}
extern "C" void vtk_hyper_tree_set_global_index_from_local(
    vtkNew<vtkHyperTree> self, long long index, long long global
) {
    return self->SetGlobalIndexFromLocal(index, global);
}
extern "C" long long vtk_hyper_tree_get_global_index_from_local(
    vtkNew<vtkHyperTree> self, long long index
) {
    return self->GetGlobalIndexFromLocal(index);
}
extern "C" long long vtk_hyper_tree_get_global_node_index_max(vtkNew<vtkHyperTree> self) {
    return self->GetGlobalNodeIndexMax();
}
extern "C" bool vtk_hyper_tree_is_leaf(vtkNew<vtkHyperTree> self, long long index) {
    return self->IsLeaf(index);
}
extern "C" void vtk_hyper_tree_subdivide_leaf(
    vtkNew<vtkHyperTree> self, long long index, unsigned int level
) {
    return self->SubdivideLeaf(index, level);
}
extern "C" bool vtk_hyper_tree_is_terminal_node(vtkNew<vtkHyperTree> self, long long index) {
    return self->IsTerminalNode(index);
}
extern "C" long long vtk_hyper_tree_get_elder_child_index(
    vtkNew<vtkHyperTree> self, unsigned int index_parent
) {
    return self->GetElderChildIndex(index_parent);
}
extern "C" bool vtk_hyper_tree_has_scales(vtkNew<vtkHyperTree> self) {
    return self->HasScales();
}
extern "C" vtkNew<vtkHyperTreeGrid> vtkHyperTreeGrid_new() {
    return vtkNew<vtkHyperTreeGrid>();
}
extern "C" void vtkHyperTreeGrid_destructor(vtkNew<vtkHyperTreeGrid> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkHyperTreeGrid_get_ptr(vtkNew<vtkHyperTreeGrid> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_hyper_tree_grid_squeeze(vtkNew<vtkHyperTreeGrid> self) {
    return self->Squeeze();
}
extern "C" int vtk_hyper_tree_grid_get_data_object_type(vtkNew<vtkHyperTreeGrid> self) {
    return self->GetDataObjectType();
}
extern "C" void vtk_hyper_tree_grid_set_dimensions_const_c_uint_3(
    vtkNew<vtkHyperTreeGrid> self, const unsigned int* dims
) {
    return self->SetDimensions(dims);
}
extern "C" void vtk_hyper_tree_grid_set_dimensions_const_c_int_3(
    vtkNew<vtkHyperTreeGrid> self, const int* dims
) {
    return self->SetDimensions(dims);
}
extern "C" void vtk_hyper_tree_grid_set_dimensions_c_uint_c_uint_c_uint(
    vtkNew<vtkHyperTreeGrid> self, unsigned int i, unsigned int j, unsigned int k
) {
    return self->SetDimensions(i, j, k);
}
extern "C" void vtk_hyper_tree_grid_set_dimensions_c_int_c_int_c_int(
    vtkNew<vtkHyperTreeGrid> self, int i, int j, int k
) {
    return self->SetDimensions(i, j, k);
}
extern "C" void vtk_hyper_tree_grid_get_dimensions_c_int_3(
    vtkNew<vtkHyperTreeGrid> self, int* dim
) {
    return self->GetDimensions(dim);
}
extern "C" void vtk_hyper_tree_grid_get_dimensions_c_uint_3(
    vtkNew<vtkHyperTreeGrid> self, unsigned int* dim
) {
    return self->GetDimensions(dim);
}
extern "C" void vtk_hyper_tree_grid_set_extent_const_c_int_6(
    vtkNew<vtkHyperTreeGrid> self, const int* extent
) {
    return self->SetExtent(extent);
}
extern "C" void vtk_hyper_tree_grid_set_extent_c_int_c_int_c_int_c_int_c_int_c_int(
    vtkNew<vtkHyperTreeGrid> self, int x1, int x2, int y1, int y2, int z1, int z2
) {
    return self->SetExtent(x1, x2, y1, y2, z1, z2);
}
extern "C" void vtk_hyper_tree_grid_get_extent_c_int_6(vtkNew<vtkHyperTreeGrid> self, int* _arg) {
    return self->GetExtent(_arg);
}
extern "C" void vtk_hyper_tree_grid_get_cell_dims_c_int_3(
    vtkNew<vtkHyperTreeGrid> self, int* cellDims
) {
    return self->GetCellDims(cellDims);
}
extern "C" void vtk_hyper_tree_grid_get_cell_dims_c_uint_3(
    vtkNew<vtkHyperTreeGrid> self, unsigned int* cellDims
) {
    return self->GetCellDims(cellDims);
}
extern "C" unsigned int vtk_hyper_tree_grid_get_dimension(vtkNew<vtkHyperTreeGrid> self) {
    return self->GetDimension();
}
extern "C" unsigned int vtk_hyper_tree_grid_get_number_of_children(vtkNew<vtkHyperTreeGrid> self) {
    return self->GetNumberOfChildren();
}
extern "C" void vtk_hyper_tree_grid_set_transposed_root_indexing(
    vtkNew<vtkHyperTreeGrid> self, bool _arg
) {
    return self->SetTransposedRootIndexing(_arg);
}
extern "C" bool vtk_hyper_tree_grid_get_transposed_root_indexing(vtkNew<vtkHyperTreeGrid> self) {
    return self->GetTransposedRootIndexing();
}
extern "C" void vtk_hyper_tree_grid_set_indexing_mode_to_kji(vtkNew<vtkHyperTreeGrid> self) {
    return self->SetIndexingModeToKJI();
}
extern "C" void vtk_hyper_tree_grid_set_indexing_mode_to_ijk(vtkNew<vtkHyperTreeGrid> self) {
    return self->SetIndexingModeToIJK();
}
extern "C" unsigned int vtk_hyper_tree_grid_get_orientation(vtkNew<vtkHyperTreeGrid> self) {
    return self->GetOrientation();
}
extern "C" bool vtk_hyper_tree_grid_get_freeze_state(vtkNew<vtkHyperTreeGrid> self) {
    return self->GetFreezeState();
}
extern "C" void vtk_hyper_tree_grid_set_branch_factor(
    vtkNew<vtkHyperTreeGrid> self, unsigned int p0
) {
    return self->SetBranchFactor(p0);
}
extern "C" unsigned int vtk_hyper_tree_grid_get_branch_factor(vtkNew<vtkHyperTreeGrid> self) {
    return self->GetBranchFactor();
}
extern "C" long long vtk_hyper_tree_grid_get_max_number_of_trees(vtkNew<vtkHyperTreeGrid> self) {
    return self->GetMaxNumberOfTrees();
}
extern "C" long long vtk_hyper_tree_grid_get_number_of_non_empty_trees(
    vtkNew<vtkHyperTreeGrid> self
) {
    return self->GetNumberOfNonEmptyTrees();
}
extern "C" long long vtk_hyper_tree_grid_get_number_of_leaves(vtkNew<vtkHyperTreeGrid> self) {
    return self->GetNumberOfLeaves();
}
extern "C" unsigned int vtk_hyper_tree_grid_get_number_of_levels_c_longlong(
    vtkNew<vtkHyperTreeGrid> self, long long p0
) {
    return self->GetNumberOfLevels(p0);
}
extern "C" unsigned int vtk_hyper_tree_grid_get_number_of_levels(vtkNew<vtkHyperTreeGrid> self) {
    return self->GetNumberOfLevels();
}
extern "C" void vtk_hyper_tree_grid_set_fixed_coordinates(
    vtkNew<vtkHyperTreeGrid> self, unsigned int axis, double value
) {
    return self->SetFixedCoordinates(axis, value);
}
extern "C" bool vtk_hyper_tree_grid_has_mask(vtkNew<vtkHyperTreeGrid> self) {
    return self->HasMask();
}
extern "C" void vtk_hyper_tree_grid_set_has_interface(vtkNew<vtkHyperTreeGrid> self, bool _arg) {
    return self->SetHasInterface(_arg);
}
extern "C" bool vtk_hyper_tree_grid_get_has_interface(vtkNew<vtkHyperTreeGrid> self) {
    return self->GetHasInterface();
}
extern "C" void vtk_hyper_tree_grid_has_interface_on(vtkNew<vtkHyperTreeGrid> self) {
    return self->HasInterfaceOn();
}
extern "C" void vtk_hyper_tree_grid_has_interface_off(vtkNew<vtkHyperTreeGrid> self) {
    return self->HasInterfaceOff();
}
extern "C" void vtk_hyper_tree_grid_set_depth_limiter(
    vtkNew<vtkHyperTreeGrid> self, unsigned int _arg
) {
    return self->SetDepthLimiter(_arg);
}
extern "C" unsigned int vtk_hyper_tree_grid_get_depth_limiter(vtkNew<vtkHyperTreeGrid> self) {
    return self->GetDepthLimiter();
}
extern "C" void vtk_hyper_tree_grid_generate_ghost_array(
    vtkNew<vtkHyperTreeGrid> self, int* zeroExt
) {
    return self->GenerateGhostArray(zeroExt);
}
extern "C" unsigned int vtk_hyper_tree_grid_find_dichotomic_x(
    vtkNew<vtkHyperTreeGrid> self, double value, double tol
) {
    return self->FindDichotomicX(value, tol);
}
extern "C" unsigned int vtk_hyper_tree_grid_find_dichotomic_y(
    vtkNew<vtkHyperTreeGrid> self, double value, double tol
) {
    return self->FindDichotomicY(value, tol);
}
extern "C" unsigned int vtk_hyper_tree_grid_find_dichotomic_z(
    vtkNew<vtkHyperTreeGrid> self, double value, double tol
) {
    return self->FindDichotomicZ(value, tol);
}
extern "C" void vtk_hyper_tree_grid_initialize(vtkNew<vtkHyperTreeGrid> self) {
    return self->Initialize();
}
extern "C" size_t vtk_hyper_tree_grid_remove_tree(vtkNew<vtkHyperTreeGrid> self, long long index) {
    return self->RemoveTree(index);
}
extern "C" int vtk_hyper_tree_grid_get_extent_type(vtkNew<vtkHyperTreeGrid> self) {
    return self->GetExtentType();
}
extern "C" unsigned long vtk_hyper_tree_grid_get_actual_memory_size_bytes(
    vtkNew<vtkHyperTreeGrid> self
) {
    return self->GetActualMemorySizeBytes();
}
extern "C" unsigned long vtk_hyper_tree_grid_get_actual_memory_size(vtkNew<vtkHyperTreeGrid> self) {
    return self->GetActualMemorySize();
}
extern "C" bool vtk_hyper_tree_grid_supports_ghost_array(vtkNew<vtkHyperTreeGrid> self, int type) {
    return self->SupportsGhostArray(type);
}
extern "C" unsigned int vtk_hyper_tree_grid_get_child_mask(
    vtkNew<vtkHyperTreeGrid> self, unsigned int p0
) {
    return self->GetChildMask(p0);
}
extern "C" long long vtk_hyper_tree_grid_get_shifted_level_zero_index(
    vtkNew<vtkHyperTreeGrid> self, long long p0, int p1, int p2, int p3
) {
    return self->GetShiftedLevelZeroIndex(p0, p1, p2, p3);
}
extern "C" long long vtk_hyper_tree_grid_get_global_node_index_max(vtkNew<vtkHyperTreeGrid> self) {
    return self->GetGlobalNodeIndexMax();
}
extern "C" void vtk_hyper_tree_grid_initialize_local_index_node(vtkNew<vtkHyperTreeGrid> self) {
    return self->InitializeLocalIndexNode();
}
extern "C" bool vtk_hyper_tree_grid_has_any_ghost_cells(vtkNew<vtkHyperTreeGrid> self) {
    return self->HasAnyGhostCells();
}
extern "C" void vtk_hyper_tree_grid_compute_bounds(vtkNew<vtkHyperTreeGrid> self) {
    return self->ComputeBounds();
}
extern "C" void vtk_hyper_tree_grid_get_bounds_double_6(
    vtkNew<vtkHyperTreeGrid> self, double* bounds
) {
    return self->GetBounds(bounds);
}
extern "C" void vtk_hyper_tree_grid_get_grid_bounds(vtkNew<vtkHyperTreeGrid> self, double* bounds) {
    return self->GetGridBounds(bounds);
}
extern "C" void vtk_hyper_tree_grid_get_center_double_3(
    vtkNew<vtkHyperTreeGrid> self, double* center
) {
    return self->GetCenter(center);
}
extern "C" long long vtk_hyper_tree_grid_get_number_of_elements(
    vtkNew<vtkHyperTreeGrid> self, int type
) {
    return self->GetNumberOfElements(type);
}
extern "C" long long vtk_hyper_tree_grid_get_number_of_cells(vtkNew<vtkHyperTreeGrid> self) {
    return self->GetNumberOfCells();
}
extern "C" vtkNew<vtkHyperTreeGridGeometricLocator> vtkHyperTreeGridGeometricLocator_new() {
    return vtkNew<vtkHyperTreeGridGeometricLocator>();
}
extern "C" void vtkHyperTreeGridGeometricLocator_destructor(
    vtkNew<vtkHyperTreeGridGeometricLocator> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkHyperTreeGridGeometricLocator_get_ptr(
    vtkNew<vtkHyperTreeGridGeometricLocator> sself
) {
    return sself.GetPointer();
}
extern "C" long long vtk_hyper_tree_grid_geometric_locator_search_const_double_3(
    vtkNew<vtkHyperTreeGridGeometricLocator> self, const double* point
) {
    return self->Search(point);
}
extern "C" vtkNew<vtkHyperTreeGridNonOrientedCursor> vtkHyperTreeGridNonOrientedCursor_new() {
    return vtkNew<vtkHyperTreeGridNonOrientedCursor>();
}
extern "C" void vtkHyperTreeGridNonOrientedCursor_destructor(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkHyperTreeGridNonOrientedCursor_get_ptr(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> sself
) {
    return sself.GetPointer();
}
extern "C" bool vtk_hyper_tree_grid_non_oriented_cursor_has_tree(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> self
) {
    return self->HasTree();
}
extern "C" long long vtk_hyper_tree_grid_non_oriented_cursor_get_vertex_id(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> self
) {
    return self->GetVertexId();
}
extern "C" long long vtk_hyper_tree_grid_non_oriented_cursor_get_global_node_index(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> self
) {
    return self->GetGlobalNodeIndex();
}
extern "C" unsigned char vtk_hyper_tree_grid_non_oriented_cursor_get_dimension(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> self
) {
    return self->GetDimension();
}
extern "C" unsigned char vtk_hyper_tree_grid_non_oriented_cursor_get_number_of_children(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> self
) {
    return self->GetNumberOfChildren();
}
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_set_global_index_start(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> self, long long index
) {
    return self->SetGlobalIndexStart(index);
}
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_set_global_index_from_local(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> self, long long index
) {
    return self->SetGlobalIndexFromLocal(index);
}
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_set_mask(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> self, bool state
) {
    return self->SetMask(state);
}
extern "C" bool vtk_hyper_tree_grid_non_oriented_cursor_is_masked(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> self
) {
    return self->IsMasked();
}
extern "C" bool vtk_hyper_tree_grid_non_oriented_cursor_is_leaf(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> self
) {
    return self->IsLeaf();
}
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_subdivide_leaf(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> self
) {
    return self->SubdivideLeaf();
}
extern "C" bool vtk_hyper_tree_grid_non_oriented_cursor_is_root(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> self
) {
    return self->IsRoot();
}
extern "C" unsigned int vtk_hyper_tree_grid_non_oriented_cursor_get_level(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> self
) {
    return self->GetLevel();
}
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_to_child(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> self, unsigned char ichild
) {
    return self->ToChild(ichild);
}
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_to_root(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> self
) {
    return self->ToRoot();
}
extern "C" void vtk_hyper_tree_grid_non_oriented_cursor_to_parent(
    vtkNew<vtkHyperTreeGridNonOrientedCursor> self
) {
    return self->ToParent();
}
extern "C" vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> vtkHyperTreeGridNonOrientedGeometryCursor_new() {
    return vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor>();
}
extern "C" void vtkHyperTreeGridNonOrientedGeometryCursor_destructor(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkHyperTreeGridNonOrientedGeometryCursor_get_ptr(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> sself
) {
    return sself.GetPointer();
}
extern "C" bool vtk_hyper_tree_grid_non_oriented_geometry_cursor_has_tree(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self
) {
    return self->HasTree();
}
extern "C" long long vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_vertex_id(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self
) {
    return self->GetVertexId();
}
extern "C" long long vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_global_node_index(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self
) {
    return self->GetGlobalNodeIndex();
}
extern "C" unsigned char vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_dimension(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self
) {
    return self->GetDimension();
}
extern "C" unsigned char vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_number_of_children(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self
) {
    return self->GetNumberOfChildren();
}
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_set_global_index_start(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self, long long index
) {
    return self->SetGlobalIndexStart(index);
}
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_set_global_index_from_local(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self, long long index
) {
    return self->SetGlobalIndexFromLocal(index);
}
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_bounds(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self, double* bounds
) {
    return self->GetBounds(bounds);
}
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_point(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self, double* point
) {
    return self->GetPoint(point);
}
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_set_mask(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self, bool state
) {
    return self->SetMask(state);
}
extern "C" bool vtk_hyper_tree_grid_non_oriented_geometry_cursor_is_masked(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self
) {
    return self->IsMasked();
}
extern "C" bool vtk_hyper_tree_grid_non_oriented_geometry_cursor_is_leaf(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self
) {
    return self->IsLeaf();
}
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_subdivide_leaf(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self
) {
    return self->SubdivideLeaf();
}
extern "C" bool vtk_hyper_tree_grid_non_oriented_geometry_cursor_is_root(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self
) {
    return self->IsRoot();
}
extern "C" unsigned int vtk_hyper_tree_grid_non_oriented_geometry_cursor_get_level(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self
) {
    return self->GetLevel();
}
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_to_child(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self, unsigned char ichild
) {
    return self->ToChild(ichild);
}
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_to_root(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self
) {
    return self->ToRoot();
}
extern "C" void vtk_hyper_tree_grid_non_oriented_geometry_cursor_to_parent(
    vtkNew<vtkHyperTreeGridNonOrientedGeometryCursor> self
) {
    return self->ToParent();
}
extern "C" vtkNew<vtkHyperTreeGridNonOrientedMooreSuperCursor> vtkHyperTreeGridNonOrientedMooreSuperCursor_new() {
    return vtkNew<vtkHyperTreeGridNonOrientedMooreSuperCursor>();
}
extern "C" void vtkHyperTreeGridNonOrientedMooreSuperCursor_destructor(
    vtkNew<vtkHyperTreeGridNonOrientedMooreSuperCursor> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkHyperTreeGridNonOrientedMooreSuperCursor_get_ptr(
    vtkNew<vtkHyperTreeGridNonOrientedMooreSuperCursor> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkHyperTreeGridNonOrientedMooreSuperCursorLight> vtkHyperTreeGridNonOrientedMooreSuperCursorLight_new() {
    return vtkNew<vtkHyperTreeGridNonOrientedMooreSuperCursorLight>();
}
extern "C" void vtkHyperTreeGridNonOrientedMooreSuperCursorLight_destructor(
    vtkNew<vtkHyperTreeGridNonOrientedMooreSuperCursorLight> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkHyperTreeGridNonOrientedMooreSuperCursorLight_get_ptr(
    vtkNew<vtkHyperTreeGridNonOrientedMooreSuperCursorLight> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor>
vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor_new() {
    return vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor>();
}
extern "C" void vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor_destructor(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor_get_ptr(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> sself
) {
    return sself.GetPointer();
}
extern "C" bool vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_has_tree(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self
) {
    return self->HasTree();
}
extern "C" long long vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_get_vertex_id(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self
) {
    return self->GetVertexId();
}
extern "C" long long vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_get_global_node_index(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self
) {
    return self->GetGlobalNodeIndex();
}
extern "C" unsigned char vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_get_dimension(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self
) {
    return self->GetDimension();
}
extern "C" unsigned char vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_get_number_of_children(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self
) {
    return self->GetNumberOfChildren();
}
extern "C" void vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_set_global_index_start(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self, long long index
) {
    return self->SetGlobalIndexStart(index);
}
extern "C" void vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_set_global_index_from_local(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self, long long index
) {
    return self->SetGlobalIndexFromLocal(index);
}
extern "C" void vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_get_bounds(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self, double* bounds
) {
    return self->GetBounds(bounds);
}
extern "C" void vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_get_point(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self, double* point
) {
    return self->GetPoint(point);
}
extern "C" void vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_set_mask(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self, bool state
) {
    return self->SetMask(state);
}
extern "C" bool vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_is_masked(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self
) {
    return self->IsMasked();
}
extern "C" bool vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_is_leaf(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self
) {
    return self->IsLeaf();
}
extern "C" bool vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_is_real_leaf(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self
) {
    return self->IsRealLeaf();
}
extern "C" bool vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_is_virtual_leaf(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self
) {
    return self->IsVirtualLeaf();
}
extern "C" bool vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_is_root(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self
) {
    return self->IsRoot();
}
extern "C" unsigned int vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_get_level(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self
) {
    return self->GetLevel();
}
extern "C" unsigned int vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_get_last_real_level(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self
) {
    return self->GetLastRealLevel();
}
extern "C" void vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_to_child(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self, unsigned char ichild
) {
    return self->ToChild(ichild);
}
extern "C" void vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_to_root(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self
) {
    return self->ToRoot();
}
extern "C" void vtk_hyper_tree_grid_non_oriented_unlimited_geometry_cursor_to_parent(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor> self
) {
    return self->ToParent();
}
extern "C" vtkNew<vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor>
vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor_new() {
    return vtkNew<vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor>();
}
extern "C" void vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor_destructor(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor_get_ptr(
    vtkNew<vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkHyperTreeGridNonOrientedVonNeumannSuperCursor> vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_new() {
    return vtkNew<vtkHyperTreeGridNonOrientedVonNeumannSuperCursor>();
}
extern "C" void vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_destructor(
    vtkNew<vtkHyperTreeGridNonOrientedVonNeumannSuperCursor> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_get_ptr(
    vtkNew<vtkHyperTreeGridNonOrientedVonNeumannSuperCursor> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight>
vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_new() {
    return vtkNew<vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight>();
}
extern "C" void vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_destructor(
    vtkNew<vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_get_ptr(
    vtkNew<vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkHyperTreeGridOrientedCursor> vtkHyperTreeGridOrientedCursor_new() {
    return vtkNew<vtkHyperTreeGridOrientedCursor>();
}
extern "C" void vtkHyperTreeGridOrientedCursor_destructor(
    vtkNew<vtkHyperTreeGridOrientedCursor> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkHyperTreeGridOrientedCursor_get_ptr(
    vtkNew<vtkHyperTreeGridOrientedCursor> sself
) {
    return sself.GetPointer();
}
extern "C" bool vtk_hyper_tree_grid_oriented_cursor_has_tree(
    vtkNew<vtkHyperTreeGridOrientedCursor> self
) {
    return self->HasTree();
}
extern "C" long long vtk_hyper_tree_grid_oriented_cursor_get_vertex_id(
    vtkNew<vtkHyperTreeGridOrientedCursor> self
) {
    return self->GetVertexId();
}
extern "C" long long vtk_hyper_tree_grid_oriented_cursor_get_global_node_index(
    vtkNew<vtkHyperTreeGridOrientedCursor> self
) {
    return self->GetGlobalNodeIndex();
}
extern "C" unsigned char vtk_hyper_tree_grid_oriented_cursor_get_dimension(
    vtkNew<vtkHyperTreeGridOrientedCursor> self
) {
    return self->GetDimension();
}
extern "C" unsigned char vtk_hyper_tree_grid_oriented_cursor_get_number_of_children(
    vtkNew<vtkHyperTreeGridOrientedCursor> self
) {
    return self->GetNumberOfChildren();
}
extern "C" void vtk_hyper_tree_grid_oriented_cursor_set_global_index_start(
    vtkNew<vtkHyperTreeGridOrientedCursor> self, long long index
) {
    return self->SetGlobalIndexStart(index);
}
extern "C" void vtk_hyper_tree_grid_oriented_cursor_set_global_index_from_local(
    vtkNew<vtkHyperTreeGridOrientedCursor> self, long long index
) {
    return self->SetGlobalIndexFromLocal(index);
}
extern "C" void vtk_hyper_tree_grid_oriented_cursor_set_mask(
    vtkNew<vtkHyperTreeGridOrientedCursor> self, bool state
) {
    return self->SetMask(state);
}
extern "C" bool vtk_hyper_tree_grid_oriented_cursor_is_masked(
    vtkNew<vtkHyperTreeGridOrientedCursor> self
) {
    return self->IsMasked();
}
extern "C" bool vtk_hyper_tree_grid_oriented_cursor_is_leaf(
    vtkNew<vtkHyperTreeGridOrientedCursor> self
) {
    return self->IsLeaf();
}
extern "C" void vtk_hyper_tree_grid_oriented_cursor_subdivide_leaf(
    vtkNew<vtkHyperTreeGridOrientedCursor> self
) {
    return self->SubdivideLeaf();
}
extern "C" bool vtk_hyper_tree_grid_oriented_cursor_is_root(
    vtkNew<vtkHyperTreeGridOrientedCursor> self
) {
    return self->IsRoot();
}
extern "C" unsigned int vtk_hyper_tree_grid_oriented_cursor_get_level(
    vtkNew<vtkHyperTreeGridOrientedCursor> self
) {
    return self->GetLevel();
}
extern "C" void vtk_hyper_tree_grid_oriented_cursor_to_child(
    vtkNew<vtkHyperTreeGridOrientedCursor> self, unsigned char ichild
) {
    return self->ToChild(ichild);
}
extern "C" vtkNew<vtkHyperTreeGridOrientedGeometryCursor> vtkHyperTreeGridOrientedGeometryCursor_new() {
    return vtkNew<vtkHyperTreeGridOrientedGeometryCursor>();
}
extern "C" void vtkHyperTreeGridOrientedGeometryCursor_destructor(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkHyperTreeGridOrientedGeometryCursor_get_ptr(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> sself
) {
    return sself.GetPointer();
}
extern "C" bool vtk_hyper_tree_grid_oriented_geometry_cursor_has_tree(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> self
) {
    return self->HasTree();
}
extern "C" long long vtk_hyper_tree_grid_oriented_geometry_cursor_get_vertex_id(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> self
) {
    return self->GetVertexId();
}
extern "C" long long vtk_hyper_tree_grid_oriented_geometry_cursor_get_global_node_index(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> self
) {
    return self->GetGlobalNodeIndex();
}
extern "C" unsigned char vtk_hyper_tree_grid_oriented_geometry_cursor_get_dimension(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> self
) {
    return self->GetDimension();
}
extern "C" unsigned char vtk_hyper_tree_grid_oriented_geometry_cursor_get_number_of_children(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> self
) {
    return self->GetNumberOfChildren();
}
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_set_global_index_start(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> self, long long index
) {
    return self->SetGlobalIndexStart(index);
}
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_set_global_index_from_local(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> self, long long index
) {
    return self->SetGlobalIndexFromLocal(index);
}
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_get_bounds(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> self, double* bounds
) {
    return self->GetBounds(bounds);
}
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_get_point(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> self, double* point
) {
    return self->GetPoint(point);
}
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_set_mask(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> self, bool state
) {
    return self->SetMask(state);
}
extern "C" bool vtk_hyper_tree_grid_oriented_geometry_cursor_is_masked(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> self
) {
    return self->IsMasked();
}
extern "C" bool vtk_hyper_tree_grid_oriented_geometry_cursor_is_leaf(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> self
) {
    return self->IsLeaf();
}
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_subdivide_leaf(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> self
) {
    return self->SubdivideLeaf();
}
extern "C" bool vtk_hyper_tree_grid_oriented_geometry_cursor_is_root(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> self
) {
    return self->IsRoot();
}
extern "C" unsigned int vtk_hyper_tree_grid_oriented_geometry_cursor_get_level(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> self
) {
    return self->GetLevel();
}
extern "C" void vtk_hyper_tree_grid_oriented_geometry_cursor_to_child(
    vtkNew<vtkHyperTreeGridOrientedGeometryCursor> self, unsigned char ichild
) {
    return self->ToChild(ichild);
}
extern "C" vtkNew<vtkImageData> vtkImageData_new() {
    return vtkNew<vtkImageData>();
}
extern "C" void vtkImageData_destructor(vtkNew<vtkImageData> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkImageData_get_ptr(vtkNew<vtkImageData> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_image_data_find_point(vtkNew<vtkImageData> self, double* x) {
    return self->FindPoint(x);
}
extern "C" int vtk_image_data_compute_structured_coordinates_const_double_3_c_int_3_double_3(
    vtkNew<vtkImageData> self, const double* x, int* ijk, double* pcoords
) {
    return self->ComputeStructuredCoordinates(x, ijk, pcoords);
}
extern "C" int vtk_image_data_compute_structured_coordinates_const_double_3_c_int_3_double_3_double(
    vtkNew<vtkImageData> self, const double* x, int* ijk, double* pcoords, double tol2
) {
    return self->ComputeStructuredCoordinates(x, ijk, pcoords, tol2);
}
extern "C" long long vtk_image_data_compute_point_id(vtkNew<vtkImageData> self, int* ijk) {
    return self->ComputePointId(ijk);
}
extern "C" long long vtk_image_data_compute_cell_id(vtkNew<vtkImageData> self, int* ijk) {
    return self->ComputeCellId(ijk);
}
extern "C" double vtk_image_data_get_scalar_type_min(vtkNew<vtkImageData> self) {
    return self->GetScalarTypeMin();
}
extern "C" double vtk_image_data_get_scalar_type_max(vtkNew<vtkImageData> self) {
    return self->GetScalarTypeMax();
}
extern "C" int vtk_image_data_get_scalar_size(vtkNew<vtkImageData> self) {
    return self->GetScalarSize();
}
extern "C" void vtk_image_data_get_increments_c_longlong_3(
    vtkNew<vtkImageData> self, long long* inc
) {
    return self->GetIncrements(inc);
}
extern "C" long long vtk_image_data_get_scalar_index_for_extent(
    vtkNew<vtkImageData> self, int* extent
) {
    return self->GetScalarIndexForExtent(extent);
}
extern "C" long long vtk_image_data_get_scalar_index_c_int_3(
    vtkNew<vtkImageData> self, int* coordinates
) {
    return self->GetScalarIndex(coordinates);
}
extern "C" long long vtk_image_data_get_scalar_index_c_int_c_int_c_int(
    vtkNew<vtkImageData> self, int x, int y, int z
) {
    return self->GetScalarIndex(x, y, z);
}
extern "C" float vtk_image_data_get_scalar_component_as_float(
    vtkNew<vtkImageData> self, int x, int y, int z, int component
) {
    return self->GetScalarComponentAsFloat(x, y, z, component);
}
extern "C" void vtk_image_data_set_scalar_component_from_float(
    vtkNew<vtkImageData> self, int x, int y, int z, int component, float v
) {
    return self->SetScalarComponentFromFloat(x, y, z, component, v);
}
extern "C" double vtk_image_data_get_scalar_component_as_double(
    vtkNew<vtkImageData> self, int x, int y, int z, int component
) {
    return self->GetScalarComponentAsDouble(x, y, z, component);
}
extern "C" void vtk_image_data_set_scalar_component_from_double(
    vtkNew<vtkImageData> self, int x, int y, int z, int component, double v
) {
    return self->SetScalarComponentFromDouble(x, y, z, component, v);
}
extern "C" void vtk_image_data_allocate_scalars_c_int_c_int(
    vtkNew<vtkImageData> self, int dataType, int numComponents
) {
    return self->AllocateScalars(dataType, numComponents);
}
extern "C" void vtk_image_data_get_spacing_double_3(vtkNew<vtkImageData> self, double* _arg) {
    return self->GetSpacing(_arg);
}
extern "C" void vtk_image_data_set_spacing_double_double_double(
    vtkNew<vtkImageData> self, double i, double j, double k
) {
    return self->SetSpacing(i, j, k);
}
extern "C" void vtk_image_data_set_spacing_const_double_3(
    vtkNew<vtkImageData> self, const double* ijk
) {
    return self->SetSpacing(ijk);
}
extern "C" void vtk_image_data_get_origin_double_3(vtkNew<vtkImageData> self, double* _arg) {
    return self->GetOrigin(_arg);
}
extern "C" void vtk_image_data_set_origin_double_double_double(
    vtkNew<vtkImageData> self, double i, double j, double k
) {
    return self->SetOrigin(i, j, k);
}
extern "C" void vtk_image_data_set_origin_const_double_3(
    vtkNew<vtkImageData> self, const double* ijk
) {
    return self->SetOrigin(ijk);
}
extern "C" void vtk_image_data_set_direction_matrix_const_double_9(
    vtkNew<vtkImageData> self, const double* elements
) {
    return self->SetDirectionMatrix(elements);
}
extern "C" void vtk_image_data_set_direction_matrix_double_double_double_double_double_double_double_double_double(
    vtkNew<vtkImageData> self,
    double e00,
    double e01,
    double e02,
    double e10,
    double e11,
    double e12,
    double e20,
    double e21,
    double e22
) {
    return self->SetDirectionMatrix(e00, e01, e02, e10, e11, e12, e20, e21, e22);
}
extern "C" void vtk_image_data_transform_continuous_index_to_physical_point_double_double_double_double_3(
    vtkNew<vtkImageData> self, double i, double j, double k, double* xyz
) {
    return self->TransformContinuousIndexToPhysicalPoint(i, j, k, xyz);
}
extern "C" void vtk_image_data_transform_continuous_index_to_physical_point_const_double_3_double_3(
    vtkNew<vtkImageData> self, const double* ijk, double* xyz
) {
    return self->TransformContinuousIndexToPhysicalPoint(ijk, xyz);
}
extern "C" void vtk_image_data_transform_index_to_physical_point_c_int_c_int_c_int_double_3(
    vtkNew<vtkImageData> self, int i, int j, int k, double* xyz
) {
    return self->TransformIndexToPhysicalPoint(i, j, k, xyz);
}
extern "C" void vtk_image_data_transform_index_to_physical_point_const_c_int_3_double_3(
    vtkNew<vtkImageData> self, const int* ijk, double* xyz
) {
    return self->TransformIndexToPhysicalPoint(ijk, xyz);
}
extern "C" void vtk_image_data_transform_continuous_index_to_physical_point_double_double_double_const_double_3_const_double_3_const_double_9_double_3(
    vtkNew<vtkImageData> self,
    double i,
    double j,
    double k,
    const double* origin,
    const double* spacing,
    const double* direction,
    double* xyz
) {
    return self->TransformContinuousIndexToPhysicalPoint(i, j, k, origin, spacing, direction, xyz);
}
extern "C" void vtk_image_data_transform_physical_point_to_continuous_index_double_double_double_double_3(
    vtkNew<vtkImageData> self, double x, double y, double z, double* ijk
) {
    return self->TransformPhysicalPointToContinuousIndex(x, y, z, ijk);
}
extern "C" void vtk_image_data_transform_physical_point_to_continuous_index_const_double_3_double_3(
    vtkNew<vtkImageData> self, const double* xyz, double* ijk
) {
    return self->TransformPhysicalPointToContinuousIndex(xyz, ijk);
}
extern "C" void vtk_image_data_transform_physical_normal_to_continuous_index(
    vtkNew<vtkImageData> self, const double* xyz, double* ijk
) {
    return self->TransformPhysicalNormalToContinuousIndex(xyz, ijk);
}
extern "C" void vtk_image_data_transform_physical_plane_to_continuous_index(
    vtkNew<vtkImageData> self, const double* pplane, double* iplane
) {
    return self->TransformPhysicalPlaneToContinuousIndex(pplane, iplane);
}
extern "C" void vtk_image_data_compute_index_to_physical_matrix(
    vtkNew<vtkImageData> self,
    const double* origin,
    const double* spacing,
    const double* direction,
    double* result
) {
    return self->ComputeIndexToPhysicalMatrix(origin, spacing, direction, result);
}
extern "C" void vtk_image_data_compute_physical_to_index_matrix(
    vtkNew<vtkImageData> self,
    const double* origin,
    const double* spacing,
    const double* direction,
    double* result
) {
    return self->ComputePhysicalToIndexMatrix(origin, spacing, direction, result);
}
extern "C" void vtk_image_data_prepare_for_new_data(vtkNew<vtkImageData> self) {
    return self->PrepareForNewData();
}
extern "C" vtkNew<vtkImageTransform> vtkImageTransform_new() {
    return vtkNew<vtkImageTransform>();
}
extern "C" void vtkImageTransform_destructor(vtkNew<vtkImageTransform> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkImageTransform_get_ptr(vtkNew<vtkImageTransform> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkImplicitBoolean> vtkImplicitBoolean_new() {
    return vtkNew<vtkImplicitBoolean>();
}
extern "C" void vtkImplicitBoolean_destructor(vtkNew<vtkImplicitBoolean> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkImplicitBoolean_get_ptr(vtkNew<vtkImplicitBoolean> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_implicit_boolean_evaluate_function(
    vtkNew<vtkImplicitBoolean> self, double* x
) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_implicit_boolean_evaluate_gradient(
    vtkNew<vtkImplicitBoolean> self, double* x, double* g
) {
    return self->EvaluateGradient(x, g);
}
extern "C" unsigned long vtk_implicit_boolean_get_m_time(vtkNew<vtkImplicitBoolean> self) {
    return self->GetMTime();
}
extern "C" void vtk_implicit_boolean_set_operation_type(vtkNew<vtkImplicitBoolean> self, int _arg) {
    return self->SetOperationType(_arg);
}
extern "C" int vtk_implicit_boolean_get_operation_type_min_value(vtkNew<vtkImplicitBoolean> self) {
    return self->GetOperationTypeMinValue();
}
extern "C" int vtk_implicit_boolean_get_operation_type_max_value(vtkNew<vtkImplicitBoolean> self) {
    return self->GetOperationTypeMaxValue();
}
extern "C" int vtk_implicit_boolean_get_operation_type(vtkNew<vtkImplicitBoolean> self) {
    return self->GetOperationType();
}
extern "C" void vtk_implicit_boolean_set_operation_type_to_union(vtkNew<vtkImplicitBoolean> self) {
    return self->SetOperationTypeToUnion();
}
extern "C" void vtk_implicit_boolean_set_operation_type_to_intersection(
    vtkNew<vtkImplicitBoolean> self
) {
    return self->SetOperationTypeToIntersection();
}
extern "C" void vtk_implicit_boolean_set_operation_type_to_difference(
    vtkNew<vtkImplicitBoolean> self
) {
    return self->SetOperationTypeToDifference();
}
extern "C" void vtk_implicit_boolean_set_operation_type_to_union_of_magnitudes(
    vtkNew<vtkImplicitBoolean> self
) {
    return self->SetOperationTypeToUnionOfMagnitudes();
}
extern "C" vtkNew<vtkImplicitDataSet> vtkImplicitDataSet_new() {
    return vtkNew<vtkImplicitDataSet>();
}
extern "C" void vtkImplicitDataSet_destructor(vtkNew<vtkImplicitDataSet> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkImplicitDataSet_get_ptr(vtkNew<vtkImplicitDataSet> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long vtk_implicit_data_set_get_m_time(vtkNew<vtkImplicitDataSet> self) {
    return self->GetMTime();
}
extern "C" double vtk_implicit_data_set_evaluate_function(
    vtkNew<vtkImplicitDataSet> self, double* x
) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_implicit_data_set_evaluate_gradient(
    vtkNew<vtkImplicitDataSet> self, double* x, double* n
) {
    return self->EvaluateGradient(x, n);
}
extern "C" void vtk_implicit_data_set_set_out_value(vtkNew<vtkImplicitDataSet> self, double _arg) {
    return self->SetOutValue(_arg);
}
extern "C" double vtk_implicit_data_set_get_out_value(vtkNew<vtkImplicitDataSet> self) {
    return self->GetOutValue();
}
extern "C" void vtk_implicit_data_set_set_out_gradient_double_double_double(
    vtkNew<vtkImplicitDataSet> self, double _arg1, double _arg2, double _arg3
) {
    return self->SetOutGradient(_arg1, _arg2, _arg3);
}
extern "C" void vtk_implicit_data_set_set_out_gradient_const_double_3(
    vtkNew<vtkImplicitDataSet> self, const double* _arg
) {
    return self->SetOutGradient(_arg);
}
extern "C" void vtk_implicit_data_set_get_out_gradient_double_3(
    vtkNew<vtkImplicitDataSet> self, double* _arg
) {
    return self->GetOutGradient(_arg);
}
extern "C" vtkNew<vtkImplicitFunctionCollection> vtkImplicitFunctionCollection_new() {
    return vtkNew<vtkImplicitFunctionCollection>();
}
extern "C" void vtkImplicitFunctionCollection_destructor(
    vtkNew<vtkImplicitFunctionCollection> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkImplicitFunctionCollection_get_ptr(
    vtkNew<vtkImplicitFunctionCollection> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkImplicitHalo> vtkImplicitHalo_new() {
    return vtkNew<vtkImplicitHalo>();
}
extern "C" void vtkImplicitHalo_destructor(vtkNew<vtkImplicitHalo> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkImplicitHalo_get_ptr(vtkNew<vtkImplicitHalo> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_implicit_halo_evaluate_function(vtkNew<vtkImplicitHalo> self, double* x) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_implicit_halo_evaluate_gradient(
    vtkNew<vtkImplicitHalo> self, double* x, double* g
) {
    return self->EvaluateGradient(x, g);
}
extern "C" void vtk_implicit_halo_set_radius(vtkNew<vtkImplicitHalo> self, double _arg) {
    return self->SetRadius(_arg);
}
extern "C" double vtk_implicit_halo_get_radius(vtkNew<vtkImplicitHalo> self) {
    return self->GetRadius();
}
extern "C" void vtk_implicit_halo_set_center_double_double_double(
    vtkNew<vtkImplicitHalo> self, double _arg1, double _arg2, double _arg3
) {
    return self->SetCenter(_arg1, _arg2, _arg3);
}
extern "C" void vtk_implicit_halo_set_center_const_double_3(
    vtkNew<vtkImplicitHalo> self, const double* _arg
) {
    return self->SetCenter(_arg);
}
extern "C" void vtk_implicit_halo_get_center_double_3(vtkNew<vtkImplicitHalo> self, double* _arg) {
    return self->GetCenter(_arg);
}
extern "C" void vtk_implicit_halo_set_fade_out(vtkNew<vtkImplicitHalo> self, double _arg) {
    return self->SetFadeOut(_arg);
}
extern "C" double vtk_implicit_halo_get_fade_out(vtkNew<vtkImplicitHalo> self) {
    return self->GetFadeOut();
}
extern "C" vtkNew<vtkImplicitSelectionLoop> vtkImplicitSelectionLoop_new() {
    return vtkNew<vtkImplicitSelectionLoop>();
}
extern "C" void vtkImplicitSelectionLoop_destructor(vtkNew<vtkImplicitSelectionLoop> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkImplicitSelectionLoop_get_ptr(vtkNew<vtkImplicitSelectionLoop> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_implicit_selection_loop_evaluate_function(
    vtkNew<vtkImplicitSelectionLoop> self, double* x
) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_implicit_selection_loop_evaluate_gradient(
    vtkNew<vtkImplicitSelectionLoop> self, double* x, double* n
) {
    return self->EvaluateGradient(x, n);
}
extern "C" void vtk_implicit_selection_loop_set_automatic_normal_generation(
    vtkNew<vtkImplicitSelectionLoop> self, int _arg
) {
    return self->SetAutomaticNormalGeneration(_arg);
}
extern "C" int vtk_implicit_selection_loop_get_automatic_normal_generation(
    vtkNew<vtkImplicitSelectionLoop> self
) {
    return self->GetAutomaticNormalGeneration();
}
extern "C" void vtk_implicit_selection_loop_automatic_normal_generation_on(
    vtkNew<vtkImplicitSelectionLoop> self
) {
    return self->AutomaticNormalGenerationOn();
}
extern "C" void vtk_implicit_selection_loop_automatic_normal_generation_off(
    vtkNew<vtkImplicitSelectionLoop> self
) {
    return self->AutomaticNormalGenerationOff();
}
extern "C" void vtk_implicit_selection_loop_set_normal_double_double_double(
    vtkNew<vtkImplicitSelectionLoop> self, double _arg1, double _arg2, double _arg3
) {
    return self->SetNormal(_arg1, _arg2, _arg3);
}
extern "C" void vtk_implicit_selection_loop_set_normal_const_double_3(
    vtkNew<vtkImplicitSelectionLoop> self, const double* _arg
) {
    return self->SetNormal(_arg);
}
extern "C" void vtk_implicit_selection_loop_get_normal_double_3(
    vtkNew<vtkImplicitSelectionLoop> self, double* data
) {
    return self->GetNormal(data);
}
extern "C" unsigned long vtk_implicit_selection_loop_get_m_time(
    vtkNew<vtkImplicitSelectionLoop> self
) {
    return self->GetMTime();
}
extern "C" vtkNew<vtkImplicitSum> vtkImplicitSum_new() {
    return vtkNew<vtkImplicitSum>();
}
extern "C" void vtkImplicitSum_destructor(vtkNew<vtkImplicitSum> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkImplicitSum_get_ptr(vtkNew<vtkImplicitSum> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_implicit_sum_evaluate_function(vtkNew<vtkImplicitSum> self, double* x) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_implicit_sum_evaluate_gradient(
    vtkNew<vtkImplicitSum> self, double* x, double* g
) {
    return self->EvaluateGradient(x, g);
}
extern "C" unsigned long vtk_implicit_sum_get_m_time(vtkNew<vtkImplicitSum> self) {
    return self->GetMTime();
}
extern "C" void vtk_implicit_sum_remove_all_functions(vtkNew<vtkImplicitSum> self) {
    return self->RemoveAllFunctions();
}
extern "C" void vtk_implicit_sum_set_normalize_by_weight(vtkNew<vtkImplicitSum> self, int _arg) {
    return self->SetNormalizeByWeight(_arg);
}
extern "C" int vtk_implicit_sum_get_normalize_by_weight(vtkNew<vtkImplicitSum> self) {
    return self->GetNormalizeByWeight();
}
extern "C" void vtk_implicit_sum_normalize_by_weight_on(vtkNew<vtkImplicitSum> self) {
    return self->NormalizeByWeightOn();
}
extern "C" void vtk_implicit_sum_normalize_by_weight_off(vtkNew<vtkImplicitSum> self) {
    return self->NormalizeByWeightOff();
}
extern "C" vtkNew<vtkImplicitVolume> vtkImplicitVolume_new() {
    return vtkNew<vtkImplicitVolume>();
}
extern "C" void vtkImplicitVolume_destructor(vtkNew<vtkImplicitVolume> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkImplicitVolume_get_ptr(vtkNew<vtkImplicitVolume> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long vtk_implicit_volume_get_m_time(vtkNew<vtkImplicitVolume> self) {
    return self->GetMTime();
}
extern "C" double vtk_implicit_volume_evaluate_function(vtkNew<vtkImplicitVolume> self, double* x) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_implicit_volume_evaluate_gradient(
    vtkNew<vtkImplicitVolume> self, double* x, double* n
) {
    return self->EvaluateGradient(x, n);
}
extern "C" void vtk_implicit_volume_set_out_value(vtkNew<vtkImplicitVolume> self, double _arg) {
    return self->SetOutValue(_arg);
}
extern "C" double vtk_implicit_volume_get_out_value(vtkNew<vtkImplicitVolume> self) {
    return self->GetOutValue();
}
extern "C" void vtk_implicit_volume_set_out_gradient_double_double_double(
    vtkNew<vtkImplicitVolume> self, double _arg1, double _arg2, double _arg3
) {
    return self->SetOutGradient(_arg1, _arg2, _arg3);
}
extern "C" void vtk_implicit_volume_set_out_gradient_const_double_3(
    vtkNew<vtkImplicitVolume> self, const double* _arg
) {
    return self->SetOutGradient(_arg);
}
extern "C" void vtk_implicit_volume_get_out_gradient_double_3(
    vtkNew<vtkImplicitVolume> self, double* _arg
) {
    return self->GetOutGradient(_arg);
}
extern "C" vtkNew<vtkImplicitWindowFunction> vtkImplicitWindowFunction_new() {
    return vtkNew<vtkImplicitWindowFunction>();
}
extern "C" void vtkImplicitWindowFunction_destructor(vtkNew<vtkImplicitWindowFunction> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkImplicitWindowFunction_get_ptr(vtkNew<vtkImplicitWindowFunction> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_implicit_window_function_evaluate_function(
    vtkNew<vtkImplicitWindowFunction> self, double* x
) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_implicit_window_function_evaluate_gradient(
    vtkNew<vtkImplicitWindowFunction> self, double* x, double* n
) {
    return self->EvaluateGradient(x, n);
}
extern "C" void vtk_implicit_window_function_set_window_range_double_double(
    vtkNew<vtkImplicitWindowFunction> self, double _arg1, double _arg2
) {
    return self->SetWindowRange(_arg1, _arg2);
}
extern "C" void vtk_implicit_window_function_set_window_range_const_double_2(
    vtkNew<vtkImplicitWindowFunction> self, const double* _arg
) {
    return self->SetWindowRange(_arg);
}
extern "C" void vtk_implicit_window_function_get_window_range_double_2(
    vtkNew<vtkImplicitWindowFunction> self, double* data
) {
    return self->GetWindowRange(data);
}
extern "C" void vtk_implicit_window_function_set_window_values_double_double(
    vtkNew<vtkImplicitWindowFunction> self, double _arg1, double _arg2
) {
    return self->SetWindowValues(_arg1, _arg2);
}
extern "C" void vtk_implicit_window_function_set_window_values_const_double_2(
    vtkNew<vtkImplicitWindowFunction> self, const double* _arg
) {
    return self->SetWindowValues(_arg);
}
extern "C" void vtk_implicit_window_function_get_window_values_double_2(
    vtkNew<vtkImplicitWindowFunction> self, double* data
) {
    return self->GetWindowValues(data);
}
extern "C" unsigned long vtk_implicit_window_function_get_m_time(
    vtkNew<vtkImplicitWindowFunction> self
) {
    return self->GetMTime();
}
extern "C" bool vtk_implicit_window_function_uses_garbage_collector(
    vtkNew<vtkImplicitWindowFunction> self
) {
    return self->UsesGarbageCollector();
}
extern "C" vtkNew<vtkInEdgeIterator> vtkInEdgeIterator_new() {
    return vtkNew<vtkInEdgeIterator>();
}
extern "C" void vtkInEdgeIterator_destructor(vtkNew<vtkInEdgeIterator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkInEdgeIterator_get_ptr(vtkNew<vtkInEdgeIterator> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_in_edge_iterator_get_vertex(vtkNew<vtkInEdgeIterator> self) {
    return self->GetVertex();
}
extern "C" bool vtk_in_edge_iterator_has_next(vtkNew<vtkInEdgeIterator> self) {
    return self->HasNext();
}
extern "C" vtkNew<vtkIncrementalOctreeNode> vtkIncrementalOctreeNode_new() {
    return vtkNew<vtkIncrementalOctreeNode>();
}
extern "C" void vtkIncrementalOctreeNode_destructor(vtkNew<vtkIncrementalOctreeNode> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIncrementalOctreeNode_get_ptr(vtkNew<vtkIncrementalOctreeNode> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_incremental_octree_node_get_number_of_points(
    vtkNew<vtkIncrementalOctreeNode> self
) {
    return self->GetNumberOfPoints();
}
extern "C" void vtk_incremental_octree_node_delete_child_nodes(
    vtkNew<vtkIncrementalOctreeNode> self
) {
    return self->DeleteChildNodes();
}
extern "C" void vtk_incremental_octree_node_set_bounds(
    vtkNew<vtkIncrementalOctreeNode> self,
    double x1,
    double x2,
    double y1,
    double y2,
    double z1,
    double z2
) {
    return self->SetBounds(x1, x2, y1, y2, z1, z2);
}
extern "C" void vtk_incremental_octree_node_get_bounds(
    vtkNew<vtkIncrementalOctreeNode> self, double* bounds
) {
    return self->GetBounds(bounds);
}
extern "C" void vtk_incremental_octree_node_get_min_bounds_double_3(
    vtkNew<vtkIncrementalOctreeNode> self, double* _arg
) {
    return self->GetMinBounds(_arg);
}
extern "C" void vtk_incremental_octree_node_get_max_bounds_double_3(
    vtkNew<vtkIncrementalOctreeNode> self, double* _arg
) {
    return self->GetMaxBounds(_arg);
}
extern "C" int vtk_incremental_octree_node_is_leaf(vtkNew<vtkIncrementalOctreeNode> self) {
    return self->IsLeaf();
}
extern "C" int vtk_incremental_octree_node_get_child_index(
    vtkNew<vtkIncrementalOctreeNode> self, const double* point
) {
    return self->GetChildIndex(point);
}
extern "C" int vtk_incremental_octree_node_contains_point(
    vtkNew<vtkIncrementalOctreeNode> self, const double* pnt
) {
    return self->ContainsPoint(pnt);
}
extern "C" int vtk_incremental_octree_node_contains_point_by_data(
    vtkNew<vtkIncrementalOctreeNode> self, const double* pnt
) {
    return self->ContainsPointByData(pnt);
}
extern "C" int vtk_incremental_octree_node_get_number_of_levels(
    vtkNew<vtkIncrementalOctreeNode> self
) {
    return self->GetNumberOfLevels();
}
extern "C" int vtk_incremental_octree_node_get_id(vtkNew<vtkIncrementalOctreeNode> self) {
    return self->GetID();
}
extern "C" vtkNew<vtkIncrementalOctreePointLocator> vtkIncrementalOctreePointLocator_new() {
    return vtkNew<vtkIncrementalOctreePointLocator>();
}
extern "C" void vtkIncrementalOctreePointLocator_destructor(
    vtkNew<vtkIncrementalOctreePointLocator> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkIncrementalOctreePointLocator_get_ptr(
    vtkNew<vtkIncrementalOctreePointLocator> sself
) {
    return sself.GetPointer();
}
extern "C" void vtk_incremental_octree_point_locator_set_max_points_per_leaf(
    vtkNew<vtkIncrementalOctreePointLocator> self, int _arg
) {
    return self->SetMaxPointsPerLeaf(_arg);
}
extern "C" int vtk_incremental_octree_point_locator_get_max_points_per_leaf(
    vtkNew<vtkIncrementalOctreePointLocator> self
) {
    return self->GetMaxPointsPerLeaf();
}
extern "C" void vtk_incremental_octree_point_locator_set_build_cubic_octree(
    vtkNew<vtkIncrementalOctreePointLocator> self, int _arg
) {
    return self->SetBuildCubicOctree(_arg);
}
extern "C" int vtk_incremental_octree_point_locator_get_build_cubic_octree(
    vtkNew<vtkIncrementalOctreePointLocator> self
) {
    return self->GetBuildCubicOctree();
}
extern "C" void vtk_incremental_octree_point_locator_build_cubic_octree_on(
    vtkNew<vtkIncrementalOctreePointLocator> self
) {
    return self->BuildCubicOctreeOn();
}
extern "C" void vtk_incremental_octree_point_locator_build_cubic_octree_off(
    vtkNew<vtkIncrementalOctreePointLocator> self
) {
    return self->BuildCubicOctreeOff();
}
extern "C" void vtk_incremental_octree_point_locator_initialize(
    vtkNew<vtkIncrementalOctreePointLocator> self
) {
    return self->Initialize();
}
extern "C" void vtk_incremental_octree_point_locator_free_search_structure(
    vtkNew<vtkIncrementalOctreePointLocator> self
) {
    return self->FreeSearchStructure();
}
extern "C" int vtk_incremental_octree_point_locator_get_number_of_points(
    vtkNew<vtkIncrementalOctreePointLocator> self
) {
    return self->GetNumberOfPoints();
}
extern "C" int vtk_incremental_octree_point_locator_get_number_of_nodes(
    vtkNew<vtkIncrementalOctreePointLocator> self
) {
    return self->GetNumberOfNodes();
}
extern "C" long long vtk_incremental_octree_point_locator_find_closest_inserted_point(
    vtkNew<vtkIncrementalOctreePointLocator> self, const double* x
) {
    return self->FindClosestInsertedPoint(x);
}
extern "C" void vtk_incremental_octree_point_locator_build_locator(
    vtkNew<vtkIncrementalOctreePointLocator> self
) {
    return self->BuildLocator();
}
extern "C" void vtk_incremental_octree_point_locator_force_build_locator(
    vtkNew<vtkIncrementalOctreePointLocator> self
) {
    return self->ForceBuildLocator();
}
extern "C" long long vtk_incremental_octree_point_locator_find_closest_point_const_double_3(
    vtkNew<vtkIncrementalOctreePointLocator> self, const double* x
) {
    return self->FindClosestPoint(x);
}
extern "C" long long vtk_incremental_octree_point_locator_find_closest_point_double_double_double(
    vtkNew<vtkIncrementalOctreePointLocator> self, double x, double y, double z
) {
    return self->FindClosestPoint(x, y, z);
}
extern "C" long long vtk_incremental_octree_point_locator_is_inserted_point_const_double_3(
    vtkNew<vtkIncrementalOctreePointLocator> self, const double* x
) {
    return self->IsInsertedPoint(x);
}
extern "C" long long vtk_incremental_octree_point_locator_is_inserted_point_double_double_double(
    vtkNew<vtkIncrementalOctreePointLocator> self, double x, double y, double z
) {
    return self->IsInsertedPoint(x, y, z);
}
extern "C" void vtk_incremental_octree_point_locator_insert_point(
    vtkNew<vtkIncrementalOctreePointLocator> self, long long ptId, const double* x
) {
    return self->InsertPoint(ptId, x);
}
extern "C" long long vtk_incremental_octree_point_locator_insert_next_point(
    vtkNew<vtkIncrementalOctreePointLocator> self, const double* x
) {
    return self->InsertNextPoint(x);
}
extern "C" int vtk_incremental_octree_point_locator_get_number_of_levels(
    vtkNew<vtkIncrementalOctreePointLocator> self
) {
    return self->GetNumberOfLevels();
}
extern "C" vtkNew<vtkIterativeClosestPointTransform> vtkIterativeClosestPointTransform_new() {
    return vtkNew<vtkIterativeClosestPointTransform>();
}
extern "C" void vtkIterativeClosestPointTransform_destructor(
    vtkNew<vtkIterativeClosestPointTransform> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkIterativeClosestPointTransform_get_ptr(
    vtkNew<vtkIterativeClosestPointTransform> sself
) {
    return sself.GetPointer();
}
extern "C" void vtk_iterative_closest_point_transform_set_maximum_number_of_iterations(
    vtkNew<vtkIterativeClosestPointTransform> self, int _arg
) {
    return self->SetMaximumNumberOfIterations(_arg);
}
extern "C" int vtk_iterative_closest_point_transform_get_maximum_number_of_iterations(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->GetMaximumNumberOfIterations();
}
extern "C" int vtk_iterative_closest_point_transform_get_number_of_iterations(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->GetNumberOfIterations();
}
extern "C" void vtk_iterative_closest_point_transform_set_check_mean_distance(
    vtkNew<vtkIterativeClosestPointTransform> self, int _arg
) {
    return self->SetCheckMeanDistance(_arg);
}
extern "C" int vtk_iterative_closest_point_transform_get_check_mean_distance(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->GetCheckMeanDistance();
}
extern "C" void vtk_iterative_closest_point_transform_check_mean_distance_on(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->CheckMeanDistanceOn();
}
extern "C" void vtk_iterative_closest_point_transform_check_mean_distance_off(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->CheckMeanDistanceOff();
}
extern "C" void vtk_iterative_closest_point_transform_set_mean_distance_mode(
    vtkNew<vtkIterativeClosestPointTransform> self, int _arg
) {
    return self->SetMeanDistanceMode(_arg);
}
extern "C" int vtk_iterative_closest_point_transform_get_mean_distance_mode_min_value(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->GetMeanDistanceModeMinValue();
}
extern "C" int vtk_iterative_closest_point_transform_get_mean_distance_mode_max_value(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->GetMeanDistanceModeMaxValue();
}
extern "C" int vtk_iterative_closest_point_transform_get_mean_distance_mode(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->GetMeanDistanceMode();
}
extern "C" void vtk_iterative_closest_point_transform_set_mean_distance_mode_to_rms(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->SetMeanDistanceModeToRMS();
}
extern "C" void vtk_iterative_closest_point_transform_set_mean_distance_mode_to_absolute_value(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->SetMeanDistanceModeToAbsoluteValue();
}
extern "C" void vtk_iterative_closest_point_transform_set_maximum_mean_distance(
    vtkNew<vtkIterativeClosestPointTransform> self, double _arg
) {
    return self->SetMaximumMeanDistance(_arg);
}
extern "C" double vtk_iterative_closest_point_transform_get_maximum_mean_distance(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->GetMaximumMeanDistance();
}
extern "C" double vtk_iterative_closest_point_transform_get_mean_distance(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->GetMeanDistance();
}
extern "C" void vtk_iterative_closest_point_transform_set_maximum_number_of_landmarks(
    vtkNew<vtkIterativeClosestPointTransform> self, int _arg
) {
    return self->SetMaximumNumberOfLandmarks(_arg);
}
extern "C" int vtk_iterative_closest_point_transform_get_maximum_number_of_landmarks(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->GetMaximumNumberOfLandmarks();
}
extern "C" void vtk_iterative_closest_point_transform_set_start_by_matching_centroids(
    vtkNew<vtkIterativeClosestPointTransform> self, int _arg
) {
    return self->SetStartByMatchingCentroids(_arg);
}
extern "C" int vtk_iterative_closest_point_transform_get_start_by_matching_centroids(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->GetStartByMatchingCentroids();
}
extern "C" void vtk_iterative_closest_point_transform_start_by_matching_centroids_on(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->StartByMatchingCentroidsOn();
}
extern "C" void vtk_iterative_closest_point_transform_start_by_matching_centroids_off(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->StartByMatchingCentroidsOff();
}
extern "C" void vtk_iterative_closest_point_transform_inverse(
    vtkNew<vtkIterativeClosestPointTransform> self
) {
    return self->Inverse();
}
extern "C" vtkNew<vtkKdNode> vtkKdNode_new() {
    return vtkNew<vtkKdNode>();
}
extern "C" void vtkKdNode_destructor(vtkNew<vtkKdNode> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkKdNode_get_ptr(vtkNew<vtkKdNode> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_kd_node_set_dim(vtkNew<vtkKdNode> self, int _arg) {
    return self->SetDim(_arg);
}
extern "C" int vtk_kd_node_get_dim(vtkNew<vtkKdNode> self) {
    return self->GetDim();
}
extern "C" double vtk_kd_node_get_division_position(vtkNew<vtkKdNode> self) {
    return self->GetDivisionPosition();
}
extern "C" void vtk_kd_node_set_number_of_points(vtkNew<vtkKdNode> self, int _arg) {
    return self->SetNumberOfPoints(_arg);
}
extern "C" int vtk_kd_node_get_number_of_points(vtkNew<vtkKdNode> self) {
    return self->GetNumberOfPoints();
}
extern "C" void vtk_kd_node_set_bounds_double_double_double_double_double_double(
    vtkNew<vtkKdNode> self, double x1, double x2, double y1, double y2, double z1, double z2
) {
    return self->SetBounds(x1, x2, y1, y2, z1, z2);
}
extern "C" void vtk_kd_node_set_bounds_const_double_6(vtkNew<vtkKdNode> self, const double* b) {
    return self->SetBounds(b);
}
extern "C" void vtk_kd_node_set_data_bounds_double_double_double_double_double_double(
    vtkNew<vtkKdNode> self, double x1, double x2, double y1, double y2, double z1, double z2
) {
    return self->SetDataBounds(x1, x2, y1, y2, z1, z2);
}
extern "C" void vtk_kd_node_set_id(vtkNew<vtkKdNode> self, int _arg) {
    return self->SetID(_arg);
}
extern "C" int vtk_kd_node_get_id(vtkNew<vtkKdNode> self) {
    return self->GetID();
}
extern "C" int vtk_kd_node_get_min_id(vtkNew<vtkKdNode> self) {
    return self->GetMinID();
}
extern "C" int vtk_kd_node_get_max_id(vtkNew<vtkKdNode> self) {
    return self->GetMaxID();
}
extern "C" void vtk_kd_node_set_min_id(vtkNew<vtkKdNode> self, int _arg) {
    return self->SetMinID(_arg);
}
extern "C" void vtk_kd_node_set_max_id(vtkNew<vtkKdNode> self, int _arg) {
    return self->SetMaxID(_arg);
}
extern "C" void vtk_kd_node_delete_child_nodes(vtkNew<vtkKdNode> self) {
    return self->DeleteChildNodes();
}
extern "C" int vtk_kd_node_intersects_box(
    vtkNew<vtkKdNode> self,
    double x1,
    double x2,
    double y1,
    double y2,
    double z1,
    double z2,
    int useDataBounds
) {
    return self->IntersectsBox(x1, x2, y1, y2, z1, z2, useDataBounds);
}
extern "C" int vtk_kd_node_intersects_sphere_2(
    vtkNew<vtkKdNode> self, double x, double y, double z, double rSquared, int useDataBounds
) {
    return self->IntersectsSphere2(x, y, z, rSquared, useDataBounds);
}
extern "C" int vtk_kd_node_contains_box(
    vtkNew<vtkKdNode> self,
    double x1,
    double x2,
    double y1,
    double y2,
    double z1,
    double z2,
    int useDataBounds
) {
    return self->ContainsBox(x1, x2, y1, y2, z1, z2, useDataBounds);
}
extern "C" int vtk_kd_node_contains_point(
    vtkNew<vtkKdNode> self, double x, double y, double z, int useDataBounds
) {
    return self->ContainsPoint(x, y, z, useDataBounds);
}
extern "C" double vtk_kd_node_get_distance_2_to_boundary_double_double_double_c_int(
    vtkNew<vtkKdNode> self, double x, double y, double z, int useDataBounds
) {
    return self->GetDistance2ToBoundary(x, y, z, useDataBounds);
}
extern "C" double vtk_kd_node_get_distance_2_to_inner_boundary(
    vtkNew<vtkKdNode> self, double x, double y, double z
) {
    return self->GetDistance2ToInnerBoundary(x, y, z);
}
extern "C" void vtk_kd_node_print_node(vtkNew<vtkKdNode> self, int depth) {
    return self->PrintNode(depth);
}
extern "C" void vtk_kd_node_print_verbose_node(vtkNew<vtkKdNode> self, int depth) {
    return self->PrintVerboseNode(depth);
}
extern "C" vtkNew<vtkKdTree> vtkKdTree_new() {
    return vtkNew<vtkKdTree>();
}
extern "C" void vtkKdTree_destructor(vtkNew<vtkKdTree> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkKdTree_get_ptr(vtkNew<vtkKdTree> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_kd_tree_timing_on(vtkNew<vtkKdTree> self) {
    return self->TimingOn();
}
extern "C" void vtk_kd_tree_timing_off(vtkNew<vtkKdTree> self) {
    return self->TimingOff();
}
extern "C" void vtk_kd_tree_set_timing(vtkNew<vtkKdTree> self, int _arg) {
    return self->SetTiming(_arg);
}
extern "C" int vtk_kd_tree_get_timing(vtkNew<vtkKdTree> self) {
    return self->GetTiming();
}
extern "C" void vtk_kd_tree_set_min_cells(vtkNew<vtkKdTree> self, int _arg) {
    return self->SetMinCells(_arg);
}
extern "C" int vtk_kd_tree_get_min_cells(vtkNew<vtkKdTree> self) {
    return self->GetMinCells();
}
extern "C" int vtk_kd_tree_get_number_of_regions_or_less(vtkNew<vtkKdTree> self) {
    return self->GetNumberOfRegionsOrLess();
}
extern "C" void vtk_kd_tree_set_number_of_regions_or_less(vtkNew<vtkKdTree> self, int _arg) {
    return self->SetNumberOfRegionsOrLess(_arg);
}
extern "C" int vtk_kd_tree_get_number_of_regions_or_more(vtkNew<vtkKdTree> self) {
    return self->GetNumberOfRegionsOrMore();
}
extern "C" void vtk_kd_tree_set_number_of_regions_or_more(vtkNew<vtkKdTree> self, int _arg) {
    return self->SetNumberOfRegionsOrMore(_arg);
}
extern "C" double vtk_kd_tree_get_fudge_factor(vtkNew<vtkKdTree> self) {
    return self->GetFudgeFactor();
}
extern "C" void vtk_kd_tree_set_fudge_factor(vtkNew<vtkKdTree> self, double _arg) {
    return self->SetFudgeFactor(_arg);
}
extern "C" void vtk_kd_tree_omit_x_partitioning(vtkNew<vtkKdTree> self) {
    return self->OmitXPartitioning();
}
extern "C" void vtk_kd_tree_omit_y_partitioning(vtkNew<vtkKdTree> self) {
    return self->OmitYPartitioning();
}
extern "C" void vtk_kd_tree_omit_z_partitioning(vtkNew<vtkKdTree> self) {
    return self->OmitZPartitioning();
}
extern "C" void vtk_kd_tree_omit_xy_partitioning(vtkNew<vtkKdTree> self) {
    return self->OmitXYPartitioning();
}
extern "C" void vtk_kd_tree_omit_yz_partitioning(vtkNew<vtkKdTree> self) {
    return self->OmitYZPartitioning();
}
extern "C" void vtk_kd_tree_omit_zx_partitioning(vtkNew<vtkKdTree> self) {
    return self->OmitZXPartitioning();
}
extern "C" void vtk_kd_tree_omit_no_partitioning(vtkNew<vtkKdTree> self) {
    return self->OmitNoPartitioning();
}
extern "C" void vtk_kd_tree_remove_data_set_c_int(vtkNew<vtkKdTree> self, int index) {
    return self->RemoveDataSet(index);
}
extern "C" void vtk_kd_tree_remove_all_data_sets(vtkNew<vtkKdTree> self) {
    return self->RemoveAllDataSets();
}
extern "C" int vtk_kd_tree_get_number_of_data_sets(vtkNew<vtkKdTree> self) {
    return self->GetNumberOfDataSets();
}
extern "C" int vtk_kd_tree_get_number_of_regions(vtkNew<vtkKdTree> self) {
    return self->GetNumberOfRegions();
}
extern "C" void vtk_kd_tree_get_region_bounds(
    vtkNew<vtkKdTree> self, int regionID, double* bounds
) {
    return self->GetRegionBounds(regionID, bounds);
}
extern "C" void vtk_kd_tree_get_region_data_bounds(
    vtkNew<vtkKdTree> self, int regionID, double* bounds
) {
    return self->GetRegionDataBounds(regionID, bounds);
}
extern "C" void vtk_kd_tree_print_tree(vtkNew<vtkKdTree> self) {
    return self->PrintTree();
}
extern "C" void vtk_kd_tree_print_verbose_tree(vtkNew<vtkKdTree> self) {
    return self->PrintVerboseTree();
}
extern "C" void vtk_kd_tree_print_region(vtkNew<vtkKdTree> self, int id) {
    return self->PrintRegion(id);
}
extern "C" void vtk_kd_tree_create_cell_lists(vtkNew<vtkKdTree> self) {
    return self->CreateCellLists();
}
extern "C" void vtk_kd_tree_set_include_region_boundary_cells(vtkNew<vtkKdTree> self, int _arg) {
    return self->SetIncludeRegionBoundaryCells(_arg);
}
extern "C" int vtk_kd_tree_get_include_region_boundary_cells(vtkNew<vtkKdTree> self) {
    return self->GetIncludeRegionBoundaryCells();
}
extern "C" void vtk_kd_tree_include_region_boundary_cells_on(vtkNew<vtkKdTree> self) {
    return self->IncludeRegionBoundaryCellsOn();
}
extern "C" void vtk_kd_tree_include_region_boundary_cells_off(vtkNew<vtkKdTree> self) {
    return self->IncludeRegionBoundaryCellsOff();
}
extern "C" void vtk_kd_tree_delete_cell_lists(vtkNew<vtkKdTree> self) {
    return self->DeleteCellLists();
}
extern "C" int vtk_kd_tree_get_region_containing_cell_c_int_c_longlong(
    vtkNew<vtkKdTree> self, int set, long long cellID
) {
    return self->GetRegionContainingCell(set, cellID);
}
extern "C" int vtk_kd_tree_get_region_containing_cell_c_longlong(
    vtkNew<vtkKdTree> self, long long cellID
) {
    return self->GetRegionContainingCell(cellID);
}
extern "C" int vtk_kd_tree_get_region_containing_point(
    vtkNew<vtkKdTree> self, double x, double y, double z
) {
    return self->GetRegionContainingPoint(x, y, z);
}
extern "C" void vtk_kd_tree_build_locator(vtkNew<vtkKdTree> self) {
    return self->BuildLocator();
}
extern "C" void vtk_kd_tree_force_build_locator(vtkNew<vtkKdTree> self) {
    return self->ForceBuildLocator();
}
extern "C" long long vtk_kd_tree_find_point_double_double_double(
    vtkNew<vtkKdTree> self, double x, double y, double z
) {
    return self->FindPoint(x, y, z);
}
extern "C" void vtk_kd_tree_free_search_structure(vtkNew<vtkKdTree> self) {
    return self->FreeSearchStructure();
}
extern "C" void vtk_kd_tree_generate_representation_using_data_bounds_on(vtkNew<vtkKdTree> self) {
    return self->GenerateRepresentationUsingDataBoundsOn();
}
extern "C" void vtk_kd_tree_generate_representation_using_data_bounds_off(vtkNew<vtkKdTree> self) {
    return self->GenerateRepresentationUsingDataBoundsOff();
}
extern "C" void vtk_kd_tree_set_generate_representation_using_data_bounds(
    vtkNew<vtkKdTree> self, int _arg
) {
    return self->SetGenerateRepresentationUsingDataBounds(_arg);
}
extern "C" int vtk_kd_tree_get_generate_representation_using_data_bounds(vtkNew<vtkKdTree> self) {
    return self->GetGenerateRepresentationUsingDataBounds();
}
extern "C" int vtk_kd_tree_new_geometry(vtkNew<vtkKdTree> self) {
    return self->NewGeometry();
}
extern "C" void vtk_kd_tree_invalidate_geometry(vtkNew<vtkKdTree> self) {
    return self->InvalidateGeometry();
}
extern "C" vtkNew<vtkKdTreePointLocator> vtkKdTreePointLocator_new() {
    return vtkNew<vtkKdTreePointLocator>();
}
extern "C" void vtkKdTreePointLocator_destructor(vtkNew<vtkKdTreePointLocator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkKdTreePointLocator_get_ptr(vtkNew<vtkKdTreePointLocator> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_kd_tree_point_locator_find_closest_point(
    vtkNew<vtkKdTreePointLocator> self, const double* x
) {
    return self->FindClosestPoint(x);
}
extern "C" void vtk_kd_tree_point_locator_free_search_structure(
    vtkNew<vtkKdTreePointLocator> self
) {
    return self->FreeSearchStructure();
}
extern "C" void vtk_kd_tree_point_locator_build_locator(vtkNew<vtkKdTreePointLocator> self) {
    return self->BuildLocator();
}
extern "C" vtkNew<vtkLagrangeCurve> vtkLagrangeCurve_new() {
    return vtkNew<vtkLagrangeCurve>();
}
extern "C" void vtkLagrangeCurve_destructor(vtkNew<vtkLagrangeCurve> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkLagrangeCurve_get_ptr(vtkNew<vtkLagrangeCurve> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_lagrange_curve_get_cell_type(vtkNew<vtkLagrangeCurve> self) {
    return self->GetCellType();
}
extern "C" vtkNew<vtkLagrangeHexahedron> vtkLagrangeHexahedron_new() {
    return vtkNew<vtkLagrangeHexahedron>();
}
extern "C" void vtkLagrangeHexahedron_destructor(vtkNew<vtkLagrangeHexahedron> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkLagrangeHexahedron_get_ptr(vtkNew<vtkLagrangeHexahedron> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_lagrange_hexahedron_get_cell_type(vtkNew<vtkLagrangeHexahedron> self) {
    return self->GetCellType();
}
extern "C" vtkNew<vtkLagrangeInterpolation> vtkLagrangeInterpolation_new() {
    return vtkNew<vtkLagrangeInterpolation>();
}
extern "C" void vtkLagrangeInterpolation_destructor(vtkNew<vtkLagrangeInterpolation> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkLagrangeInterpolation_get_ptr(vtkNew<vtkLagrangeInterpolation> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkLagrangeQuadrilateral> vtkLagrangeQuadrilateral_new() {
    return vtkNew<vtkLagrangeQuadrilateral>();
}
extern "C" void vtkLagrangeQuadrilateral_destructor(vtkNew<vtkLagrangeQuadrilateral> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkLagrangeQuadrilateral_get_ptr(vtkNew<vtkLagrangeQuadrilateral> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_lagrange_quadrilateral_get_cell_type(vtkNew<vtkLagrangeQuadrilateral> self) {
    return self->GetCellType();
}
extern "C" vtkNew<vtkLagrangeTetra> vtkLagrangeTetra_new() {
    return vtkNew<vtkLagrangeTetra>();
}
extern "C" void vtkLagrangeTetra_destructor(vtkNew<vtkLagrangeTetra> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkLagrangeTetra_get_ptr(vtkNew<vtkLagrangeTetra> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_lagrange_tetra_get_cell_type(vtkNew<vtkLagrangeTetra> self) {
    return self->GetCellType();
}
extern "C" vtkNew<vtkLagrangeTriangle> vtkLagrangeTriangle_new() {
    return vtkNew<vtkLagrangeTriangle>();
}
extern "C" void vtkLagrangeTriangle_destructor(vtkNew<vtkLagrangeTriangle> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkLagrangeTriangle_get_ptr(vtkNew<vtkLagrangeTriangle> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_lagrange_triangle_get_cell_type(vtkNew<vtkLagrangeTriangle> self) {
    return self->GetCellType();
}
extern "C" vtkNew<vtkLagrangeWedge> vtkLagrangeWedge_new() {
    return vtkNew<vtkLagrangeWedge>();
}
extern "C" void vtkLagrangeWedge_destructor(vtkNew<vtkLagrangeWedge> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkLagrangeWedge_get_ptr(vtkNew<vtkLagrangeWedge> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_lagrange_wedge_get_cell_type(vtkNew<vtkLagrangeWedge> self) {
    return self->GetCellType();
}
extern "C" vtkNew<vtkLine> vtkLine_new() {
    return vtkNew<vtkLine>();
}
extern "C" void vtkLine_destructor(vtkNew<vtkLine> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkLine_get_ptr(vtkNew<vtkLine> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_line_get_cell_type(vtkNew<vtkLine> self) {
    return self->GetCellType();
}
extern "C" int vtk_line_get_cell_dimension(vtkNew<vtkLine> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_line_get_number_of_edges(vtkNew<vtkLine> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_line_get_number_of_faces(vtkNew<vtkLine> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_line_inflate(vtkNew<vtkLine> self, double dist) {
    return self->Inflate(dist);
}
extern "C" int vtk_line_get_parametric_center(vtkNew<vtkLine> self, double* pcoords) {
    return self->GetParametricCenter(pcoords);
}
extern "C" double vtk_line_distance_to_line_const_double_3_const_double_3_const_double_3(
    vtkNew<vtkLine> self, const double* x, const double* p1, const double* p2
) {
    return self->DistanceToLine(x, p1, p2);
}
extern "C" void vtk_line_interpolation_functions(
    vtkNew<vtkLine> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_line_interpolation_derivs(
    vtkNew<vtkLine> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_line_interpolate_functions(
    vtkNew<vtkLine> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_line_interpolate_derivs(
    vtkNew<vtkLine> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkMeanValueCoordinatesInterpolator> vtkMeanValueCoordinatesInterpolator_new() {
    return vtkNew<vtkMeanValueCoordinatesInterpolator>();
}
extern "C" void vtkMeanValueCoordinatesInterpolator_destructor(
    vtkNew<vtkMeanValueCoordinatesInterpolator> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkMeanValueCoordinatesInterpolator_get_ptr(
    vtkNew<vtkMeanValueCoordinatesInterpolator> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkMergePoints> vtkMergePoints_new() {
    return vtkNew<vtkMergePoints>();
}
extern "C" void vtkMergePoints_destructor(vtkNew<vtkMergePoints> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkMergePoints_get_ptr(vtkNew<vtkMergePoints> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_merge_points_is_inserted_point(
    vtkNew<vtkMergePoints> self, const double* x
) {
    return self->IsInsertedPoint(x);
}
extern "C" vtkNew<vtkMolecule> vtkMolecule_new() {
    return vtkNew<vtkMolecule>();
}
extern "C" void vtkMolecule_destructor(vtkNew<vtkMolecule> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkMolecule_get_ptr(vtkNew<vtkMolecule> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_molecule_get_number_of_atoms(vtkNew<vtkMolecule> self) {
    return self->GetNumberOfAtoms();
}
extern "C" long long vtk_molecule_get_number_of_bonds(vtkNew<vtkMolecule> self) {
    return self->GetNumberOfBonds();
}
extern "C" unsigned short vtk_molecule_get_atom_atomic_number(
    vtkNew<vtkMolecule> self, long long atomId
) {
    return self->GetAtomAtomicNumber(atomId);
}
extern "C" void vtk_molecule_set_atom_atomic_number(
    vtkNew<vtkMolecule> self, long long atomId, unsigned short atomicNum
) {
    return self->SetAtomAtomicNumber(atomId, atomicNum);
}
extern "C" void vtk_molecule_set_atom_position_c_longlong_double_double_double(
    vtkNew<vtkMolecule> self, long long atomId, double x, double y, double z
) {
    return self->SetAtomPosition(atomId, x, y, z);
}
extern "C" void vtk_molecule_set_atom_position_c_longlong_double_3(
    vtkNew<vtkMolecule> self, long long atomId, double* pos
) {
    return self->SetAtomPosition(atomId, pos);
}
extern "C" void vtk_molecule_get_atom_position_c_longlong_float_3(
    vtkNew<vtkMolecule> self, long long atomId, float* pos
) {
    return self->GetAtomPosition(atomId, pos);
}
extern "C" void vtk_molecule_get_atom_position_c_longlong_double_3(
    vtkNew<vtkMolecule> self, long long atomId, double* pos
) {
    return self->GetAtomPosition(atomId, pos);
}
extern "C" void vtk_molecule_set_bond_order(
    vtkNew<vtkMolecule> self, long long bondId, unsigned short order
) {
    return self->SetBondOrder(bondId, order);
}
extern "C" unsigned short vtk_molecule_get_bond_order(vtkNew<vtkMolecule> self, long long bondId) {
    return self->GetBondOrder(bondId);
}
extern "C" double vtk_molecule_get_bond_length(vtkNew<vtkMolecule> self, long long bondId) {
    return self->GetBondLength(bondId);
}
extern "C" bool vtk_molecule_has_lattice(vtkNew<vtkMolecule> self) {
    return self->HasLattice();
}
extern "C" void vtk_molecule_clear_lattice(vtkNew<vtkMolecule> self) {
    return self->ClearLattice();
}
extern "C" void vtk_molecule_allocate_atom_ghost_array(vtkNew<vtkMolecule> self) {
    return self->AllocateAtomGhostArray();
}
extern "C" void vtk_molecule_allocate_bond_ghost_array(vtkNew<vtkMolecule> self) {
    return self->AllocateBondGhostArray();
}
extern "C" long long vtk_molecule_get_bond_id(vtkNew<vtkMolecule> self, long long a, long long b) {
    return self->GetBondId(a, b);
}
extern "C" vtkNew<vtkMultiBlockDataSet> vtkMultiBlockDataSet_new() {
    return vtkNew<vtkMultiBlockDataSet>();
}
extern "C" void vtkMultiBlockDataSet_destructor(vtkNew<vtkMultiBlockDataSet> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkMultiBlockDataSet_get_ptr(vtkNew<vtkMultiBlockDataSet> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_multi_block_data_set_set_number_of_blocks(
    vtkNew<vtkMultiBlockDataSet> self, unsigned int numBlocks
) {
    return self->SetNumberOfBlocks(numBlocks);
}
extern "C" unsigned int vtk_multi_block_data_set_get_number_of_blocks(
    vtkNew<vtkMultiBlockDataSet> self
) {
    return self->GetNumberOfBlocks();
}
extern "C" void vtk_multi_block_data_set_remove_block(
    vtkNew<vtkMultiBlockDataSet> self, unsigned int blockno
) {
    return self->RemoveBlock(blockno);
}
extern "C" int vtk_multi_block_data_set_has_meta_data_c_uint(
    vtkNew<vtkMultiBlockDataSet> self, unsigned int blockno
) {
    return self->HasMetaData(blockno);
}
extern "C" vtkNew<vtkMultiPieceDataSet> vtkMultiPieceDataSet_new() {
    return vtkNew<vtkMultiPieceDataSet>();
}
extern "C" void vtkMultiPieceDataSet_destructor(vtkNew<vtkMultiPieceDataSet> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkMultiPieceDataSet_get_ptr(vtkNew<vtkMultiPieceDataSet> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_multi_piece_data_set_set_number_of_pieces(
    vtkNew<vtkMultiPieceDataSet> self, unsigned int numpieces
) {
    return self->SetNumberOfPieces(numpieces);
}
extern "C" unsigned int vtk_multi_piece_data_set_get_number_of_pieces(
    vtkNew<vtkMultiPieceDataSet> self
) {
    return self->GetNumberOfPieces();
}
extern "C" vtkNew<vtkMutableDirectedGraph> vtkMutableDirectedGraph_new() {
    return vtkNew<vtkMutableDirectedGraph>();
}
extern "C" void vtkMutableDirectedGraph_destructor(vtkNew<vtkMutableDirectedGraph> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkMutableDirectedGraph_get_ptr(vtkNew<vtkMutableDirectedGraph> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_mutable_directed_graph_set_number_of_vertices(
    vtkNew<vtkMutableDirectedGraph> self, long long numVerts
) {
    return self->SetNumberOfVertices(numVerts);
}
extern "C" long long vtk_mutable_directed_graph_add_vertex(vtkNew<vtkMutableDirectedGraph> self) {
    return self->AddVertex();
}
extern "C" void vtk_mutable_directed_graph_lazy_add_vertex(vtkNew<vtkMutableDirectedGraph> self) {
    return self->LazyAddVertex();
}
extern "C" long long vtk_mutable_directed_graph_add_child_c_longlong(
    vtkNew<vtkMutableDirectedGraph> self, long long parent
) {
    return self->AddChild(parent);
}
extern "C" void vtk_mutable_directed_graph_remove_vertex(
    vtkNew<vtkMutableDirectedGraph> self, long long v
) {
    return self->RemoveVertex(v);
}
extern "C" void vtk_mutable_directed_graph_remove_edge(
    vtkNew<vtkMutableDirectedGraph> self, long long e
) {
    return self->RemoveEdge(e);
}
extern "C" vtkNew<vtkMutableUndirectedGraph> vtkMutableUndirectedGraph_new() {
    return vtkNew<vtkMutableUndirectedGraph>();
}
extern "C" void vtkMutableUndirectedGraph_destructor(vtkNew<vtkMutableUndirectedGraph> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkMutableUndirectedGraph_get_ptr(vtkNew<vtkMutableUndirectedGraph> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_mutable_undirected_graph_set_number_of_vertices(
    vtkNew<vtkMutableUndirectedGraph> self, long long numVerts
) {
    return self->SetNumberOfVertices(numVerts);
}
extern "C" long long vtk_mutable_undirected_graph_add_vertex(
    vtkNew<vtkMutableUndirectedGraph> self
) {
    return self->AddVertex();
}
extern "C" void vtk_mutable_undirected_graph_lazy_add_vertex(
    vtkNew<vtkMutableUndirectedGraph> self
) {
    return self->LazyAddVertex();
}
extern "C" void vtk_mutable_undirected_graph_lazy_add_edge_c_longlong_c_longlong(
    vtkNew<vtkMutableUndirectedGraph> self, long long u, long long v
) {
    return self->LazyAddEdge(u, v);
}
extern "C" void vtk_mutable_undirected_graph_remove_vertex(
    vtkNew<vtkMutableUndirectedGraph> self, long long v
) {
    return self->RemoveVertex(v);
}
extern "C" void vtk_mutable_undirected_graph_remove_edge(
    vtkNew<vtkMutableUndirectedGraph> self, long long e
) {
    return self->RemoveEdge(e);
}
extern "C" vtkNew<vtkNonMergingPointLocator> vtkNonMergingPointLocator_new() {
    return vtkNew<vtkNonMergingPointLocator>();
}
extern "C" void vtkNonMergingPointLocator_destructor(vtkNew<vtkNonMergingPointLocator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkNonMergingPointLocator_get_ptr(vtkNew<vtkNonMergingPointLocator> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_non_merging_point_locator_is_inserted_point_const_double_3(
    vtkNew<vtkNonMergingPointLocator> self, const double* p0
) {
    return self->IsInsertedPoint(p0);
}
extern "C" long long vtk_non_merging_point_locator_is_inserted_point_double_double_double(
    vtkNew<vtkNonMergingPointLocator> self, double p0, double p1, double p2
) {
    return self->IsInsertedPoint(p0, p1, p2);
}
extern "C" vtkNew<vtkNonOverlappingAMR> vtkNonOverlappingAMR_new() {
    return vtkNew<vtkNonOverlappingAMR>();
}
extern "C" void vtkNonOverlappingAMR_destructor(vtkNew<vtkNonOverlappingAMR> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkNonOverlappingAMR_get_ptr(vtkNew<vtkNonOverlappingAMR> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_non_overlapping_amr_get_data_object_type(vtkNew<vtkNonOverlappingAMR> self) {
    return self->GetDataObjectType();
}
extern "C" vtkNew<vtkOctreePointLocator> vtkOctreePointLocator_new() {
    return vtkNew<vtkOctreePointLocator>();
}
extern "C" void vtkOctreePointLocator_destructor(vtkNew<vtkOctreePointLocator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkOctreePointLocator_get_ptr(vtkNew<vtkOctreePointLocator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_octree_point_locator_set_maximum_points_per_region(
    vtkNew<vtkOctreePointLocator> self, int _arg
) {
    return self->SetMaximumPointsPerRegion(_arg);
}
extern "C" int vtk_octree_point_locator_get_maximum_points_per_region(
    vtkNew<vtkOctreePointLocator> self
) {
    return self->GetMaximumPointsPerRegion();
}
extern "C" void vtk_octree_point_locator_set_create_cubic_octants(
    vtkNew<vtkOctreePointLocator> self, int _arg
) {
    return self->SetCreateCubicOctants(_arg);
}
extern "C" int vtk_octree_point_locator_get_create_cubic_octants(
    vtkNew<vtkOctreePointLocator> self
) {
    return self->GetCreateCubicOctants();
}
extern "C" double vtk_octree_point_locator_get_fudge_factor(vtkNew<vtkOctreePointLocator> self) {
    return self->GetFudgeFactor();
}
extern "C" void vtk_octree_point_locator_set_fudge_factor(
    vtkNew<vtkOctreePointLocator> self, double _arg
) {
    return self->SetFudgeFactor(_arg);
}
extern "C" int vtk_octree_point_locator_get_number_of_leaf_nodes(
    vtkNew<vtkOctreePointLocator> self
) {
    return self->GetNumberOfLeafNodes();
}
extern "C" void vtk_octree_point_locator_get_region_bounds(
    vtkNew<vtkOctreePointLocator> self, int regionID, double* bounds
) {
    return self->GetRegionBounds(regionID, bounds);
}
extern "C" void vtk_octree_point_locator_get_region_data_bounds(
    vtkNew<vtkOctreePointLocator> self, int leafNodeID, double* bounds
) {
    return self->GetRegionDataBounds(leafNodeID, bounds);
}
extern "C" int vtk_octree_point_locator_get_region_containing_point(
    vtkNew<vtkOctreePointLocator> self, double x, double y, double z
) {
    return self->GetRegionContainingPoint(x, y, z);
}
extern "C" void vtk_octree_point_locator_build_locator(vtkNew<vtkOctreePointLocator> self) {
    return self->BuildLocator();
}
extern "C" void vtk_octree_point_locator_force_build_locator(vtkNew<vtkOctreePointLocator> self) {
    return self->ForceBuildLocator();
}
extern "C" long long vtk_octree_point_locator_find_closest_point_const_double_3(
    vtkNew<vtkOctreePointLocator> self, const double* x
) {
    return self->FindClosestPoint(x);
}
extern "C" void vtk_octree_point_locator_free_search_structure(vtkNew<vtkOctreePointLocator> self) {
    return self->FreeSearchStructure();
}
extern "C" vtkNew<vtkOctreePointLocatorNode> vtkOctreePointLocatorNode_new() {
    return vtkNew<vtkOctreePointLocatorNode>();
}
extern "C" void vtkOctreePointLocatorNode_destructor(vtkNew<vtkOctreePointLocatorNode> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkOctreePointLocatorNode_get_ptr(vtkNew<vtkOctreePointLocatorNode> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_octree_point_locator_node_set_number_of_points(
    vtkNew<vtkOctreePointLocatorNode> self, int numberOfPoints
) {
    return self->SetNumberOfPoints(numberOfPoints);
}
extern "C" int vtk_octree_point_locator_node_get_number_of_points(
    vtkNew<vtkOctreePointLocatorNode> self
) {
    return self->GetNumberOfPoints();
}
extern "C" void vtk_octree_point_locator_node_set_bounds_double_double_double_double_double_double(
    vtkNew<vtkOctreePointLocatorNode> self,
    double xMin,
    double xMax,
    double yMin,
    double yMax,
    double zMin,
    double zMax
) {
    return self->SetBounds(xMin, xMax, yMin, yMax, zMin, zMax);
}
extern "C" void vtk_octree_point_locator_node_set_bounds_const_double_6(
    vtkNew<vtkOctreePointLocatorNode> self, const double* b
) {
    return self->SetBounds(b);
}
extern "C" void vtk_octree_point_locator_node_set_data_bounds(
    vtkNew<vtkOctreePointLocatorNode> self,
    double xMin,
    double xMax,
    double yMin,
    double yMax,
    double zMin,
    double zMax
) {
    return self->SetDataBounds(xMin, xMax, yMin, yMax, zMin, zMax);
}
extern "C" void vtk_octree_point_locator_node_set_min_bounds(
    vtkNew<vtkOctreePointLocatorNode> self, double* minBounds
) {
    return self->SetMinBounds(minBounds);
}
extern "C" void vtk_octree_point_locator_node_set_max_bounds(
    vtkNew<vtkOctreePointLocatorNode> self, double* maxBounds
) {
    return self->SetMaxBounds(maxBounds);
}
extern "C" void vtk_octree_point_locator_node_set_min_data_bounds(
    vtkNew<vtkOctreePointLocatorNode> self, double* minDataBounds
) {
    return self->SetMinDataBounds(minDataBounds);
}
extern "C" void vtk_octree_point_locator_node_set_max_data_bounds(
    vtkNew<vtkOctreePointLocatorNode> self, double* maxDataBounds
) {
    return self->SetMaxDataBounds(maxDataBounds);
}
extern "C" int vtk_octree_point_locator_node_get_id(vtkNew<vtkOctreePointLocatorNode> self) {
    return self->GetID();
}
extern "C" int vtk_octree_point_locator_node_get_min_id(vtkNew<vtkOctreePointLocatorNode> self) {
    return self->GetMinID();
}
extern "C" void vtk_octree_point_locator_node_create_child_nodes(
    vtkNew<vtkOctreePointLocatorNode> self
) {
    return self->CreateChildNodes();
}
extern "C" void vtk_octree_point_locator_node_delete_child_nodes(
    vtkNew<vtkOctreePointLocatorNode> self
) {
    return self->DeleteChildNodes();
}
extern "C" int vtk_octree_point_locator_node_contains_point(
    vtkNew<vtkOctreePointLocatorNode> self, double x, double y, double z, int useDataBounds
) {
    return self->ContainsPoint(x, y, z, useDataBounds);
}
extern "C" vtkNew<vtkOrderedTriangulator> vtkOrderedTriangulator_new() {
    return vtkNew<vtkOrderedTriangulator>();
}
extern "C" void vtkOrderedTriangulator_destructor(vtkNew<vtkOrderedTriangulator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkOrderedTriangulator_get_ptr(vtkNew<vtkOrderedTriangulator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_ordered_triangulator_init_triangulation_double_double_double_double_double_double_c_int(
    vtkNew<vtkOrderedTriangulator> self,
    double xmin,
    double xmax,
    double ymin,
    double ymax,
    double zmin,
    double zmax,
    int numPts
) {
    return self->InitTriangulation(xmin, xmax, ymin, ymax, zmin, zmax, numPts);
}
extern "C" void vtk_ordered_triangulator_init_triangulation_double_6_c_int(
    vtkNew<vtkOrderedTriangulator> self, double* bounds, int numPts
) {
    return self->InitTriangulation(bounds, numPts);
}
extern "C" long long vtk_ordered_triangulator_insert_point_c_longlong_double_3_double_3_c_int(
    vtkNew<vtkOrderedTriangulator> self, long long id, double* x, double* p, int type
) {
    return self->InsertPoint(id, x, p, type);
}
extern "C" long long vtk_ordered_triangulator_insert_point_c_longlong_c_longlong_double_3_double_3_c_int(
    vtkNew<vtkOrderedTriangulator> self,
    long long id,
    long long sortid,
    double* x,
    double* p,
    int type
) {
    return self->InsertPoint(id, sortid, x, p, type);
}
extern "C" long long vtk_ordered_triangulator_insert_point_c_longlong_c_longlong_c_longlong_double_3_double_3_c_int(
    vtkNew<vtkOrderedTriangulator> self,
    long long id,
    long long sortid,
    long long sortid2,
    double* x,
    double* p,
    int type
) {
    return self->InsertPoint(id, sortid, sortid2, x, p, type);
}
extern "C" void vtk_ordered_triangulator_triangulate(vtkNew<vtkOrderedTriangulator> self) {
    return self->Triangulate();
}
extern "C" void vtk_ordered_triangulator_template_triangulate(
    vtkNew<vtkOrderedTriangulator> self, int cellType, int numPts, int numEdges
) {
    return self->TemplateTriangulate(cellType, numPts, numEdges);
}
extern "C" void vtk_ordered_triangulator_update_point_type(
    vtkNew<vtkOrderedTriangulator> self, long long internalId, int type
) {
    return self->UpdatePointType(internalId, type);
}
extern "C" long long vtk_ordered_triangulator_get_point_id(
    vtkNew<vtkOrderedTriangulator> self, long long internalId
) {
    return self->GetPointId(internalId);
}
extern "C" int vtk_ordered_triangulator_get_number_of_points(vtkNew<vtkOrderedTriangulator> self) {
    return self->GetNumberOfPoints();
}
extern "C" void vtk_ordered_triangulator_set_use_templates(
    vtkNew<vtkOrderedTriangulator> self, int _arg
) {
    return self->SetUseTemplates(_arg);
}
extern "C" int vtk_ordered_triangulator_get_use_templates(vtkNew<vtkOrderedTriangulator> self) {
    return self->GetUseTemplates();
}
extern "C" void vtk_ordered_triangulator_use_templates_on(vtkNew<vtkOrderedTriangulator> self) {
    return self->UseTemplatesOn();
}
extern "C" void vtk_ordered_triangulator_use_templates_off(vtkNew<vtkOrderedTriangulator> self) {
    return self->UseTemplatesOff();
}
extern "C" void vtk_ordered_triangulator_set_pre_sorted(
    vtkNew<vtkOrderedTriangulator> self, int _arg
) {
    return self->SetPreSorted(_arg);
}
extern "C" int vtk_ordered_triangulator_get_pre_sorted(vtkNew<vtkOrderedTriangulator> self) {
    return self->GetPreSorted();
}
extern "C" void vtk_ordered_triangulator_pre_sorted_on(vtkNew<vtkOrderedTriangulator> self) {
    return self->PreSortedOn();
}
extern "C" void vtk_ordered_triangulator_pre_sorted_off(vtkNew<vtkOrderedTriangulator> self) {
    return self->PreSortedOff();
}
extern "C" void vtk_ordered_triangulator_set_use_two_sort_ids(
    vtkNew<vtkOrderedTriangulator> self, int _arg
) {
    return self->SetUseTwoSortIds(_arg);
}
extern "C" int vtk_ordered_triangulator_get_use_two_sort_ids(vtkNew<vtkOrderedTriangulator> self) {
    return self->GetUseTwoSortIds();
}
extern "C" void vtk_ordered_triangulator_use_two_sort_ids_on(vtkNew<vtkOrderedTriangulator> self) {
    return self->UseTwoSortIdsOn();
}
extern "C" void vtk_ordered_triangulator_use_two_sort_ids_off(vtkNew<vtkOrderedTriangulator> self) {
    return self->UseTwoSortIdsOff();
}
extern "C" void vtk_ordered_triangulator_init_tetra_traversal(vtkNew<vtkOrderedTriangulator> self) {
    return self->InitTetraTraversal();
}
extern "C" vtkNew<vtkOutEdgeIterator> vtkOutEdgeIterator_new() {
    return vtkNew<vtkOutEdgeIterator>();
}
extern "C" void vtkOutEdgeIterator_destructor(vtkNew<vtkOutEdgeIterator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkOutEdgeIterator_get_ptr(vtkNew<vtkOutEdgeIterator> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_out_edge_iterator_get_vertex(vtkNew<vtkOutEdgeIterator> self) {
    return self->GetVertex();
}
extern "C" bool vtk_out_edge_iterator_has_next(vtkNew<vtkOutEdgeIterator> self) {
    return self->HasNext();
}
extern "C" vtkNew<vtkOverlappingAMR> vtkOverlappingAMR_new() {
    return vtkNew<vtkOverlappingAMR>();
}
extern "C" void vtkOverlappingAMR_destructor(vtkNew<vtkOverlappingAMR> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkOverlappingAMR_get_ptr(vtkNew<vtkOverlappingAMR> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_overlapping_amr_set_origin(
    vtkNew<vtkOverlappingAMR> self, const double* origin
) {
    return self->SetOrigin(origin);
}
extern "C" void vtk_overlapping_amr_set_spacing(
    vtkNew<vtkOverlappingAMR> self, unsigned int level, const double* spacing
) {
    return self->SetSpacing(level, spacing);
}
extern "C" void vtk_overlapping_amr_get_spacing(
    vtkNew<vtkOverlappingAMR> self, unsigned int level, double* spacing
) {
    return self->GetSpacing(level, spacing);
}
extern "C" void vtk_overlapping_amr_get_bounds_c_uint_c_uint_double_6(
    vtkNew<vtkOverlappingAMR> self, unsigned int level, unsigned int id, double* bb
) {
    return self->GetBounds(level, id, bb);
}
extern "C" void vtk_overlapping_amr_get_origin_c_uint_c_uint_double_3(
    vtkNew<vtkOverlappingAMR> self, unsigned int level, unsigned int id, double* origin
) {
    return self->GetOrigin(level, id, origin);
}
extern "C" void vtk_overlapping_amr_set_refinement_ratio(
    vtkNew<vtkOverlappingAMR> self, unsigned int level, int refRatio
) {
    return self->SetRefinementRatio(level, refRatio);
}
extern "C" int vtk_overlapping_amr_get_refinement_ratio_c_uint(
    vtkNew<vtkOverlappingAMR> self, unsigned int level
) {
    return self->GetRefinementRatio(level);
}
extern "C" void vtk_overlapping_amr_set_amr_block_source_index(
    vtkNew<vtkOverlappingAMR> self, unsigned int level, unsigned int id, int sourceId
) {
    return self->SetAMRBlockSourceIndex(level, id, sourceId);
}
extern "C" int vtk_overlapping_amr_get_amr_block_source_index(
    vtkNew<vtkOverlappingAMR> self, unsigned int level, unsigned int id
) {
    return self->GetAMRBlockSourceIndex(level, id);
}
extern "C" bool vtk_overlapping_amr_has_children_information(vtkNew<vtkOverlappingAMR> self) {
    return self->HasChildrenInformation();
}
extern "C" void vtk_overlapping_amr_generate_parent_child_information(
    vtkNew<vtkOverlappingAMR> self
) {
    return self->GenerateParentChildInformation();
}
extern "C" void vtk_overlapping_amr_print_parent_child_info(
    vtkNew<vtkOverlappingAMR> self, unsigned int level, unsigned int index
) {
    return self->PrintParentChildInfo(level, index);
}
extern "C" void vtk_overlapping_amr_audit(vtkNew<vtkOverlappingAMR> self) {
    return self->Audit();
}
extern "C" bool vtk_overlapping_amr_check_validity(vtkNew<vtkOverlappingAMR> self) {
    return self->CheckValidity();
}
extern "C" vtkNew<vtkOverlappingAMRMetaData> vtkOverlappingAMRMetaData_new() {
    return vtkNew<vtkOverlappingAMRMetaData>();
}
extern "C" void vtkOverlappingAMRMetaData_destructor(vtkNew<vtkOverlappingAMRMetaData> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkOverlappingAMRMetaData_get_ptr(vtkNew<vtkOverlappingAMRMetaData> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_overlapping_amr_meta_data_get_origin_double_3(
    vtkNew<vtkOverlappingAMRMetaData> self, double* origin
) {
    return self->GetOrigin(origin);
}
extern "C" void vtk_overlapping_amr_meta_data_get_spacing(
    vtkNew<vtkOverlappingAMRMetaData> self, unsigned int level, double* spacing
) {
    return self->GetSpacing(level, spacing);
}
extern "C" bool vtk_overlapping_amr_meta_data_has_spacing_c_uint(
    vtkNew<vtkOverlappingAMRMetaData> self, unsigned int level
) {
    return self->HasSpacing(level);
}
extern "C" bool vtk_overlapping_amr_meta_data_has_spacing(vtkNew<vtkOverlappingAMRMetaData> self) {
    return self->HasSpacing();
}
extern "C" bool vtk_overlapping_amr_meta_data_has_block_bounds_c_uint(
    vtkNew<vtkOverlappingAMRMetaData> self, unsigned int index
) {
    return self->HasBlockBounds(index);
}
extern "C" bool vtk_overlapping_amr_meta_data_has_block_bounds(
    vtkNew<vtkOverlappingAMRMetaData> self
) {
    return self->HasBlockBounds();
}
extern "C" int vtk_overlapping_amr_meta_data_get_amr_block_source_index(
    vtkNew<vtkOverlappingAMRMetaData> self, int index
) {
    return self->GetAMRBlockSourceIndex(index);
}
extern "C" void vtk_overlapping_amr_meta_data_set_amr_block_source_index(
    vtkNew<vtkOverlappingAMRMetaData> self, int index, int sourceId
) {
    return self->SetAMRBlockSourceIndex(index, sourceId);
}
extern "C" bool vtk_overlapping_amr_meta_data_generate_refinement_ratio(
    vtkNew<vtkOverlappingAMRMetaData> self
) {
    return self->GenerateRefinementRatio();
}
extern "C" bool vtk_overlapping_amr_meta_data_has_refinement_ratio(
    vtkNew<vtkOverlappingAMRMetaData> self
) {
    return self->HasRefinementRatio();
}
extern "C" void vtk_overlapping_amr_meta_data_set_refinement_ratio(
    vtkNew<vtkOverlappingAMRMetaData> self, unsigned int level, int ratio
) {
    return self->SetRefinementRatio(level, ratio);
}
extern "C" int vtk_overlapping_amr_meta_data_get_refinement_ratio(
    vtkNew<vtkOverlappingAMRMetaData> self, unsigned int level
) {
    return self->GetRefinementRatio(level);
}
extern "C" bool vtk_overlapping_amr_meta_data_has_children_information(
    vtkNew<vtkOverlappingAMRMetaData> self
) {
    return self->HasChildrenInformation();
}
extern "C" void vtk_overlapping_amr_meta_data_print_parent_child_info(
    vtkNew<vtkOverlappingAMRMetaData> self, unsigned int level, unsigned int index
) {
    return self->PrintParentChildInfo(level, index);
}
extern "C" void vtk_overlapping_amr_meta_data_generate_parent_child_information(
    vtkNew<vtkOverlappingAMRMetaData> self
) {
    return self->GenerateParentChildInformation();
}
extern "C" bool vtk_overlapping_amr_meta_data_check_validity(
    vtkNew<vtkOverlappingAMRMetaData> self
) {
    return self->CheckValidity();
}
extern "C" unsigned int vtk_overlapping_amr_meta_data_get_grid_type(
    vtkNew<vtkOverlappingAMRMetaData> self
) {
    return self->GetGridType();
}
extern "C" void vtk_overlapping_amr_meta_data_set_grid_type(
    vtkNew<vtkOverlappingAMRMetaData> self, unsigned int _arg
) {
    return self->SetGridType(_arg);
}
extern "C" vtkNew<vtkPartitionedDataSet> vtkPartitionedDataSet_new() {
    return vtkNew<vtkPartitionedDataSet>();
}
extern "C" void vtkPartitionedDataSet_destructor(vtkNew<vtkPartitionedDataSet> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPartitionedDataSet_get_ptr(vtkNew<vtkPartitionedDataSet> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_partitioned_data_set_set_number_of_partitions(
    vtkNew<vtkPartitionedDataSet> self, unsigned int numPartitions
) {
    return self->SetNumberOfPartitions(numPartitions);
}
extern "C" unsigned int vtk_partitioned_data_set_get_number_of_partitions(
    vtkNew<vtkPartitionedDataSet> self
) {
    return self->GetNumberOfPartitions();
}
extern "C" int vtk_partitioned_data_set_has_meta_data_c_uint(
    vtkNew<vtkPartitionedDataSet> self, unsigned int idx
) {
    return self->HasMetaData(idx);
}
extern "C" void vtk_partitioned_data_set_remove_null_partitions(
    vtkNew<vtkPartitionedDataSet> self
) {
    return self->RemoveNullPartitions();
}
extern "C" vtkNew<vtkPartitionedDataSetCollection> vtkPartitionedDataSetCollection_new() {
    return vtkNew<vtkPartitionedDataSetCollection>();
}
extern "C" void vtkPartitionedDataSetCollection_destructor(
    vtkNew<vtkPartitionedDataSetCollection> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkPartitionedDataSetCollection_get_ptr(
    vtkNew<vtkPartitionedDataSetCollection> sself
) {
    return sself.GetPointer();
}
extern "C" void vtk_partitioned_data_set_collection_set_number_of_partitioned_data_sets(
    vtkNew<vtkPartitionedDataSetCollection> self, unsigned int numDataSets
) {
    return self->SetNumberOfPartitionedDataSets(numDataSets);
}
extern "C" unsigned int vtk_partitioned_data_set_collection_get_number_of_partitioned_data_sets(
    vtkNew<vtkPartitionedDataSetCollection> self
) {
    return self->GetNumberOfPartitionedDataSets();
}
extern "C" void vtk_partitioned_data_set_collection_remove_partitioned_data_set(
    vtkNew<vtkPartitionedDataSetCollection> self, unsigned int idx
) {
    return self->RemovePartitionedDataSet(idx);
}
extern "C" unsigned int vtk_partitioned_data_set_collection_get_number_of_partitions(
    vtkNew<vtkPartitionedDataSetCollection> self, unsigned int idx
) {
    return self->GetNumberOfPartitions(idx);
}
extern "C" void vtk_partitioned_data_set_collection_set_number_of_partitions(
    vtkNew<vtkPartitionedDataSetCollection> self, unsigned int idx, unsigned int numPartitions
) {
    return self->SetNumberOfPartitions(idx, numPartitions);
}
extern "C" int vtk_partitioned_data_set_collection_has_meta_data_c_uint(
    vtkNew<vtkPartitionedDataSetCollection> self, unsigned int idx
) {
    return self->HasMetaData(idx);
}
extern "C" unsigned int vtk_partitioned_data_set_collection_get_composite_index_c_uint(
    vtkNew<vtkPartitionedDataSetCollection> self, unsigned int idx
) {
    return self->GetCompositeIndex(idx);
}
extern "C" unsigned int vtk_partitioned_data_set_collection_get_composite_index_c_uint_c_uint(
    vtkNew<vtkPartitionedDataSetCollection> self, unsigned int idx, unsigned int partition
) {
    return self->GetCompositeIndex(idx, partition);
}
extern "C" unsigned long vtk_partitioned_data_set_collection_get_m_time(
    vtkNew<vtkPartitionedDataSetCollection> self
) {
    return self->GetMTime();
}
extern "C" vtkNew<vtkPath> vtkPath_new() {
    return vtkNew<vtkPath>();
}
extern "C" void vtkPath_destructor(vtkNew<vtkPath> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPath_get_ptr(vtkNew<vtkPath> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_path_get_data_object_type(vtkNew<vtkPath> self) {
    return self->GetDataObjectType();
}
extern "C" void vtk_path_insert_next_point_float_3_c_int(
    vtkNew<vtkPath> self, float* pts, int code
) {
    return self->InsertNextPoint(pts, code);
}
extern "C" void vtk_path_insert_next_point_double_3_c_int(
    vtkNew<vtkPath> self, double* pts, int code
) {
    return self->InsertNextPoint(pts, code);
}
extern "C" void vtk_path_insert_next_point_double_double_double_c_int(
    vtkNew<vtkPath> self, double x, double y, double z, int code
) {
    return self->InsertNextPoint(x, y, z, code);
}
extern "C" long long vtk_path_get_number_of_cells(vtkNew<vtkPath> self) {
    return self->GetNumberOfCells();
}
extern "C" int vtk_path_get_max_cell_size(vtkNew<vtkPath> self) {
    return self->GetMaxCellSize();
}
extern "C" int vtk_path_get_max_spatial_dimension(vtkNew<vtkPath> self) {
    return self->GetMaxSpatialDimension();
}
extern "C" void vtk_path_allocate(vtkNew<vtkPath> self, long long size, int extSize) {
    return self->Allocate(size, extSize);
}
extern "C" void vtk_path_reset(vtkNew<vtkPath> self) {
    return self->Reset();
}
extern "C" vtkNew<vtkPentagonalPrism> vtkPentagonalPrism_new() {
    return vtkNew<vtkPentagonalPrism>();
}
extern "C" void vtkPentagonalPrism_destructor(vtkNew<vtkPentagonalPrism> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPentagonalPrism_get_ptr(vtkNew<vtkPentagonalPrism> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_pentagonal_prism_get_centroid(
    vtkNew<vtkPentagonalPrism> self, double* centroid
) {
    return self->GetCentroid(centroid);
}
extern "C" int vtk_pentagonal_prism_get_cell_type(vtkNew<vtkPentagonalPrism> self) {
    return self->GetCellType();
}
extern "C" int vtk_pentagonal_prism_get_number_of_edges(vtkNew<vtkPentagonalPrism> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_pentagonal_prism_get_number_of_faces(vtkNew<vtkPentagonalPrism> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_pentagonal_prism_get_parametric_center(
    vtkNew<vtkPentagonalPrism> self, double* pcoords
) {
    return self->GetParametricCenter(pcoords);
}
extern "C" void vtk_pentagonal_prism_interpolation_functions(
    vtkNew<vtkPentagonalPrism> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_pentagonal_prism_interpolation_derivs(
    vtkNew<vtkPentagonalPrism> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_pentagonal_prism_interpolate_functions(
    vtkNew<vtkPentagonalPrism> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_pentagonal_prism_interpolate_derivs(
    vtkNew<vtkPentagonalPrism> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkPerlinNoise> vtkPerlinNoise_new() {
    return vtkNew<vtkPerlinNoise>();
}
extern "C" void vtkPerlinNoise_destructor(vtkNew<vtkPerlinNoise> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPerlinNoise_get_ptr(vtkNew<vtkPerlinNoise> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_perlin_noise_evaluate_function(vtkNew<vtkPerlinNoise> self, double* x) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_perlin_noise_evaluate_gradient(
    vtkNew<vtkPerlinNoise> self, double* x, double* n
) {
    return self->EvaluateGradient(x, n);
}
extern "C" void vtk_perlin_noise_set_frequency_double_double_double(
    vtkNew<vtkPerlinNoise> self, double _arg1, double _arg2, double _arg3
) {
    return self->SetFrequency(_arg1, _arg2, _arg3);
}
extern "C" void vtk_perlin_noise_set_frequency_const_double_3(
    vtkNew<vtkPerlinNoise> self, const double* _arg
) {
    return self->SetFrequency(_arg);
}
extern "C" void vtk_perlin_noise_get_frequency_double_3(vtkNew<vtkPerlinNoise> self, double* data) {
    return self->GetFrequency(data);
}
extern "C" void vtk_perlin_noise_set_phase_double_double_double(
    vtkNew<vtkPerlinNoise> self, double _arg1, double _arg2, double _arg3
) {
    return self->SetPhase(_arg1, _arg2, _arg3);
}
extern "C" void vtk_perlin_noise_set_phase_const_double_3(
    vtkNew<vtkPerlinNoise> self, const double* _arg
) {
    return self->SetPhase(_arg);
}
extern "C" void vtk_perlin_noise_get_phase_double_3(vtkNew<vtkPerlinNoise> self, double* data) {
    return self->GetPhase(data);
}
extern "C" void vtk_perlin_noise_set_amplitude(vtkNew<vtkPerlinNoise> self, double _arg) {
    return self->SetAmplitude(_arg);
}
extern "C" double vtk_perlin_noise_get_amplitude(vtkNew<vtkPerlinNoise> self) {
    return self->GetAmplitude();
}
extern "C" vtkNew<vtkPiecewiseFunction> vtkPiecewiseFunction_new() {
    return vtkNew<vtkPiecewiseFunction>();
}
extern "C" void vtkPiecewiseFunction_destructor(vtkNew<vtkPiecewiseFunction> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPiecewiseFunction_get_ptr(vtkNew<vtkPiecewiseFunction> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_piecewise_function_get_data_object_type(vtkNew<vtkPiecewiseFunction> self) {
    return self->GetDataObjectType();
}
extern "C" int vtk_piecewise_function_get_size(vtkNew<vtkPiecewiseFunction> self) {
    return self->GetSize();
}
extern "C" int vtk_piecewise_function_add_point_double_double(
    vtkNew<vtkPiecewiseFunction> self, double x, double y
) {
    return self->AddPoint(x, y);
}
extern "C" int vtk_piecewise_function_add_point_double_double_double_double(
    vtkNew<vtkPiecewiseFunction> self, double x, double y, double midpoint, double sharpness
) {
    return self->AddPoint(x, y, midpoint, sharpness);
}
extern "C" bool vtk_piecewise_function_remove_point_by_index(
    vtkNew<vtkPiecewiseFunction> self, size_t id
) {
    return self->RemovePointByIndex(id);
}
extern "C" int vtk_piecewise_function_remove_point_double(
    vtkNew<vtkPiecewiseFunction> self, double x
) {
    return self->RemovePoint(x);
}
extern "C" int vtk_piecewise_function_remove_point_double_double(
    vtkNew<vtkPiecewiseFunction> self, double x, double y
) {
    return self->RemovePoint(x, y);
}
extern "C" void vtk_piecewise_function_remove_all_points(vtkNew<vtkPiecewiseFunction> self) {
    return self->RemoveAllPoints();
}
extern "C" void vtk_piecewise_function_add_segment(
    vtkNew<vtkPiecewiseFunction> self, double x1, double y1, double x2, double y2
) {
    return self->AddSegment(x1, y1, x2, y2);
}
extern "C" double vtk_piecewise_function_get_value(vtkNew<vtkPiecewiseFunction> self, double x) {
    return self->GetValue(x);
}
extern "C" int vtk_piecewise_function_get_node_value(
    vtkNew<vtkPiecewiseFunction> self, int index, double* val
) {
    return self->GetNodeValue(index, val);
}
extern "C" int vtk_piecewise_function_set_node_value(
    vtkNew<vtkPiecewiseFunction> self, int index, double* val
) {
    return self->SetNodeValue(index, val);
}
extern "C" void vtk_piecewise_function_get_range_double_2(
    vtkNew<vtkPiecewiseFunction> self, double* _arg
) {
    return self->GetRange(_arg);
}
extern "C" int vtk_piecewise_function_adjust_range(
    vtkNew<vtkPiecewiseFunction> self, double* range
) {
    return self->AdjustRange(range);
}
extern "C" void vtk_piecewise_function_set_clamping(vtkNew<vtkPiecewiseFunction> self, int _arg) {
    return self->SetClamping(_arg);
}
extern "C" int vtk_piecewise_function_get_clamping(vtkNew<vtkPiecewiseFunction> self) {
    return self->GetClamping();
}
extern "C" void vtk_piecewise_function_clamping_on(vtkNew<vtkPiecewiseFunction> self) {
    return self->ClampingOn();
}
extern "C" void vtk_piecewise_function_clamping_off(vtkNew<vtkPiecewiseFunction> self) {
    return self->ClampingOff();
}
extern "C" void vtk_piecewise_function_set_use_log_scale(
    vtkNew<vtkPiecewiseFunction> self, bool _arg
) {
    return self->SetUseLogScale(_arg);
}
extern "C" bool vtk_piecewise_function_get_use_log_scale(vtkNew<vtkPiecewiseFunction> self) {
    return self->GetUseLogScale();
}
extern "C" void vtk_piecewise_function_use_log_scale_on(vtkNew<vtkPiecewiseFunction> self) {
    return self->UseLogScaleOn();
}
extern "C" void vtk_piecewise_function_use_log_scale_off(vtkNew<vtkPiecewiseFunction> self) {
    return self->UseLogScaleOff();
}
extern "C" double vtk_piecewise_function_get_first_non_zero_value(
    vtkNew<vtkPiecewiseFunction> self
) {
    return self->GetFirstNonZeroValue();
}
extern "C" void vtk_piecewise_function_initialize(vtkNew<vtkPiecewiseFunction> self) {
    return self->Initialize();
}
extern "C" void vtk_piecewise_function_set_allow_duplicate_scalars(
    vtkNew<vtkPiecewiseFunction> self, int _arg
) {
    return self->SetAllowDuplicateScalars(_arg);
}
extern "C" int vtk_piecewise_function_get_allow_duplicate_scalars(
    vtkNew<vtkPiecewiseFunction> self
) {
    return self->GetAllowDuplicateScalars();
}
extern "C" void vtk_piecewise_function_allow_duplicate_scalars_on(
    vtkNew<vtkPiecewiseFunction> self
) {
    return self->AllowDuplicateScalarsOn();
}
extern "C" void vtk_piecewise_function_allow_duplicate_scalars_off(
    vtkNew<vtkPiecewiseFunction> self
) {
    return self->AllowDuplicateScalarsOff();
}
extern "C" void vtk_piecewise_function_update_search_method(
    vtkNew<vtkPiecewiseFunction> self, double epsilon, double thresh
) {
    return self->UpdateSearchMethod(epsilon, thresh);
}
extern "C" int vtk_piecewise_function_get_automatic_search_method(
    vtkNew<vtkPiecewiseFunction> self
) {
    return self->GetAutomaticSearchMethod();
}
extern "C" void vtk_piecewise_function_set_use_custom_search_method(
    vtkNew<vtkPiecewiseFunction> self, bool use
) {
    return self->SetUseCustomSearchMethod(use);
}
extern "C" void vtk_piecewise_function_set_custom_search_method(
    vtkNew<vtkPiecewiseFunction> self, int type
) {
    return self->SetCustomSearchMethod(type);
}
extern "C" int vtk_piecewise_function_get_custom_search_method(vtkNew<vtkPiecewiseFunction> self) {
    return self->GetCustomSearchMethod();
}
extern "C" vtkNew<vtkPixel> vtkPixel_new() {
    return vtkNew<vtkPixel>();
}
extern "C" void vtkPixel_destructor(vtkNew<vtkPixel> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPixel_get_ptr(vtkNew<vtkPixel> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_pixel_get_cell_type(vtkNew<vtkPixel> self) {
    return self->GetCellType();
}
extern "C" int vtk_pixel_get_cell_dimension(vtkNew<vtkPixel> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_pixel_get_number_of_edges(vtkNew<vtkPixel> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_pixel_get_number_of_faces(vtkNew<vtkPixel> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_pixel_inflate(vtkNew<vtkPixel> self, double dist) {
    return self->Inflate(dist);
}
extern "C" double vtk_pixel_compute_bounding_sphere(vtkNew<vtkPixel> self, double* center) {
    return self->ComputeBoundingSphere(center);
}
extern "C" int vtk_pixel_get_parametric_center(vtkNew<vtkPixel> self, double* pcoords) {
    return self->GetParametricCenter(pcoords);
}
extern "C" void vtk_pixel_interpolation_functions(
    vtkNew<vtkPixel> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_pixel_interpolation_derivs(
    vtkNew<vtkPixel> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_pixel_interpolate_functions(
    vtkNew<vtkPixel> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_pixel_interpolate_derivs(
    vtkNew<vtkPixel> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" int vtk_pixel_compute_normal(vtkNew<vtkPixel> self, double* n) {
    return self->ComputeNormal(n);
}
extern "C" vtkNew<vtkPlane> vtkPlane_new() {
    return vtkNew<vtkPlane>();
}
extern "C" void vtkPlane_destructor(vtkNew<vtkPlane> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPlane_get_ptr(vtkNew<vtkPlane> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_plane_evaluate_function(vtkNew<vtkPlane> self, double* x) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_plane_evaluate_gradient(vtkNew<vtkPlane> self, double* x, double* g) {
    return self->EvaluateGradient(x, g);
}
extern "C" void vtk_plane_set_normal_double_double_double(
    vtkNew<vtkPlane> self, double x, double y, double z
) {
    return self->SetNormal(x, y, z);
}
extern "C" void vtk_plane_set_normal_const_double_3(vtkNew<vtkPlane> self, const double* normal) {
    return self->SetNormal(normal);
}
extern "C" void vtk_plane_get_normal_double_3(vtkNew<vtkPlane> self, double* data) {
    return self->GetNormal(data);
}
extern "C" void vtk_plane_set_origin_double_double_double(
    vtkNew<vtkPlane> self, double x, double y, double z
) {
    return self->SetOrigin(x, y, z);
}
extern "C" void vtk_plane_set_origin_const_double_3(vtkNew<vtkPlane> self, const double* origin) {
    return self->SetOrigin(origin);
}
extern "C" void vtk_plane_get_origin_double_3(vtkNew<vtkPlane> self, double* data) {
    return self->GetOrigin(data);
}
extern "C" void vtk_plane_set_offset(vtkNew<vtkPlane> self, double _arg) {
    return self->SetOffset(_arg);
}
extern "C" double vtk_plane_get_offset(vtkNew<vtkPlane> self) {
    return self->GetOffset();
}
extern "C" void vtk_plane_set_axis_aligned(vtkNew<vtkPlane> self, bool _arg) {
    return self->SetAxisAligned(_arg);
}
extern "C" bool vtk_plane_get_axis_aligned(vtkNew<vtkPlane> self) {
    return self->GetAxisAligned();
}
extern "C" void vtk_plane_push(vtkNew<vtkPlane> self, double distance) {
    return self->Push(distance);
}
extern "C" void vtk_plane_project_point_const_double_3_const_double_3_const_double_3_double_3(
    vtkNew<vtkPlane> self,
    const double* x,
    const double* origin,
    const double* normal,
    double* xproj
) {
    return self->ProjectPoint(x, origin, normal, xproj);
}
extern "C" void vtk_plane_project_point_const_double_3_double_3(
    vtkNew<vtkPlane> self, const double* x, double* xproj
) {
    return self->ProjectPoint(x, xproj);
}
extern "C" void vtk_plane_project_vector_const_double_3_const_double_3_const_double_3_double_3(
    vtkNew<vtkPlane> self,
    const double* v,
    const double* origin,
    const double* normal,
    double* vproj
) {
    return self->ProjectVector(v, origin, normal, vproj);
}
extern "C" void vtk_plane_project_vector_const_double_3_double_3(
    vtkNew<vtkPlane> self, const double* v, double* vproj
) {
    return self->ProjectVector(v, vproj);
}
extern "C" void vtk_plane_generalized_project_point_const_double_3_const_double_3_const_double_3_double_3(
    vtkNew<vtkPlane> self,
    const double* x,
    const double* origin,
    const double* normal,
    double* xproj
) {
    return self->GeneralizedProjectPoint(x, origin, normal, xproj);
}
extern "C" void vtk_plane_generalized_project_point_const_double_3_double_3(
    vtkNew<vtkPlane> self, const double* x, double* xproj
) {
    return self->GeneralizedProjectPoint(x, xproj);
}
extern "C" double vtk_plane_evaluate(
    vtkNew<vtkPlane> self, double* normal, double* origin, double* x
) {
    return self->Evaluate(normal, origin, x);
}
extern "C" double vtk_plane_distance_to_plane_double_3_double_3_double_3(
    vtkNew<vtkPlane> self, double* x, double* n, double* p0
) {
    return self->DistanceToPlane(x, n, p0);
}
extern "C" double vtk_plane_distance_to_plane_double_3(vtkNew<vtkPlane> self, double* x) {
    return self->DistanceToPlane(x);
}
extern "C" int vtk_plane_intersect_with_finite_plane_double_3_double_3_double_3_double_3_double_3_double_3_double_3(
    vtkNew<vtkPlane> self,
    double* n,
    double* o,
    double* pOrigin,
    double* px,
    double* py,
    double* x0,
    double* x1
) {
    return self->IntersectWithFinitePlane(n, o, pOrigin, px, py, x0, x1);
}
extern "C" int vtk_plane_intersect_with_finite_plane_double_3_double_3_double_3_double_3_double_3(
    vtkNew<vtkPlane> self, double* pOrigin, double* px, double* py, double* x0, double* x1
) {
    return self->IntersectWithFinitePlane(pOrigin, px, py, x0, x1);
}
extern "C" vtkNew<vtkPlaneCollection> vtkPlaneCollection_new() {
    return vtkNew<vtkPlaneCollection>();
}
extern "C" void vtkPlaneCollection_destructor(vtkNew<vtkPlaneCollection> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPlaneCollection_get_ptr(vtkNew<vtkPlaneCollection> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_plane_collection_get_number_of_items(vtkNew<vtkPlaneCollection> self) {
    return self->GetNumberOfItems();
}
extern "C" vtkNew<vtkPlanes> vtkPlanes_new() {
    return vtkNew<vtkPlanes>();
}
extern "C" void vtkPlanes_destructor(vtkNew<vtkPlanes> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPlanes_get_ptr(vtkNew<vtkPlanes> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_planes_evaluate_function(vtkNew<vtkPlanes> self, double* x) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_planes_evaluate_gradient(vtkNew<vtkPlanes> self, double* x, double* n) {
    return self->EvaluateGradient(x, n);
}
extern "C" void vtk_planes_set_frustum_planes(vtkNew<vtkPlanes> self, double* planes) {
    return self->SetFrustumPlanes(planes);
}
extern "C" void vtk_planes_set_bounds_const_double_6(vtkNew<vtkPlanes> self, const double* bounds) {
    return self->SetBounds(bounds);
}
extern "C" void vtk_planes_set_bounds_double_double_double_double_double_double(
    vtkNew<vtkPlanes> self,
    double xmin,
    double xmax,
    double ymin,
    double ymax,
    double zmin,
    double zmax
) {
    return self->SetBounds(xmin, xmax, ymin, ymax, zmin, zmax);
}
extern "C" int vtk_planes_get_number_of_planes(vtkNew<vtkPlanes> self) {
    return self->GetNumberOfPlanes();
}
extern "C" vtkNew<vtkPlanesIntersection> vtkPlanesIntersection_new() {
    return vtkNew<vtkPlanesIntersection>();
}
extern "C" void vtkPlanesIntersection_destructor(vtkNew<vtkPlanesIntersection> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPlanesIntersection_get_ptr(vtkNew<vtkPlanesIntersection> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_planes_intersection_get_number_of_region_vertices(
    vtkNew<vtkPlanesIntersection> self
) {
    return self->GetNumberOfRegionVertices();
}
extern "C" int vtk_planes_intersection_get_num_region_vertices(vtkNew<vtkPlanesIntersection> self) {
    return self->GetNumRegionVertices();
}
extern "C" vtkNew<vtkPointData> vtkPointData_new() {
    return vtkNew<vtkPointData>();
}
extern "C" void vtkPointData_destructor(vtkNew<vtkPointData> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPointData_get_ptr(vtkNew<vtkPointData> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkPointLocator> vtkPointLocator_new() {
    return vtkNew<vtkPointLocator>();
}
extern "C" void vtkPointLocator_destructor(vtkNew<vtkPointLocator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPointLocator_get_ptr(vtkNew<vtkPointLocator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_point_locator_set_divisions_c_int_c_int_c_int(
    vtkNew<vtkPointLocator> self, int _arg1, int _arg2, int _arg3
) {
    return self->SetDivisions(_arg1, _arg2, _arg3);
}
extern "C" void vtk_point_locator_set_divisions_const_c_int_3(
    vtkNew<vtkPointLocator> self, const int* _arg
) {
    return self->SetDivisions(_arg);
}
extern "C" void vtk_point_locator_get_divisions_c_int_3(vtkNew<vtkPointLocator> self, int* data) {
    return self->GetDivisions(data);
}
extern "C" void vtk_point_locator_set_number_of_points_per_bucket(
    vtkNew<vtkPointLocator> self, int _arg
) {
    return self->SetNumberOfPointsPerBucket(_arg);
}
extern "C" int vtk_point_locator_get_number_of_points_per_bucket_min_value(
    vtkNew<vtkPointLocator> self
) {
    return self->GetNumberOfPointsPerBucketMinValue();
}
extern "C" int vtk_point_locator_get_number_of_points_per_bucket_max_value(
    vtkNew<vtkPointLocator> self
) {
    return self->GetNumberOfPointsPerBucketMaxValue();
}
extern "C" int vtk_point_locator_get_number_of_points_per_bucket(vtkNew<vtkPointLocator> self) {
    return self->GetNumberOfPointsPerBucket();
}
extern "C" long long vtk_point_locator_find_closest_point(
    vtkNew<vtkPointLocator> self, const double* x
) {
    return self->FindClosestPoint(x);
}
extern "C" void vtk_point_locator_insert_point(
    vtkNew<vtkPointLocator> self, long long ptId, const double* x
) {
    return self->InsertPoint(ptId, x);
}
extern "C" long long vtk_point_locator_insert_next_point(
    vtkNew<vtkPointLocator> self, const double* x
) {
    return self->InsertNextPoint(x);
}
extern "C" long long vtk_point_locator_is_inserted_point_double_double_double(
    vtkNew<vtkPointLocator> self, double x, double y, double z
) {
    return self->IsInsertedPoint(x, y, z);
}
extern "C" long long vtk_point_locator_is_inserted_point_const_double_3(
    vtkNew<vtkPointLocator> self, const double* x
) {
    return self->IsInsertedPoint(x);
}
extern "C" long long vtk_point_locator_find_closest_inserted_point(
    vtkNew<vtkPointLocator> self, const double* x
) {
    return self->FindClosestInsertedPoint(x);
}
extern "C" void vtk_point_locator_initialize(vtkNew<vtkPointLocator> self) {
    return self->Initialize();
}
extern "C" void vtk_point_locator_free_search_structure(vtkNew<vtkPointLocator> self) {
    return self->FreeSearchStructure();
}
extern "C" void vtk_point_locator_build_locator(vtkNew<vtkPointLocator> self) {
    return self->BuildLocator();
}
extern "C" vtkNew<vtkPointSet> vtkPointSet_new() {
    return vtkNew<vtkPointSet>();
}
extern "C" void vtkPointSet_destructor(vtkNew<vtkPointSet> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPointSet_get_ptr(vtkNew<vtkPointSet> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_point_set_get_data_object_type(vtkNew<vtkPointSet> self) {
    return self->GetDataObjectType();
}
extern "C" void vtk_point_set_set_editable(vtkNew<vtkPointSet> self, bool _arg) {
    return self->SetEditable(_arg);
}
extern "C" bool vtk_point_set_get_editable(vtkNew<vtkPointSet> self) {
    return self->GetEditable();
}
extern "C" void vtk_point_set_editable_on(vtkNew<vtkPointSet> self) {
    return self->EditableOn();
}
extern "C" void vtk_point_set_editable_off(vtkNew<vtkPointSet> self) {
    return self->EditableOff();
}
extern "C" void vtk_point_set_initialize(vtkNew<vtkPointSet> self) {
    return self->Initialize();
}
extern "C" long long vtk_point_set_get_number_of_points(vtkNew<vtkPointSet> self) {
    return self->GetNumberOfPoints();
}
extern "C" void vtk_point_set_get_point_c_longlong_double_3(
    vtkNew<vtkPointSet> self, long long ptId, double* x
) {
    return self->GetPoint(ptId, x);
}
extern "C" long long vtk_point_set_find_point(vtkNew<vtkPointSet> self, double* x) {
    return self->FindPoint(x);
}
extern "C" long long vtk_point_set_get_number_of_cells(vtkNew<vtkPointSet> self) {
    return self->GetNumberOfCells();
}
extern "C" int vtk_point_set_get_max_cell_size(vtkNew<vtkPointSet> self) {
    return self->GetMaxCellSize();
}
extern "C" int vtk_point_set_get_cell_type(vtkNew<vtkPointSet> self, long long p0) {
    return self->GetCellType(p0);
}
extern "C" long long vtk_point_set_get_cell_size(vtkNew<vtkPointSet> self, long long p0) {
    return self->GetCellSize(p0);
}
extern "C" void vtk_point_set_build_point_locator(vtkNew<vtkPointSet> self) {
    return self->BuildPointLocator();
}
extern "C" void vtk_point_set_build_locator(vtkNew<vtkPointSet> self) {
    return self->BuildLocator();
}
extern "C" void vtk_point_set_build_cell_locator(vtkNew<vtkPointSet> self) {
    return self->BuildCellLocator();
}
extern "C" unsigned long vtk_point_set_get_m_time(vtkNew<vtkPointSet> self) {
    return self->GetMTime();
}
extern "C" void vtk_point_set_compute_bounds(vtkNew<vtkPointSet> self) {
    return self->ComputeBounds();
}
extern "C" void vtk_point_set_squeeze(vtkNew<vtkPointSet> self) {
    return self->Squeeze();
}
extern "C" bool vtk_point_set_uses_garbage_collector(vtkNew<vtkPointSet> self) {
    return self->UsesGarbageCollector();
}
extern "C" vtkNew<vtkPointSetCellIterator> vtkPointSetCellIterator_new() {
    return vtkNew<vtkPointSetCellIterator>();
}
extern "C" void vtkPointSetCellIterator_destructor(vtkNew<vtkPointSetCellIterator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPointSetCellIterator_get_ptr(vtkNew<vtkPointSetCellIterator> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_point_set_cell_iterator_is_done_with_traversal(
    vtkNew<vtkPointSetCellIterator> self
) {
    return self->IsDoneWithTraversal();
}
extern "C" long long vtk_point_set_cell_iterator_get_cell_id(vtkNew<vtkPointSetCellIterator> self) {
    return self->GetCellId();
}
extern "C" vtkNew<vtkPointsProjectedHull> vtkPointsProjectedHull_new() {
    return vtkNew<vtkPointsProjectedHull>();
}
extern "C" void vtkPointsProjectedHull_destructor(vtkNew<vtkPointsProjectedHull> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPointsProjectedHull_get_ptr(vtkNew<vtkPointsProjectedHull> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_points_projected_hull_rectangle_intersection_x_float_float_float_float(
    vtkNew<vtkPointsProjectedHull> self, float ymin, float ymax, float zmin, float zmax
) {
    return self->RectangleIntersectionX(ymin, ymax, zmin, zmax);
}
extern "C" int vtk_points_projected_hull_rectangle_intersection_x_double_double_double_double(
    vtkNew<vtkPointsProjectedHull> self, double ymin, double ymax, double zmin, double zmax
) {
    return self->RectangleIntersectionX(ymin, ymax, zmin, zmax);
}
extern "C" int vtk_points_projected_hull_rectangle_intersection_y_float_float_float_float(
    vtkNew<vtkPointsProjectedHull> self, float zmin, float zmax, float xmin, float xmax
) {
    return self->RectangleIntersectionY(zmin, zmax, xmin, xmax);
}
extern "C" int vtk_points_projected_hull_rectangle_intersection_y_double_double_double_double(
    vtkNew<vtkPointsProjectedHull> self, double zmin, double zmax, double xmin, double xmax
) {
    return self->RectangleIntersectionY(zmin, zmax, xmin, xmax);
}
extern "C" int vtk_points_projected_hull_rectangle_intersection_z_float_float_float_float(
    vtkNew<vtkPointsProjectedHull> self, float xmin, float xmax, float ymin, float ymax
) {
    return self->RectangleIntersectionZ(xmin, xmax, ymin, ymax);
}
extern "C" int vtk_points_projected_hull_rectangle_intersection_z_double_double_double_double(
    vtkNew<vtkPointsProjectedHull> self, double xmin, double xmax, double ymin, double ymax
) {
    return self->RectangleIntersectionZ(xmin, xmax, ymin, ymax);
}
extern "C" int vtk_points_projected_hull_get_size_ccw_hull_x(vtkNew<vtkPointsProjectedHull> self) {
    return self->GetSizeCCWHullX();
}
extern "C" int vtk_points_projected_hull_get_size_ccw_hull_y(vtkNew<vtkPointsProjectedHull> self) {
    return self->GetSizeCCWHullY();
}
extern "C" int vtk_points_projected_hull_get_size_ccw_hull_z(vtkNew<vtkPointsProjectedHull> self) {
    return self->GetSizeCCWHullZ();
}
extern "C" void vtk_points_projected_hull_update(vtkNew<vtkPointsProjectedHull> self) {
    return self->Update();
}
extern "C" vtkNew<vtkPolyData> vtkPolyData_new() {
    return vtkNew<vtkPolyData>();
}
extern "C" void vtkPolyData_destructor(vtkNew<vtkPolyData> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPolyData_get_ptr(vtkNew<vtkPolyData> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_poly_data_get_data_object_type(vtkNew<vtkPolyData> self) {
    return self->GetDataObjectType();
}
extern "C" long long vtk_poly_data_get_number_of_cells(vtkNew<vtkPolyData> self) {
    return self->GetNumberOfCells();
}
extern "C" int vtk_poly_data_get_cell_type(vtkNew<vtkPolyData> self, long long cellId) {
    return self->GetCellType(cellId);
}
extern "C" long long vtk_poly_data_get_cell_size(vtkNew<vtkPolyData> self, long long cellId) {
    return self->GetCellSize(cellId);
}
extern "C" void vtk_poly_data_compute_cells_bounds(vtkNew<vtkPolyData> self) {
    return self->ComputeCellsBounds();
}
extern "C" void vtk_poly_data_get_cells_bounds(vtkNew<vtkPolyData> self, double* bounds) {
    return self->GetCellsBounds(bounds);
}
extern "C" void vtk_poly_data_squeeze(vtkNew<vtkPolyData> self) {
    return self->Squeeze();
}
extern "C" int vtk_poly_data_get_max_cell_size(vtkNew<vtkPolyData> self) {
    return self->GetMaxCellSize();
}
extern "C" int vtk_poly_data_get_max_spatial_dimension(vtkNew<vtkPolyData> self) {
    return self->GetMaxSpatialDimension();
}
extern "C" long long vtk_poly_data_get_cell_id_relative_to_cell_array(
    vtkNew<vtkPolyData> self, long long cellId
) {
    return self->GetCellIdRelativeToCellArray(cellId);
}
extern "C" long long vtk_poly_data_get_number_of_verts(vtkNew<vtkPolyData> self) {
    return self->GetNumberOfVerts();
}
extern "C" long long vtk_poly_data_get_number_of_lines(vtkNew<vtkPolyData> self) {
    return self->GetNumberOfLines();
}
extern "C" long long vtk_poly_data_get_number_of_polys(vtkNew<vtkPolyData> self) {
    return self->GetNumberOfPolys();
}
extern "C" long long vtk_poly_data_get_number_of_strips(vtkNew<vtkPolyData> self) {
    return self->GetNumberOfStrips();
}
extern "C" bool vtk_poly_data_allocate_estimate_c_longlong_c_longlong(
    vtkNew<vtkPolyData> self, long long numCells, long long maxCellSize
) {
    return self->AllocateEstimate(numCells, maxCellSize);
}
extern "C" bool vtk_poly_data_allocate_estimate_c_longlong_c_longlong_c_longlong_c_longlong_c_longlong_c_longlong_c_longlong_c_longlong(
    vtkNew<vtkPolyData> self,
    long long numVerts,
    long long maxVertSize,
    long long numLines,
    long long maxLineSize,
    long long numPolys,
    long long maxPolySize,
    long long numStrips,
    long long maxStripSize
) {
    return self->AllocateEstimate(
        numVerts, maxVertSize, numLines, maxLineSize, numPolys, maxPolySize, numStrips, maxStripSize
    );
}
extern "C" bool vtk_poly_data_allocate_exact_c_longlong_c_longlong(
    vtkNew<vtkPolyData> self, long long numCells, long long connectivitySize
) {
    return self->AllocateExact(numCells, connectivitySize);
}
extern "C" bool vtk_poly_data_allocate_exact_c_longlong_c_longlong_c_longlong_c_longlong_c_longlong_c_longlong_c_longlong_c_longlong(
    vtkNew<vtkPolyData> self,
    long long numVerts,
    long long vertConnSize,
    long long numLines,
    long long lineConnSize,
    long long numPolys,
    long long polyConnSize,
    long long numStrips,
    long long stripConnSize
) {
    return self->AllocateExact(
        numVerts, vertConnSize, numLines, lineConnSize, numPolys, polyConnSize, numStrips,
        stripConnSize
    );
}
extern "C" void vtk_poly_data_allocate_c_longlong_c_int(
    vtkNew<vtkPolyData> self, long long numCells, int extSize
) {
    return self->Allocate(numCells, extSize);
}
extern "C" void vtk_poly_data_reset(vtkNew<vtkPolyData> self) {
    return self->Reset();
}
extern "C" void vtk_poly_data_build_cells(vtkNew<vtkPolyData> self) {
    return self->BuildCells();
}
extern "C" bool vtk_poly_data_need_to_build_cells(vtkNew<vtkPolyData> self) {
    return self->NeedToBuildCells();
}
extern "C" void vtk_poly_data_build_links(vtkNew<vtkPolyData> self, int initialSize) {
    return self->BuildLinks(initialSize);
}
extern "C" void vtk_poly_data_delete_cells(vtkNew<vtkPolyData> self) {
    return self->DeleteCells();
}
extern "C" void vtk_poly_data_delete_links(vtkNew<vtkPolyData> self) {
    return self->DeleteLinks();
}
extern "C" int vtk_poly_data_is_triangle(vtkNew<vtkPolyData> self, int v1, int v2, int v3) {
    return self->IsTriangle(v1, v2, v3);
}
extern "C" int vtk_poly_data_is_edge(vtkNew<vtkPolyData> self, long long p1, long long p2) {
    return self->IsEdge(p1, p2);
}
extern "C" int vtk_poly_data_is_point_used_by_cell(
    vtkNew<vtkPolyData> self, long long ptId, long long cellId
) {
    return self->IsPointUsedByCell(ptId, cellId);
}
extern "C" void vtk_poly_data_replace_cell_point_c_longlong_c_longlong_c_longlong(
    vtkNew<vtkPolyData> self, long long cellId, long long oldPtId, long long newPtId
) {
    return self->ReplaceCellPoint(cellId, oldPtId, newPtId);
}
extern "C" void vtk_poly_data_reverse_cell(vtkNew<vtkPolyData> self, long long cellId) {
    return self->ReverseCell(cellId);
}
extern "C" void vtk_poly_data_delete_point(vtkNew<vtkPolyData> self, long long ptId) {
    return self->DeletePoint(ptId);
}
extern "C" void vtk_poly_data_delete_cell(vtkNew<vtkPolyData> self, long long cellId) {
    return self->DeleteCell(cellId);
}
extern "C" void vtk_poly_data_remove_deleted_cells(vtkNew<vtkPolyData> self) {
    return self->RemoveDeletedCells();
}
extern "C" long long vtk_poly_data_insert_next_linked_point_c_int(
    vtkNew<vtkPolyData> self, int numLinks
) {
    return self->InsertNextLinkedPoint(numLinks);
}
extern "C" long long vtk_poly_data_insert_next_linked_point_double_3_c_int(
    vtkNew<vtkPolyData> self, double* x, int numLinks
) {
    return self->InsertNextLinkedPoint(x, numLinks);
}
extern "C" void vtk_poly_data_remove_cell_reference(vtkNew<vtkPolyData> self, long long cellId) {
    return self->RemoveCellReference(cellId);
}
extern "C" void vtk_poly_data_add_cell_reference(vtkNew<vtkPolyData> self, long long cellId) {
    return self->AddCellReference(cellId);
}
extern "C" void vtk_poly_data_remove_reference_to_cell(
    vtkNew<vtkPolyData> self, long long ptId, long long cellId
) {
    return self->RemoveReferenceToCell(ptId, cellId);
}
extern "C" void vtk_poly_data_add_reference_to_cell(
    vtkNew<vtkPolyData> self, long long ptId, long long cellId
) {
    return self->AddReferenceToCell(ptId, cellId);
}
extern "C" void vtk_poly_data_resize_cell_list(vtkNew<vtkPolyData> self, long long ptId, int size) {
    return self->ResizeCellList(ptId, size);
}
extern "C" void vtk_poly_data_initialize(vtkNew<vtkPolyData> self) {
    return self->Initialize();
}
extern "C" int vtk_poly_data_get_piece(vtkNew<vtkPolyData> self) {
    return self->GetPiece();
}
extern "C" int vtk_poly_data_get_number_of_pieces(vtkNew<vtkPolyData> self) {
    return self->GetNumberOfPieces();
}
extern "C" int vtk_poly_data_get_ghost_level(vtkNew<vtkPolyData> self) {
    return self->GetGhostLevel();
}
extern "C" void vtk_poly_data_remove_ghost_cells(vtkNew<vtkPolyData> self) {
    return self->RemoveGhostCells();
}
extern "C" int vtk_poly_data_get_scalar_field_critical_index_c_longlong_c_int(
    vtkNew<vtkPolyData> self, long long pointId, int fieldId
) {
    return self->GetScalarFieldCriticalIndex(pointId, fieldId);
}
extern "C" unsigned long vtk_poly_data_get_mesh_m_time(vtkNew<vtkPolyData> self) {
    return self->GetMeshMTime();
}
extern "C" unsigned long vtk_poly_data_get_m_time(vtkNew<vtkPolyData> self) {
    return self->GetMTime();
}
extern "C" vtkNew<vtkPolyDataCollection> vtkPolyDataCollection_new() {
    return vtkNew<vtkPolyDataCollection>();
}
extern "C" void vtkPolyDataCollection_destructor(vtkNew<vtkPolyDataCollection> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPolyDataCollection_get_ptr(vtkNew<vtkPolyDataCollection> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkPolyLine> vtkPolyLine_new() {
    return vtkNew<vtkPolyLine>();
}
extern "C" void vtkPolyLine_destructor(vtkNew<vtkPolyLine> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPolyLine_get_ptr(vtkNew<vtkPolyLine> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_poly_line_get_cell_type(vtkNew<vtkPolyLine> self) {
    return self->GetCellType();
}
extern "C" int vtk_poly_line_get_cell_dimension(vtkNew<vtkPolyLine> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_poly_line_get_number_of_edges(vtkNew<vtkPolyLine> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_poly_line_get_number_of_faces(vtkNew<vtkPolyLine> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_poly_line_get_parametric_center(vtkNew<vtkPolyLine> self, double* pcoords) {
    return self->GetParametricCenter(pcoords);
}
extern "C" vtkNew<vtkPolyPlane> vtkPolyPlane_new() {
    return vtkNew<vtkPolyPlane>();
}
extern "C" void vtkPolyPlane_destructor(vtkNew<vtkPolyPlane> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPolyPlane_get_ptr(vtkNew<vtkPolyPlane> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_poly_plane_evaluate_function(vtkNew<vtkPolyPlane> self, double* x) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_poly_plane_evaluate_gradient(vtkNew<vtkPolyPlane> self, double* x, double* g) {
    return self->EvaluateGradient(x, g);
}
extern "C" unsigned long vtk_poly_plane_get_m_time(vtkNew<vtkPolyPlane> self) {
    return self->GetMTime();
}
extern "C" vtkNew<vtkPolyVertex> vtkPolyVertex_new() {
    return vtkNew<vtkPolyVertex>();
}
extern "C" void vtkPolyVertex_destructor(vtkNew<vtkPolyVertex> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPolyVertex_get_ptr(vtkNew<vtkPolyVertex> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_poly_vertex_get_cell_type(vtkNew<vtkPolyVertex> self) {
    return self->GetCellType();
}
extern "C" int vtk_poly_vertex_get_cell_dimension(vtkNew<vtkPolyVertex> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_poly_vertex_get_number_of_edges(vtkNew<vtkPolyVertex> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_poly_vertex_get_number_of_faces(vtkNew<vtkPolyVertex> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_poly_vertex_get_parametric_center(vtkNew<vtkPolyVertex> self, double* pcoords) {
    return self->GetParametricCenter(pcoords);
}
extern "C" vtkNew<vtkPolygon> vtkPolygon_new() {
    return vtkNew<vtkPolygon>();
}
extern "C" void vtkPolygon_destructor(vtkNew<vtkPolygon> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPolygon_get_ptr(vtkNew<vtkPolygon> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_polygon_get_cell_type(vtkNew<vtkPolygon> self) {
    return self->GetCellType();
}
extern "C" int vtk_polygon_get_cell_dimension(vtkNew<vtkPolygon> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_polygon_get_number_of_edges(vtkNew<vtkPolygon> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_polygon_get_number_of_faces(vtkNew<vtkPolygon> self) {
    return self->GetNumberOfFaces();
}
extern "C" double vtk_polygon_compute_area(vtkNew<vtkPolygon> self) {
    return self->ComputeArea();
}
extern "C" bool vtk_polygon_is_convex(vtkNew<vtkPolygon> self) {
    return self->IsConvex();
}
extern "C" bool vtk_polygon_get_use_mvc_interpolation(vtkNew<vtkPolygon> self) {
    return self->GetUseMVCInterpolation();
}
extern "C" void vtk_polygon_set_use_mvc_interpolation(vtkNew<vtkPolygon> self, bool _arg) {
    return self->SetUseMVCInterpolation(_arg);
}
extern "C" void vtk_polygon_set_tolerance(vtkNew<vtkPolygon> self, double _arg) {
    return self->SetTolerance(_arg);
}
extern "C" double vtk_polygon_get_tolerance_min_value(vtkNew<vtkPolygon> self) {
    return self->GetToleranceMinValue();
}
extern "C" double vtk_polygon_get_tolerance_max_value(vtkNew<vtkPolygon> self) {
    return self->GetToleranceMaxValue();
}
extern "C" double vtk_polygon_get_tolerance(vtkNew<vtkPolygon> self) {
    return self->GetTolerance();
}
extern "C" int vtk_polygon_ear_cut_triangulation_c_int(vtkNew<vtkPolygon> self, int measure) {
    return self->EarCutTriangulation(measure);
}
extern "C" int vtk_polygon_unbiased_ear_cut_triangulation_c_int_c_int(
    vtkNew<vtkPolygon> self, int seed, int measure
) {
    return self->UnbiasedEarCutTriangulation(seed, measure);
}
extern "C" vtkNew<vtkPolyhedron> vtkPolyhedron_new() {
    return vtkNew<vtkPolyhedron>();
}
extern "C" void vtkPolyhedron_destructor(vtkNew<vtkPolyhedron> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPolyhedron_get_ptr(vtkNew<vtkPolyhedron> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_polyhedron_get_centroid(vtkNew<vtkPolyhedron> self, double* centroid) {
    return self->GetCentroid(centroid);
}
extern "C" int vtk_polyhedron_get_cell_type(vtkNew<vtkPolyhedron> self) {
    return self->GetCellType();
}
extern "C" int vtk_polyhedron_requires_initialization(vtkNew<vtkPolyhedron> self) {
    return self->RequiresInitialization();
}
extern "C" void vtk_polyhedron_initialize(vtkNew<vtkPolyhedron> self) {
    return self->Initialize();
}
extern "C" int vtk_polyhedron_get_number_of_edges(vtkNew<vtkPolyhedron> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_polyhedron_get_number_of_faces(vtkNew<vtkPolyhedron> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_polyhedron_get_parametric_center(vtkNew<vtkPolyhedron> self, double* pcoords) {
    return self->GetParametricCenter(pcoords);
}
extern "C" int vtk_polyhedron_is_primary_cell(vtkNew<vtkPolyhedron> self) {
    return self->IsPrimaryCell();
}
extern "C" int vtk_polyhedron_requires_explicit_face_representation(vtkNew<vtkPolyhedron> self) {
    return self->RequiresExplicitFaceRepresentation();
}
extern "C" int vtk_polyhedron_is_inside(
    vtkNew<vtkPolyhedron> self, const double* x, double tolerance
) {
    return self->IsInside(x, tolerance);
}
extern "C" bool vtk_polyhedron_is_convex(vtkNew<vtkPolyhedron> self) {
    return self->IsConvex();
}
extern "C" vtkNew<vtkPyramid> vtkPyramid_new() {
    return vtkNew<vtkPyramid>();
}
extern "C" void vtkPyramid_destructor(vtkNew<vtkPyramid> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPyramid_get_ptr(vtkNew<vtkPyramid> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_pyramid_get_centroid(vtkNew<vtkPyramid> self, double* centroid) {
    return self->GetCentroid(centroid);
}
extern "C" int vtk_pyramid_get_cell_type(vtkNew<vtkPyramid> self) {
    return self->GetCellType();
}
extern "C" int vtk_pyramid_get_number_of_edges(vtkNew<vtkPyramid> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_pyramid_get_number_of_faces(vtkNew<vtkPyramid> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_pyramid_get_parametric_center(vtkNew<vtkPyramid> self, double* pcoords) {
    return self->GetParametricCenter(pcoords);
}
extern "C" void vtk_pyramid_interpolation_functions(
    vtkNew<vtkPyramid> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_pyramid_interpolation_derivs(
    vtkNew<vtkPyramid> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_pyramid_interpolate_functions(
    vtkNew<vtkPyramid> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_pyramid_interpolate_derivs(
    vtkNew<vtkPyramid> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkQuad> vtkQuad_new() {
    return vtkNew<vtkQuad>();
}
extern "C" void vtkQuad_destructor(vtkNew<vtkQuad> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkQuad_get_ptr(vtkNew<vtkQuad> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_quad_get_cell_type(vtkNew<vtkQuad> self) {
    return self->GetCellType();
}
extern "C" int vtk_quad_get_cell_dimension(vtkNew<vtkQuad> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_quad_get_number_of_edges(vtkNew<vtkQuad> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_quad_get_number_of_faces(vtkNew<vtkQuad> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_quad_get_parametric_center(vtkNew<vtkQuad> self, double* pcoords) {
    return self->GetParametricCenter(pcoords);
}
extern "C" void vtk_quad_interpolation_functions(
    vtkNew<vtkQuad> self, const double* pcoords, double* sf
) {
    return self->InterpolationFunctions(pcoords, sf);
}
extern "C" void vtk_quad_interpolation_derivs(
    vtkNew<vtkQuad> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_quad_interpolate_functions(
    vtkNew<vtkQuad> self, const double* pcoords, double* sf
) {
    return self->InterpolateFunctions(pcoords, sf);
}
extern "C" void vtk_quad_interpolate_derivs(
    vtkNew<vtkQuad> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkQuadraticEdge> vtkQuadraticEdge_new() {
    return vtkNew<vtkQuadraticEdge>();
}
extern "C" void vtkQuadraticEdge_destructor(vtkNew<vtkQuadraticEdge> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkQuadraticEdge_get_ptr(vtkNew<vtkQuadraticEdge> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_quadratic_edge_get_cell_type(vtkNew<vtkQuadraticEdge> self) {
    return self->GetCellType();
}
extern "C" int vtk_quadratic_edge_get_cell_dimension(vtkNew<vtkQuadraticEdge> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_quadratic_edge_get_number_of_edges(vtkNew<vtkQuadraticEdge> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_quadratic_edge_get_number_of_faces(vtkNew<vtkQuadraticEdge> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_quadratic_edge_get_parametric_center(
    vtkNew<vtkQuadraticEdge> self, double* pcoords
) {
    return self->GetParametricCenter(pcoords);
}
extern "C" void vtk_quadratic_edge_interpolation_functions(
    vtkNew<vtkQuadraticEdge> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_edge_interpolation_derivs(
    vtkNew<vtkQuadraticEdge> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_quadratic_edge_interpolate_functions(
    vtkNew<vtkQuadraticEdge> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_edge_interpolate_derivs(
    vtkNew<vtkQuadraticEdge> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkQuadraticHexahedron> vtkQuadraticHexahedron_new() {
    return vtkNew<vtkQuadraticHexahedron>();
}
extern "C" void vtkQuadraticHexahedron_destructor(vtkNew<vtkQuadraticHexahedron> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkQuadraticHexahedron_get_ptr(vtkNew<vtkQuadraticHexahedron> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_quadratic_hexahedron_get_cell_type(vtkNew<vtkQuadraticHexahedron> self) {
    return self->GetCellType();
}
extern "C" int vtk_quadratic_hexahedron_get_cell_dimension(vtkNew<vtkQuadraticHexahedron> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_quadratic_hexahedron_get_number_of_edges(vtkNew<vtkQuadraticHexahedron> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_quadratic_hexahedron_get_number_of_faces(vtkNew<vtkQuadraticHexahedron> self) {
    return self->GetNumberOfFaces();
}
extern "C" void vtk_quadratic_hexahedron_interpolation_functions(
    vtkNew<vtkQuadraticHexahedron> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_hexahedron_interpolation_derivs(
    vtkNew<vtkQuadraticHexahedron> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_quadratic_hexahedron_interpolate_functions(
    vtkNew<vtkQuadraticHexahedron> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_hexahedron_interpolate_derivs(
    vtkNew<vtkQuadraticHexahedron> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkQuadraticLinearQuad> vtkQuadraticLinearQuad_new() {
    return vtkNew<vtkQuadraticLinearQuad>();
}
extern "C" void vtkQuadraticLinearQuad_destructor(vtkNew<vtkQuadraticLinearQuad> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkQuadraticLinearQuad_get_ptr(vtkNew<vtkQuadraticLinearQuad> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_quadratic_linear_quad_get_cell_type(vtkNew<vtkQuadraticLinearQuad> self) {
    return self->GetCellType();
}
extern "C" int vtk_quadratic_linear_quad_get_cell_dimension(vtkNew<vtkQuadraticLinearQuad> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_quadratic_linear_quad_get_number_of_edges(vtkNew<vtkQuadraticLinearQuad> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_quadratic_linear_quad_get_number_of_faces(vtkNew<vtkQuadraticLinearQuad> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_quadratic_linear_quad_get_parametric_center(
    vtkNew<vtkQuadraticLinearQuad> self, double* pcoords
) {
    return self->GetParametricCenter(pcoords);
}
extern "C" void vtk_quadratic_linear_quad_interpolation_functions(
    vtkNew<vtkQuadraticLinearQuad> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_linear_quad_interpolation_derivs(
    vtkNew<vtkQuadraticLinearQuad> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_quadratic_linear_quad_interpolate_functions(
    vtkNew<vtkQuadraticLinearQuad> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_linear_quad_interpolate_derivs(
    vtkNew<vtkQuadraticLinearQuad> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkQuadraticLinearWedge> vtkQuadraticLinearWedge_new() {
    return vtkNew<vtkQuadraticLinearWedge>();
}
extern "C" void vtkQuadraticLinearWedge_destructor(vtkNew<vtkQuadraticLinearWedge> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkQuadraticLinearWedge_get_ptr(vtkNew<vtkQuadraticLinearWedge> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_quadratic_linear_wedge_get_cell_type(vtkNew<vtkQuadraticLinearWedge> self) {
    return self->GetCellType();
}
extern "C" int vtk_quadratic_linear_wedge_get_cell_dimension(vtkNew<vtkQuadraticLinearWedge> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_quadratic_linear_wedge_get_number_of_edges(
    vtkNew<vtkQuadraticLinearWedge> self
) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_quadratic_linear_wedge_get_number_of_faces(
    vtkNew<vtkQuadraticLinearWedge> self
) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_quadratic_linear_wedge_get_parametric_center(
    vtkNew<vtkQuadraticLinearWedge> self, double* pcoords
) {
    return self->GetParametricCenter(pcoords);
}
extern "C" void vtk_quadratic_linear_wedge_interpolation_functions(
    vtkNew<vtkQuadraticLinearWedge> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_linear_wedge_interpolation_derivs(
    vtkNew<vtkQuadraticLinearWedge> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_quadratic_linear_wedge_interpolate_functions(
    vtkNew<vtkQuadraticLinearWedge> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_linear_wedge_interpolate_derivs(
    vtkNew<vtkQuadraticLinearWedge> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkQuadraticPolygon> vtkQuadraticPolygon_new() {
    return vtkNew<vtkQuadraticPolygon>();
}
extern "C" void vtkQuadraticPolygon_destructor(vtkNew<vtkQuadraticPolygon> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkQuadraticPolygon_get_ptr(vtkNew<vtkQuadraticPolygon> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_quadratic_polygon_get_cell_type(vtkNew<vtkQuadraticPolygon> self) {
    return self->GetCellType();
}
extern "C" int vtk_quadratic_polygon_get_cell_dimension(vtkNew<vtkQuadraticPolygon> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_quadratic_polygon_get_number_of_edges(vtkNew<vtkQuadraticPolygon> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_quadratic_polygon_get_number_of_faces(vtkNew<vtkQuadraticPolygon> self) {
    return self->GetNumberOfFaces();
}
extern "C" bool vtk_quadratic_polygon_get_use_mvc_interpolation(vtkNew<vtkQuadraticPolygon> self) {
    return self->GetUseMVCInterpolation();
}
extern "C" void vtk_quadratic_polygon_set_use_mvc_interpolation(
    vtkNew<vtkQuadraticPolygon> self, bool _arg
) {
    return self->SetUseMVCInterpolation(_arg);
}
extern "C" vtkNew<vtkQuadraticPyramid> vtkQuadraticPyramid_new() {
    return vtkNew<vtkQuadraticPyramid>();
}
extern "C" void vtkQuadraticPyramid_destructor(vtkNew<vtkQuadraticPyramid> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkQuadraticPyramid_get_ptr(vtkNew<vtkQuadraticPyramid> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_quadratic_pyramid_get_cell_type(vtkNew<vtkQuadraticPyramid> self) {
    return self->GetCellType();
}
extern "C" int vtk_quadratic_pyramid_get_cell_dimension(vtkNew<vtkQuadraticPyramid> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_quadratic_pyramid_get_number_of_edges(vtkNew<vtkQuadraticPyramid> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_quadratic_pyramid_get_number_of_faces(vtkNew<vtkQuadraticPyramid> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_quadratic_pyramid_get_parametric_center(
    vtkNew<vtkQuadraticPyramid> self, double* pcoords
) {
    return self->GetParametricCenter(pcoords);
}
extern "C" void vtk_quadratic_pyramid_interpolation_functions(
    vtkNew<vtkQuadraticPyramid> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_pyramid_interpolation_derivs(
    vtkNew<vtkQuadraticPyramid> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_quadratic_pyramid_interpolate_functions(
    vtkNew<vtkQuadraticPyramid> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_pyramid_interpolate_derivs(
    vtkNew<vtkQuadraticPyramid> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkQuadraticQuad> vtkQuadraticQuad_new() {
    return vtkNew<vtkQuadraticQuad>();
}
extern "C" void vtkQuadraticQuad_destructor(vtkNew<vtkQuadraticQuad> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkQuadraticQuad_get_ptr(vtkNew<vtkQuadraticQuad> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_quadratic_quad_get_cell_type(vtkNew<vtkQuadraticQuad> self) {
    return self->GetCellType();
}
extern "C" int vtk_quadratic_quad_get_cell_dimension(vtkNew<vtkQuadraticQuad> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_quadratic_quad_get_number_of_edges(vtkNew<vtkQuadraticQuad> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_quadratic_quad_get_number_of_faces(vtkNew<vtkQuadraticQuad> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_quadratic_quad_get_parametric_center(
    vtkNew<vtkQuadraticQuad> self, double* pcoords
) {
    return self->GetParametricCenter(pcoords);
}
extern "C" void vtk_quadratic_quad_interpolation_functions(
    vtkNew<vtkQuadraticQuad> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_quad_interpolation_derivs(
    vtkNew<vtkQuadraticQuad> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_quadratic_quad_interpolate_functions(
    vtkNew<vtkQuadraticQuad> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_quad_interpolate_derivs(
    vtkNew<vtkQuadraticQuad> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkQuadraticTetra> vtkQuadraticTetra_new() {
    return vtkNew<vtkQuadraticTetra>();
}
extern "C" void vtkQuadraticTetra_destructor(vtkNew<vtkQuadraticTetra> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkQuadraticTetra_get_ptr(vtkNew<vtkQuadraticTetra> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_quadratic_tetra_get_cell_type(vtkNew<vtkQuadraticTetra> self) {
    return self->GetCellType();
}
extern "C" int vtk_quadratic_tetra_get_cell_dimension(vtkNew<vtkQuadraticTetra> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_quadratic_tetra_get_number_of_edges(vtkNew<vtkQuadraticTetra> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_quadratic_tetra_get_number_of_faces(vtkNew<vtkQuadraticTetra> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_quadratic_tetra_get_parametric_center(
    vtkNew<vtkQuadraticTetra> self, double* pcoords
) {
    return self->GetParametricCenter(pcoords);
}
extern "C" double vtk_quadratic_tetra_get_parametric_distance(
    vtkNew<vtkQuadraticTetra> self, const double* pcoords
) {
    return self->GetParametricDistance(pcoords);
}
extern "C" void vtk_quadratic_tetra_interpolation_functions(
    vtkNew<vtkQuadraticTetra> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_tetra_interpolation_derivs(
    vtkNew<vtkQuadraticTetra> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_quadratic_tetra_interpolate_functions(
    vtkNew<vtkQuadraticTetra> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_tetra_interpolate_derivs(
    vtkNew<vtkQuadraticTetra> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkQuadraticTriangle> vtkQuadraticTriangle_new() {
    return vtkNew<vtkQuadraticTriangle>();
}
extern "C" void vtkQuadraticTriangle_destructor(vtkNew<vtkQuadraticTriangle> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkQuadraticTriangle_get_ptr(vtkNew<vtkQuadraticTriangle> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_quadratic_triangle_get_cell_type(vtkNew<vtkQuadraticTriangle> self) {
    return self->GetCellType();
}
extern "C" int vtk_quadratic_triangle_get_cell_dimension(vtkNew<vtkQuadraticTriangle> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_quadratic_triangle_get_number_of_edges(vtkNew<vtkQuadraticTriangle> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_quadratic_triangle_get_number_of_faces(vtkNew<vtkQuadraticTriangle> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_quadratic_triangle_get_parametric_center(
    vtkNew<vtkQuadraticTriangle> self, double* pcoords
) {
    return self->GetParametricCenter(pcoords);
}
extern "C" double vtk_quadratic_triangle_get_parametric_distance(
    vtkNew<vtkQuadraticTriangle> self, const double* pcoords
) {
    return self->GetParametricDistance(pcoords);
}
extern "C" void vtk_quadratic_triangle_interpolation_functions(
    vtkNew<vtkQuadraticTriangle> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_triangle_interpolation_derivs(
    vtkNew<vtkQuadraticTriangle> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_quadratic_triangle_interpolate_functions(
    vtkNew<vtkQuadraticTriangle> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_triangle_interpolate_derivs(
    vtkNew<vtkQuadraticTriangle> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkQuadraticWedge> vtkQuadraticWedge_new() {
    return vtkNew<vtkQuadraticWedge>();
}
extern "C" void vtkQuadraticWedge_destructor(vtkNew<vtkQuadraticWedge> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkQuadraticWedge_get_ptr(vtkNew<vtkQuadraticWedge> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_quadratic_wedge_get_cell_type(vtkNew<vtkQuadraticWedge> self) {
    return self->GetCellType();
}
extern "C" int vtk_quadratic_wedge_get_cell_dimension(vtkNew<vtkQuadraticWedge> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_quadratic_wedge_get_number_of_edges(vtkNew<vtkQuadraticWedge> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_quadratic_wedge_get_number_of_faces(vtkNew<vtkQuadraticWedge> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_quadratic_wedge_get_parametric_center(
    vtkNew<vtkQuadraticWedge> self, double* pcoords
) {
    return self->GetParametricCenter(pcoords);
}
extern "C" void vtk_quadratic_wedge_interpolation_functions(
    vtkNew<vtkQuadraticWedge> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_wedge_interpolation_derivs(
    vtkNew<vtkQuadraticWedge> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_quadratic_wedge_interpolate_functions(
    vtkNew<vtkQuadraticWedge> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_quadratic_wedge_interpolate_derivs(
    vtkNew<vtkQuadraticWedge> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkQuadratureSchemeDefinition> vtkQuadratureSchemeDefinition_new() {
    return vtkNew<vtkQuadratureSchemeDefinition>();
}
extern "C" void vtkQuadratureSchemeDefinition_destructor(
    vtkNew<vtkQuadratureSchemeDefinition> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkQuadratureSchemeDefinition_get_ptr(
    vtkNew<vtkQuadratureSchemeDefinition> sself
) {
    return sself.GetPointer();
}
extern "C" int vtk_quadrature_scheme_definition_get_cell_type(
    vtkNew<vtkQuadratureSchemeDefinition> self
) {
    return self->GetCellType();
}
extern "C" int vtk_quadrature_scheme_definition_get_quadrature_key(
    vtkNew<vtkQuadratureSchemeDefinition> self
) {
    return self->GetQuadratureKey();
}
extern "C" int vtk_quadrature_scheme_definition_get_number_of_nodes(
    vtkNew<vtkQuadratureSchemeDefinition> self
) {
    return self->GetNumberOfNodes();
}
extern "C" int vtk_quadrature_scheme_definition_get_number_of_quadrature_points(
    vtkNew<vtkQuadratureSchemeDefinition> self
) {
    return self->GetNumberOfQuadraturePoints();
}
extern "C" int vtk_quadrature_scheme_definition_get_dimension(
    vtkNew<vtkQuadratureSchemeDefinition> self
) {
    return self->GetDimension();
}
extern "C" vtkNew<vtkQuadric> vtkQuadric_new() {
    return vtkNew<vtkQuadric>();
}
extern "C" void vtkQuadric_destructor(vtkNew<vtkQuadric> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkQuadric_get_ptr(vtkNew<vtkQuadric> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_quadric_evaluate_function(vtkNew<vtkQuadric> self, double* x) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_quadric_evaluate_gradient(vtkNew<vtkQuadric> self, double* x, double* g) {
    return self->EvaluateGradient(x, g);
}
extern "C" void vtk_quadric_set_coefficients_double_10(vtkNew<vtkQuadric> self, double* a) {
    return self->SetCoefficients(a);
}
extern "C" void vtk_quadric_set_coefficients_double_double_double_double_double_double_double_double_double_double(
    vtkNew<vtkQuadric> self,
    double a0,
    double a1,
    double a2,
    double a3,
    double a4,
    double a5,
    double a6,
    double a7,
    double a8,
    double a9
) {
    return self->SetCoefficients(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9);
}
extern "C" void vtk_quadric_get_coefficients_double_10(vtkNew<vtkQuadric> self, double* data) {
    return self->GetCoefficients(data);
}
extern "C" vtkNew<vtkRectilinearGrid> vtkRectilinearGrid_new() {
    return vtkNew<vtkRectilinearGrid>();
}
extern "C" void vtkRectilinearGrid_destructor(vtkNew<vtkRectilinearGrid> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkRectilinearGrid_get_ptr(vtkNew<vtkRectilinearGrid> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_rectilinear_grid_initialize(vtkNew<vtkRectilinearGrid> self) {
    return self->Initialize();
}
extern "C" long long vtk_rectilinear_grid_find_point(vtkNew<vtkRectilinearGrid> self, double* x) {
    return self->FindPoint(x);
}
extern "C" int vtk_rectilinear_grid_compute_structured_coordinates_const_double_3_c_int_3_double_3(
    vtkNew<vtkRectilinearGrid> self, const double* x, int* ijk, double* pcoords
) {
    return self->ComputeStructuredCoordinates(x, ijk, pcoords);
}
extern "C" int vtk_rectilinear_grid_compute_structured_coordinates_double_3_c_int_3_double_3(
    vtkNew<vtkRectilinearGrid> self, double* x, int* ijk, double* pcoords
) {
    return self->ComputeStructuredCoordinates(x, ijk, pcoords);
}
extern "C" long long vtk_rectilinear_grid_compute_point_id(
    vtkNew<vtkRectilinearGrid> self, int* ijk
) {
    return self->ComputePointId(ijk);
}
extern "C" long long vtk_rectilinear_grid_compute_cell_id(
    vtkNew<vtkRectilinearGrid> self, int* ijk
) {
    return self->ComputeCellId(ijk);
}
extern "C" void vtk_rectilinear_grid_get_point(
    vtkNew<vtkRectilinearGrid> self, int i, int j, int k, double* p
) {
    return self->GetPoint(i, j, k, p);
}
extern "C" vtkNew<vtkReebGraph> vtkReebGraph_new() {
    return vtkNew<vtkReebGraph>();
}
extern "C" void vtkReebGraph_destructor(vtkNew<vtkReebGraph> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkReebGraph_get_ptr(vtkNew<vtkReebGraph> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_reeb_graph_stream_triangle(
    vtkNew<vtkReebGraph> self,
    long long vertex0Id,
    double scalar0,
    long long vertex1Id,
    double scalar1,
    long long vertex2Id,
    double scalar2
) {
    return self->StreamTriangle(vertex0Id, scalar0, vertex1Id, scalar1, vertex2Id, scalar2);
}
extern "C" int vtk_reeb_graph_stream_tetrahedron(
    vtkNew<vtkReebGraph> self,
    long long vertex0Id,
    double scalar0,
    long long vertex1Id,
    double scalar1,
    long long vertex2Id,
    double scalar2,
    long long vertex3Id,
    double scalar3
) {
    return self->StreamTetrahedron(
        vertex0Id, scalar0, vertex1Id, scalar1, vertex2Id, scalar2, vertex3Id, scalar3
    );
}
extern "C" void vtk_reeb_graph_close_stream(vtkNew<vtkReebGraph> self) {
    return self->CloseStream();
}
extern "C" vtkNew<vtkReebGraphSimplificationMetric> vtkReebGraphSimplificationMetric_new() {
    return vtkNew<vtkReebGraphSimplificationMetric>();
}
extern "C" void vtkReebGraphSimplificationMetric_destructor(
    vtkNew<vtkReebGraphSimplificationMetric> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkReebGraphSimplificationMetric_get_ptr(
    vtkNew<vtkReebGraphSimplificationMetric> sself
) {
    return sself.GetPointer();
}
extern "C" void vtk_reeb_graph_simplification_metric_set_lower_bound(
    vtkNew<vtkReebGraphSimplificationMetric> self, double _arg
) {
    return self->SetLowerBound(_arg);
}
extern "C" double vtk_reeb_graph_simplification_metric_get_lower_bound(
    vtkNew<vtkReebGraphSimplificationMetric> self
) {
    return self->GetLowerBound();
}
extern "C" void vtk_reeb_graph_simplification_metric_set_upper_bound(
    vtkNew<vtkReebGraphSimplificationMetric> self, double _arg
) {
    return self->SetUpperBound(_arg);
}
extern "C" double vtk_reeb_graph_simplification_metric_get_upper_bound(
    vtkNew<vtkReebGraphSimplificationMetric> self
) {
    return self->GetUpperBound();
}
extern "C" vtkNew<vtkSelection> vtkSelection_new() {
    return vtkNew<vtkSelection>();
}
extern "C" void vtkSelection_destructor(vtkNew<vtkSelection> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSelection_get_ptr(vtkNew<vtkSelection> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_selection_get_data_object_type(vtkNew<vtkSelection> self) {
    return self->GetDataObjectType();
}
extern "C" unsigned int vtk_selection_get_number_of_nodes(vtkNew<vtkSelection> self) {
    return self->GetNumberOfNodes();
}
extern "C" void vtk_selection_remove_node_c_uint(vtkNew<vtkSelection> self, unsigned int idx) {
    return self->RemoveNode(idx);
}
extern "C" void vtk_selection_remove_all_nodes(vtkNew<vtkSelection> self) {
    return self->RemoveAllNodes();
}
extern "C" unsigned long vtk_selection_get_m_time(vtkNew<vtkSelection> self) {
    return self->GetMTime();
}
extern "C" void vtk_selection_dump(vtkNew<vtkSelection> self) {
    return self->Dump();
}
extern "C" vtkNew<vtkSelectionNode> vtkSelectionNode_new() {
    return vtkNew<vtkSelectionNode>();
}
extern "C" void vtkSelectionNode_destructor(vtkNew<vtkSelectionNode> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSelectionNode_get_ptr(vtkNew<vtkSelectionNode> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_selection_node_initialize(vtkNew<vtkSelectionNode> self) {
    return self->Initialize();
}
extern "C" unsigned long vtk_selection_node_get_m_time(vtkNew<vtkSelectionNode> self) {
    return self->GetMTime();
}
extern "C" void vtk_selection_node_set_content_type(vtkNew<vtkSelectionNode> self, int type) {
    return self->SetContentType(type);
}
extern "C" int vtk_selection_node_get_content_type(vtkNew<vtkSelectionNode> self) {
    return self->GetContentType();
}
extern "C" void vtk_selection_node_set_field_type(vtkNew<vtkSelectionNode> self, int type) {
    return self->SetFieldType(type);
}
extern "C" int vtk_selection_node_get_field_type(vtkNew<vtkSelectionNode> self) {
    return self->GetFieldType();
}
extern "C" int vtk_selection_node_convert_selection_field_to_attribute_type(
    vtkNew<vtkSelectionNode> self, int val
) {
    return self->ConvertSelectionFieldToAttributeType(val);
}
extern "C" int vtk_selection_node_convert_attribute_type_to_selection_field(
    vtkNew<vtkSelectionNode> self, int val
) {
    return self->ConvertAttributeTypeToSelectionField(val);
}
extern "C" vtkNew<vtkSimpleCellTessellator> vtkSimpleCellTessellator_new() {
    return vtkNew<vtkSimpleCellTessellator>();
}
extern "C" void vtkSimpleCellTessellator_destructor(vtkNew<vtkSimpleCellTessellator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSimpleCellTessellator_get_ptr(vtkNew<vtkSimpleCellTessellator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_simple_cell_tessellator_reset(vtkNew<vtkSimpleCellTessellator> self) {
    return self->Reset();
}
extern "C" int vtk_simple_cell_tessellator_get_fixed_subdivisions(
    vtkNew<vtkSimpleCellTessellator> self
) {
    return self->GetFixedSubdivisions();
}
extern "C" int vtk_simple_cell_tessellator_get_max_subdivision_level(
    vtkNew<vtkSimpleCellTessellator> self
) {
    return self->GetMaxSubdivisionLevel();
}
extern "C" int vtk_simple_cell_tessellator_get_max_adaptive_subdivisions(
    vtkNew<vtkSimpleCellTessellator> self
) {
    return self->GetMaxAdaptiveSubdivisions();
}
extern "C" void vtk_simple_cell_tessellator_set_fixed_subdivisions(
    vtkNew<vtkSimpleCellTessellator> self, int level
) {
    return self->SetFixedSubdivisions(level);
}
extern "C" void vtk_simple_cell_tessellator_set_max_subdivision_level(
    vtkNew<vtkSimpleCellTessellator> self, int level
) {
    return self->SetMaxSubdivisionLevel(level);
}
extern "C" void vtk_simple_cell_tessellator_set_subdivision_levels(
    vtkNew<vtkSimpleCellTessellator> self, int fixed, int maxLevel
) {
    return self->SetSubdivisionLevels(fixed, maxLevel);
}
extern "C" vtkNew<vtkSmoothErrorMetric> vtkSmoothErrorMetric_new() {
    return vtkNew<vtkSmoothErrorMetric>();
}
extern "C" void vtkSmoothErrorMetric_destructor(vtkNew<vtkSmoothErrorMetric> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSmoothErrorMetric_get_ptr(vtkNew<vtkSmoothErrorMetric> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_smooth_error_metric_get_angle_tolerance(vtkNew<vtkSmoothErrorMetric> self) {
    return self->GetAngleTolerance();
}
extern "C" void vtk_smooth_error_metric_set_angle_tolerance(
    vtkNew<vtkSmoothErrorMetric> self, double value
) {
    return self->SetAngleTolerance(value);
}
extern "C" vtkNew<vtkSortFieldData> vtkSortFieldData_new() {
    return vtkNew<vtkSortFieldData>();
}
extern "C" void vtkSortFieldData_destructor(vtkNew<vtkSortFieldData> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSortFieldData_get_ptr(vtkNew<vtkSortFieldData> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkSphere> vtkSphere_new() {
    return vtkNew<vtkSphere>();
}
extern "C" void vtkSphere_destructor(vtkNew<vtkSphere> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSphere_get_ptr(vtkNew<vtkSphere> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_sphere_evaluate_function(vtkNew<vtkSphere> self, double* x) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_sphere_evaluate_gradient(vtkNew<vtkSphere> self, double* x, double* n) {
    return self->EvaluateGradient(x, n);
}
extern "C" void vtk_sphere_set_radius(vtkNew<vtkSphere> self, double _arg) {
    return self->SetRadius(_arg);
}
extern "C" double vtk_sphere_get_radius(vtkNew<vtkSphere> self) {
    return self->GetRadius();
}
extern "C" void vtk_sphere_set_center_double_double_double(
    vtkNew<vtkSphere> self, double _arg1, double _arg2, double _arg3
) {
    return self->SetCenter(_arg1, _arg2, _arg3);
}
extern "C" void vtk_sphere_set_center_const_double_3(vtkNew<vtkSphere> self, const double* _arg) {
    return self->SetCenter(_arg);
}
extern "C" void vtk_sphere_get_center_double_3(vtkNew<vtkSphere> self, double* data) {
    return self->GetCenter(data);
}
extern "C" double vtk_sphere_evaluate(vtkNew<vtkSphere> self, double* center, double R, double* x) {
    return self->Evaluate(center, R, x);
}
extern "C" vtkNew<vtkSpheres> vtkSpheres_new() {
    return vtkNew<vtkSpheres>();
}
extern "C" void vtkSpheres_destructor(vtkNew<vtkSpheres> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSpheres_get_ptr(vtkNew<vtkSpheres> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_spheres_evaluate_function(vtkNew<vtkSpheres> self, double* x) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_spheres_evaluate_gradient(vtkNew<vtkSpheres> self, double* x, double* n) {
    return self->EvaluateGradient(x, n);
}
extern "C" int vtk_spheres_get_number_of_spheres(vtkNew<vtkSpheres> self) {
    return self->GetNumberOfSpheres();
}
extern "C" vtkNew<vtkSphericalPointIterator> vtkSphericalPointIterator_new() {
    return vtkNew<vtkSphericalPointIterator>();
}
extern "C" void vtkSphericalPointIterator_destructor(vtkNew<vtkSphericalPointIterator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSphericalPointIterator_get_ptr(vtkNew<vtkSphericalPointIterator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_spherical_point_iterator_set_axes_c_int_c_int(
    vtkNew<vtkSphericalPointIterator> self, int axesType, int resolution
) {
    return self->SetAxes(axesType, resolution);
}
extern "C" void vtk_spherical_point_iterator_set_sorting(
    vtkNew<vtkSphericalPointIterator> self, int _arg
) {
    return self->SetSorting(_arg);
}
extern "C" int vtk_spherical_point_iterator_get_sorting_min_value(
    vtkNew<vtkSphericalPointIterator> self
) {
    return self->GetSortingMinValue();
}
extern "C" int vtk_spherical_point_iterator_get_sorting_max_value(
    vtkNew<vtkSphericalPointIterator> self
) {
    return self->GetSortingMaxValue();
}
extern "C" int vtk_spherical_point_iterator_get_sorting(vtkNew<vtkSphericalPointIterator> self) {
    return self->GetSorting();
}
extern "C" void vtk_spherical_point_iterator_set_sort_type_to_none(
    vtkNew<vtkSphericalPointIterator> self
) {
    return self->SetSortTypeToNone();
}
extern "C" void vtk_spherical_point_iterator_set_sort_type_to_ascending(
    vtkNew<vtkSphericalPointIterator> self
) {
    return self->SetSortTypeToAscending();
}
extern "C" void vtk_spherical_point_iterator_set_sort_type_to_descending(
    vtkNew<vtkSphericalPointIterator> self
) {
    return self->SetSortTypeToDescending();
}
extern "C" bool vtk_spherical_point_iterator_initialize_double_3(
    vtkNew<vtkSphericalPointIterator> self, double* center
) {
    return self->Initialize(center);
}
extern "C" void vtk_spherical_point_iterator_go_to_first_point(
    vtkNew<vtkSphericalPointIterator> self
) {
    return self->GoToFirstPoint();
}
extern "C" bool vtk_spherical_point_iterator_is_done_with_traversal(
    vtkNew<vtkSphericalPointIterator> self
) {
    return self->IsDoneWithTraversal();
}
extern "C" void vtk_spherical_point_iterator_go_to_next_point(
    vtkNew<vtkSphericalPointIterator> self
) {
    return self->GoToNextPoint();
}
extern "C" long long vtk_spherical_point_iterator_get_current_point(
    vtkNew<vtkSphericalPointIterator> self
) {
    return self->GetCurrentPoint();
}
extern "C" long long vtk_spherical_point_iterator_get_point(
    vtkNew<vtkSphericalPointIterator> self, int axis, int ptIdx
) {
    return self->GetPoint(axis, ptIdx);
}
extern "C" long long vtk_spherical_point_iterator_get_number_of_axes(
    vtkNew<vtkSphericalPointIterator> self
) {
    return self->GetNumberOfAxes();
}
extern "C" vtkNew<vtkStaticCellLinks> vtkStaticCellLinks_new() {
    return vtkNew<vtkStaticCellLinks>();
}
extern "C" void vtkStaticCellLinks_destructor(vtkNew<vtkStaticCellLinks> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStaticCellLinks_get_ptr(vtkNew<vtkStaticCellLinks> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_static_cell_links_build_links(vtkNew<vtkStaticCellLinks> self) {
    return self->BuildLinks();
}
extern "C" long long vtk_static_cell_links_get_number_of_cells(
    vtkNew<vtkStaticCellLinks> self, long long ptId
) {
    return self->GetNumberOfCells(ptId);
}
extern "C" long long vtk_static_cell_links_get_ncells(
    vtkNew<vtkStaticCellLinks> self, long long ptId
) {
    return self->GetNcells(ptId);
}
extern "C" void vtk_static_cell_links_initialize(vtkNew<vtkStaticCellLinks> self) {
    return self->Initialize();
}
extern "C" void vtk_static_cell_links_squeeze(vtkNew<vtkStaticCellLinks> self) {
    return self->Squeeze();
}
extern "C" void vtk_static_cell_links_reset(vtkNew<vtkStaticCellLinks> self) {
    return self->Reset();
}
extern "C" unsigned long vtk_static_cell_links_get_actual_memory_size(
    vtkNew<vtkStaticCellLinks> self
) {
    return self->GetActualMemorySize();
}
extern "C" vtkNew<vtkStaticCellLocator> vtkStaticCellLocator_new() {
    return vtkNew<vtkStaticCellLocator>();
}
extern "C" void vtkStaticCellLocator_destructor(vtkNew<vtkStaticCellLocator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStaticCellLocator_get_ptr(vtkNew<vtkStaticCellLocator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_static_cell_locator_set_divisions_c_int_c_int_c_int(
    vtkNew<vtkStaticCellLocator> self, int _arg1, int _arg2, int _arg3
) {
    return self->SetDivisions(_arg1, _arg2, _arg3);
}
extern "C" void vtk_static_cell_locator_set_divisions_const_c_int_3(
    vtkNew<vtkStaticCellLocator> self, const int* _arg
) {
    return self->SetDivisions(_arg);
}
extern "C" void vtk_static_cell_locator_get_divisions_c_int_3(
    vtkNew<vtkStaticCellLocator> self, int* data
) {
    return self->GetDivisions(data);
}
extern "C" void vtk_static_cell_locator_set_max_number_of_buckets(
    vtkNew<vtkStaticCellLocator> self, long long _arg
) {
    return self->SetMaxNumberOfBuckets(_arg);
}
extern "C" long long vtk_static_cell_locator_get_max_number_of_buckets_min_value(
    vtkNew<vtkStaticCellLocator> self
) {
    return self->GetMaxNumberOfBucketsMinValue();
}
extern "C" long long vtk_static_cell_locator_get_max_number_of_buckets_max_value(
    vtkNew<vtkStaticCellLocator> self
) {
    return self->GetMaxNumberOfBucketsMaxValue();
}
extern "C" long long vtk_static_cell_locator_get_max_number_of_buckets(
    vtkNew<vtkStaticCellLocator> self
) {
    return self->GetMaxNumberOfBuckets();
}
extern "C" bool vtk_static_cell_locator_get_large_ids(vtkNew<vtkStaticCellLocator> self) {
    return self->GetLargeIds();
}
extern "C" bool vtk_static_cell_locator_inside_cell_bounds(
    vtkNew<vtkStaticCellLocator> self, double* x, long long cellId
) {
    return self->InsideCellBounds(x, cellId);
}
extern "C" void vtk_static_cell_locator_free_search_structure(vtkNew<vtkStaticCellLocator> self) {
    return self->FreeSearchStructure();
}
extern "C" void vtk_static_cell_locator_build_locator(vtkNew<vtkStaticCellLocator> self) {
    return self->BuildLocator();
}
extern "C" vtkNew<vtkStaticPointLocator> vtkStaticPointLocator_new() {
    return vtkNew<vtkStaticPointLocator>();
}
extern "C" void vtkStaticPointLocator_destructor(vtkNew<vtkStaticPointLocator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStaticPointLocator_get_ptr(vtkNew<vtkStaticPointLocator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_static_point_locator_set_number_of_points_per_bucket(
    vtkNew<vtkStaticPointLocator> self, int _arg
) {
    return self->SetNumberOfPointsPerBucket(_arg);
}
extern "C" int vtk_static_point_locator_get_number_of_points_per_bucket_min_value(
    vtkNew<vtkStaticPointLocator> self
) {
    return self->GetNumberOfPointsPerBucketMinValue();
}
extern "C" int vtk_static_point_locator_get_number_of_points_per_bucket_max_value(
    vtkNew<vtkStaticPointLocator> self
) {
    return self->GetNumberOfPointsPerBucketMaxValue();
}
extern "C" int vtk_static_point_locator_get_number_of_points_per_bucket(
    vtkNew<vtkStaticPointLocator> self
) {
    return self->GetNumberOfPointsPerBucket();
}
extern "C" void vtk_static_point_locator_set_divisions_c_int_c_int_c_int(
    vtkNew<vtkStaticPointLocator> self, int _arg1, int _arg2, int _arg3
) {
    return self->SetDivisions(_arg1, _arg2, _arg3);
}
extern "C" void vtk_static_point_locator_set_divisions_const_c_int_3(
    vtkNew<vtkStaticPointLocator> self, const int* _arg
) {
    return self->SetDivisions(_arg);
}
extern "C" void vtk_static_point_locator_get_divisions_c_int_3(
    vtkNew<vtkStaticPointLocator> self, int* data
) {
    return self->GetDivisions(data);
}
extern "C" long long vtk_static_point_locator_find_closest_point(
    vtkNew<vtkStaticPointLocator> self, const double* x
) {
    return self->FindClosestPoint(x);
}
extern "C" void vtk_static_point_locator_initialize(vtkNew<vtkStaticPointLocator> self) {
    return self->Initialize();
}
extern "C" void vtk_static_point_locator_free_search_structure(vtkNew<vtkStaticPointLocator> self) {
    return self->FreeSearchStructure();
}
extern "C" void vtk_static_point_locator_build_locator(vtkNew<vtkStaticPointLocator> self) {
    return self->BuildLocator();
}
extern "C" long long vtk_static_point_locator_get_number_of_points_in_bucket(
    vtkNew<vtkStaticPointLocator> self, long long bNum
) {
    return self->GetNumberOfPointsInBucket(bNum);
}
extern "C" void vtk_static_point_locator_get_bucket_center(
    vtkNew<vtkStaticPointLocator> self, int i, int j, int k, double* center
) {
    return self->GetBucketCenter(i, j, k, center);
}
extern "C" void vtk_static_point_locator_set_max_number_of_buckets(
    vtkNew<vtkStaticPointLocator> self, long long _arg
) {
    return self->SetMaxNumberOfBuckets(_arg);
}
extern "C" long long vtk_static_point_locator_get_max_number_of_buckets_min_value(
    vtkNew<vtkStaticPointLocator> self
) {
    return self->GetMaxNumberOfBucketsMinValue();
}
extern "C" long long vtk_static_point_locator_get_max_number_of_buckets_max_value(
    vtkNew<vtkStaticPointLocator> self
) {
    return self->GetMaxNumberOfBucketsMaxValue();
}
extern "C" long long vtk_static_point_locator_get_max_number_of_buckets(
    vtkNew<vtkStaticPointLocator> self
) {
    return self->GetMaxNumberOfBuckets();
}
extern "C" bool vtk_static_point_locator_get_large_ids(vtkNew<vtkStaticPointLocator> self) {
    return self->GetLargeIds();
}
extern "C" void vtk_static_point_locator_get_spacing_double_3(
    vtkNew<vtkStaticPointLocator> self, double* spacing
) {
    return self->GetSpacing(spacing);
}
extern "C" void vtk_static_point_locator_set_traversal_order(
    vtkNew<vtkStaticPointLocator> self, int _arg
) {
    return self->SetTraversalOrder(_arg);
}
extern "C" int vtk_static_point_locator_get_traversal_order_min_value(
    vtkNew<vtkStaticPointLocator> self
) {
    return self->GetTraversalOrderMinValue();
}
extern "C" int vtk_static_point_locator_get_traversal_order_max_value(
    vtkNew<vtkStaticPointLocator> self
) {
    return self->GetTraversalOrderMaxValue();
}
extern "C" int vtk_static_point_locator_get_traversal_order(vtkNew<vtkStaticPointLocator> self) {
    return self->GetTraversalOrder();
}
extern "C" void vtk_static_point_locator_set_traversal_order_to_point_order(
    vtkNew<vtkStaticPointLocator> self
) {
    return self->SetTraversalOrderToPointOrder();
}
extern "C" void vtk_static_point_locator_set_traversal_order_to_bin_order(
    vtkNew<vtkStaticPointLocator> self
) {
    return self->SetTraversalOrderToBinOrder();
}
extern "C" void vtk_static_point_locator_set_padding(
    vtkNew<vtkStaticPointLocator> self, double _arg
) {
    return self->SetPadding(_arg);
}
extern "C" double vtk_static_point_locator_get_padding(vtkNew<vtkStaticPointLocator> self) {
    return self->GetPadding();
}
extern "C" void vtk_static_point_locator_static_on(vtkNew<vtkStaticPointLocator> self) {
    return self->StaticOn();
}
extern "C" void vtk_static_point_locator_static_off(vtkNew<vtkStaticPointLocator> self) {
    return self->StaticOff();
}
extern "C" int vtk_static_point_locator_get_static(vtkNew<vtkStaticPointLocator> self) {
    return self->GetStatic();
}
extern "C" vtkNew<vtkStaticPointLocator2D> vtkStaticPointLocator2D_new() {
    return vtkNew<vtkStaticPointLocator2D>();
}
extern "C" void vtkStaticPointLocator2D_destructor(vtkNew<vtkStaticPointLocator2D> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStaticPointLocator2D_get_ptr(vtkNew<vtkStaticPointLocator2D> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_static_point_locator_2_d_set_number_of_points_per_bucket(
    vtkNew<vtkStaticPointLocator2D> self, int _arg
) {
    return self->SetNumberOfPointsPerBucket(_arg);
}
extern "C" int vtk_static_point_locator_2_d_get_number_of_points_per_bucket_min_value(
    vtkNew<vtkStaticPointLocator2D> self
) {
    return self->GetNumberOfPointsPerBucketMinValue();
}
extern "C" int vtk_static_point_locator_2_d_get_number_of_points_per_bucket_max_value(
    vtkNew<vtkStaticPointLocator2D> self
) {
    return self->GetNumberOfPointsPerBucketMaxValue();
}
extern "C" int vtk_static_point_locator_2_d_get_number_of_points_per_bucket(
    vtkNew<vtkStaticPointLocator2D> self
) {
    return self->GetNumberOfPointsPerBucket();
}
extern "C" void vtk_static_point_locator_2_d_set_divisions_c_int_c_int(
    vtkNew<vtkStaticPointLocator2D> self, int _arg1, int _arg2
) {
    return self->SetDivisions(_arg1, _arg2);
}
extern "C" void vtk_static_point_locator_2_d_set_divisions_const_c_int_2(
    vtkNew<vtkStaticPointLocator2D> self, const int* _arg
) {
    return self->SetDivisions(_arg);
}
extern "C" void vtk_static_point_locator_2_d_get_divisions_c_int_2(
    vtkNew<vtkStaticPointLocator2D> self, int* data
) {
    return self->GetDivisions(data);
}
extern "C" long long vtk_static_point_locator_2_d_find_closest_point(
    vtkNew<vtkStaticPointLocator2D> self, const double* x
) {
    return self->FindClosestPoint(x);
}
extern "C" void vtk_static_point_locator_2_d_initialize(vtkNew<vtkStaticPointLocator2D> self) {
    return self->Initialize();
}
extern "C" void vtk_static_point_locator_2_d_free_search_structure(
    vtkNew<vtkStaticPointLocator2D> self
) {
    return self->FreeSearchStructure();
}
extern "C" void vtk_static_point_locator_2_d_build_locator(vtkNew<vtkStaticPointLocator2D> self) {
    return self->BuildLocator();
}
extern "C" long long vtk_static_point_locator_2_d_get_number_of_points_in_bucket(
    vtkNew<vtkStaticPointLocator2D> self, long long bNum
) {
    return self->GetNumberOfPointsInBucket(bNum);
}
extern "C" void vtk_static_point_locator_2_d_set_max_number_of_buckets(
    vtkNew<vtkStaticPointLocator2D> self, long long _arg
) {
    return self->SetMaxNumberOfBuckets(_arg);
}
extern "C" long long vtk_static_point_locator_2_d_get_max_number_of_buckets_min_value(
    vtkNew<vtkStaticPointLocator2D> self
) {
    return self->GetMaxNumberOfBucketsMinValue();
}
extern "C" long long vtk_static_point_locator_2_d_get_max_number_of_buckets_max_value(
    vtkNew<vtkStaticPointLocator2D> self
) {
    return self->GetMaxNumberOfBucketsMaxValue();
}
extern "C" long long vtk_static_point_locator_2_d_get_max_number_of_buckets(
    vtkNew<vtkStaticPointLocator2D> self
) {
    return self->GetMaxNumberOfBuckets();
}
extern "C" bool vtk_static_point_locator_2_d_get_large_ids(vtkNew<vtkStaticPointLocator2D> self) {
    return self->GetLargeIds();
}
extern "C" void vtk_static_point_locator_2_d_get_spacing_double_3(
    vtkNew<vtkStaticPointLocator2D> self, double* spacing
) {
    return self->GetSpacing(spacing);
}
extern "C" void vtk_static_point_locator_2_d_static_on(vtkNew<vtkStaticPointLocator2D> self) {
    return self->StaticOn();
}
extern "C" void vtk_static_point_locator_2_d_static_off(vtkNew<vtkStaticPointLocator2D> self) {
    return self->StaticOff();
}
extern "C" int vtk_static_point_locator_2_d_get_static(vtkNew<vtkStaticPointLocator2D> self) {
    return self->GetStatic();
}
extern "C" vtkNew<vtkStatisticalModel> vtkStatisticalModel_new() {
    return vtkNew<vtkStatisticalModel>();
}
extern "C" void vtkStatisticalModel_destructor(vtkNew<vtkStatisticalModel> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStatisticalModel_get_ptr(vtkNew<vtkStatisticalModel> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long vtk_statistical_model_get_m_time(vtkNew<vtkStatisticalModel> self) {
    return self->GetMTime();
}
extern "C" void vtk_statistical_model_initialize(vtkNew<vtkStatisticalModel> self) {
    return self->Initialize();
}
extern "C" bool vtk_statistical_model_is_empty(vtkNew<vtkStatisticalModel> self) {
    return self->IsEmpty();
}
extern "C" unsigned long vtk_statistical_model_get_actual_memory_size(
    vtkNew<vtkStatisticalModel> self
) {
    return self->GetActualMemorySize();
}
extern "C" int vtk_statistical_model_get_data_object_type(vtkNew<vtkStatisticalModel> self) {
    return self->GetDataObjectType();
}
extern "C" int vtk_statistical_model_get_number_of_table_types(vtkNew<vtkStatisticalModel> self) {
    return self->GetNumberOfTableTypes();
}
extern "C" int vtk_statistical_model_get_number_of_tables(vtkNew<vtkStatisticalModel> self) {
    return self->GetNumberOfTables();
}
extern "C" int vtk_statistical_model_get_number_of_tables_c_int(
    vtkNew<vtkStatisticalModel> self, int type
) {
    return self->GetNumberOfTables(type);
}
extern "C" bool vtk_statistical_model_set_number_of_tables(
    vtkNew<vtkStatisticalModel> self, int type, int number
) {
    return self->SetNumberOfTables(type, number);
}
extern "C" vtkNew<vtkStructuredCellArray> vtkStructuredCellArray_new() {
    return vtkNew<vtkStructuredCellArray>();
}
extern "C" void vtkStructuredCellArray_destructor(vtkNew<vtkStructuredCellArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStructuredCellArray_get_ptr(vtkNew<vtkStructuredCellArray> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_structured_cell_array_initialize(vtkNew<vtkStructuredCellArray> self) {
    return self->Initialize();
}
extern "C" long long vtk_structured_cell_array_get_number_of_cells(
    vtkNew<vtkStructuredCellArray> self
) {
    return self->GetNumberOfCells();
}
extern "C" long long vtk_structured_cell_array_get_number_of_offsets(
    vtkNew<vtkStructuredCellArray> self
) {
    return self->GetNumberOfOffsets();
}
extern "C" long long vtk_structured_cell_array_get_offset(
    vtkNew<vtkStructuredCellArray> self, long long cellId
) {
    return self->GetOffset(cellId);
}
extern "C" long long vtk_structured_cell_array_get_number_of_connectivity_ids(
    vtkNew<vtkStructuredCellArray> self
) {
    return self->GetNumberOfConnectivityIds();
}
extern "C" void vtk_structured_cell_array_set_data(
    vtkNew<vtkStructuredCellArray> self, int* extent, bool usePixelVoxelOrientation
) {
    return self->SetData(extent, usePixelVoxelOrientation);
}
extern "C" bool vtk_structured_cell_array_is_storage_shareable(
    vtkNew<vtkStructuredCellArray> self
) {
    return self->IsStorageShareable();
}
extern "C" long long vtk_structured_cell_array_is_homogeneous(vtkNew<vtkStructuredCellArray> self) {
    return self->IsHomogeneous();
}
extern "C" long long vtk_structured_cell_array_get_cell_size(
    vtkNew<vtkStructuredCellArray> self, long long cellId
) {
    return self->GetCellSize(cellId);
}
extern "C" int vtk_structured_cell_array_get_max_cell_size(vtkNew<vtkStructuredCellArray> self) {
    return self->GetMaxCellSize();
}
extern "C" vtkNew<vtkStructuredExtent> vtkStructuredExtent_new() {
    return vtkNew<vtkStructuredExtent>();
}
extern "C" void vtkStructuredExtent_destructor(vtkNew<vtkStructuredExtent> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStructuredExtent_get_ptr(vtkNew<vtkStructuredExtent> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_structured_extent_clamp(
    vtkNew<vtkStructuredExtent> self, int* ext, const int* wholeExt
) {
    return self->Clamp(ext, wholeExt);
}
extern "C" bool vtk_structured_extent_strictly_smaller(
    vtkNew<vtkStructuredExtent> self, const int* ext, const int* wholeExt
) {
    return self->StrictlySmaller(ext, wholeExt);
}
extern "C" bool vtk_structured_extent_smaller(
    vtkNew<vtkStructuredExtent> self, const int* ext, const int* wholeExt
) {
    return self->Smaller(ext, wholeExt);
}
extern "C" void vtk_structured_extent_grow_c_int_6_c_int(
    vtkNew<vtkStructuredExtent> self, int* ext, int count
) {
    return self->Grow(ext, count);
}
extern "C" void vtk_structured_extent_grow_c_int_6_c_int_c_int_6(
    vtkNew<vtkStructuredExtent> self, int* ext, int count, int* wholeExt
) {
    return self->Grow(ext, count, wholeExt);
}
extern "C" void vtk_structured_extent_transform(
    vtkNew<vtkStructuredExtent> self, int* ext, int* wholeExt
) {
    return self->Transform(ext, wholeExt);
}
extern "C" void vtk_structured_extent_get_dimensions(
    vtkNew<vtkStructuredExtent> self, const int* ext, int* dims
) {
    return self->GetDimensions(ext, dims);
}
extern "C" vtkNew<vtkStructuredGrid> vtkStructuredGrid_new() {
    return vtkNew<vtkStructuredGrid>();
}
extern "C" void vtkStructuredGrid_destructor(vtkNew<vtkStructuredGrid> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStructuredGrid_get_ptr(vtkNew<vtkStructuredGrid> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_structured_grid_get_data_object_type(vtkNew<vtkStructuredGrid> self) {
    return self->GetDataObjectType();
}
extern "C" void vtk_structured_grid_initialize(vtkNew<vtkStructuredGrid> self) {
    return self->Initialize();
}
extern "C" long long vtk_structured_grid_get_number_of_cells(vtkNew<vtkStructuredGrid> self) {
    return self->GetNumberOfCells();
}
extern "C" void vtk_structured_grid_get_point_c_longlong_double_3(
    vtkNew<vtkStructuredGrid> self, long long ptId, double* p
) {
    return self->GetPoint(ptId, p);
}
extern "C" int vtk_structured_grid_get_cell_type(vtkNew<vtkStructuredGrid> self, long long cellId) {
    return self->GetCellType(cellId);
}
extern "C" long long vtk_structured_grid_get_cell_size(
    vtkNew<vtkStructuredGrid> self, long long cellId
) {
    return self->GetCellSize(cellId);
}
extern "C" void vtk_structured_grid_blank_point(vtkNew<vtkStructuredGrid> self, long long ptId) {
    return self->BlankPoint(ptId);
}
extern "C" void vtk_structured_grid_un_blank_point(vtkNew<vtkStructuredGrid> self, long long ptId) {
    return self->UnBlankPoint(ptId);
}
extern "C" void vtk_structured_grid_blank_cell(vtkNew<vtkStructuredGrid> self, long long ptId) {
    return self->BlankCell(ptId);
}
extern "C" void vtk_structured_grid_un_blank_cell(vtkNew<vtkStructuredGrid> self, long long ptId) {
    return self->UnBlankCell(ptId);
}
extern "C" unsigned char vtk_structured_grid_is_point_visible(
    vtkNew<vtkStructuredGrid> self, long long ptId
) {
    return self->IsPointVisible(ptId);
}
extern "C" unsigned char vtk_structured_grid_is_cell_visible(
    vtkNew<vtkStructuredGrid> self, long long cellId
) {
    return self->IsCellVisible(cellId);
}
extern "C" bool vtk_structured_grid_has_any_blank_points(vtkNew<vtkStructuredGrid> self) {
    return self->HasAnyBlankPoints();
}
extern "C" bool vtk_structured_grid_has_any_blank_cells(vtkNew<vtkStructuredGrid> self) {
    return self->HasAnyBlankCells();
}
extern "C" int vtk_structured_grid_get_data_description(vtkNew<vtkStructuredGrid> self) {
    return self->GetDataDescription();
}
extern "C" void vtk_structured_grid_get_cell_dims(vtkNew<vtkStructuredGrid> self, int* cellDims) {
    return self->GetCellDims(cellDims);
}
extern "C" void vtk_structured_grid_set_dimensions_c_int_c_int_c_int(
    vtkNew<vtkStructuredGrid> self, int i, int j, int k
) {
    return self->SetDimensions(i, j, k);
}
extern "C" void vtk_structured_grid_set_dimensions_const_c_int_3(
    vtkNew<vtkStructuredGrid> self, const int* dims
) {
    return self->SetDimensions(dims);
}
extern "C" void vtk_structured_grid_get_dimensions(vtkNew<vtkStructuredGrid> self, int* dims) {
    return self->GetDimensions(dims);
}
extern "C" int vtk_structured_grid_get_data_dimension(vtkNew<vtkStructuredGrid> self) {
    return self->GetDataDimension();
}
extern "C" void vtk_structured_grid_set_extent_c_int_6(
    vtkNew<vtkStructuredGrid> self, int* extent
) {
    return self->SetExtent(extent);
}
extern "C" void vtk_structured_grid_set_extent_c_int_c_int_c_int_c_int_c_int_c_int(
    vtkNew<vtkStructuredGrid> self, int xMin, int xMax, int yMin, int yMax, int zMin, int zMax
) {
    return self->SetExtent(xMin, xMax, yMin, yMax, zMin, zMax);
}
extern "C" void vtk_structured_grid_get_extent_c_int_6(vtkNew<vtkStructuredGrid> self, int* _arg) {
    return self->GetExtent(_arg);
}
extern "C" int vtk_structured_grid_get_extent_type(vtkNew<vtkStructuredGrid> self) {
    return self->GetExtentType();
}
extern "C" void vtk_structured_grid_get_point_c_int_c_int_c_int_double_3_bool(
    vtkNew<vtkStructuredGrid> self, int i, int j, int k, double* p, bool adjustForExtent
) {
    return self->GetPoint(i, j, k, p, adjustForExtent);
}
extern "C" vtkNew<vtkStructuredPoints> vtkStructuredPoints_new() {
    return vtkNew<vtkStructuredPoints>();
}
extern "C" void vtkStructuredPoints_destructor(vtkNew<vtkStructuredPoints> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStructuredPoints_get_ptr(vtkNew<vtkStructuredPoints> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_structured_points_get_data_object_type(vtkNew<vtkStructuredPoints> self) {
    return self->GetDataObjectType();
}
extern "C" vtkNew<vtkStructuredPointsCollection> vtkStructuredPointsCollection_new() {
    return vtkNew<vtkStructuredPointsCollection>();
}
extern "C" void vtkStructuredPointsCollection_destructor(
    vtkNew<vtkStructuredPointsCollection> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkStructuredPointsCollection_get_ptr(
    vtkNew<vtkStructuredPointsCollection> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkSuperquadric> vtkSuperquadric_new() {
    return vtkNew<vtkSuperquadric>();
}
extern "C" void vtkSuperquadric_destructor(vtkNew<vtkSuperquadric> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSuperquadric_get_ptr(vtkNew<vtkSuperquadric> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_superquadric_evaluate_function(vtkNew<vtkSuperquadric> self, double* x) {
    return self->EvaluateFunction(x);
}
extern "C" void vtk_superquadric_evaluate_gradient(
    vtkNew<vtkSuperquadric> self, double* x, double* g
) {
    return self->EvaluateGradient(x, g);
}
extern "C" void vtk_superquadric_set_center_double_double_double(
    vtkNew<vtkSuperquadric> self, double _arg1, double _arg2, double _arg3
) {
    return self->SetCenter(_arg1, _arg2, _arg3);
}
extern "C" void vtk_superquadric_set_center_const_double_3(
    vtkNew<vtkSuperquadric> self, const double* _arg
) {
    return self->SetCenter(_arg);
}
extern "C" void vtk_superquadric_get_center_double_3(vtkNew<vtkSuperquadric> self, double* data) {
    return self->GetCenter(data);
}
extern "C" void vtk_superquadric_set_scale_double_double_double(
    vtkNew<vtkSuperquadric> self, double _arg1, double _arg2, double _arg3
) {
    return self->SetScale(_arg1, _arg2, _arg3);
}
extern "C" void vtk_superquadric_set_scale_const_double_3(
    vtkNew<vtkSuperquadric> self, const double* _arg
) {
    return self->SetScale(_arg);
}
extern "C" void vtk_superquadric_get_scale_double_3(vtkNew<vtkSuperquadric> self, double* data) {
    return self->GetScale(data);
}
extern "C" double vtk_superquadric_get_thickness(vtkNew<vtkSuperquadric> self) {
    return self->GetThickness();
}
extern "C" void vtk_superquadric_set_thickness(vtkNew<vtkSuperquadric> self, double _arg) {
    return self->SetThickness(_arg);
}
extern "C" double vtk_superquadric_get_thickness_min_value(vtkNew<vtkSuperquadric> self) {
    return self->GetThicknessMinValue();
}
extern "C" double vtk_superquadric_get_thickness_max_value(vtkNew<vtkSuperquadric> self) {
    return self->GetThicknessMaxValue();
}
extern "C" double vtk_superquadric_get_phi_roundness(vtkNew<vtkSuperquadric> self) {
    return self->GetPhiRoundness();
}
extern "C" void vtk_superquadric_set_phi_roundness(vtkNew<vtkSuperquadric> self, double e) {
    return self->SetPhiRoundness(e);
}
extern "C" double vtk_superquadric_get_theta_roundness(vtkNew<vtkSuperquadric> self) {
    return self->GetThetaRoundness();
}
extern "C" void vtk_superquadric_set_theta_roundness(vtkNew<vtkSuperquadric> self, double e) {
    return self->SetThetaRoundness(e);
}
extern "C" void vtk_superquadric_set_size(vtkNew<vtkSuperquadric> self, double _arg) {
    return self->SetSize(_arg);
}
extern "C" double vtk_superquadric_get_size(vtkNew<vtkSuperquadric> self) {
    return self->GetSize();
}
extern "C" void vtk_superquadric_toroidal_on(vtkNew<vtkSuperquadric> self) {
    return self->ToroidalOn();
}
extern "C" void vtk_superquadric_toroidal_off(vtkNew<vtkSuperquadric> self) {
    return self->ToroidalOff();
}
extern "C" int vtk_superquadric_get_toroidal(vtkNew<vtkSuperquadric> self) {
    return self->GetToroidal();
}
extern "C" void vtk_superquadric_set_toroidal(vtkNew<vtkSuperquadric> self, int _arg) {
    return self->SetToroidal(_arg);
}
extern "C" vtkNew<vtkTable> vtkTable_new() {
    return vtkNew<vtkTable>();
}
extern "C" void vtkTable_destructor(vtkNew<vtkTable> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTable_get_ptr(vtkNew<vtkTable> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_table_dump(
    vtkNew<vtkTable> self, unsigned int colWidth, int rowLimit, unsigned int indent
) {
    return self->Dump(colWidth, rowLimit, indent);
}
extern "C" int vtk_table_get_data_object_type(vtkNew<vtkTable> self) {
    return self->GetDataObjectType();
}
extern "C" long long vtk_table_get_number_of_rows(vtkNew<vtkTable> self) {
    return self->GetNumberOfRows();
}
extern "C" void vtk_table_set_number_of_rows(vtkNew<vtkTable> self, long long p0) {
    return self->SetNumberOfRows(p0);
}
extern "C" void vtk_table_squeeze_rows(vtkNew<vtkTable> self) {
    return self->SqueezeRows();
}
extern "C" void vtk_table_insert_row(vtkNew<vtkTable> self, long long row) {
    return self->InsertRow(row);
}
extern "C" void vtk_table_insert_rows(vtkNew<vtkTable> self, long long row, long long n) {
    return self->InsertRows(row, n);
}
extern "C" long long vtk_table_insert_next_blank_row(
    vtkNew<vtkTable> self, double default_num_val
) {
    return self->InsertNextBlankRow(default_num_val);
}
extern "C" void vtk_table_remove_row(vtkNew<vtkTable> self, long long row) {
    return self->RemoveRow(row);
}
extern "C" void vtk_table_remove_rows(vtkNew<vtkTable> self, long long row, long long n) {
    return self->RemoveRows(row, n);
}
extern "C" void vtk_table_remove_all_rows(vtkNew<vtkTable> self) {
    return self->RemoveAllRows();
}
extern "C" long long vtk_table_get_number_of_columns(vtkNew<vtkTable> self) {
    return self->GetNumberOfColumns();
}
extern "C" void vtk_table_remove_column(vtkNew<vtkTable> self, long long col) {
    return self->RemoveColumn(col);
}
extern "C" void vtk_table_remove_all_columns(vtkNew<vtkTable> self) {
    return self->RemoveAllColumns();
}
extern "C" void vtk_table_initialize(vtkNew<vtkTable> self) {
    return self->Initialize();
}
extern "C" long long vtk_table_get_number_of_elements(vtkNew<vtkTable> self, int type) {
    return self->GetNumberOfElements(type);
}
extern "C" vtkNew<vtkTetra> vtkTetra_new() {
    return vtkNew<vtkTetra>();
}
extern "C" void vtkTetra_destructor(vtkNew<vtkTetra> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTetra_get_ptr(vtkNew<vtkTetra> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_tetra_get_centroid(vtkNew<vtkTetra> self, double* centroid) {
    return self->GetCentroid(centroid);
}
extern "C" int vtk_tetra_get_cell_type(vtkNew<vtkTetra> self) {
    return self->GetCellType();
}
extern "C" int vtk_tetra_get_number_of_edges(vtkNew<vtkTetra> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_tetra_get_number_of_faces(vtkNew<vtkTetra> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_tetra_get_parametric_center(vtkNew<vtkTetra> self, double* pcoords) {
    return self->GetParametricCenter(pcoords);
}
extern "C" double vtk_tetra_get_parametric_distance(vtkNew<vtkTetra> self, const double* pcoords) {
    return self->GetParametricDistance(pcoords);
}
extern "C" void vtk_tetra_tetra_center(
    vtkNew<vtkTetra> self, double* p1, double* p2, double* p3, double* p4, double* center
) {
    return self->TetraCenter(p1, p2, p3, p4, center);
}
extern "C" double vtk_tetra_circumsphere(
    vtkNew<vtkTetra> self, double* x1, double* x2, double* x3, double* x4, double* center
) {
    return self->Circumsphere(x1, x2, x3, x4, center);
}
extern "C" double vtk_tetra_insphere(
    vtkNew<vtkTetra> self, double* p1, double* p2, double* p3, double* p4, double* center
) {
    return self->Insphere(p1, p2, p3, p4, center);
}
extern "C" int vtk_tetra_barycentric_coords(
    vtkNew<vtkTetra> self,
    double* x,
    double* x1,
    double* x2,
    double* x3,
    double* x4,
    double* bcoords
) {
    return self->BarycentricCoords(x, x1, x2, x3, x4, bcoords);
}
extern "C" double vtk_tetra_compute_volume(
    vtkNew<vtkTetra> self, double* p1, double* p2, double* p3, double* p4
) {
    return self->ComputeVolume(p1, p2, p3, p4);
}
extern "C" void vtk_tetra_interpolation_functions(
    vtkNew<vtkTetra> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_tetra_interpolation_derivs(
    vtkNew<vtkTetra> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_tetra_interpolate_functions(
    vtkNew<vtkTetra> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_tetra_interpolate_derivs(
    vtkNew<vtkTetra> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkTree> vtkTree_new() {
    return vtkNew<vtkTree>();
}
extern "C" void vtkTree_destructor(vtkNew<vtkTree> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTree_get_ptr(vtkNew<vtkTree> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_tree_get_root(vtkNew<vtkTree> self) {
    return self->GetRoot();
}
extern "C" void vtk_tree_set_root(vtkNew<vtkTree> self, long long _arg) {
    return self->SetRoot(_arg);
}
extern "C" long long vtk_tree_get_number_of_children(vtkNew<vtkTree> self, long long v) {
    return self->GetNumberOfChildren(v);
}
extern "C" long long vtk_tree_get_child(vtkNew<vtkTree> self, long long v, long long i) {
    return self->GetChild(v, i);
}
extern "C" long long vtk_tree_get_parent(vtkNew<vtkTree> self, long long v) {
    return self->GetParent(v);
}
extern "C" long long vtk_tree_get_level(vtkNew<vtkTree> self, long long v) {
    return self->GetLevel(v);
}
extern "C" bool vtk_tree_is_leaf(vtkNew<vtkTree> self, long long vertex) {
    return self->IsLeaf(vertex);
}
extern "C" vtkNew<vtkTreeBFSIterator> vtkTreeBFSIterator_new() {
    return vtkNew<vtkTreeBFSIterator>();
}
extern "C" void vtkTreeBFSIterator_destructor(vtkNew<vtkTreeBFSIterator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTreeBFSIterator_get_ptr(vtkNew<vtkTreeBFSIterator> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkTreeDFSIterator> vtkTreeDFSIterator_new() {
    return vtkNew<vtkTreeDFSIterator>();
}
extern "C" void vtkTreeDFSIterator_destructor(vtkNew<vtkTreeDFSIterator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTreeDFSIterator_get_ptr(vtkNew<vtkTreeDFSIterator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_tree_dfs_iterator_set_mode(vtkNew<vtkTreeDFSIterator> self, int mode) {
    return self->SetMode(mode);
}
extern "C" int vtk_tree_dfs_iterator_get_mode(vtkNew<vtkTreeDFSIterator> self) {
    return self->GetMode();
}
extern "C" vtkNew<vtkTriQuadraticHexahedron> vtkTriQuadraticHexahedron_new() {
    return vtkNew<vtkTriQuadraticHexahedron>();
}
extern "C" void vtkTriQuadraticHexahedron_destructor(vtkNew<vtkTriQuadraticHexahedron> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTriQuadraticHexahedron_get_ptr(vtkNew<vtkTriQuadraticHexahedron> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_tri_quadratic_hexahedron_get_cell_type(vtkNew<vtkTriQuadraticHexahedron> self) {
    return self->GetCellType();
}
extern "C" int vtk_tri_quadratic_hexahedron_get_cell_dimension(
    vtkNew<vtkTriQuadraticHexahedron> self
) {
    return self->GetCellDimension();
}
extern "C" int vtk_tri_quadratic_hexahedron_get_number_of_edges(
    vtkNew<vtkTriQuadraticHexahedron> self
) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_tri_quadratic_hexahedron_get_number_of_faces(
    vtkNew<vtkTriQuadraticHexahedron> self
) {
    return self->GetNumberOfFaces();
}
extern "C" void vtk_tri_quadratic_hexahedron_interpolation_functions(
    vtkNew<vtkTriQuadraticHexahedron> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_tri_quadratic_hexahedron_interpolation_derivs(
    vtkNew<vtkTriQuadraticHexahedron> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_tri_quadratic_hexahedron_interpolate_functions(
    vtkNew<vtkTriQuadraticHexahedron> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_tri_quadratic_hexahedron_interpolate_derivs(
    vtkNew<vtkTriQuadraticHexahedron> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkTriQuadraticPyramid> vtkTriQuadraticPyramid_new() {
    return vtkNew<vtkTriQuadraticPyramid>();
}
extern "C" void vtkTriQuadraticPyramid_destructor(vtkNew<vtkTriQuadraticPyramid> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTriQuadraticPyramid_get_ptr(vtkNew<vtkTriQuadraticPyramid> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_tri_quadratic_pyramid_get_cell_type(vtkNew<vtkTriQuadraticPyramid> self) {
    return self->GetCellType();
}
extern "C" int vtk_tri_quadratic_pyramid_get_cell_dimension(vtkNew<vtkTriQuadraticPyramid> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_tri_quadratic_pyramid_get_number_of_edges(vtkNew<vtkTriQuadraticPyramid> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_tri_quadratic_pyramid_get_number_of_faces(vtkNew<vtkTriQuadraticPyramid> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_tri_quadratic_pyramid_get_parametric_center(
    vtkNew<vtkTriQuadraticPyramid> self, double* pcoords
) {
    return self->GetParametricCenter(pcoords);
}
extern "C" double vtk_tri_quadratic_pyramid_get_parametric_distance(
    vtkNew<vtkTriQuadraticPyramid> self, const double* pcoords
) {
    return self->GetParametricDistance(pcoords);
}
extern "C" void vtk_tri_quadratic_pyramid_interpolation_functions(
    vtkNew<vtkTriQuadraticPyramid> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_tri_quadratic_pyramid_interpolation_derivs(
    vtkNew<vtkTriQuadraticPyramid> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_tri_quadratic_pyramid_interpolate_functions(
    vtkNew<vtkTriQuadraticPyramid> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_tri_quadratic_pyramid_interpolate_derivs(
    vtkNew<vtkTriQuadraticPyramid> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkTriangle> vtkTriangle_new() {
    return vtkNew<vtkTriangle>();
}
extern "C" void vtkTriangle_destructor(vtkNew<vtkTriangle> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTriangle_get_ptr(vtkNew<vtkTriangle> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_triangle_get_cell_type(vtkNew<vtkTriangle> self) {
    return self->GetCellType();
}
extern "C" int vtk_triangle_get_cell_dimension(vtkNew<vtkTriangle> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_triangle_get_number_of_edges(vtkNew<vtkTriangle> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_triangle_get_number_of_faces(vtkNew<vtkTriangle> self) {
    return self->GetNumberOfFaces();
}
extern "C" double vtk_triangle_compute_area(vtkNew<vtkTriangle> self) {
    return self->ComputeArea();
}
extern "C" void vtk_triangle_interpolation_functions(
    vtkNew<vtkTriangle> self, const double* pcoords, double* sf
) {
    return self->InterpolationFunctions(pcoords, sf);
}
extern "C" void vtk_triangle_interpolation_derivs(
    vtkNew<vtkTriangle> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_triangle_interpolate_functions(
    vtkNew<vtkTriangle> self, const double* pcoords, double* sf
) {
    return self->InterpolateFunctions(pcoords, sf);
}
extern "C" void vtk_triangle_interpolate_derivs(
    vtkNew<vtkTriangle> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" int vtk_triangle_get_parametric_center(vtkNew<vtkTriangle> self, double* pcoords) {
    return self->GetParametricCenter(pcoords);
}
extern "C" double vtk_triangle_get_parametric_distance(
    vtkNew<vtkTriangle> self, const double* pcoords
) {
    return self->GetParametricDistance(pcoords);
}
extern "C" void vtk_triangle_triangle_center(
    vtkNew<vtkTriangle> self, const double* p1, const double* p2, const double* p3, double* center
) {
    return self->TriangleCenter(p1, p2, p3, center);
}
extern "C" double vtk_triangle_triangle_area(
    vtkNew<vtkTriangle> self, const double* p1, const double* p2, const double* p3
) {
    return self->TriangleArea(p1, p2, p3);
}
extern "C" double vtk_triangle_circumcircle(
    vtkNew<vtkTriangle> self, const double* p1, const double* p2, const double* p3, double* center
) {
    return self->Circumcircle(p1, p2, p3, center);
}
extern "C" int vtk_triangle_barycentric_coords(
    vtkNew<vtkTriangle> self,
    const double* x,
    const double* x1,
    const double* x2,
    const double* x3,
    double* bcoords
) {
    return self->BarycentricCoords(x, x1, x2, x3, bcoords);
}
extern "C" int vtk_triangle_project_to_2_d(
    vtkNew<vtkTriangle> self,
    const double* x1,
    const double* x2,
    const double* x3,
    double* v1,
    double* v2,
    double* v3
) {
    return self->ProjectTo2D(x1, x2, x3, v1, v2, v3);
}
extern "C" void vtk_triangle_compute_normal_const_double_3_const_double_3_const_double_3_double_3(
    vtkNew<vtkTriangle> self, const double* v1, const double* v2, const double* v3, double* n
) {
    return self->ComputeNormal(v1, v2, v3, n);
}
extern "C" void vtk_triangle_compute_normal_direction(
    vtkNew<vtkTriangle> self, const double* v1, const double* v2, const double* v3, double* n
) {
    return self->ComputeNormalDirection(v1, v2, v3, n);
}
extern "C" int vtk_triangle_triangles_intersect(
    vtkNew<vtkTriangle> self,
    const double* p1,
    const double* q1,
    const double* r1,
    const double* p2,
    const double* q2,
    const double* r2
) {
    return self->TrianglesIntersect(p1, q1, r1, p2, q2, r2);
}
extern "C" int vtk_triangle_point_in_triangle(
    vtkNew<vtkTriangle> self,
    const double* x,
    const double* x1,
    const double* x2,
    const double* x3,
    double tol2
) {
    return self->PointInTriangle(x, x1, x2, x3, tol2);
}
extern "C" vtkNew<vtkTriangleStrip> vtkTriangleStrip_new() {
    return vtkNew<vtkTriangleStrip>();
}
extern "C" void vtkTriangleStrip_destructor(vtkNew<vtkTriangleStrip> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTriangleStrip_get_ptr(vtkNew<vtkTriangleStrip> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_triangle_strip_get_cell_type(vtkNew<vtkTriangleStrip> self) {
    return self->GetCellType();
}
extern "C" int vtk_triangle_strip_get_cell_dimension(vtkNew<vtkTriangleStrip> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_triangle_strip_get_number_of_edges(vtkNew<vtkTriangleStrip> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_triangle_strip_get_number_of_faces(vtkNew<vtkTriangleStrip> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_triangle_strip_get_parametric_center(
    vtkNew<vtkTriangleStrip> self, double* pcoords
) {
    return self->GetParametricCenter(pcoords);
}
extern "C" vtkNew<vtkUndirectedGraph> vtkUndirectedGraph_new() {
    return vtkNew<vtkUndirectedGraph>();
}
extern "C" void vtkUndirectedGraph_destructor(vtkNew<vtkUndirectedGraph> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkUndirectedGraph_get_ptr(vtkNew<vtkUndirectedGraph> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_undirected_graph_get_in_degree(
    vtkNew<vtkUndirectedGraph> self, long long v
) {
    return self->GetInDegree(v);
}
extern "C" vtkNew<vtkUniformGrid> vtkUniformGrid_new() {
    return vtkNew<vtkUniformGrid>();
}
extern "C" void vtkUniformGrid_destructor(vtkNew<vtkUniformGrid> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkUniformGrid_get_ptr(vtkNew<vtkUniformGrid> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkUniformGridAMR> vtkUniformGridAMR_new() {
    return vtkNew<vtkUniformGridAMR>();
}
extern "C" void vtkUniformGridAMR_destructor(vtkNew<vtkUniformGridAMR> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkUniformGridAMR_get_ptr(vtkNew<vtkUniformGridAMR> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkUniformGridAMRDataIterator> vtkUniformGridAMRDataIterator_new() {
    return vtkNew<vtkUniformGridAMRDataIterator>();
}
extern "C" void vtkUniformGridAMRDataIterator_destructor(
    vtkNew<vtkUniformGridAMRDataIterator> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkUniformGridAMRDataIterator_get_ptr(
    vtkNew<vtkUniformGridAMRDataIterator> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkUniformGridAMRIterator> vtkUniformGridAMRIterator_new() {
    return vtkNew<vtkUniformGridAMRIterator>();
}
extern "C" void vtkUniformGridAMRIterator_destructor(vtkNew<vtkUniformGridAMRIterator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkUniformGridAMRIterator_get_ptr(vtkNew<vtkUniformGridAMRIterator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_uniform_grid_amr_iterator_go_to_first_item(
    vtkNew<vtkUniformGridAMRIterator> self
) {
    return self->GoToFirstItem();
}
extern "C" void vtk_uniform_grid_amr_iterator_go_to_next_item(
    vtkNew<vtkUniformGridAMRIterator> self
) {
    return self->GoToNextItem();
}
extern "C" unsigned int vtk_uniform_grid_amr_iterator_get_current_level(
    vtkNew<vtkUniformGridAMRIterator> self
) {
    return self->GetCurrentLevel();
}
extern "C" unsigned int vtk_uniform_grid_amr_iterator_get_current_index(
    vtkNew<vtkUniformGridAMRIterator> self
) {
    return self->GetCurrentIndex();
}
extern "C" vtkNew<vtkUniformHyperTreeGrid> vtkUniformHyperTreeGrid_new() {
    return vtkNew<vtkUniformHyperTreeGrid>();
}
extern "C" void vtkUniformHyperTreeGrid_destructor(vtkNew<vtkUniformHyperTreeGrid> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkUniformHyperTreeGrid_get_ptr(vtkNew<vtkUniformHyperTreeGrid> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_uniform_hyper_tree_grid_set_origin_double_double_double(
    vtkNew<vtkUniformHyperTreeGrid> self, double _arg1, double _arg2, double _arg3
) {
    return self->SetOrigin(_arg1, _arg2, _arg3);
}
extern "C" void vtk_uniform_hyper_tree_grid_set_origin_const_double_3(
    vtkNew<vtkUniformHyperTreeGrid> self, const double* _arg
) {
    return self->SetOrigin(_arg);
}
extern "C" void vtk_uniform_hyper_tree_grid_get_origin_double_3(
    vtkNew<vtkUniformHyperTreeGrid> self, double* _arg
) {
    return self->GetOrigin(_arg);
}
extern "C" void vtk_uniform_hyper_tree_grid_set_grid_scale_double_double_double(
    vtkNew<vtkUniformHyperTreeGrid> self, double p0, double p1, double p2
) {
    return self->SetGridScale(p0, p1, p2);
}
extern "C" void vtk_uniform_hyper_tree_grid_get_grid_scale_double_3(
    vtkNew<vtkUniformHyperTreeGrid> self, double* _arg
) {
    return self->GetGridScale(_arg);
}
extern "C" void vtk_uniform_hyper_tree_grid_set_grid_scale_double(
    vtkNew<vtkUniformHyperTreeGrid> self, double p0
) {
    return self->SetGridScale(p0);
}
extern "C" unsigned long vtk_uniform_hyper_tree_grid_get_actual_memory_size_bytes(
    vtkNew<vtkUniformHyperTreeGrid> self
) {
    return self->GetActualMemorySizeBytes();
}
extern "C" vtkNew<vtkUnstructuredGrid> vtkUnstructuredGrid_new() {
    return vtkNew<vtkUnstructuredGrid>();
}
extern "C" void vtkUnstructuredGrid_destructor(vtkNew<vtkUnstructuredGrid> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkUnstructuredGrid_get_ptr(vtkNew<vtkUnstructuredGrid> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_unstructured_grid_allocate_estimate(
    vtkNew<vtkUnstructuredGrid> self, long long numCells, long long maxCellSize
) {
    return self->AllocateEstimate(numCells, maxCellSize);
}
extern "C" bool vtk_unstructured_grid_allocate_exact(
    vtkNew<vtkUnstructuredGrid> self, long long numCells, long long connectivitySize
) {
    return self->AllocateExact(numCells, connectivitySize);
}
extern "C" void vtk_unstructured_grid_allocate(
    vtkNew<vtkUnstructuredGrid> self, long long numCells, int extSize
) {
    return self->Allocate(numCells, extSize);
}
extern "C" void vtk_unstructured_grid_reset(vtkNew<vtkUnstructuredGrid> self) {
    return self->Reset();
}
extern "C" int vtk_unstructured_grid_get_cell_type(
    vtkNew<vtkUnstructuredGrid> self, long long cellId
) {
    return self->GetCellType(cellId);
}
extern "C" long long vtk_unstructured_grid_get_cell_size(
    vtkNew<vtkUnstructuredGrid> self, long long cellId
) {
    return self->GetCellSize(cellId);
}
extern "C" void vtk_unstructured_grid_squeeze(vtkNew<vtkUnstructuredGrid> self) {
    return self->Squeeze();
}
extern "C" void vtk_unstructured_grid_initialize(vtkNew<vtkUnstructuredGrid> self) {
    return self->Initialize();
}
extern "C" int vtk_unstructured_grid_get_max_cell_size(vtkNew<vtkUnstructuredGrid> self) {
    return self->GetMaxCellSize();
}
extern "C" int vtk_unstructured_grid_get_max_spatial_dimension(vtkNew<vtkUnstructuredGrid> self) {
    return self->GetMaxSpatialDimension();
}
extern "C" void vtk_unstructured_grid_build_links(vtkNew<vtkUnstructuredGrid> self) {
    return self->BuildLinks();
}
extern "C" void vtk_unstructured_grid_remove_reference_to_cell(
    vtkNew<vtkUnstructuredGrid> self, long long ptId, long long cellId
) {
    return self->RemoveReferenceToCell(ptId, cellId);
}
extern "C" void vtk_unstructured_grid_add_reference_to_cell(
    vtkNew<vtkUnstructuredGrid> self, long long ptId, long long cellId
) {
    return self->AddReferenceToCell(ptId, cellId);
}
extern "C" void vtk_unstructured_grid_resize_cell_list(
    vtkNew<vtkUnstructuredGrid> self, long long ptId, int size
) {
    return self->ResizeCellList(ptId, size);
}
extern "C" int vtk_unstructured_grid_get_piece(vtkNew<vtkUnstructuredGrid> self) {
    return self->GetPiece();
}
extern "C" int vtk_unstructured_grid_get_number_of_pieces(vtkNew<vtkUnstructuredGrid> self) {
    return self->GetNumberOfPieces();
}
extern "C" int vtk_unstructured_grid_get_ghost_level(vtkNew<vtkUnstructuredGrid> self) {
    return self->GetGhostLevel();
}
extern "C" int vtk_unstructured_grid_is_homogeneous(vtkNew<vtkUnstructuredGrid> self) {
    return self->IsHomogeneous();
}
extern "C" void vtk_unstructured_grid_remove_ghost_cells(vtkNew<vtkUnstructuredGrid> self) {
    return self->RemoveGhostCells();
}
extern "C" int vtk_unstructured_grid_initialize_faces_representation(
    vtkNew<vtkUnstructuredGrid> self, long long numPrevCells
) {
    return self->InitializeFacesRepresentation(numPrevCells);
}
extern "C" unsigned long vtk_unstructured_grid_get_mesh_m_time(vtkNew<vtkUnstructuredGrid> self) {
    return self->GetMeshMTime();
}
extern "C" vtkNew<vtkUnstructuredGridCellIterator> vtkUnstructuredGridCellIterator_new() {
    return vtkNew<vtkUnstructuredGridCellIterator>();
}
extern "C" void vtkUnstructuredGridCellIterator_destructor(
    vtkNew<vtkUnstructuredGridCellIterator> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkUnstructuredGridCellIterator_get_ptr(
    vtkNew<vtkUnstructuredGridCellIterator> sself
) {
    return sself.GetPointer();
}
extern "C" bool vtk_unstructured_grid_cell_iterator_is_done_with_traversal(
    vtkNew<vtkUnstructuredGridCellIterator> self
) {
    return self->IsDoneWithTraversal();
}
extern "C" long long vtk_unstructured_grid_cell_iterator_get_cell_id(
    vtkNew<vtkUnstructuredGridCellIterator> self
) {
    return self->GetCellId();
}
extern "C" void vtk_unstructured_grid_cell_iterator_go_to_cell(
    vtkNew<vtkUnstructuredGridCellIterator> self, long long cellId
) {
    return self->GoToCell(cellId);
}
extern "C" vtkNew<vtkVertex> vtkVertex_new() {
    return vtkNew<vtkVertex>();
}
extern "C" void vtkVertex_destructor(vtkNew<vtkVertex> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkVertex_get_ptr(vtkNew<vtkVertex> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_vertex_get_cell_type(vtkNew<vtkVertex> self) {
    return self->GetCellType();
}
extern "C" int vtk_vertex_get_cell_dimension(vtkNew<vtkVertex> self) {
    return self->GetCellDimension();
}
extern "C" int vtk_vertex_get_number_of_edges(vtkNew<vtkVertex> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_vertex_get_number_of_faces(vtkNew<vtkVertex> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_vertex_inflate(vtkNew<vtkVertex> self, double p0) {
    return self->Inflate(p0);
}
extern "C" int vtk_vertex_get_parametric_center(vtkNew<vtkVertex> self, double* pcoords) {
    return self->GetParametricCenter(pcoords);
}
extern "C" void vtk_vertex_interpolation_functions(
    vtkNew<vtkVertex> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_vertex_interpolation_derivs(
    vtkNew<vtkVertex> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_vertex_interpolate_functions(
    vtkNew<vtkVertex> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_vertex_interpolate_derivs(
    vtkNew<vtkVertex> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkVertexListIterator> vtkVertexListIterator_new() {
    return vtkNew<vtkVertexListIterator>();
}
extern "C" void vtkVertexListIterator_destructor(vtkNew<vtkVertexListIterator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkVertexListIterator_get_ptr(vtkNew<vtkVertexListIterator> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_vertex_list_iterator_next(vtkNew<vtkVertexListIterator> self) {
    return self->Next();
}
extern "C" bool vtk_vertex_list_iterator_has_next(vtkNew<vtkVertexListIterator> self) {
    return self->HasNext();
}
extern "C" vtkNew<vtkVoxel> vtkVoxel_new() {
    return vtkNew<vtkVoxel>();
}
extern "C" void vtkVoxel_destructor(vtkNew<vtkVoxel> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkVoxel_get_ptr(vtkNew<vtkVoxel> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_voxel_get_centroid(vtkNew<vtkVoxel> self, double* centroid) {
    return self->GetCentroid(centroid);
}
extern "C" double vtk_voxel_compute_bounding_sphere(vtkNew<vtkVoxel> self, double* center) {
    return self->ComputeBoundingSphere(center);
}
extern "C" int vtk_voxel_get_cell_type(vtkNew<vtkVoxel> self) {
    return self->GetCellType();
}
extern "C" int vtk_voxel_get_number_of_edges(vtkNew<vtkVoxel> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_voxel_get_number_of_faces(vtkNew<vtkVoxel> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_voxel_inflate(vtkNew<vtkVoxel> self, double dist) {
    return self->Inflate(dist);
}
extern "C" void vtk_voxel_interpolation_derivs(
    vtkNew<vtkVoxel> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_voxel_interpolate_functions(
    vtkNew<vtkVoxel> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_voxel_interpolate_derivs(
    vtkNew<vtkVoxel> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" void vtk_voxel_interpolation_functions(
    vtkNew<vtkVoxel> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" vtkNew<vtkWedge> vtkWedge_new() {
    return vtkNew<vtkWedge>();
}
extern "C" void vtkWedge_destructor(vtkNew<vtkWedge> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkWedge_get_ptr(vtkNew<vtkWedge> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_wedge_get_centroid(vtkNew<vtkWedge> self, double* centroid) {
    return self->GetCentroid(centroid);
}
extern "C" int vtk_wedge_get_cell_type(vtkNew<vtkWedge> self) {
    return self->GetCellType();
}
extern "C" int vtk_wedge_get_number_of_edges(vtkNew<vtkWedge> self) {
    return self->GetNumberOfEdges();
}
extern "C" int vtk_wedge_get_number_of_faces(vtkNew<vtkWedge> self) {
    return self->GetNumberOfFaces();
}
extern "C" int vtk_wedge_get_parametric_center(vtkNew<vtkWedge> self, double* pcoords) {
    return self->GetParametricCenter(pcoords);
}
extern "C" void vtk_wedge_interpolation_functions(
    vtkNew<vtkWedge> self, const double* pcoords, double* weights
) {
    return self->InterpolationFunctions(pcoords, weights);
}
extern "C" void vtk_wedge_interpolation_derivs(
    vtkNew<vtkWedge> self, const double* pcoords, double* derivs
) {
    return self->InterpolationDerivs(pcoords, derivs);
}
extern "C" void vtk_wedge_interpolate_functions(
    vtkNew<vtkWedge> self, const double* pcoords, double* weights
) {
    return self->InterpolateFunctions(pcoords, weights);
}
extern "C" void vtk_wedge_interpolate_derivs(
    vtkNew<vtkWedge> self, const double* pcoords, double* derivs
) {
    return self->InterpolateDerivs(pcoords, derivs);
}
extern "C" vtkNew<vtkXMLDataElement> vtkXMLDataElement_new() {
    return vtkNew<vtkXMLDataElement>();
}
extern "C" void vtkXMLDataElement_destructor(vtkNew<vtkXMLDataElement> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkXMLDataElement_get_ptr(vtkNew<vtkXMLDataElement> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_xml_data_element_get_number_of_attributes(vtkNew<vtkXMLDataElement> self) {
    return self->GetNumberOfAttributes();
}
extern "C" void vtk_xml_data_element_remove_all_attributes(vtkNew<vtkXMLDataElement> self) {
    return self->RemoveAllAttributes();
}
extern "C" int vtk_xml_data_element_get_number_of_nested_elements(vtkNew<vtkXMLDataElement> self) {
    return self->GetNumberOfNestedElements();
}
extern "C" void vtk_xml_data_element_remove_all_nested_elements(vtkNew<vtkXMLDataElement> self) {
    return self->RemoveAllNestedElements();
}
extern "C" long long vtk_xml_data_element_get_xml_byte_index(vtkNew<vtkXMLDataElement> self) {
    return self->GetXMLByteIndex();
}
extern "C" void vtk_xml_data_element_set_xml_byte_index(
    vtkNew<vtkXMLDataElement> self, long long _arg
) {
    return self->SetXMLByteIndex(_arg);
}
extern "C" void vtk_xml_data_element_set_attribute_encoding(
    vtkNew<vtkXMLDataElement> self, int _arg
) {
    return self->SetAttributeEncoding(_arg);
}
extern "C" int vtk_xml_data_element_get_attribute_encoding_min_value(
    vtkNew<vtkXMLDataElement> self
) {
    return self->GetAttributeEncodingMinValue();
}
extern "C" int vtk_xml_data_element_get_attribute_encoding_max_value(
    vtkNew<vtkXMLDataElement> self
) {
    return self->GetAttributeEncodingMaxValue();
}
extern "C" int vtk_xml_data_element_get_attribute_encoding(vtkNew<vtkXMLDataElement> self) {
    return self->GetAttributeEncoding();
}
extern "C" int vtk_xml_data_element_get_character_data_width(vtkNew<vtkXMLDataElement> self) {
    return self->GetCharacterDataWidth();
}
extern "C" void vtk_xml_data_element_set_character_data_width(
    vtkNew<vtkXMLDataElement> self, int _arg
) {
    return self->SetCharacterDataWidth(_arg);
}
