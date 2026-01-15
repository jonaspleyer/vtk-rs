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
#include<vtkAbstractCellArray.h>
#include<vtkAbstractCellLinks.h>
#include<vtkAbstractCellLocator.h>
#include<vtkAbstractElectronicData.h>
#include<vtkAbstractPointLocator.h>
#include<vtkAdjacentVertexIterator.h>
#include<vtkAngularPeriodicDataArray.h>
#include<vtkAnimationScene.h>
#include<vtkAnnotation.h>
#include<vtkAnnotationLayers.h>
#include<vtkAnnulus.h>
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
#include<vtkCellAttribute.h>
#include<vtkCellAttributeCalculator.h>
#include<vtkCellData.h>
#include<vtkCellGrid.h>
#include<vtkCellGridBoundsQuery.h>
#include<vtkCellGridCopyQuery.h>
#include<vtkCellGridEvaluator.h>
#include<vtkCellGridQuery.h>
#include<vtkCellGridRangeQuery.h>
#include<vtkCellGridResponder.h>
#include<vtkCellGridResponderBase.h>
#include<vtkCellGridResponders.h>
#include<vtkCellGridSidesCache.h>
#include<vtkCellGridSidesQuery.h>
#include<vtkCellIterator.h>
#include<vtkCellLinks.h>
#include<vtkCellLocator.h>
#include<vtkCellLocatorStrategy.h>
#include<vtkCellMetadata.h>
#include<vtkCellTreeLocator.h>
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
#include<vtkCoordinateFrame.h>
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
#include<vtkHierarchicalBoxDataSet.h>
#include<vtkHigherOrderCurve.h>
#include<vtkHigherOrderHexahedron.h>
#include<vtkHigherOrderInterpolation.h>
#include<vtkHigherOrderQuadrilateral.h>
#include<vtkHigherOrderTetra.h>
#include<vtkHigherOrderTriangle.h>
#include<vtkHigherOrderWedge.h>
#include<vtkHyperTree.h>
#include<vtkHyperTreeGrid.h>
#include<vtkHyperTreeGridGeometricLocator.h>
#include<vtkHyperTreeGridLocator.h>
#include<vtkHyperTreeGridNonOrientedCursor.h>
#include<vtkHyperTreeGridNonOrientedGeometryCursor.h>
#include<vtkHyperTreeGridNonOrientedMooreSuperCursor.h>
#include<vtkHyperTreeGridNonOrientedMooreSuperCursorLight.h>
#include<vtkHyperTreeGridNonOrientedSuperCursor.h>
#include<vtkHyperTreeGridNonOrientedSuperCursorLight.h>
#include<vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor.h>
#include<vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor.h>
#include<vtkHyperTreeGridNonOrientedUnlimitedSuperCursor.h>
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
#include<vtkPolyhedronUtilities.h>
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
#include<vtkSphericalPointIterator.h>
#include<vtkSpline.h>
#include<vtkStaticCellLinks.h>
#include<vtkStaticCellLinksTemplate.h>
#include<vtkStaticCellLocator.h>
#include<vtkStaticEdgeLocatorTemplate.h>
#include<vtkStaticFaceHashLinksTemplate.h>
#include<vtkStaticPointLocator.h>
#include<vtkStaticPointLocator2D.h>
#include<vtkStructuredCellArray.h>
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
extern "C" vtkNew < vtkAMRDataInternals > vtkAMRDataInternals_new () {return vtkNew < vtkAMRDataInternals > () ;}
extern "C" void vtkAMRDataInternals_destructor (vtkNew < vtkAMRDataInternals > sself) {sself . Reset () ; return ;}
extern "C" void * vtkAMRDataInternals_get_ptr (vtkNew < vtkAMRDataInternals > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkAdjacentVertexIterator > vtkAdjacentVertexIterator_new () {return vtkNew < vtkAdjacentVertexIterator > () ;}
extern "C" void vtkAdjacentVertexIterator_destructor (vtkNew < vtkAdjacentVertexIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkAdjacentVertexIterator_get_ptr (vtkNew < vtkAdjacentVertexIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkAnimationScene > vtkAnimationScene_new () {return vtkNew < vtkAnimationScene > () ;}
extern "C" void vtkAnimationScene_destructor (vtkNew < vtkAnimationScene > sself) {sself . Reset () ; return ;}
extern "C" void * vtkAnimationScene_get_ptr (vtkNew < vtkAnimationScene > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkAnnotation > vtkAnnotation_new () {return vtkNew < vtkAnnotation > () ;}
extern "C" void vtkAnnotation_destructor (vtkNew < vtkAnnotation > sself) {sself . Reset () ; return ;}
extern "C" void * vtkAnnotation_get_ptr (vtkNew < vtkAnnotation > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkAnnotationLayers > vtkAnnotationLayers_new () {return vtkNew < vtkAnnotationLayers > () ;}
extern "C" void vtkAnnotationLayers_destructor (vtkNew < vtkAnnotationLayers > sself) {sself . Reset () ; return ;}
extern "C" void * vtkAnnotationLayers_get_ptr (vtkNew < vtkAnnotationLayers > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkAnnulus > vtkAnnulus_new () {return vtkNew < vtkAnnulus > () ;}
extern "C" void vtkAnnulus_destructor (vtkNew < vtkAnnulus > sself) {sself . Reset () ; return ;}
extern "C" void * vtkAnnulus_get_ptr (vtkNew < vtkAnnulus > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkArrayData > vtkArrayData_new () {return vtkNew < vtkArrayData > () ;}
extern "C" void vtkArrayData_destructor (vtkNew < vtkArrayData > sself) {sself . Reset () ; return ;}
extern "C" void * vtkArrayData_get_ptr (vtkNew < vtkArrayData > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkAttributesErrorMetric > vtkAttributesErrorMetric_new () {return vtkNew < vtkAttributesErrorMetric > () ;}
extern "C" void vtkAttributesErrorMetric_destructor (vtkNew < vtkAttributesErrorMetric > sself) {sself . Reset () ; return ;}
extern "C" void * vtkAttributesErrorMetric_get_ptr (vtkNew < vtkAttributesErrorMetric > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBSPCuts > vtkBSPCuts_new () {return vtkNew < vtkBSPCuts > () ;}
extern "C" void vtkBSPCuts_destructor (vtkNew < vtkBSPCuts > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBSPCuts_get_ptr (vtkNew < vtkBSPCuts > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBSPIntersections > vtkBSPIntersections_new () {return vtkNew < vtkBSPIntersections > () ;}
extern "C" void vtkBSPIntersections_destructor (vtkNew < vtkBSPIntersections > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBSPIntersections_get_ptr (vtkNew < vtkBSPIntersections > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBezierCurve > vtkBezierCurve_new () {return vtkNew < vtkBezierCurve > () ;}
extern "C" void vtkBezierCurve_destructor (vtkNew < vtkBezierCurve > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBezierCurve_get_ptr (vtkNew < vtkBezierCurve > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBezierHexahedron > vtkBezierHexahedron_new () {return vtkNew < vtkBezierHexahedron > () ;}
extern "C" void vtkBezierHexahedron_destructor (vtkNew < vtkBezierHexahedron > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBezierHexahedron_get_ptr (vtkNew < vtkBezierHexahedron > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBezierInterpolation > vtkBezierInterpolation_new () {return vtkNew < vtkBezierInterpolation > () ;}
extern "C" void vtkBezierInterpolation_destructor (vtkNew < vtkBezierInterpolation > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBezierInterpolation_get_ptr (vtkNew < vtkBezierInterpolation > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBezierQuadrilateral > vtkBezierQuadrilateral_new () {return vtkNew < vtkBezierQuadrilateral > () ;}
extern "C" void vtkBezierQuadrilateral_destructor (vtkNew < vtkBezierQuadrilateral > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBezierQuadrilateral_get_ptr (vtkNew < vtkBezierQuadrilateral > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBezierTetra > vtkBezierTetra_new () {return vtkNew < vtkBezierTetra > () ;}
extern "C" void vtkBezierTetra_destructor (vtkNew < vtkBezierTetra > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBezierTetra_get_ptr (vtkNew < vtkBezierTetra > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBezierTriangle > vtkBezierTriangle_new () {return vtkNew < vtkBezierTriangle > () ;}
extern "C" void vtkBezierTriangle_destructor (vtkNew < vtkBezierTriangle > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBezierTriangle_get_ptr (vtkNew < vtkBezierTriangle > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBezierWedge > vtkBezierWedge_new () {return vtkNew < vtkBezierWedge > () ;}
extern "C" void vtkBezierWedge_destructor (vtkNew < vtkBezierWedge > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBezierWedge_get_ptr (vtkNew < vtkBezierWedge > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBiQuadraticQuad > vtkBiQuadraticQuad_new () {return vtkNew < vtkBiQuadraticQuad > () ;}
extern "C" void vtkBiQuadraticQuad_destructor (vtkNew < vtkBiQuadraticQuad > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBiQuadraticQuad_get_ptr (vtkNew < vtkBiQuadraticQuad > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBiQuadraticQuadraticHexahedron > vtkBiQuadraticQuadraticHexahedron_new () {return vtkNew < vtkBiQuadraticQuadraticHexahedron > () ;}
extern "C" void vtkBiQuadraticQuadraticHexahedron_destructor (vtkNew < vtkBiQuadraticQuadraticHexahedron > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBiQuadraticQuadraticHexahedron_get_ptr (vtkNew < vtkBiQuadraticQuadraticHexahedron > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBiQuadraticQuadraticWedge > vtkBiQuadraticQuadraticWedge_new () {return vtkNew < vtkBiQuadraticQuadraticWedge > () ;}
extern "C" void vtkBiQuadraticQuadraticWedge_destructor (vtkNew < vtkBiQuadraticQuadraticWedge > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBiQuadraticQuadraticWedge_get_ptr (vtkNew < vtkBiQuadraticQuadraticWedge > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBiQuadraticTriangle > vtkBiQuadraticTriangle_new () {return vtkNew < vtkBiQuadraticTriangle > () ;}
extern "C" void vtkBiQuadraticTriangle_destructor (vtkNew < vtkBiQuadraticTriangle > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBiQuadraticTriangle_get_ptr (vtkNew < vtkBiQuadraticTriangle > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBox > vtkBox_new () {return vtkNew < vtkBox > () ;}
extern "C" void vtkBox_destructor (vtkNew < vtkBox > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBox_get_ptr (vtkNew < vtkBox > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellArray > vtkCellArray_new () {return vtkNew < vtkCellArray > () ;}
extern "C" void vtkCellArray_destructor (vtkNew < vtkCellArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellArray_get_ptr (vtkNew < vtkCellArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellArrayIterator > vtkCellArrayIterator_new () {return vtkNew < vtkCellArrayIterator > () ;}
extern "C" void vtkCellArrayIterator_destructor (vtkNew < vtkCellArrayIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellArrayIterator_get_ptr (vtkNew < vtkCellArrayIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellAttribute > vtkCellAttribute_new () {return vtkNew < vtkCellAttribute > () ;}
extern "C" void vtkCellAttribute_destructor (vtkNew < vtkCellAttribute > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellAttribute_get_ptr (vtkNew < vtkCellAttribute > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellAttributeCalculator > vtkCellAttributeCalculator_new () {return vtkNew < vtkCellAttributeCalculator > () ;}
extern "C" void vtkCellAttributeCalculator_destructor (vtkNew < vtkCellAttributeCalculator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellAttributeCalculator_get_ptr (vtkNew < vtkCellAttributeCalculator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellData > vtkCellData_new () {return vtkNew < vtkCellData > () ;}
extern "C" void vtkCellData_destructor (vtkNew < vtkCellData > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellData_get_ptr (vtkNew < vtkCellData > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellGrid > vtkCellGrid_new () {return vtkNew < vtkCellGrid > () ;}
extern "C" void vtkCellGrid_destructor (vtkNew < vtkCellGrid > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellGrid_get_ptr (vtkNew < vtkCellGrid > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellGridBoundsQuery > vtkCellGridBoundsQuery_new () {return vtkNew < vtkCellGridBoundsQuery > () ;}
extern "C" void vtkCellGridBoundsQuery_destructor (vtkNew < vtkCellGridBoundsQuery > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellGridBoundsQuery_get_ptr (vtkNew < vtkCellGridBoundsQuery > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellGridCopyQuery > vtkCellGridCopyQuery_new () {return vtkNew < vtkCellGridCopyQuery > () ;}
extern "C" void vtkCellGridCopyQuery_destructor (vtkNew < vtkCellGridCopyQuery > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellGridCopyQuery_get_ptr (vtkNew < vtkCellGridCopyQuery > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellGridEvaluator > vtkCellGridEvaluator_new () {return vtkNew < vtkCellGridEvaluator > () ;}
extern "C" void vtkCellGridEvaluator_destructor (vtkNew < vtkCellGridEvaluator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellGridEvaluator_get_ptr (vtkNew < vtkCellGridEvaluator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellGridRangeQuery > vtkCellGridRangeQuery_new () {return vtkNew < vtkCellGridRangeQuery > () ;}
extern "C" void vtkCellGridRangeQuery_destructor (vtkNew < vtkCellGridRangeQuery > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellGridRangeQuery_get_ptr (vtkNew < vtkCellGridRangeQuery > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellGridResponders > vtkCellGridResponders_new () {return vtkNew < vtkCellGridResponders > () ;}
extern "C" void vtkCellGridResponders_destructor (vtkNew < vtkCellGridResponders > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellGridResponders_get_ptr (vtkNew < vtkCellGridResponders > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellGridSidesCache > vtkCellGridSidesCache_new () {return vtkNew < vtkCellGridSidesCache > () ;}
extern "C" void vtkCellGridSidesCache_destructor (vtkNew < vtkCellGridSidesCache > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellGridSidesCache_get_ptr (vtkNew < vtkCellGridSidesCache > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellGridSidesQuery > vtkCellGridSidesQuery_new () {return vtkNew < vtkCellGridSidesQuery > () ;}
extern "C" void vtkCellGridSidesQuery_destructor (vtkNew < vtkCellGridSidesQuery > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellGridSidesQuery_get_ptr (vtkNew < vtkCellGridSidesQuery > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellLinks > vtkCellLinks_new () {return vtkNew < vtkCellLinks > () ;}
extern "C" void vtkCellLinks_destructor (vtkNew < vtkCellLinks > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellLinks_get_ptr (vtkNew < vtkCellLinks > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellLocator > vtkCellLocator_new () {return vtkNew < vtkCellLocator > () ;}
extern "C" void vtkCellLocator_destructor (vtkNew < vtkCellLocator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellLocator_get_ptr (vtkNew < vtkCellLocator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellLocatorStrategy > vtkCellLocatorStrategy_new () {return vtkNew < vtkCellLocatorStrategy > () ;}
extern "C" void vtkCellLocatorStrategy_destructor (vtkNew < vtkCellLocatorStrategy > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellLocatorStrategy_get_ptr (vtkNew < vtkCellLocatorStrategy > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellTreeLocator > vtkCellTreeLocator_new () {return vtkNew < vtkCellTreeLocator > () ;}
extern "C" void vtkCellTreeLocator_destructor (vtkNew < vtkCellTreeLocator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellTreeLocator_get_ptr (vtkNew < vtkCellTreeLocator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCellTypes > vtkCellTypes_new () {return vtkNew < vtkCellTypes > () ;}
extern "C" void vtkCellTypes_destructor (vtkNew < vtkCellTypes > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCellTypes_get_ptr (vtkNew < vtkCellTypes > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkClosestNPointsStrategy > vtkClosestNPointsStrategy_new () {return vtkNew < vtkClosestNPointsStrategy > () ;}
extern "C" void vtkClosestNPointsStrategy_destructor (vtkNew < vtkClosestNPointsStrategy > sself) {sself . Reset () ; return ;}
extern "C" void * vtkClosestNPointsStrategy_get_ptr (vtkNew < vtkClosestNPointsStrategy > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkClosestPointStrategy > vtkClosestPointStrategy_new () {return vtkNew < vtkClosestPointStrategy > () ;}
extern "C" void vtkClosestPointStrategy_destructor (vtkNew < vtkClosestPointStrategy > sself) {sself . Reset () ; return ;}
extern "C" void * vtkClosestPointStrategy_get_ptr (vtkNew < vtkClosestPointStrategy > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCone > vtkCone_new () {return vtkNew < vtkCone > () ;}
extern "C" void vtkCone_destructor (vtkNew < vtkCone > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCone_get_ptr (vtkNew < vtkCone > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkConvexPointSet > vtkConvexPointSet_new () {return vtkNew < vtkConvexPointSet > () ;}
extern "C" void vtkConvexPointSet_destructor (vtkNew < vtkConvexPointSet > sself) {sself . Reset () ; return ;}
extern "C" void * vtkConvexPointSet_get_ptr (vtkNew < vtkConvexPointSet > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCoordinateFrame > vtkCoordinateFrame_new () {return vtkNew < vtkCoordinateFrame > () ;}
extern "C" void vtkCoordinateFrame_destructor (vtkNew < vtkCoordinateFrame > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCoordinateFrame_get_ptr (vtkNew < vtkCoordinateFrame > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCubicLine > vtkCubicLine_new () {return vtkNew < vtkCubicLine > () ;}
extern "C" void vtkCubicLine_destructor (vtkNew < vtkCubicLine > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCubicLine_get_ptr (vtkNew < vtkCubicLine > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCylinder > vtkCylinder_new () {return vtkNew < vtkCylinder > () ;}
extern "C" void vtkCylinder_destructor (vtkNew < vtkCylinder > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCylinder_get_ptr (vtkNew < vtkCylinder > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDataAssembly > vtkDataAssembly_new () {return vtkNew < vtkDataAssembly > () ;}
extern "C" void vtkDataAssembly_destructor (vtkNew < vtkDataAssembly > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDataAssembly_get_ptr (vtkNew < vtkDataAssembly > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDataAssemblyUtilities > vtkDataAssemblyUtilities_new () {return vtkNew < vtkDataAssemblyUtilities > () ;}
extern "C" void vtkDataAssemblyUtilities_destructor (vtkNew < vtkDataAssemblyUtilities > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDataAssemblyUtilities_get_ptr (vtkNew < vtkDataAssemblyUtilities > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDataObject > vtkDataObject_new () {return vtkNew < vtkDataObject > () ;}
extern "C" void vtkDataObject_destructor (vtkNew < vtkDataObject > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDataObject_get_ptr (vtkNew < vtkDataObject > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDataObjectCollection > vtkDataObjectCollection_new () {return vtkNew < vtkDataObjectCollection > () ;}
extern "C" void vtkDataObjectCollection_destructor (vtkNew < vtkDataObjectCollection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDataObjectCollection_get_ptr (vtkNew < vtkDataObjectCollection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDataObjectTreeIterator > vtkDataObjectTreeIterator_new () {return vtkNew < vtkDataObjectTreeIterator > () ;}
extern "C" void vtkDataObjectTreeIterator_destructor (vtkNew < vtkDataObjectTreeIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDataObjectTreeIterator_get_ptr (vtkNew < vtkDataObjectTreeIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDataObjectTypes > vtkDataObjectTypes_new () {return vtkNew < vtkDataObjectTypes > () ;}
extern "C" void vtkDataObjectTypes_destructor (vtkNew < vtkDataObjectTypes > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDataObjectTypes_get_ptr (vtkNew < vtkDataObjectTypes > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDataSetAttributes > vtkDataSetAttributes_new () {return vtkNew < vtkDataSetAttributes > () ;}
extern "C" void vtkDataSetAttributes_destructor (vtkNew < vtkDataSetAttributes > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDataSetAttributes_get_ptr (vtkNew < vtkDataSetAttributes > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDataSetCellIterator > vtkDataSetCellIterator_new () {return vtkNew < vtkDataSetCellIterator > () ;}
extern "C" void vtkDataSetCellIterator_destructor (vtkNew < vtkDataSetCellIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDataSetCellIterator_get_ptr (vtkNew < vtkDataSetCellIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDataSetCollection > vtkDataSetCollection_new () {return vtkNew < vtkDataSetCollection > () ;}
extern "C" void vtkDataSetCollection_destructor (vtkNew < vtkDataSetCollection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDataSetCollection_get_ptr (vtkNew < vtkDataSetCollection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDirectedAcyclicGraph > vtkDirectedAcyclicGraph_new () {return vtkNew < vtkDirectedAcyclicGraph > () ;}
extern "C" void vtkDirectedAcyclicGraph_destructor (vtkNew < vtkDirectedAcyclicGraph > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDirectedAcyclicGraph_get_ptr (vtkNew < vtkDirectedAcyclicGraph > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDirectedGraph > vtkDirectedGraph_new () {return vtkNew < vtkDirectedGraph > () ;}
extern "C" void vtkDirectedGraph_destructor (vtkNew < vtkDirectedGraph > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDirectedGraph_get_ptr (vtkNew < vtkDirectedGraph > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkEdgeListIterator > vtkEdgeListIterator_new () {return vtkNew < vtkEdgeListIterator > () ;}
extern "C" void vtkEdgeListIterator_destructor (vtkNew < vtkEdgeListIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkEdgeListIterator_get_ptr (vtkNew < vtkEdgeListIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkEdgeTable > vtkEdgeTable_new () {return vtkNew < vtkEdgeTable > () ;}
extern "C" void vtkEdgeTable_destructor (vtkNew < vtkEdgeTable > sself) {sself . Reset () ; return ;}
extern "C" void * vtkEdgeTable_get_ptr (vtkNew < vtkEdgeTable > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkEmptyCell > vtkEmptyCell_new () {return vtkNew < vtkEmptyCell > () ;}
extern "C" void vtkEmptyCell_destructor (vtkNew < vtkEmptyCell > sself) {sself . Reset () ; return ;}
extern "C" void * vtkEmptyCell_get_ptr (vtkNew < vtkEmptyCell > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkExplicitStructuredGrid > vtkExplicitStructuredGrid_new () {return vtkNew < vtkExplicitStructuredGrid > () ;}
extern "C" void vtkExplicitStructuredGrid_destructor (vtkNew < vtkExplicitStructuredGrid > sself) {sself . Reset () ; return ;}
extern "C" void * vtkExplicitStructuredGrid_get_ptr (vtkNew < vtkExplicitStructuredGrid > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkExtractStructuredGridHelper > vtkExtractStructuredGridHelper_new () {return vtkNew < vtkExtractStructuredGridHelper > () ;}
extern "C" void vtkExtractStructuredGridHelper_destructor (vtkNew < vtkExtractStructuredGridHelper > sself) {sself . Reset () ; return ;}
extern "C" void * vtkExtractStructuredGridHelper_get_ptr (vtkNew < vtkExtractStructuredGridHelper > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkFieldData > vtkFieldData_new () {return vtkNew < vtkFieldData > () ;}
extern "C" void vtkFieldData_destructor (vtkNew < vtkFieldData > sself) {sself . Reset () ; return ;}
extern "C" void * vtkFieldData_get_ptr (vtkNew < vtkFieldData > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkGenericAttributeCollection > vtkGenericAttributeCollection_new () {return vtkNew < vtkGenericAttributeCollection > () ;}
extern "C" void vtkGenericAttributeCollection_destructor (vtkNew < vtkGenericAttributeCollection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkGenericAttributeCollection_get_ptr (vtkNew < vtkGenericAttributeCollection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkGenericCell > vtkGenericCell_new () {return vtkNew < vtkGenericCell > () ;}
extern "C" void vtkGenericCell_destructor (vtkNew < vtkGenericCell > sself) {sself . Reset () ; return ;}
extern "C" void * vtkGenericCell_get_ptr (vtkNew < vtkGenericCell > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkGenericEdgeTable > vtkGenericEdgeTable_new () {return vtkNew < vtkGenericEdgeTable > () ;}
extern "C" void vtkGenericEdgeTable_destructor (vtkNew < vtkGenericEdgeTable > sself) {sself . Reset () ; return ;}
extern "C" void * vtkGenericEdgeTable_get_ptr (vtkNew < vtkGenericEdgeTable > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkGenericInterpolatedVelocityField > vtkGenericInterpolatedVelocityField_new () {return vtkNew < vtkGenericInterpolatedVelocityField > () ;}
extern "C" void vtkGenericInterpolatedVelocityField_destructor (vtkNew < vtkGenericInterpolatedVelocityField > sself) {sself . Reset () ; return ;}
extern "C" void * vtkGenericInterpolatedVelocityField_get_ptr (vtkNew < vtkGenericInterpolatedVelocityField > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkGeometricErrorMetric > vtkGeometricErrorMetric_new () {return vtkNew < vtkGeometricErrorMetric > () ;}
extern "C" void vtkGeometricErrorMetric_destructor (vtkNew < vtkGeometricErrorMetric > sself) {sself . Reset () ; return ;}
extern "C" void * vtkGeometricErrorMetric_get_ptr (vtkNew < vtkGeometricErrorMetric > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkGraphEdge > vtkGraphEdge_new () {return vtkNew < vtkGraphEdge > () ;}
extern "C" void vtkGraphEdge_destructor (vtkNew < vtkGraphEdge > sself) {sself . Reset () ; return ;}
extern "C" void * vtkGraphEdge_get_ptr (vtkNew < vtkGraphEdge > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkGraphInternals > vtkGraphInternals_new () {return vtkNew < vtkGraphInternals > () ;}
extern "C" void vtkGraphInternals_destructor (vtkNew < vtkGraphInternals > sself) {sself . Reset () ; return ;}
extern "C" void * vtkGraphInternals_get_ptr (vtkNew < vtkGraphInternals > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHexagonalPrism > vtkHexagonalPrism_new () {return vtkNew < vtkHexagonalPrism > () ;}
extern "C" void vtkHexagonalPrism_destructor (vtkNew < vtkHexagonalPrism > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHexagonalPrism_get_ptr (vtkNew < vtkHexagonalPrism > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHexahedron > vtkHexahedron_new () {return vtkNew < vtkHexahedron > () ;}
extern "C" void vtkHexahedron_destructor (vtkNew < vtkHexahedron > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHexahedron_get_ptr (vtkNew < vtkHexahedron > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHierarchicalBoxDataSet > vtkHierarchicalBoxDataSet_new () {return vtkNew < vtkHierarchicalBoxDataSet > () ;}
extern "C" void vtkHierarchicalBoxDataSet_destructor (vtkNew < vtkHierarchicalBoxDataSet > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHierarchicalBoxDataSet_get_ptr (vtkNew < vtkHierarchicalBoxDataSet > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHyperTreeGrid > vtkHyperTreeGrid_new () {return vtkNew < vtkHyperTreeGrid > () ;}
extern "C" void vtkHyperTreeGrid_destructor (vtkNew < vtkHyperTreeGrid > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHyperTreeGrid_get_ptr (vtkNew < vtkHyperTreeGrid > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHyperTreeGridGeometricLocator > vtkHyperTreeGridGeometricLocator_new () {return vtkNew < vtkHyperTreeGridGeometricLocator > () ;}
extern "C" void vtkHyperTreeGridGeometricLocator_destructor (vtkNew < vtkHyperTreeGridGeometricLocator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHyperTreeGridGeometricLocator_get_ptr (vtkNew < vtkHyperTreeGridGeometricLocator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHyperTreeGridNonOrientedCursor > vtkHyperTreeGridNonOrientedCursor_new () {return vtkNew < vtkHyperTreeGridNonOrientedCursor > () ;}
extern "C" void vtkHyperTreeGridNonOrientedCursor_destructor (vtkNew < vtkHyperTreeGridNonOrientedCursor > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHyperTreeGridNonOrientedCursor_get_ptr (vtkNew < vtkHyperTreeGridNonOrientedCursor > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHyperTreeGridNonOrientedGeometryCursor > vtkHyperTreeGridNonOrientedGeometryCursor_new () {return vtkNew < vtkHyperTreeGridNonOrientedGeometryCursor > () ;}
extern "C" void vtkHyperTreeGridNonOrientedGeometryCursor_destructor (vtkNew < vtkHyperTreeGridNonOrientedGeometryCursor > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHyperTreeGridNonOrientedGeometryCursor_get_ptr (vtkNew < vtkHyperTreeGridNonOrientedGeometryCursor > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHyperTreeGridNonOrientedMooreSuperCursor > vtkHyperTreeGridNonOrientedMooreSuperCursor_new () {return vtkNew < vtkHyperTreeGridNonOrientedMooreSuperCursor > () ;}
extern "C" void vtkHyperTreeGridNonOrientedMooreSuperCursor_destructor (vtkNew < vtkHyperTreeGridNonOrientedMooreSuperCursor > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHyperTreeGridNonOrientedMooreSuperCursor_get_ptr (vtkNew < vtkHyperTreeGridNonOrientedMooreSuperCursor > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHyperTreeGridNonOrientedMooreSuperCursorLight > vtkHyperTreeGridNonOrientedMooreSuperCursorLight_new () {return vtkNew < vtkHyperTreeGridNonOrientedMooreSuperCursorLight > () ;}
extern "C" void vtkHyperTreeGridNonOrientedMooreSuperCursorLight_destructor (vtkNew < vtkHyperTreeGridNonOrientedMooreSuperCursorLight > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHyperTreeGridNonOrientedMooreSuperCursorLight_get_ptr (vtkNew < vtkHyperTreeGridNonOrientedMooreSuperCursorLight > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor > vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor_new () {return vtkNew < vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor > () ;}
extern "C" void vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor_destructor (vtkNew < vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor_get_ptr (vtkNew < vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor > vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor_new () {return vtkNew < vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor > () ;}
extern "C" void vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor_destructor (vtkNew < vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor_get_ptr (vtkNew < vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHyperTreeGridNonOrientedVonNeumannSuperCursor > vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_new () {return vtkNew < vtkHyperTreeGridNonOrientedVonNeumannSuperCursor > () ;}
extern "C" void vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_destructor (vtkNew < vtkHyperTreeGridNonOrientedVonNeumannSuperCursor > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_get_ptr (vtkNew < vtkHyperTreeGridNonOrientedVonNeumannSuperCursor > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight > vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_new () {return vtkNew < vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight > () ;}
extern "C" void vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_destructor (vtkNew < vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_get_ptr (vtkNew < vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHyperTreeGridOrientedCursor > vtkHyperTreeGridOrientedCursor_new () {return vtkNew < vtkHyperTreeGridOrientedCursor > () ;}
extern "C" void vtkHyperTreeGridOrientedCursor_destructor (vtkNew < vtkHyperTreeGridOrientedCursor > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHyperTreeGridOrientedCursor_get_ptr (vtkNew < vtkHyperTreeGridOrientedCursor > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHyperTreeGridOrientedGeometryCursor > vtkHyperTreeGridOrientedGeometryCursor_new () {return vtkNew < vtkHyperTreeGridOrientedGeometryCursor > () ;}
extern "C" void vtkHyperTreeGridOrientedGeometryCursor_destructor (vtkNew < vtkHyperTreeGridOrientedGeometryCursor > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHyperTreeGridOrientedGeometryCursor_get_ptr (vtkNew < vtkHyperTreeGridOrientedGeometryCursor > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkImageData > vtkImageData_new () {return vtkNew < vtkImageData > () ;}
extern "C" void vtkImageData_destructor (vtkNew < vtkImageData > sself) {sself . Reset () ; return ;}
extern "C" void * vtkImageData_get_ptr (vtkNew < vtkImageData > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkImageTransform > vtkImageTransform_new () {return vtkNew < vtkImageTransform > () ;}
extern "C" void vtkImageTransform_destructor (vtkNew < vtkImageTransform > sself) {sself . Reset () ; return ;}
extern "C" void * vtkImageTransform_get_ptr (vtkNew < vtkImageTransform > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkImplicitBoolean > vtkImplicitBoolean_new () {return vtkNew < vtkImplicitBoolean > () ;}
extern "C" void vtkImplicitBoolean_destructor (vtkNew < vtkImplicitBoolean > sself) {sself . Reset () ; return ;}
extern "C" void * vtkImplicitBoolean_get_ptr (vtkNew < vtkImplicitBoolean > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkImplicitDataSet > vtkImplicitDataSet_new () {return vtkNew < vtkImplicitDataSet > () ;}
extern "C" void vtkImplicitDataSet_destructor (vtkNew < vtkImplicitDataSet > sself) {sself . Reset () ; return ;}
extern "C" void * vtkImplicitDataSet_get_ptr (vtkNew < vtkImplicitDataSet > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkImplicitFunctionCollection > vtkImplicitFunctionCollection_new () {return vtkNew < vtkImplicitFunctionCollection > () ;}
extern "C" void vtkImplicitFunctionCollection_destructor (vtkNew < vtkImplicitFunctionCollection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkImplicitFunctionCollection_get_ptr (vtkNew < vtkImplicitFunctionCollection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkImplicitHalo > vtkImplicitHalo_new () {return vtkNew < vtkImplicitHalo > () ;}
extern "C" void vtkImplicitHalo_destructor (vtkNew < vtkImplicitHalo > sself) {sself . Reset () ; return ;}
extern "C" void * vtkImplicitHalo_get_ptr (vtkNew < vtkImplicitHalo > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkImplicitSelectionLoop > vtkImplicitSelectionLoop_new () {return vtkNew < vtkImplicitSelectionLoop > () ;}
extern "C" void vtkImplicitSelectionLoop_destructor (vtkNew < vtkImplicitSelectionLoop > sself) {sself . Reset () ; return ;}
extern "C" void * vtkImplicitSelectionLoop_get_ptr (vtkNew < vtkImplicitSelectionLoop > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkImplicitSum > vtkImplicitSum_new () {return vtkNew < vtkImplicitSum > () ;}
extern "C" void vtkImplicitSum_destructor (vtkNew < vtkImplicitSum > sself) {sself . Reset () ; return ;}
extern "C" void * vtkImplicitSum_get_ptr (vtkNew < vtkImplicitSum > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkImplicitVolume > vtkImplicitVolume_new () {return vtkNew < vtkImplicitVolume > () ;}
extern "C" void vtkImplicitVolume_destructor (vtkNew < vtkImplicitVolume > sself) {sself . Reset () ; return ;}
extern "C" void * vtkImplicitVolume_get_ptr (vtkNew < vtkImplicitVolume > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkImplicitWindowFunction > vtkImplicitWindowFunction_new () {return vtkNew < vtkImplicitWindowFunction > () ;}
extern "C" void vtkImplicitWindowFunction_destructor (vtkNew < vtkImplicitWindowFunction > sself) {sself . Reset () ; return ;}
extern "C" void * vtkImplicitWindowFunction_get_ptr (vtkNew < vtkImplicitWindowFunction > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkInEdgeIterator > vtkInEdgeIterator_new () {return vtkNew < vtkInEdgeIterator > () ;}
extern "C" void vtkInEdgeIterator_destructor (vtkNew < vtkInEdgeIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkInEdgeIterator_get_ptr (vtkNew < vtkInEdgeIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkIncrementalOctreeNode > vtkIncrementalOctreeNode_new () {return vtkNew < vtkIncrementalOctreeNode > () ;}
extern "C" void vtkIncrementalOctreeNode_destructor (vtkNew < vtkIncrementalOctreeNode > sself) {sself . Reset () ; return ;}
extern "C" void * vtkIncrementalOctreeNode_get_ptr (vtkNew < vtkIncrementalOctreeNode > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkIncrementalOctreePointLocator > vtkIncrementalOctreePointLocator_new () {return vtkNew < vtkIncrementalOctreePointLocator > () ;}
extern "C" void vtkIncrementalOctreePointLocator_destructor (vtkNew < vtkIncrementalOctreePointLocator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkIncrementalOctreePointLocator_get_ptr (vtkNew < vtkIncrementalOctreePointLocator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkIterativeClosestPointTransform > vtkIterativeClosestPointTransform_new () {return vtkNew < vtkIterativeClosestPointTransform > () ;}
extern "C" void vtkIterativeClosestPointTransform_destructor (vtkNew < vtkIterativeClosestPointTransform > sself) {sself . Reset () ; return ;}
extern "C" void * vtkIterativeClosestPointTransform_get_ptr (vtkNew < vtkIterativeClosestPointTransform > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkKdNode > vtkKdNode_new () {return vtkNew < vtkKdNode > () ;}
extern "C" void vtkKdNode_destructor (vtkNew < vtkKdNode > sself) {sself . Reset () ; return ;}
extern "C" void * vtkKdNode_get_ptr (vtkNew < vtkKdNode > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkKdTree > vtkKdTree_new () {return vtkNew < vtkKdTree > () ;}
extern "C" void vtkKdTree_destructor (vtkNew < vtkKdTree > sself) {sself . Reset () ; return ;}
extern "C" void * vtkKdTree_get_ptr (vtkNew < vtkKdTree > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkKdTreePointLocator > vtkKdTreePointLocator_new () {return vtkNew < vtkKdTreePointLocator > () ;}
extern "C" void vtkKdTreePointLocator_destructor (vtkNew < vtkKdTreePointLocator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkKdTreePointLocator_get_ptr (vtkNew < vtkKdTreePointLocator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkLagrangeCurve > vtkLagrangeCurve_new () {return vtkNew < vtkLagrangeCurve > () ;}
extern "C" void vtkLagrangeCurve_destructor (vtkNew < vtkLagrangeCurve > sself) {sself . Reset () ; return ;}
extern "C" void * vtkLagrangeCurve_get_ptr (vtkNew < vtkLagrangeCurve > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkLagrangeHexahedron > vtkLagrangeHexahedron_new () {return vtkNew < vtkLagrangeHexahedron > () ;}
extern "C" void vtkLagrangeHexahedron_destructor (vtkNew < vtkLagrangeHexahedron > sself) {sself . Reset () ; return ;}
extern "C" void * vtkLagrangeHexahedron_get_ptr (vtkNew < vtkLagrangeHexahedron > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkLagrangeInterpolation > vtkLagrangeInterpolation_new () {return vtkNew < vtkLagrangeInterpolation > () ;}
extern "C" void vtkLagrangeInterpolation_destructor (vtkNew < vtkLagrangeInterpolation > sself) {sself . Reset () ; return ;}
extern "C" void * vtkLagrangeInterpolation_get_ptr (vtkNew < vtkLagrangeInterpolation > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkLagrangeQuadrilateral > vtkLagrangeQuadrilateral_new () {return vtkNew < vtkLagrangeQuadrilateral > () ;}
extern "C" void vtkLagrangeQuadrilateral_destructor (vtkNew < vtkLagrangeQuadrilateral > sself) {sself . Reset () ; return ;}
extern "C" void * vtkLagrangeQuadrilateral_get_ptr (vtkNew < vtkLagrangeQuadrilateral > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkLagrangeTetra > vtkLagrangeTetra_new () {return vtkNew < vtkLagrangeTetra > () ;}
extern "C" void vtkLagrangeTetra_destructor (vtkNew < vtkLagrangeTetra > sself) {sself . Reset () ; return ;}
extern "C" void * vtkLagrangeTetra_get_ptr (vtkNew < vtkLagrangeTetra > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkLagrangeTriangle > vtkLagrangeTriangle_new () {return vtkNew < vtkLagrangeTriangle > () ;}
extern "C" void vtkLagrangeTriangle_destructor (vtkNew < vtkLagrangeTriangle > sself) {sself . Reset () ; return ;}
extern "C" void * vtkLagrangeTriangle_get_ptr (vtkNew < vtkLagrangeTriangle > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkLagrangeWedge > vtkLagrangeWedge_new () {return vtkNew < vtkLagrangeWedge > () ;}
extern "C" void vtkLagrangeWedge_destructor (vtkNew < vtkLagrangeWedge > sself) {sself . Reset () ; return ;}
extern "C" void * vtkLagrangeWedge_get_ptr (vtkNew < vtkLagrangeWedge > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkLine > vtkLine_new () {return vtkNew < vtkLine > () ;}
extern "C" void vtkLine_destructor (vtkNew < vtkLine > sself) {sself . Reset () ; return ;}
extern "C" void * vtkLine_get_ptr (vtkNew < vtkLine > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMeanValueCoordinatesInterpolator > vtkMeanValueCoordinatesInterpolator_new () {return vtkNew < vtkMeanValueCoordinatesInterpolator > () ;}
extern "C" void vtkMeanValueCoordinatesInterpolator_destructor (vtkNew < vtkMeanValueCoordinatesInterpolator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMeanValueCoordinatesInterpolator_get_ptr (vtkNew < vtkMeanValueCoordinatesInterpolator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMergePoints > vtkMergePoints_new () {return vtkNew < vtkMergePoints > () ;}
extern "C" void vtkMergePoints_destructor (vtkNew < vtkMergePoints > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMergePoints_get_ptr (vtkNew < vtkMergePoints > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMolecule > vtkMolecule_new () {return vtkNew < vtkMolecule > () ;}
extern "C" void vtkMolecule_destructor (vtkNew < vtkMolecule > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMolecule_get_ptr (vtkNew < vtkMolecule > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMultiBlockDataSet > vtkMultiBlockDataSet_new () {return vtkNew < vtkMultiBlockDataSet > () ;}
extern "C" void vtkMultiBlockDataSet_destructor (vtkNew < vtkMultiBlockDataSet > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMultiBlockDataSet_get_ptr (vtkNew < vtkMultiBlockDataSet > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMultiPieceDataSet > vtkMultiPieceDataSet_new () {return vtkNew < vtkMultiPieceDataSet > () ;}
extern "C" void vtkMultiPieceDataSet_destructor (vtkNew < vtkMultiPieceDataSet > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMultiPieceDataSet_get_ptr (vtkNew < vtkMultiPieceDataSet > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMutableDirectedGraph > vtkMutableDirectedGraph_new () {return vtkNew < vtkMutableDirectedGraph > () ;}
extern "C" void vtkMutableDirectedGraph_destructor (vtkNew < vtkMutableDirectedGraph > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMutableDirectedGraph_get_ptr (vtkNew < vtkMutableDirectedGraph > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMutableUndirectedGraph > vtkMutableUndirectedGraph_new () {return vtkNew < vtkMutableUndirectedGraph > () ;}
extern "C" void vtkMutableUndirectedGraph_destructor (vtkNew < vtkMutableUndirectedGraph > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMutableUndirectedGraph_get_ptr (vtkNew < vtkMutableUndirectedGraph > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkNonMergingPointLocator > vtkNonMergingPointLocator_new () {return vtkNew < vtkNonMergingPointLocator > () ;}
extern "C" void vtkNonMergingPointLocator_destructor (vtkNew < vtkNonMergingPointLocator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkNonMergingPointLocator_get_ptr (vtkNew < vtkNonMergingPointLocator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkNonOverlappingAMR > vtkNonOverlappingAMR_new () {return vtkNew < vtkNonOverlappingAMR > () ;}
extern "C" void vtkNonOverlappingAMR_destructor (vtkNew < vtkNonOverlappingAMR > sself) {sself . Reset () ; return ;}
extern "C" void * vtkNonOverlappingAMR_get_ptr (vtkNew < vtkNonOverlappingAMR > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkOctreePointLocator > vtkOctreePointLocator_new () {return vtkNew < vtkOctreePointLocator > () ;}
extern "C" void vtkOctreePointLocator_destructor (vtkNew < vtkOctreePointLocator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkOctreePointLocator_get_ptr (vtkNew < vtkOctreePointLocator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkOctreePointLocatorNode > vtkOctreePointLocatorNode_new () {return vtkNew < vtkOctreePointLocatorNode > () ;}
extern "C" void vtkOctreePointLocatorNode_destructor (vtkNew < vtkOctreePointLocatorNode > sself) {sself . Reset () ; return ;}
extern "C" void * vtkOctreePointLocatorNode_get_ptr (vtkNew < vtkOctreePointLocatorNode > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkOrderedTriangulator > vtkOrderedTriangulator_new () {return vtkNew < vtkOrderedTriangulator > () ;}
extern "C" void vtkOrderedTriangulator_destructor (vtkNew < vtkOrderedTriangulator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkOrderedTriangulator_get_ptr (vtkNew < vtkOrderedTriangulator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkOutEdgeIterator > vtkOutEdgeIterator_new () {return vtkNew < vtkOutEdgeIterator > () ;}
extern "C" void vtkOutEdgeIterator_destructor (vtkNew < vtkOutEdgeIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkOutEdgeIterator_get_ptr (vtkNew < vtkOutEdgeIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkOverlappingAMR > vtkOverlappingAMR_new () {return vtkNew < vtkOverlappingAMR > () ;}
extern "C" void vtkOverlappingAMR_destructor (vtkNew < vtkOverlappingAMR > sself) {sself . Reset () ; return ;}
extern "C" void * vtkOverlappingAMR_get_ptr (vtkNew < vtkOverlappingAMR > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPartitionedDataSet > vtkPartitionedDataSet_new () {return vtkNew < vtkPartitionedDataSet > () ;}
extern "C" void vtkPartitionedDataSet_destructor (vtkNew < vtkPartitionedDataSet > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPartitionedDataSet_get_ptr (vtkNew < vtkPartitionedDataSet > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPartitionedDataSetCollection > vtkPartitionedDataSetCollection_new () {return vtkNew < vtkPartitionedDataSetCollection > () ;}
extern "C" void vtkPartitionedDataSetCollection_destructor (vtkNew < vtkPartitionedDataSetCollection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPartitionedDataSetCollection_get_ptr (vtkNew < vtkPartitionedDataSetCollection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPath > vtkPath_new () {return vtkNew < vtkPath > () ;}
extern "C" void vtkPath_destructor (vtkNew < vtkPath > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPath_get_ptr (vtkNew < vtkPath > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPentagonalPrism > vtkPentagonalPrism_new () {return vtkNew < vtkPentagonalPrism > () ;}
extern "C" void vtkPentagonalPrism_destructor (vtkNew < vtkPentagonalPrism > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPentagonalPrism_get_ptr (vtkNew < vtkPentagonalPrism > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPerlinNoise > vtkPerlinNoise_new () {return vtkNew < vtkPerlinNoise > () ;}
extern "C" void vtkPerlinNoise_destructor (vtkNew < vtkPerlinNoise > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPerlinNoise_get_ptr (vtkNew < vtkPerlinNoise > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPiecewiseFunction > vtkPiecewiseFunction_new () {return vtkNew < vtkPiecewiseFunction > () ;}
extern "C" void vtkPiecewiseFunction_destructor (vtkNew < vtkPiecewiseFunction > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPiecewiseFunction_get_ptr (vtkNew < vtkPiecewiseFunction > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPixel > vtkPixel_new () {return vtkNew < vtkPixel > () ;}
extern "C" void vtkPixel_destructor (vtkNew < vtkPixel > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPixel_get_ptr (vtkNew < vtkPixel > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPlane > vtkPlane_new () {return vtkNew < vtkPlane > () ;}
extern "C" void vtkPlane_destructor (vtkNew < vtkPlane > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPlane_get_ptr (vtkNew < vtkPlane > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPlaneCollection > vtkPlaneCollection_new () {return vtkNew < vtkPlaneCollection > () ;}
extern "C" void vtkPlaneCollection_destructor (vtkNew < vtkPlaneCollection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPlaneCollection_get_ptr (vtkNew < vtkPlaneCollection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPlanes > vtkPlanes_new () {return vtkNew < vtkPlanes > () ;}
extern "C" void vtkPlanes_destructor (vtkNew < vtkPlanes > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPlanes_get_ptr (vtkNew < vtkPlanes > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPlanesIntersection > vtkPlanesIntersection_new () {return vtkNew < vtkPlanesIntersection > () ;}
extern "C" void vtkPlanesIntersection_destructor (vtkNew < vtkPlanesIntersection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPlanesIntersection_get_ptr (vtkNew < vtkPlanesIntersection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPointData > vtkPointData_new () {return vtkNew < vtkPointData > () ;}
extern "C" void vtkPointData_destructor (vtkNew < vtkPointData > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPointData_get_ptr (vtkNew < vtkPointData > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPointLocator > vtkPointLocator_new () {return vtkNew < vtkPointLocator > () ;}
extern "C" void vtkPointLocator_destructor (vtkNew < vtkPointLocator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPointLocator_get_ptr (vtkNew < vtkPointLocator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPointSet > vtkPointSet_new () {return vtkNew < vtkPointSet > () ;}
extern "C" void vtkPointSet_destructor (vtkNew < vtkPointSet > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPointSet_get_ptr (vtkNew < vtkPointSet > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPointSetCellIterator > vtkPointSetCellIterator_new () {return vtkNew < vtkPointSetCellIterator > () ;}
extern "C" void vtkPointSetCellIterator_destructor (vtkNew < vtkPointSetCellIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPointSetCellIterator_get_ptr (vtkNew < vtkPointSetCellIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPointsProjectedHull > vtkPointsProjectedHull_new () {return vtkNew < vtkPointsProjectedHull > () ;}
extern "C" void vtkPointsProjectedHull_destructor (vtkNew < vtkPointsProjectedHull > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPointsProjectedHull_get_ptr (vtkNew < vtkPointsProjectedHull > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPolyData > vtkPolyData_new () {return vtkNew < vtkPolyData > () ;}
extern "C" void vtkPolyData_destructor (vtkNew < vtkPolyData > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPolyData_get_ptr (vtkNew < vtkPolyData > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPolyDataCollection > vtkPolyDataCollection_new () {return vtkNew < vtkPolyDataCollection > () ;}
extern "C" void vtkPolyDataCollection_destructor (vtkNew < vtkPolyDataCollection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPolyDataCollection_get_ptr (vtkNew < vtkPolyDataCollection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPolyLine > vtkPolyLine_new () {return vtkNew < vtkPolyLine > () ;}
extern "C" void vtkPolyLine_destructor (vtkNew < vtkPolyLine > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPolyLine_get_ptr (vtkNew < vtkPolyLine > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPolyPlane > vtkPolyPlane_new () {return vtkNew < vtkPolyPlane > () ;}
extern "C" void vtkPolyPlane_destructor (vtkNew < vtkPolyPlane > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPolyPlane_get_ptr (vtkNew < vtkPolyPlane > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPolyVertex > vtkPolyVertex_new () {return vtkNew < vtkPolyVertex > () ;}
extern "C" void vtkPolyVertex_destructor (vtkNew < vtkPolyVertex > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPolyVertex_get_ptr (vtkNew < vtkPolyVertex > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPolygon > vtkPolygon_new () {return vtkNew < vtkPolygon > () ;}
extern "C" void vtkPolygon_destructor (vtkNew < vtkPolygon > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPolygon_get_ptr (vtkNew < vtkPolygon > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPolyhedron > vtkPolyhedron_new () {return vtkNew < vtkPolyhedron > () ;}
extern "C" void vtkPolyhedron_destructor (vtkNew < vtkPolyhedron > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPolyhedron_get_ptr (vtkNew < vtkPolyhedron > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPyramid > vtkPyramid_new () {return vtkNew < vtkPyramid > () ;}
extern "C" void vtkPyramid_destructor (vtkNew < vtkPyramid > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPyramid_get_ptr (vtkNew < vtkPyramid > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkQuad > vtkQuad_new () {return vtkNew < vtkQuad > () ;}
extern "C" void vtkQuad_destructor (vtkNew < vtkQuad > sself) {sself . Reset () ; return ;}
extern "C" void * vtkQuad_get_ptr (vtkNew < vtkQuad > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkQuadraticEdge > vtkQuadraticEdge_new () {return vtkNew < vtkQuadraticEdge > () ;}
extern "C" void vtkQuadraticEdge_destructor (vtkNew < vtkQuadraticEdge > sself) {sself . Reset () ; return ;}
extern "C" void * vtkQuadraticEdge_get_ptr (vtkNew < vtkQuadraticEdge > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkQuadraticHexahedron > vtkQuadraticHexahedron_new () {return vtkNew < vtkQuadraticHexahedron > () ;}
extern "C" void vtkQuadraticHexahedron_destructor (vtkNew < vtkQuadraticHexahedron > sself) {sself . Reset () ; return ;}
extern "C" void * vtkQuadraticHexahedron_get_ptr (vtkNew < vtkQuadraticHexahedron > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkQuadraticLinearQuad > vtkQuadraticLinearQuad_new () {return vtkNew < vtkQuadraticLinearQuad > () ;}
extern "C" void vtkQuadraticLinearQuad_destructor (vtkNew < vtkQuadraticLinearQuad > sself) {sself . Reset () ; return ;}
extern "C" void * vtkQuadraticLinearQuad_get_ptr (vtkNew < vtkQuadraticLinearQuad > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkQuadraticLinearWedge > vtkQuadraticLinearWedge_new () {return vtkNew < vtkQuadraticLinearWedge > () ;}
extern "C" void vtkQuadraticLinearWedge_destructor (vtkNew < vtkQuadraticLinearWedge > sself) {sself . Reset () ; return ;}
extern "C" void * vtkQuadraticLinearWedge_get_ptr (vtkNew < vtkQuadraticLinearWedge > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkQuadraticPolygon > vtkQuadraticPolygon_new () {return vtkNew < vtkQuadraticPolygon > () ;}
extern "C" void vtkQuadraticPolygon_destructor (vtkNew < vtkQuadraticPolygon > sself) {sself . Reset () ; return ;}
extern "C" void * vtkQuadraticPolygon_get_ptr (vtkNew < vtkQuadraticPolygon > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkQuadraticPyramid > vtkQuadraticPyramid_new () {return vtkNew < vtkQuadraticPyramid > () ;}
extern "C" void vtkQuadraticPyramid_destructor (vtkNew < vtkQuadraticPyramid > sself) {sself . Reset () ; return ;}
extern "C" void * vtkQuadraticPyramid_get_ptr (vtkNew < vtkQuadraticPyramid > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkQuadraticQuad > vtkQuadraticQuad_new () {return vtkNew < vtkQuadraticQuad > () ;}
extern "C" void vtkQuadraticQuad_destructor (vtkNew < vtkQuadraticQuad > sself) {sself . Reset () ; return ;}
extern "C" void * vtkQuadraticQuad_get_ptr (vtkNew < vtkQuadraticQuad > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkQuadraticTetra > vtkQuadraticTetra_new () {return vtkNew < vtkQuadraticTetra > () ;}
extern "C" void vtkQuadraticTetra_destructor (vtkNew < vtkQuadraticTetra > sself) {sself . Reset () ; return ;}
extern "C" void * vtkQuadraticTetra_get_ptr (vtkNew < vtkQuadraticTetra > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkQuadraticTriangle > vtkQuadraticTriangle_new () {return vtkNew < vtkQuadraticTriangle > () ;}
extern "C" void vtkQuadraticTriangle_destructor (vtkNew < vtkQuadraticTriangle > sself) {sself . Reset () ; return ;}
extern "C" void * vtkQuadraticTriangle_get_ptr (vtkNew < vtkQuadraticTriangle > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkQuadraticWedge > vtkQuadraticWedge_new () {return vtkNew < vtkQuadraticWedge > () ;}
extern "C" void vtkQuadraticWedge_destructor (vtkNew < vtkQuadraticWedge > sself) {sself . Reset () ; return ;}
extern "C" void * vtkQuadraticWedge_get_ptr (vtkNew < vtkQuadraticWedge > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkQuadratureSchemeDefinition > vtkQuadratureSchemeDefinition_new () {return vtkNew < vtkQuadratureSchemeDefinition > () ;}
extern "C" void vtkQuadratureSchemeDefinition_destructor (vtkNew < vtkQuadratureSchemeDefinition > sself) {sself . Reset () ; return ;}
extern "C" void * vtkQuadratureSchemeDefinition_get_ptr (vtkNew < vtkQuadratureSchemeDefinition > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkQuadric > vtkQuadric_new () {return vtkNew < vtkQuadric > () ;}
extern "C" void vtkQuadric_destructor (vtkNew < vtkQuadric > sself) {sself . Reset () ; return ;}
extern "C" void * vtkQuadric_get_ptr (vtkNew < vtkQuadric > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkRectilinearGrid > vtkRectilinearGrid_new () {return vtkNew < vtkRectilinearGrid > () ;}
extern "C" void vtkRectilinearGrid_destructor (vtkNew < vtkRectilinearGrid > sself) {sself . Reset () ; return ;}
extern "C" void * vtkRectilinearGrid_get_ptr (vtkNew < vtkRectilinearGrid > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkReebGraph > vtkReebGraph_new () {return vtkNew < vtkReebGraph > () ;}
extern "C" void vtkReebGraph_destructor (vtkNew < vtkReebGraph > sself) {sself . Reset () ; return ;}
extern "C" void * vtkReebGraph_get_ptr (vtkNew < vtkReebGraph > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkReebGraphSimplificationMetric > vtkReebGraphSimplificationMetric_new () {return vtkNew < vtkReebGraphSimplificationMetric > () ;}
extern "C" void vtkReebGraphSimplificationMetric_destructor (vtkNew < vtkReebGraphSimplificationMetric > sself) {sself . Reset () ; return ;}
extern "C" void * vtkReebGraphSimplificationMetric_get_ptr (vtkNew < vtkReebGraphSimplificationMetric > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSelection > vtkSelection_new () {return vtkNew < vtkSelection > () ;}
extern "C" void vtkSelection_destructor (vtkNew < vtkSelection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSelection_get_ptr (vtkNew < vtkSelection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSelectionNode > vtkSelectionNode_new () {return vtkNew < vtkSelectionNode > () ;}
extern "C" void vtkSelectionNode_destructor (vtkNew < vtkSelectionNode > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSelectionNode_get_ptr (vtkNew < vtkSelectionNode > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSimpleCellTessellator > vtkSimpleCellTessellator_new () {return vtkNew < vtkSimpleCellTessellator > () ;}
extern "C" void vtkSimpleCellTessellator_destructor (vtkNew < vtkSimpleCellTessellator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSimpleCellTessellator_get_ptr (vtkNew < vtkSimpleCellTessellator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSmoothErrorMetric > vtkSmoothErrorMetric_new () {return vtkNew < vtkSmoothErrorMetric > () ;}
extern "C" void vtkSmoothErrorMetric_destructor (vtkNew < vtkSmoothErrorMetric > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSmoothErrorMetric_get_ptr (vtkNew < vtkSmoothErrorMetric > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSortFieldData > vtkSortFieldData_new () {return vtkNew < vtkSortFieldData > () ;}
extern "C" void vtkSortFieldData_destructor (vtkNew < vtkSortFieldData > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSortFieldData_get_ptr (vtkNew < vtkSortFieldData > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSphere > vtkSphere_new () {return vtkNew < vtkSphere > () ;}
extern "C" void vtkSphere_destructor (vtkNew < vtkSphere > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSphere_get_ptr (vtkNew < vtkSphere > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSpheres > vtkSpheres_new () {return vtkNew < vtkSpheres > () ;}
extern "C" void vtkSpheres_destructor (vtkNew < vtkSpheres > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSpheres_get_ptr (vtkNew < vtkSpheres > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSphericalPointIterator > vtkSphericalPointIterator_new () {return vtkNew < vtkSphericalPointIterator > () ;}
extern "C" void vtkSphericalPointIterator_destructor (vtkNew < vtkSphericalPointIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSphericalPointIterator_get_ptr (vtkNew < vtkSphericalPointIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkStaticCellLinks > vtkStaticCellLinks_new () {return vtkNew < vtkStaticCellLinks > () ;}
extern "C" void vtkStaticCellLinks_destructor (vtkNew < vtkStaticCellLinks > sself) {sself . Reset () ; return ;}
extern "C" void * vtkStaticCellLinks_get_ptr (vtkNew < vtkStaticCellLinks > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkStaticCellLocator > vtkStaticCellLocator_new () {return vtkNew < vtkStaticCellLocator > () ;}
extern "C" void vtkStaticCellLocator_destructor (vtkNew < vtkStaticCellLocator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkStaticCellLocator_get_ptr (vtkNew < vtkStaticCellLocator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkStaticPointLocator > vtkStaticPointLocator_new () {return vtkNew < vtkStaticPointLocator > () ;}
extern "C" void vtkStaticPointLocator_destructor (vtkNew < vtkStaticPointLocator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkStaticPointLocator_get_ptr (vtkNew < vtkStaticPointLocator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkStaticPointLocator2D > vtkStaticPointLocator2D_new () {return vtkNew < vtkStaticPointLocator2D > () ;}
extern "C" void vtkStaticPointLocator2D_destructor (vtkNew < vtkStaticPointLocator2D > sself) {sself . Reset () ; return ;}
extern "C" void * vtkStaticPointLocator2D_get_ptr (vtkNew < vtkStaticPointLocator2D > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkStructuredCellArray > vtkStructuredCellArray_new () {return vtkNew < vtkStructuredCellArray > () ;}
extern "C" void vtkStructuredCellArray_destructor (vtkNew < vtkStructuredCellArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkStructuredCellArray_get_ptr (vtkNew < vtkStructuredCellArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkStructuredExtent > vtkStructuredExtent_new () {return vtkNew < vtkStructuredExtent > () ;}
extern "C" void vtkStructuredExtent_destructor (vtkNew < vtkStructuredExtent > sself) {sself . Reset () ; return ;}
extern "C" void * vtkStructuredExtent_get_ptr (vtkNew < vtkStructuredExtent > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkStructuredGrid > vtkStructuredGrid_new () {return vtkNew < vtkStructuredGrid > () ;}
extern "C" void vtkStructuredGrid_destructor (vtkNew < vtkStructuredGrid > sself) {sself . Reset () ; return ;}
extern "C" void * vtkStructuredGrid_get_ptr (vtkNew < vtkStructuredGrid > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkStructuredPoints > vtkStructuredPoints_new () {return vtkNew < vtkStructuredPoints > () ;}
extern "C" void vtkStructuredPoints_destructor (vtkNew < vtkStructuredPoints > sself) {sself . Reset () ; return ;}
extern "C" void * vtkStructuredPoints_get_ptr (vtkNew < vtkStructuredPoints > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkStructuredPointsCollection > vtkStructuredPointsCollection_new () {return vtkNew < vtkStructuredPointsCollection > () ;}
extern "C" void vtkStructuredPointsCollection_destructor (vtkNew < vtkStructuredPointsCollection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkStructuredPointsCollection_get_ptr (vtkNew < vtkStructuredPointsCollection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSuperquadric > vtkSuperquadric_new () {return vtkNew < vtkSuperquadric > () ;}
extern "C" void vtkSuperquadric_destructor (vtkNew < vtkSuperquadric > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSuperquadric_get_ptr (vtkNew < vtkSuperquadric > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTable > vtkTable_new () {return vtkNew < vtkTable > () ;}
extern "C" void vtkTable_destructor (vtkNew < vtkTable > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTable_get_ptr (vtkNew < vtkTable > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTetra > vtkTetra_new () {return vtkNew < vtkTetra > () ;}
extern "C" void vtkTetra_destructor (vtkNew < vtkTetra > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTetra_get_ptr (vtkNew < vtkTetra > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTree > vtkTree_new () {return vtkNew < vtkTree > () ;}
extern "C" void vtkTree_destructor (vtkNew < vtkTree > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTree_get_ptr (vtkNew < vtkTree > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTreeBFSIterator > vtkTreeBFSIterator_new () {return vtkNew < vtkTreeBFSIterator > () ;}
extern "C" void vtkTreeBFSIterator_destructor (vtkNew < vtkTreeBFSIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTreeBFSIterator_get_ptr (vtkNew < vtkTreeBFSIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTreeDFSIterator > vtkTreeDFSIterator_new () {return vtkNew < vtkTreeDFSIterator > () ;}
extern "C" void vtkTreeDFSIterator_destructor (vtkNew < vtkTreeDFSIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTreeDFSIterator_get_ptr (vtkNew < vtkTreeDFSIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTriQuadraticHexahedron > vtkTriQuadraticHexahedron_new () {return vtkNew < vtkTriQuadraticHexahedron > () ;}
extern "C" void vtkTriQuadraticHexahedron_destructor (vtkNew < vtkTriQuadraticHexahedron > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTriQuadraticHexahedron_get_ptr (vtkNew < vtkTriQuadraticHexahedron > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTriQuadraticPyramid > vtkTriQuadraticPyramid_new () {return vtkNew < vtkTriQuadraticPyramid > () ;}
extern "C" void vtkTriQuadraticPyramid_destructor (vtkNew < vtkTriQuadraticPyramid > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTriQuadraticPyramid_get_ptr (vtkNew < vtkTriQuadraticPyramid > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTriangle > vtkTriangle_new () {return vtkNew < vtkTriangle > () ;}
extern "C" void vtkTriangle_destructor (vtkNew < vtkTriangle > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTriangle_get_ptr (vtkNew < vtkTriangle > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTriangleStrip > vtkTriangleStrip_new () {return vtkNew < vtkTriangleStrip > () ;}
extern "C" void vtkTriangleStrip_destructor (vtkNew < vtkTriangleStrip > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTriangleStrip_get_ptr (vtkNew < vtkTriangleStrip > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUndirectedGraph > vtkUndirectedGraph_new () {return vtkNew < vtkUndirectedGraph > () ;}
extern "C" void vtkUndirectedGraph_destructor (vtkNew < vtkUndirectedGraph > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUndirectedGraph_get_ptr (vtkNew < vtkUndirectedGraph > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUniformGrid > vtkUniformGrid_new () {return vtkNew < vtkUniformGrid > () ;}
extern "C" void vtkUniformGrid_destructor (vtkNew < vtkUniformGrid > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUniformGrid_get_ptr (vtkNew < vtkUniformGrid > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUniformGridAMR > vtkUniformGridAMR_new () {return vtkNew < vtkUniformGridAMR > () ;}
extern "C" void vtkUniformGridAMR_destructor (vtkNew < vtkUniformGridAMR > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUniformGridAMR_get_ptr (vtkNew < vtkUniformGridAMR > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUniformGridAMRDataIterator > vtkUniformGridAMRDataIterator_new () {return vtkNew < vtkUniformGridAMRDataIterator > () ;}
extern "C" void vtkUniformGridAMRDataIterator_destructor (vtkNew < vtkUniformGridAMRDataIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUniformGridAMRDataIterator_get_ptr (vtkNew < vtkUniformGridAMRDataIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUniformHyperTreeGrid > vtkUniformHyperTreeGrid_new () {return vtkNew < vtkUniformHyperTreeGrid > () ;}
extern "C" void vtkUniformHyperTreeGrid_destructor (vtkNew < vtkUniformHyperTreeGrid > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUniformHyperTreeGrid_get_ptr (vtkNew < vtkUniformHyperTreeGrid > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUnstructuredGrid > vtkUnstructuredGrid_new () {return vtkNew < vtkUnstructuredGrid > () ;}
extern "C" void vtkUnstructuredGrid_destructor (vtkNew < vtkUnstructuredGrid > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUnstructuredGrid_get_ptr (vtkNew < vtkUnstructuredGrid > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUnstructuredGridCellIterator > vtkUnstructuredGridCellIterator_new () {return vtkNew < vtkUnstructuredGridCellIterator > () ;}
extern "C" void vtkUnstructuredGridCellIterator_destructor (vtkNew < vtkUnstructuredGridCellIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUnstructuredGridCellIterator_get_ptr (vtkNew < vtkUnstructuredGridCellIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkVertex > vtkVertex_new () {return vtkNew < vtkVertex > () ;}
extern "C" void vtkVertex_destructor (vtkNew < vtkVertex > sself) {sself . Reset () ; return ;}
extern "C" void * vtkVertex_get_ptr (vtkNew < vtkVertex > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkVertexListIterator > vtkVertexListIterator_new () {return vtkNew < vtkVertexListIterator > () ;}
extern "C" void vtkVertexListIterator_destructor (vtkNew < vtkVertexListIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkVertexListIterator_get_ptr (vtkNew < vtkVertexListIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkVoxel > vtkVoxel_new () {return vtkNew < vtkVoxel > () ;}
extern "C" void vtkVoxel_destructor (vtkNew < vtkVoxel > sself) {sself . Reset () ; return ;}
extern "C" void * vtkVoxel_get_ptr (vtkNew < vtkVoxel > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkWedge > vtkWedge_new () {return vtkNew < vtkWedge > () ;}
extern "C" void vtkWedge_destructor (vtkNew < vtkWedge > sself) {sself . Reset () ; return ;}
extern "C" void * vtkWedge_get_ptr (vtkNew < vtkWedge > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkXMLDataElement > vtkXMLDataElement_new () {return vtkNew < vtkXMLDataElement > () ;}
extern "C" void vtkXMLDataElement_destructor (vtkNew < vtkXMLDataElement > sself) {sself . Reset () ; return ;}
extern "C" void * vtkXMLDataElement_get_ptr (vtkNew < vtkXMLDataElement > sself) {return sself . GetPointer () ;}
