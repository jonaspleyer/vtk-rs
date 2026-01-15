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
#include<vtkFrustum.h>
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

// Declare exported functions
extern "C" vtkNew < vtkAMRDataInternals > vtkAMRDataInternals_new () ;
extern "C" void vtkAMRDataInternals_destructor (vtkNew < vtkAMRDataInternals > sself) ;
extern "C" void * vtkAMRDataInternals_get_ptr (vtkNew < vtkAMRDataInternals > sself) ;
extern "C" vtkNew < vtkAdjacentVertexIterator > vtkAdjacentVertexIterator_new () ;
extern "C" void vtkAdjacentVertexIterator_destructor (vtkNew < vtkAdjacentVertexIterator > sself) ;
extern "C" void * vtkAdjacentVertexIterator_get_ptr (vtkNew < vtkAdjacentVertexIterator > sself) ;
extern "C" vtkNew < vtkAnimationScene > vtkAnimationScene_new () ;
extern "C" void vtkAnimationScene_destructor (vtkNew < vtkAnimationScene > sself) ;
extern "C" void * vtkAnimationScene_get_ptr (vtkNew < vtkAnimationScene > sself) ;
extern "C" vtkNew < vtkAnnotation > vtkAnnotation_new () ;
extern "C" void vtkAnnotation_destructor (vtkNew < vtkAnnotation > sself) ;
extern "C" void * vtkAnnotation_get_ptr (vtkNew < vtkAnnotation > sself) ;
extern "C" vtkNew < vtkAnnotationLayers > vtkAnnotationLayers_new () ;
extern "C" void vtkAnnotationLayers_destructor (vtkNew < vtkAnnotationLayers > sself) ;
extern "C" void * vtkAnnotationLayers_get_ptr (vtkNew < vtkAnnotationLayers > sself) ;
extern "C" vtkNew < vtkAnnulus > vtkAnnulus_new () ;
extern "C" void vtkAnnulus_destructor (vtkNew < vtkAnnulus > sself) ;
extern "C" void * vtkAnnulus_get_ptr (vtkNew < vtkAnnulus > sself) ;
extern "C" vtkNew < vtkArrayData > vtkArrayData_new () ;
extern "C" void vtkArrayData_destructor (vtkNew < vtkArrayData > sself) ;
extern "C" void * vtkArrayData_get_ptr (vtkNew < vtkArrayData > sself) ;
extern "C" vtkNew < vtkAttributesErrorMetric > vtkAttributesErrorMetric_new () ;
extern "C" void vtkAttributesErrorMetric_destructor (vtkNew < vtkAttributesErrorMetric > sself) ;
extern "C" void * vtkAttributesErrorMetric_get_ptr (vtkNew < vtkAttributesErrorMetric > sself) ;
extern "C" vtkNew < vtkBSPCuts > vtkBSPCuts_new () ;
extern "C" void vtkBSPCuts_destructor (vtkNew < vtkBSPCuts > sself) ;
extern "C" void * vtkBSPCuts_get_ptr (vtkNew < vtkBSPCuts > sself) ;
extern "C" vtkNew < vtkBSPIntersections > vtkBSPIntersections_new () ;
extern "C" void vtkBSPIntersections_destructor (vtkNew < vtkBSPIntersections > sself) ;
extern "C" void * vtkBSPIntersections_get_ptr (vtkNew < vtkBSPIntersections > sself) ;
extern "C" vtkNew < vtkBezierCurve > vtkBezierCurve_new () ;
extern "C" void vtkBezierCurve_destructor (vtkNew < vtkBezierCurve > sself) ;
extern "C" void * vtkBezierCurve_get_ptr (vtkNew < vtkBezierCurve > sself) ;
extern "C" vtkNew < vtkBezierHexahedron > vtkBezierHexahedron_new () ;
extern "C" void vtkBezierHexahedron_destructor (vtkNew < vtkBezierHexahedron > sself) ;
extern "C" void * vtkBezierHexahedron_get_ptr (vtkNew < vtkBezierHexahedron > sself) ;
extern "C" vtkNew < vtkBezierInterpolation > vtkBezierInterpolation_new () ;
extern "C" void vtkBezierInterpolation_destructor (vtkNew < vtkBezierInterpolation > sself) ;
extern "C" void * vtkBezierInterpolation_get_ptr (vtkNew < vtkBezierInterpolation > sself) ;
extern "C" vtkNew < vtkBezierQuadrilateral > vtkBezierQuadrilateral_new () ;
extern "C" void vtkBezierQuadrilateral_destructor (vtkNew < vtkBezierQuadrilateral > sself) ;
extern "C" void * vtkBezierQuadrilateral_get_ptr (vtkNew < vtkBezierQuadrilateral > sself) ;
extern "C" vtkNew < vtkBezierTetra > vtkBezierTetra_new () ;
extern "C" void vtkBezierTetra_destructor (vtkNew < vtkBezierTetra > sself) ;
extern "C" void * vtkBezierTetra_get_ptr (vtkNew < vtkBezierTetra > sself) ;
extern "C" vtkNew < vtkBezierTriangle > vtkBezierTriangle_new () ;
extern "C" void vtkBezierTriangle_destructor (vtkNew < vtkBezierTriangle > sself) ;
extern "C" void * vtkBezierTriangle_get_ptr (vtkNew < vtkBezierTriangle > sself) ;
extern "C" vtkNew < vtkBezierWedge > vtkBezierWedge_new () ;
extern "C" void vtkBezierWedge_destructor (vtkNew < vtkBezierWedge > sself) ;
extern "C" void * vtkBezierWedge_get_ptr (vtkNew < vtkBezierWedge > sself) ;
extern "C" vtkNew < vtkBiQuadraticQuad > vtkBiQuadraticQuad_new () ;
extern "C" void vtkBiQuadraticQuad_destructor (vtkNew < vtkBiQuadraticQuad > sself) ;
extern "C" void * vtkBiQuadraticQuad_get_ptr (vtkNew < vtkBiQuadraticQuad > sself) ;
extern "C" vtkNew < vtkBiQuadraticQuadraticHexahedron > vtkBiQuadraticQuadraticHexahedron_new () ;
extern "C" void vtkBiQuadraticQuadraticHexahedron_destructor (vtkNew < vtkBiQuadraticQuadraticHexahedron > sself) ;
extern "C" void * vtkBiQuadraticQuadraticHexahedron_get_ptr (vtkNew < vtkBiQuadraticQuadraticHexahedron > sself) ;
extern "C" vtkNew < vtkBiQuadraticQuadraticWedge > vtkBiQuadraticQuadraticWedge_new () ;
extern "C" void vtkBiQuadraticQuadraticWedge_destructor (vtkNew < vtkBiQuadraticQuadraticWedge > sself) ;
extern "C" void * vtkBiQuadraticQuadraticWedge_get_ptr (vtkNew < vtkBiQuadraticQuadraticWedge > sself) ;
extern "C" vtkNew < vtkBiQuadraticTriangle > vtkBiQuadraticTriangle_new () ;
extern "C" void vtkBiQuadraticTriangle_destructor (vtkNew < vtkBiQuadraticTriangle > sself) ;
extern "C" void * vtkBiQuadraticTriangle_get_ptr (vtkNew < vtkBiQuadraticTriangle > sself) ;
extern "C" vtkNew < vtkBox > vtkBox_new () ;
extern "C" void vtkBox_destructor (vtkNew < vtkBox > sself) ;
extern "C" void * vtkBox_get_ptr (vtkNew < vtkBox > sself) ;
extern "C" vtkNew < vtkCellArray > vtkCellArray_new () ;
extern "C" void vtkCellArray_destructor (vtkNew < vtkCellArray > sself) ;
extern "C" void * vtkCellArray_get_ptr (vtkNew < vtkCellArray > sself) ;
extern "C" vtkNew < vtkCellArrayIterator > vtkCellArrayIterator_new () ;
extern "C" void vtkCellArrayIterator_destructor (vtkNew < vtkCellArrayIterator > sself) ;
extern "C" void * vtkCellArrayIterator_get_ptr (vtkNew < vtkCellArrayIterator > sself) ;
extern "C" vtkNew < vtkCellAttribute > vtkCellAttribute_new () ;
extern "C" void vtkCellAttribute_destructor (vtkNew < vtkCellAttribute > sself) ;
extern "C" void * vtkCellAttribute_get_ptr (vtkNew < vtkCellAttribute > sself) ;
extern "C" vtkNew < vtkCellAttributeCalculator > vtkCellAttributeCalculator_new () ;
extern "C" void vtkCellAttributeCalculator_destructor (vtkNew < vtkCellAttributeCalculator > sself) ;
extern "C" void * vtkCellAttributeCalculator_get_ptr (vtkNew < vtkCellAttributeCalculator > sself) ;
extern "C" vtkNew < vtkCellData > vtkCellData_new () ;
extern "C" void vtkCellData_destructor (vtkNew < vtkCellData > sself) ;
extern "C" void * vtkCellData_get_ptr (vtkNew < vtkCellData > sself) ;
extern "C" vtkNew < vtkCellGrid > vtkCellGrid_new () ;
extern "C" void vtkCellGrid_destructor (vtkNew < vtkCellGrid > sself) ;
extern "C" void * vtkCellGrid_get_ptr (vtkNew < vtkCellGrid > sself) ;
extern "C" vtkNew < vtkCellGridBoundsQuery > vtkCellGridBoundsQuery_new () ;
extern "C" void vtkCellGridBoundsQuery_destructor (vtkNew < vtkCellGridBoundsQuery > sself) ;
extern "C" void * vtkCellGridBoundsQuery_get_ptr (vtkNew < vtkCellGridBoundsQuery > sself) ;
extern "C" vtkNew < vtkCellGridCopyQuery > vtkCellGridCopyQuery_new () ;
extern "C" void vtkCellGridCopyQuery_destructor (vtkNew < vtkCellGridCopyQuery > sself) ;
extern "C" void * vtkCellGridCopyQuery_get_ptr (vtkNew < vtkCellGridCopyQuery > sself) ;
extern "C" vtkNew < vtkCellGridEvaluator > vtkCellGridEvaluator_new () ;
extern "C" void vtkCellGridEvaluator_destructor (vtkNew < vtkCellGridEvaluator > sself) ;
extern "C" void * vtkCellGridEvaluator_get_ptr (vtkNew < vtkCellGridEvaluator > sself) ;
extern "C" vtkNew < vtkCellGridRangeQuery > vtkCellGridRangeQuery_new () ;
extern "C" void vtkCellGridRangeQuery_destructor (vtkNew < vtkCellGridRangeQuery > sself) ;
extern "C" void * vtkCellGridRangeQuery_get_ptr (vtkNew < vtkCellGridRangeQuery > sself) ;
extern "C" vtkNew < vtkCellGridResponders > vtkCellGridResponders_new () ;
extern "C" void vtkCellGridResponders_destructor (vtkNew < vtkCellGridResponders > sself) ;
extern "C" void * vtkCellGridResponders_get_ptr (vtkNew < vtkCellGridResponders > sself) ;
extern "C" vtkNew < vtkCellGridSidesCache > vtkCellGridSidesCache_new () ;
extern "C" void vtkCellGridSidesCache_destructor (vtkNew < vtkCellGridSidesCache > sself) ;
extern "C" void * vtkCellGridSidesCache_get_ptr (vtkNew < vtkCellGridSidesCache > sself) ;
extern "C" vtkNew < vtkCellGridSidesQuery > vtkCellGridSidesQuery_new () ;
extern "C" void vtkCellGridSidesQuery_destructor (vtkNew < vtkCellGridSidesQuery > sself) ;
extern "C" void * vtkCellGridSidesQuery_get_ptr (vtkNew < vtkCellGridSidesQuery > sself) ;
extern "C" vtkNew < vtkCellLinks > vtkCellLinks_new () ;
extern "C" void vtkCellLinks_destructor (vtkNew < vtkCellLinks > sself) ;
extern "C" void * vtkCellLinks_get_ptr (vtkNew < vtkCellLinks > sself) ;
extern "C" vtkNew < vtkCellLocator > vtkCellLocator_new () ;
extern "C" void vtkCellLocator_destructor (vtkNew < vtkCellLocator > sself) ;
extern "C" void * vtkCellLocator_get_ptr (vtkNew < vtkCellLocator > sself) ;
extern "C" vtkNew < vtkCellLocatorStrategy > vtkCellLocatorStrategy_new () ;
extern "C" void vtkCellLocatorStrategy_destructor (vtkNew < vtkCellLocatorStrategy > sself) ;
extern "C" void * vtkCellLocatorStrategy_get_ptr (vtkNew < vtkCellLocatorStrategy > sself) ;
extern "C" vtkNew < vtkCellTreeLocator > vtkCellTreeLocator_new () ;
extern "C" void vtkCellTreeLocator_destructor (vtkNew < vtkCellTreeLocator > sself) ;
extern "C" void * vtkCellTreeLocator_get_ptr (vtkNew < vtkCellTreeLocator > sself) ;
extern "C" vtkNew < vtkCellTypes > vtkCellTypes_new () ;
extern "C" void vtkCellTypes_destructor (vtkNew < vtkCellTypes > sself) ;
extern "C" void * vtkCellTypes_get_ptr (vtkNew < vtkCellTypes > sself) ;
extern "C" vtkNew < vtkClosestNPointsStrategy > vtkClosestNPointsStrategy_new () ;
extern "C" void vtkClosestNPointsStrategy_destructor (vtkNew < vtkClosestNPointsStrategy > sself) ;
extern "C" void * vtkClosestNPointsStrategy_get_ptr (vtkNew < vtkClosestNPointsStrategy > sself) ;
extern "C" vtkNew < vtkClosestPointStrategy > vtkClosestPointStrategy_new () ;
extern "C" void vtkClosestPointStrategy_destructor (vtkNew < vtkClosestPointStrategy > sself) ;
extern "C" void * vtkClosestPointStrategy_get_ptr (vtkNew < vtkClosestPointStrategy > sself) ;
extern "C" vtkNew < vtkCone > vtkCone_new () ;
extern "C" void vtkCone_destructor (vtkNew < vtkCone > sself) ;
extern "C" void * vtkCone_get_ptr (vtkNew < vtkCone > sself) ;
extern "C" vtkNew < vtkConvexPointSet > vtkConvexPointSet_new () ;
extern "C" void vtkConvexPointSet_destructor (vtkNew < vtkConvexPointSet > sself) ;
extern "C" void * vtkConvexPointSet_get_ptr (vtkNew < vtkConvexPointSet > sself) ;
extern "C" vtkNew < vtkCoordinateFrame > vtkCoordinateFrame_new () ;
extern "C" void vtkCoordinateFrame_destructor (vtkNew < vtkCoordinateFrame > sself) ;
extern "C" void * vtkCoordinateFrame_get_ptr (vtkNew < vtkCoordinateFrame > sself) ;
extern "C" vtkNew < vtkCubicLine > vtkCubicLine_new () ;
extern "C" void vtkCubicLine_destructor (vtkNew < vtkCubicLine > sself) ;
extern "C" void * vtkCubicLine_get_ptr (vtkNew < vtkCubicLine > sself) ;
extern "C" vtkNew < vtkCylinder > vtkCylinder_new () ;
extern "C" void vtkCylinder_destructor (vtkNew < vtkCylinder > sself) ;
extern "C" void * vtkCylinder_get_ptr (vtkNew < vtkCylinder > sself) ;
extern "C" vtkNew < vtkDataAssembly > vtkDataAssembly_new () ;
extern "C" void vtkDataAssembly_destructor (vtkNew < vtkDataAssembly > sself) ;
extern "C" void * vtkDataAssembly_get_ptr (vtkNew < vtkDataAssembly > sself) ;
extern "C" vtkNew < vtkDataAssemblyUtilities > vtkDataAssemblyUtilities_new () ;
extern "C" void vtkDataAssemblyUtilities_destructor (vtkNew < vtkDataAssemblyUtilities > sself) ;
extern "C" void * vtkDataAssemblyUtilities_get_ptr (vtkNew < vtkDataAssemblyUtilities > sself) ;
extern "C" vtkNew < vtkDataObject > vtkDataObject_new () ;
extern "C" void vtkDataObject_destructor (vtkNew < vtkDataObject > sself) ;
extern "C" void * vtkDataObject_get_ptr (vtkNew < vtkDataObject > sself) ;
extern "C" vtkNew < vtkDataObjectCollection > vtkDataObjectCollection_new () ;
extern "C" void vtkDataObjectCollection_destructor (vtkNew < vtkDataObjectCollection > sself) ;
extern "C" void * vtkDataObjectCollection_get_ptr (vtkNew < vtkDataObjectCollection > sself) ;
extern "C" vtkNew < vtkDataObjectTreeIterator > vtkDataObjectTreeIterator_new () ;
extern "C" void vtkDataObjectTreeIterator_destructor (vtkNew < vtkDataObjectTreeIterator > sself) ;
extern "C" void * vtkDataObjectTreeIterator_get_ptr (vtkNew < vtkDataObjectTreeIterator > sself) ;
extern "C" vtkNew < vtkDataObjectTypes > vtkDataObjectTypes_new () ;
extern "C" void vtkDataObjectTypes_destructor (vtkNew < vtkDataObjectTypes > sself) ;
extern "C" void * vtkDataObjectTypes_get_ptr (vtkNew < vtkDataObjectTypes > sself) ;
extern "C" vtkNew < vtkDataSetAttributes > vtkDataSetAttributes_new () ;
extern "C" void vtkDataSetAttributes_destructor (vtkNew < vtkDataSetAttributes > sself) ;
extern "C" void * vtkDataSetAttributes_get_ptr (vtkNew < vtkDataSetAttributes > sself) ;
extern "C" vtkNew < vtkDataSetCellIterator > vtkDataSetCellIterator_new () ;
extern "C" void vtkDataSetCellIterator_destructor (vtkNew < vtkDataSetCellIterator > sself) ;
extern "C" void * vtkDataSetCellIterator_get_ptr (vtkNew < vtkDataSetCellIterator > sself) ;
extern "C" vtkNew < vtkDataSetCollection > vtkDataSetCollection_new () ;
extern "C" void vtkDataSetCollection_destructor (vtkNew < vtkDataSetCollection > sself) ;
extern "C" void * vtkDataSetCollection_get_ptr (vtkNew < vtkDataSetCollection > sself) ;
extern "C" vtkNew < vtkDirectedAcyclicGraph > vtkDirectedAcyclicGraph_new () ;
extern "C" void vtkDirectedAcyclicGraph_destructor (vtkNew < vtkDirectedAcyclicGraph > sself) ;
extern "C" void * vtkDirectedAcyclicGraph_get_ptr (vtkNew < vtkDirectedAcyclicGraph > sself) ;
extern "C" vtkNew < vtkDirectedGraph > vtkDirectedGraph_new () ;
extern "C" void vtkDirectedGraph_destructor (vtkNew < vtkDirectedGraph > sself) ;
extern "C" void * vtkDirectedGraph_get_ptr (vtkNew < vtkDirectedGraph > sself) ;
extern "C" vtkNew < vtkEdgeListIterator > vtkEdgeListIterator_new () ;
extern "C" void vtkEdgeListIterator_destructor (vtkNew < vtkEdgeListIterator > sself) ;
extern "C" void * vtkEdgeListIterator_get_ptr (vtkNew < vtkEdgeListIterator > sself) ;
extern "C" vtkNew < vtkEdgeTable > vtkEdgeTable_new () ;
extern "C" void vtkEdgeTable_destructor (vtkNew < vtkEdgeTable > sself) ;
extern "C" void * vtkEdgeTable_get_ptr (vtkNew < vtkEdgeTable > sself) ;
extern "C" vtkNew < vtkEmptyCell > vtkEmptyCell_new () ;
extern "C" void vtkEmptyCell_destructor (vtkNew < vtkEmptyCell > sself) ;
extern "C" void * vtkEmptyCell_get_ptr (vtkNew < vtkEmptyCell > sself) ;
extern "C" vtkNew < vtkExplicitStructuredGrid > vtkExplicitStructuredGrid_new () ;
extern "C" void vtkExplicitStructuredGrid_destructor (vtkNew < vtkExplicitStructuredGrid > sself) ;
extern "C" void * vtkExplicitStructuredGrid_get_ptr (vtkNew < vtkExplicitStructuredGrid > sself) ;
extern "C" vtkNew < vtkExtractStructuredGridHelper > vtkExtractStructuredGridHelper_new () ;
extern "C" void vtkExtractStructuredGridHelper_destructor (vtkNew < vtkExtractStructuredGridHelper > sself) ;
extern "C" void * vtkExtractStructuredGridHelper_get_ptr (vtkNew < vtkExtractStructuredGridHelper > sself) ;
extern "C" vtkNew < vtkFieldData > vtkFieldData_new () ;
extern "C" void vtkFieldData_destructor (vtkNew < vtkFieldData > sself) ;
extern "C" void * vtkFieldData_get_ptr (vtkNew < vtkFieldData > sself) ;
extern "C" vtkNew < vtkFrustum > vtkFrustum_new () ;
extern "C" void vtkFrustum_destructor (vtkNew < vtkFrustum > sself) ;
extern "C" void * vtkFrustum_get_ptr (vtkNew < vtkFrustum > sself) ;
extern "C" vtkNew < vtkGenericAttributeCollection > vtkGenericAttributeCollection_new () ;
extern "C" void vtkGenericAttributeCollection_destructor (vtkNew < vtkGenericAttributeCollection > sself) ;
extern "C" void * vtkGenericAttributeCollection_get_ptr (vtkNew < vtkGenericAttributeCollection > sself) ;
extern "C" vtkNew < vtkGenericCell > vtkGenericCell_new () ;
extern "C" void vtkGenericCell_destructor (vtkNew < vtkGenericCell > sself) ;
extern "C" void * vtkGenericCell_get_ptr (vtkNew < vtkGenericCell > sself) ;
extern "C" vtkNew < vtkGenericEdgeTable > vtkGenericEdgeTable_new () ;
extern "C" void vtkGenericEdgeTable_destructor (vtkNew < vtkGenericEdgeTable > sself) ;
extern "C" void * vtkGenericEdgeTable_get_ptr (vtkNew < vtkGenericEdgeTable > sself) ;
extern "C" vtkNew < vtkGenericInterpolatedVelocityField > vtkGenericInterpolatedVelocityField_new () ;
extern "C" void vtkGenericInterpolatedVelocityField_destructor (vtkNew < vtkGenericInterpolatedVelocityField > sself) ;
extern "C" void * vtkGenericInterpolatedVelocityField_get_ptr (vtkNew < vtkGenericInterpolatedVelocityField > sself) ;
extern "C" vtkNew < vtkGeometricErrorMetric > vtkGeometricErrorMetric_new () ;
extern "C" void vtkGeometricErrorMetric_destructor (vtkNew < vtkGeometricErrorMetric > sself) ;
extern "C" void * vtkGeometricErrorMetric_get_ptr (vtkNew < vtkGeometricErrorMetric > sself) ;
extern "C" vtkNew < vtkGraphEdge > vtkGraphEdge_new () ;
extern "C" void vtkGraphEdge_destructor (vtkNew < vtkGraphEdge > sself) ;
extern "C" void * vtkGraphEdge_get_ptr (vtkNew < vtkGraphEdge > sself) ;
extern "C" vtkNew < vtkGraphInternals > vtkGraphInternals_new () ;
extern "C" void vtkGraphInternals_destructor (vtkNew < vtkGraphInternals > sself) ;
extern "C" void * vtkGraphInternals_get_ptr (vtkNew < vtkGraphInternals > sself) ;
extern "C" vtkNew < vtkHexagonalPrism > vtkHexagonalPrism_new () ;
extern "C" void vtkHexagonalPrism_destructor (vtkNew < vtkHexagonalPrism > sself) ;
extern "C" void * vtkHexagonalPrism_get_ptr (vtkNew < vtkHexagonalPrism > sself) ;
extern "C" vtkNew < vtkHexahedron > vtkHexahedron_new () ;
extern "C" void vtkHexahedron_destructor (vtkNew < vtkHexahedron > sself) ;
extern "C" void * vtkHexahedron_get_ptr (vtkNew < vtkHexahedron > sself) ;
extern "C" vtkNew < vtkHierarchicalBoxDataSet > vtkHierarchicalBoxDataSet_new () ;
extern "C" void vtkHierarchicalBoxDataSet_destructor (vtkNew < vtkHierarchicalBoxDataSet > sself) ;
extern "C" void * vtkHierarchicalBoxDataSet_get_ptr (vtkNew < vtkHierarchicalBoxDataSet > sself) ;
extern "C" vtkNew < vtkHyperTreeGrid > vtkHyperTreeGrid_new () ;
extern "C" void vtkHyperTreeGrid_destructor (vtkNew < vtkHyperTreeGrid > sself) ;
extern "C" void * vtkHyperTreeGrid_get_ptr (vtkNew < vtkHyperTreeGrid > sself) ;
extern "C" vtkNew < vtkHyperTreeGridGeometricLocator > vtkHyperTreeGridGeometricLocator_new () ;
extern "C" void vtkHyperTreeGridGeometricLocator_destructor (vtkNew < vtkHyperTreeGridGeometricLocator > sself) ;
extern "C" void * vtkHyperTreeGridGeometricLocator_get_ptr (vtkNew < vtkHyperTreeGridGeometricLocator > sself) ;
extern "C" vtkNew < vtkHyperTreeGridNonOrientedCursor > vtkHyperTreeGridNonOrientedCursor_new () ;
extern "C" void vtkHyperTreeGridNonOrientedCursor_destructor (vtkNew < vtkHyperTreeGridNonOrientedCursor > sself) ;
extern "C" void * vtkHyperTreeGridNonOrientedCursor_get_ptr (vtkNew < vtkHyperTreeGridNonOrientedCursor > sself) ;
extern "C" vtkNew < vtkHyperTreeGridNonOrientedGeometryCursor > vtkHyperTreeGridNonOrientedGeometryCursor_new () ;
extern "C" void vtkHyperTreeGridNonOrientedGeometryCursor_destructor (vtkNew < vtkHyperTreeGridNonOrientedGeometryCursor > sself) ;
extern "C" void * vtkHyperTreeGridNonOrientedGeometryCursor_get_ptr (vtkNew < vtkHyperTreeGridNonOrientedGeometryCursor > sself) ;
extern "C" vtkNew < vtkHyperTreeGridNonOrientedMooreSuperCursor > vtkHyperTreeGridNonOrientedMooreSuperCursor_new () ;
extern "C" void vtkHyperTreeGridNonOrientedMooreSuperCursor_destructor (vtkNew < vtkHyperTreeGridNonOrientedMooreSuperCursor > sself) ;
extern "C" void * vtkHyperTreeGridNonOrientedMooreSuperCursor_get_ptr (vtkNew < vtkHyperTreeGridNonOrientedMooreSuperCursor > sself) ;
extern "C" vtkNew < vtkHyperTreeGridNonOrientedMooreSuperCursorLight > vtkHyperTreeGridNonOrientedMooreSuperCursorLight_new () ;
extern "C" void vtkHyperTreeGridNonOrientedMooreSuperCursorLight_destructor (vtkNew < vtkHyperTreeGridNonOrientedMooreSuperCursorLight > sself) ;
extern "C" void * vtkHyperTreeGridNonOrientedMooreSuperCursorLight_get_ptr (vtkNew < vtkHyperTreeGridNonOrientedMooreSuperCursorLight > sself) ;
extern "C" vtkNew < vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor > vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor_new () ;
extern "C" void vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor_destructor (vtkNew < vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor > sself) ;
extern "C" void * vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor_get_ptr (vtkNew < vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor > sself) ;
extern "C" vtkNew < vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor > vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor_new () ;
extern "C" void vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor_destructor (vtkNew < vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor > sself) ;
extern "C" void * vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor_get_ptr (vtkNew < vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor > sself) ;
extern "C" vtkNew < vtkHyperTreeGridNonOrientedVonNeumannSuperCursor > vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_new () ;
extern "C" void vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_destructor (vtkNew < vtkHyperTreeGridNonOrientedVonNeumannSuperCursor > sself) ;
extern "C" void * vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_get_ptr (vtkNew < vtkHyperTreeGridNonOrientedVonNeumannSuperCursor > sself) ;
extern "C" vtkNew < vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight > vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_new () ;
extern "C" void vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_destructor (vtkNew < vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight > sself) ;
extern "C" void * vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_get_ptr (vtkNew < vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight > sself) ;
extern "C" vtkNew < vtkHyperTreeGridOrientedCursor > vtkHyperTreeGridOrientedCursor_new () ;
extern "C" void vtkHyperTreeGridOrientedCursor_destructor (vtkNew < vtkHyperTreeGridOrientedCursor > sself) ;
extern "C" void * vtkHyperTreeGridOrientedCursor_get_ptr (vtkNew < vtkHyperTreeGridOrientedCursor > sself) ;
extern "C" vtkNew < vtkHyperTreeGridOrientedGeometryCursor > vtkHyperTreeGridOrientedGeometryCursor_new () ;
extern "C" void vtkHyperTreeGridOrientedGeometryCursor_destructor (vtkNew < vtkHyperTreeGridOrientedGeometryCursor > sself) ;
extern "C" void * vtkHyperTreeGridOrientedGeometryCursor_get_ptr (vtkNew < vtkHyperTreeGridOrientedGeometryCursor > sself) ;
extern "C" vtkNew < vtkImageData > vtkImageData_new () ;
extern "C" void vtkImageData_destructor (vtkNew < vtkImageData > sself) ;
extern "C" void * vtkImageData_get_ptr (vtkNew < vtkImageData > sself) ;
extern "C" vtkNew < vtkImageTransform > vtkImageTransform_new () ;
extern "C" void vtkImageTransform_destructor (vtkNew < vtkImageTransform > sself) ;
extern "C" void * vtkImageTransform_get_ptr (vtkNew < vtkImageTransform > sself) ;
extern "C" vtkNew < vtkImplicitBoolean > vtkImplicitBoolean_new () ;
extern "C" void vtkImplicitBoolean_destructor (vtkNew < vtkImplicitBoolean > sself) ;
extern "C" void * vtkImplicitBoolean_get_ptr (vtkNew < vtkImplicitBoolean > sself) ;
extern "C" vtkNew < vtkImplicitDataSet > vtkImplicitDataSet_new () ;
extern "C" void vtkImplicitDataSet_destructor (vtkNew < vtkImplicitDataSet > sself) ;
extern "C" void * vtkImplicitDataSet_get_ptr (vtkNew < vtkImplicitDataSet > sself) ;
extern "C" vtkNew < vtkImplicitFunctionCollection > vtkImplicitFunctionCollection_new () ;
extern "C" void vtkImplicitFunctionCollection_destructor (vtkNew < vtkImplicitFunctionCollection > sself) ;
extern "C" void * vtkImplicitFunctionCollection_get_ptr (vtkNew < vtkImplicitFunctionCollection > sself) ;
extern "C" vtkNew < vtkImplicitHalo > vtkImplicitHalo_new () ;
extern "C" void vtkImplicitHalo_destructor (vtkNew < vtkImplicitHalo > sself) ;
extern "C" void * vtkImplicitHalo_get_ptr (vtkNew < vtkImplicitHalo > sself) ;
extern "C" vtkNew < vtkImplicitSelectionLoop > vtkImplicitSelectionLoop_new () ;
extern "C" void vtkImplicitSelectionLoop_destructor (vtkNew < vtkImplicitSelectionLoop > sself) ;
extern "C" void * vtkImplicitSelectionLoop_get_ptr (vtkNew < vtkImplicitSelectionLoop > sself) ;
extern "C" vtkNew < vtkImplicitSum > vtkImplicitSum_new () ;
extern "C" void vtkImplicitSum_destructor (vtkNew < vtkImplicitSum > sself) ;
extern "C" void * vtkImplicitSum_get_ptr (vtkNew < vtkImplicitSum > sself) ;
extern "C" vtkNew < vtkImplicitVolume > vtkImplicitVolume_new () ;
extern "C" void vtkImplicitVolume_destructor (vtkNew < vtkImplicitVolume > sself) ;
extern "C" void * vtkImplicitVolume_get_ptr (vtkNew < vtkImplicitVolume > sself) ;
extern "C" vtkNew < vtkImplicitWindowFunction > vtkImplicitWindowFunction_new () ;
extern "C" void vtkImplicitWindowFunction_destructor (vtkNew < vtkImplicitWindowFunction > sself) ;
extern "C" void * vtkImplicitWindowFunction_get_ptr (vtkNew < vtkImplicitWindowFunction > sself) ;
extern "C" vtkNew < vtkInEdgeIterator > vtkInEdgeIterator_new () ;
extern "C" void vtkInEdgeIterator_destructor (vtkNew < vtkInEdgeIterator > sself) ;
extern "C" void * vtkInEdgeIterator_get_ptr (vtkNew < vtkInEdgeIterator > sself) ;
extern "C" vtkNew < vtkIncrementalOctreeNode > vtkIncrementalOctreeNode_new () ;
extern "C" void vtkIncrementalOctreeNode_destructor (vtkNew < vtkIncrementalOctreeNode > sself) ;
extern "C" void * vtkIncrementalOctreeNode_get_ptr (vtkNew < vtkIncrementalOctreeNode > sself) ;
extern "C" vtkNew < vtkIncrementalOctreePointLocator > vtkIncrementalOctreePointLocator_new () ;
extern "C" void vtkIncrementalOctreePointLocator_destructor (vtkNew < vtkIncrementalOctreePointLocator > sself) ;
extern "C" void * vtkIncrementalOctreePointLocator_get_ptr (vtkNew < vtkIncrementalOctreePointLocator > sself) ;
extern "C" vtkNew < vtkIterativeClosestPointTransform > vtkIterativeClosestPointTransform_new () ;
extern "C" void vtkIterativeClosestPointTransform_destructor (vtkNew < vtkIterativeClosestPointTransform > sself) ;
extern "C" void * vtkIterativeClosestPointTransform_get_ptr (vtkNew < vtkIterativeClosestPointTransform > sself) ;
extern "C" vtkNew < vtkKdNode > vtkKdNode_new () ;
extern "C" void vtkKdNode_destructor (vtkNew < vtkKdNode > sself) ;
extern "C" void * vtkKdNode_get_ptr (vtkNew < vtkKdNode > sself) ;
extern "C" vtkNew < vtkKdTree > vtkKdTree_new () ;
extern "C" void vtkKdTree_destructor (vtkNew < vtkKdTree > sself) ;
extern "C" void * vtkKdTree_get_ptr (vtkNew < vtkKdTree > sself) ;
extern "C" vtkNew < vtkKdTreePointLocator > vtkKdTreePointLocator_new () ;
extern "C" void vtkKdTreePointLocator_destructor (vtkNew < vtkKdTreePointLocator > sself) ;
extern "C" void * vtkKdTreePointLocator_get_ptr (vtkNew < vtkKdTreePointLocator > sself) ;
extern "C" vtkNew < vtkLagrangeCurve > vtkLagrangeCurve_new () ;
extern "C" void vtkLagrangeCurve_destructor (vtkNew < vtkLagrangeCurve > sself) ;
extern "C" void * vtkLagrangeCurve_get_ptr (vtkNew < vtkLagrangeCurve > sself) ;
extern "C" vtkNew < vtkLagrangeHexahedron > vtkLagrangeHexahedron_new () ;
extern "C" void vtkLagrangeHexahedron_destructor (vtkNew < vtkLagrangeHexahedron > sself) ;
extern "C" void * vtkLagrangeHexahedron_get_ptr (vtkNew < vtkLagrangeHexahedron > sself) ;
extern "C" vtkNew < vtkLagrangeInterpolation > vtkLagrangeInterpolation_new () ;
extern "C" void vtkLagrangeInterpolation_destructor (vtkNew < vtkLagrangeInterpolation > sself) ;
extern "C" void * vtkLagrangeInterpolation_get_ptr (vtkNew < vtkLagrangeInterpolation > sself) ;
extern "C" vtkNew < vtkLagrangeQuadrilateral > vtkLagrangeQuadrilateral_new () ;
extern "C" void vtkLagrangeQuadrilateral_destructor (vtkNew < vtkLagrangeQuadrilateral > sself) ;
extern "C" void * vtkLagrangeQuadrilateral_get_ptr (vtkNew < vtkLagrangeQuadrilateral > sself) ;
extern "C" vtkNew < vtkLagrangeTetra > vtkLagrangeTetra_new () ;
extern "C" void vtkLagrangeTetra_destructor (vtkNew < vtkLagrangeTetra > sself) ;
extern "C" void * vtkLagrangeTetra_get_ptr (vtkNew < vtkLagrangeTetra > sself) ;
extern "C" vtkNew < vtkLagrangeTriangle > vtkLagrangeTriangle_new () ;
extern "C" void vtkLagrangeTriangle_destructor (vtkNew < vtkLagrangeTriangle > sself) ;
extern "C" void * vtkLagrangeTriangle_get_ptr (vtkNew < vtkLagrangeTriangle > sself) ;
extern "C" vtkNew < vtkLagrangeWedge > vtkLagrangeWedge_new () ;
extern "C" void vtkLagrangeWedge_destructor (vtkNew < vtkLagrangeWedge > sself) ;
extern "C" void * vtkLagrangeWedge_get_ptr (vtkNew < vtkLagrangeWedge > sself) ;
extern "C" vtkNew < vtkLine > vtkLine_new () ;
extern "C" void vtkLine_destructor (vtkNew < vtkLine > sself) ;
extern "C" void * vtkLine_get_ptr (vtkNew < vtkLine > sself) ;
extern "C" vtkNew < vtkMeanValueCoordinatesInterpolator > vtkMeanValueCoordinatesInterpolator_new () ;
extern "C" void vtkMeanValueCoordinatesInterpolator_destructor (vtkNew < vtkMeanValueCoordinatesInterpolator > sself) ;
extern "C" void * vtkMeanValueCoordinatesInterpolator_get_ptr (vtkNew < vtkMeanValueCoordinatesInterpolator > sself) ;
extern "C" vtkNew < vtkMergePoints > vtkMergePoints_new () ;
extern "C" void vtkMergePoints_destructor (vtkNew < vtkMergePoints > sself) ;
extern "C" void * vtkMergePoints_get_ptr (vtkNew < vtkMergePoints > sself) ;
extern "C" vtkNew < vtkMolecule > vtkMolecule_new () ;
extern "C" void vtkMolecule_destructor (vtkNew < vtkMolecule > sself) ;
extern "C" void * vtkMolecule_get_ptr (vtkNew < vtkMolecule > sself) ;
extern "C" vtkNew < vtkMultiBlockDataSet > vtkMultiBlockDataSet_new () ;
extern "C" void vtkMultiBlockDataSet_destructor (vtkNew < vtkMultiBlockDataSet > sself) ;
extern "C" void * vtkMultiBlockDataSet_get_ptr (vtkNew < vtkMultiBlockDataSet > sself) ;
extern "C" vtkNew < vtkMultiPieceDataSet > vtkMultiPieceDataSet_new () ;
extern "C" void vtkMultiPieceDataSet_destructor (vtkNew < vtkMultiPieceDataSet > sself) ;
extern "C" void * vtkMultiPieceDataSet_get_ptr (vtkNew < vtkMultiPieceDataSet > sself) ;
extern "C" vtkNew < vtkMutableDirectedGraph > vtkMutableDirectedGraph_new () ;
extern "C" void vtkMutableDirectedGraph_destructor (vtkNew < vtkMutableDirectedGraph > sself) ;
extern "C" void * vtkMutableDirectedGraph_get_ptr (vtkNew < vtkMutableDirectedGraph > sself) ;
extern "C" vtkNew < vtkMutableUndirectedGraph > vtkMutableUndirectedGraph_new () ;
extern "C" void vtkMutableUndirectedGraph_destructor (vtkNew < vtkMutableUndirectedGraph > sself) ;
extern "C" void * vtkMutableUndirectedGraph_get_ptr (vtkNew < vtkMutableUndirectedGraph > sself) ;
extern "C" vtkNew < vtkNonMergingPointLocator > vtkNonMergingPointLocator_new () ;
extern "C" void vtkNonMergingPointLocator_destructor (vtkNew < vtkNonMergingPointLocator > sself) ;
extern "C" void * vtkNonMergingPointLocator_get_ptr (vtkNew < vtkNonMergingPointLocator > sself) ;
extern "C" vtkNew < vtkNonOverlappingAMR > vtkNonOverlappingAMR_new () ;
extern "C" void vtkNonOverlappingAMR_destructor (vtkNew < vtkNonOverlappingAMR > sself) ;
extern "C" void * vtkNonOverlappingAMR_get_ptr (vtkNew < vtkNonOverlappingAMR > sself) ;
extern "C" vtkNew < vtkOctreePointLocator > vtkOctreePointLocator_new () ;
extern "C" void vtkOctreePointLocator_destructor (vtkNew < vtkOctreePointLocator > sself) ;
extern "C" void * vtkOctreePointLocator_get_ptr (vtkNew < vtkOctreePointLocator > sself) ;
extern "C" vtkNew < vtkOctreePointLocatorNode > vtkOctreePointLocatorNode_new () ;
extern "C" void vtkOctreePointLocatorNode_destructor (vtkNew < vtkOctreePointLocatorNode > sself) ;
extern "C" void * vtkOctreePointLocatorNode_get_ptr (vtkNew < vtkOctreePointLocatorNode > sself) ;
extern "C" vtkNew < vtkOrderedTriangulator > vtkOrderedTriangulator_new () ;
extern "C" void vtkOrderedTriangulator_destructor (vtkNew < vtkOrderedTriangulator > sself) ;
extern "C" void * vtkOrderedTriangulator_get_ptr (vtkNew < vtkOrderedTriangulator > sself) ;
extern "C" vtkNew < vtkOutEdgeIterator > vtkOutEdgeIterator_new () ;
extern "C" void vtkOutEdgeIterator_destructor (vtkNew < vtkOutEdgeIterator > sself) ;
extern "C" void * vtkOutEdgeIterator_get_ptr (vtkNew < vtkOutEdgeIterator > sself) ;
extern "C" vtkNew < vtkOverlappingAMR > vtkOverlappingAMR_new () ;
extern "C" void vtkOverlappingAMR_destructor (vtkNew < vtkOverlappingAMR > sself) ;
extern "C" void * vtkOverlappingAMR_get_ptr (vtkNew < vtkOverlappingAMR > sself) ;
extern "C" vtkNew < vtkPartitionedDataSet > vtkPartitionedDataSet_new () ;
extern "C" void vtkPartitionedDataSet_destructor (vtkNew < vtkPartitionedDataSet > sself) ;
extern "C" void * vtkPartitionedDataSet_get_ptr (vtkNew < vtkPartitionedDataSet > sself) ;
extern "C" vtkNew < vtkPartitionedDataSetCollection > vtkPartitionedDataSetCollection_new () ;
extern "C" void vtkPartitionedDataSetCollection_destructor (vtkNew < vtkPartitionedDataSetCollection > sself) ;
extern "C" void * vtkPartitionedDataSetCollection_get_ptr (vtkNew < vtkPartitionedDataSetCollection > sself) ;
extern "C" vtkNew < vtkPath > vtkPath_new () ;
extern "C" void vtkPath_destructor (vtkNew < vtkPath > sself) ;
extern "C" void * vtkPath_get_ptr (vtkNew < vtkPath > sself) ;
extern "C" vtkNew < vtkPentagonalPrism > vtkPentagonalPrism_new () ;
extern "C" void vtkPentagonalPrism_destructor (vtkNew < vtkPentagonalPrism > sself) ;
extern "C" void * vtkPentagonalPrism_get_ptr (vtkNew < vtkPentagonalPrism > sself) ;
extern "C" vtkNew < vtkPerlinNoise > vtkPerlinNoise_new () ;
extern "C" void vtkPerlinNoise_destructor (vtkNew < vtkPerlinNoise > sself) ;
extern "C" void * vtkPerlinNoise_get_ptr (vtkNew < vtkPerlinNoise > sself) ;
extern "C" vtkNew < vtkPiecewiseFunction > vtkPiecewiseFunction_new () ;
extern "C" void vtkPiecewiseFunction_destructor (vtkNew < vtkPiecewiseFunction > sself) ;
extern "C" void * vtkPiecewiseFunction_get_ptr (vtkNew < vtkPiecewiseFunction > sself) ;
extern "C" vtkNew < vtkPixel > vtkPixel_new () ;
extern "C" void vtkPixel_destructor (vtkNew < vtkPixel > sself) ;
extern "C" void * vtkPixel_get_ptr (vtkNew < vtkPixel > sself) ;
extern "C" vtkNew < vtkPlane > vtkPlane_new () ;
extern "C" void vtkPlane_destructor (vtkNew < vtkPlane > sself) ;
extern "C" void * vtkPlane_get_ptr (vtkNew < vtkPlane > sself) ;
extern "C" vtkNew < vtkPlaneCollection > vtkPlaneCollection_new () ;
extern "C" void vtkPlaneCollection_destructor (vtkNew < vtkPlaneCollection > sself) ;
extern "C" void * vtkPlaneCollection_get_ptr (vtkNew < vtkPlaneCollection > sself) ;
extern "C" vtkNew < vtkPlanes > vtkPlanes_new () ;
extern "C" void vtkPlanes_destructor (vtkNew < vtkPlanes > sself) ;
extern "C" void * vtkPlanes_get_ptr (vtkNew < vtkPlanes > sself) ;
extern "C" vtkNew < vtkPlanesIntersection > vtkPlanesIntersection_new () ;
extern "C" void vtkPlanesIntersection_destructor (vtkNew < vtkPlanesIntersection > sself) ;
extern "C" void * vtkPlanesIntersection_get_ptr (vtkNew < vtkPlanesIntersection > sself) ;
extern "C" vtkNew < vtkPointData > vtkPointData_new () ;
extern "C" void vtkPointData_destructor (vtkNew < vtkPointData > sself) ;
extern "C" void * vtkPointData_get_ptr (vtkNew < vtkPointData > sself) ;
extern "C" vtkNew < vtkPointLocator > vtkPointLocator_new () ;
extern "C" void vtkPointLocator_destructor (vtkNew < vtkPointLocator > sself) ;
extern "C" void * vtkPointLocator_get_ptr (vtkNew < vtkPointLocator > sself) ;
extern "C" vtkNew < vtkPointSet > vtkPointSet_new () ;
extern "C" void vtkPointSet_destructor (vtkNew < vtkPointSet > sself) ;
extern "C" void * vtkPointSet_get_ptr (vtkNew < vtkPointSet > sself) ;
extern "C" vtkNew < vtkPointSetCellIterator > vtkPointSetCellIterator_new () ;
extern "C" void vtkPointSetCellIterator_destructor (vtkNew < vtkPointSetCellIterator > sself) ;
extern "C" void * vtkPointSetCellIterator_get_ptr (vtkNew < vtkPointSetCellIterator > sself) ;
extern "C" vtkNew < vtkPointsProjectedHull > vtkPointsProjectedHull_new () ;
extern "C" void vtkPointsProjectedHull_destructor (vtkNew < vtkPointsProjectedHull > sself) ;
extern "C" void * vtkPointsProjectedHull_get_ptr (vtkNew < vtkPointsProjectedHull > sself) ;
extern "C" vtkNew < vtkPolyData > vtkPolyData_new () ;
extern "C" void vtkPolyData_destructor (vtkNew < vtkPolyData > sself) ;
extern "C" void * vtkPolyData_get_ptr (vtkNew < vtkPolyData > sself) ;
extern "C" vtkNew < vtkPolyDataCollection > vtkPolyDataCollection_new () ;
extern "C" void vtkPolyDataCollection_destructor (vtkNew < vtkPolyDataCollection > sself) ;
extern "C" void * vtkPolyDataCollection_get_ptr (vtkNew < vtkPolyDataCollection > sself) ;
extern "C" vtkNew < vtkPolyLine > vtkPolyLine_new () ;
extern "C" void vtkPolyLine_destructor (vtkNew < vtkPolyLine > sself) ;
extern "C" void * vtkPolyLine_get_ptr (vtkNew < vtkPolyLine > sself) ;
extern "C" vtkNew < vtkPolyPlane > vtkPolyPlane_new () ;
extern "C" void vtkPolyPlane_destructor (vtkNew < vtkPolyPlane > sself) ;
extern "C" void * vtkPolyPlane_get_ptr (vtkNew < vtkPolyPlane > sself) ;
extern "C" vtkNew < vtkPolyVertex > vtkPolyVertex_new () ;
extern "C" void vtkPolyVertex_destructor (vtkNew < vtkPolyVertex > sself) ;
extern "C" void * vtkPolyVertex_get_ptr (vtkNew < vtkPolyVertex > sself) ;
extern "C" vtkNew < vtkPolygon > vtkPolygon_new () ;
extern "C" void vtkPolygon_destructor (vtkNew < vtkPolygon > sself) ;
extern "C" void * vtkPolygon_get_ptr (vtkNew < vtkPolygon > sself) ;
extern "C" vtkNew < vtkPolyhedron > vtkPolyhedron_new () ;
extern "C" void vtkPolyhedron_destructor (vtkNew < vtkPolyhedron > sself) ;
extern "C" void * vtkPolyhedron_get_ptr (vtkNew < vtkPolyhedron > sself) ;
extern "C" vtkNew < vtkPyramid > vtkPyramid_new () ;
extern "C" void vtkPyramid_destructor (vtkNew < vtkPyramid > sself) ;
extern "C" void * vtkPyramid_get_ptr (vtkNew < vtkPyramid > sself) ;
extern "C" vtkNew < vtkQuad > vtkQuad_new () ;
extern "C" void vtkQuad_destructor (vtkNew < vtkQuad > sself) ;
extern "C" void * vtkQuad_get_ptr (vtkNew < vtkQuad > sself) ;
extern "C" vtkNew < vtkQuadraticEdge > vtkQuadraticEdge_new () ;
extern "C" void vtkQuadraticEdge_destructor (vtkNew < vtkQuadraticEdge > sself) ;
extern "C" void * vtkQuadraticEdge_get_ptr (vtkNew < vtkQuadraticEdge > sself) ;
extern "C" vtkNew < vtkQuadraticHexahedron > vtkQuadraticHexahedron_new () ;
extern "C" void vtkQuadraticHexahedron_destructor (vtkNew < vtkQuadraticHexahedron > sself) ;
extern "C" void * vtkQuadraticHexahedron_get_ptr (vtkNew < vtkQuadraticHexahedron > sself) ;
extern "C" vtkNew < vtkQuadraticLinearQuad > vtkQuadraticLinearQuad_new () ;
extern "C" void vtkQuadraticLinearQuad_destructor (vtkNew < vtkQuadraticLinearQuad > sself) ;
extern "C" void * vtkQuadraticLinearQuad_get_ptr (vtkNew < vtkQuadraticLinearQuad > sself) ;
extern "C" vtkNew < vtkQuadraticLinearWedge > vtkQuadraticLinearWedge_new () ;
extern "C" void vtkQuadraticLinearWedge_destructor (vtkNew < vtkQuadraticLinearWedge > sself) ;
extern "C" void * vtkQuadraticLinearWedge_get_ptr (vtkNew < vtkQuadraticLinearWedge > sself) ;
extern "C" vtkNew < vtkQuadraticPolygon > vtkQuadraticPolygon_new () ;
extern "C" void vtkQuadraticPolygon_destructor (vtkNew < vtkQuadraticPolygon > sself) ;
extern "C" void * vtkQuadraticPolygon_get_ptr (vtkNew < vtkQuadraticPolygon > sself) ;
extern "C" vtkNew < vtkQuadraticPyramid > vtkQuadraticPyramid_new () ;
extern "C" void vtkQuadraticPyramid_destructor (vtkNew < vtkQuadraticPyramid > sself) ;
extern "C" void * vtkQuadraticPyramid_get_ptr (vtkNew < vtkQuadraticPyramid > sself) ;
extern "C" vtkNew < vtkQuadraticQuad > vtkQuadraticQuad_new () ;
extern "C" void vtkQuadraticQuad_destructor (vtkNew < vtkQuadraticQuad > sself) ;
extern "C" void * vtkQuadraticQuad_get_ptr (vtkNew < vtkQuadraticQuad > sself) ;
extern "C" vtkNew < vtkQuadraticTetra > vtkQuadraticTetra_new () ;
extern "C" void vtkQuadraticTetra_destructor (vtkNew < vtkQuadraticTetra > sself) ;
extern "C" void * vtkQuadraticTetra_get_ptr (vtkNew < vtkQuadraticTetra > sself) ;
extern "C" vtkNew < vtkQuadraticTriangle > vtkQuadraticTriangle_new () ;
extern "C" void vtkQuadraticTriangle_destructor (vtkNew < vtkQuadraticTriangle > sself) ;
extern "C" void * vtkQuadraticTriangle_get_ptr (vtkNew < vtkQuadraticTriangle > sself) ;
extern "C" vtkNew < vtkQuadraticWedge > vtkQuadraticWedge_new () ;
extern "C" void vtkQuadraticWedge_destructor (vtkNew < vtkQuadraticWedge > sself) ;
extern "C" void * vtkQuadraticWedge_get_ptr (vtkNew < vtkQuadraticWedge > sself) ;
extern "C" vtkNew < vtkQuadratureSchemeDefinition > vtkQuadratureSchemeDefinition_new () ;
extern "C" void vtkQuadratureSchemeDefinition_destructor (vtkNew < vtkQuadratureSchemeDefinition > sself) ;
extern "C" void * vtkQuadratureSchemeDefinition_get_ptr (vtkNew < vtkQuadratureSchemeDefinition > sself) ;
extern "C" vtkNew < vtkQuadric > vtkQuadric_new () ;
extern "C" void vtkQuadric_destructor (vtkNew < vtkQuadric > sself) ;
extern "C" void * vtkQuadric_get_ptr (vtkNew < vtkQuadric > sself) ;
extern "C" vtkNew < vtkRectilinearGrid > vtkRectilinearGrid_new () ;
extern "C" void vtkRectilinearGrid_destructor (vtkNew < vtkRectilinearGrid > sself) ;
extern "C" void * vtkRectilinearGrid_get_ptr (vtkNew < vtkRectilinearGrid > sself) ;
extern "C" vtkNew < vtkReebGraph > vtkReebGraph_new () ;
extern "C" void vtkReebGraph_destructor (vtkNew < vtkReebGraph > sself) ;
extern "C" void * vtkReebGraph_get_ptr (vtkNew < vtkReebGraph > sself) ;
extern "C" vtkNew < vtkReebGraphSimplificationMetric > vtkReebGraphSimplificationMetric_new () ;
extern "C" void vtkReebGraphSimplificationMetric_destructor (vtkNew < vtkReebGraphSimplificationMetric > sself) ;
extern "C" void * vtkReebGraphSimplificationMetric_get_ptr (vtkNew < vtkReebGraphSimplificationMetric > sself) ;
extern "C" vtkNew < vtkSelection > vtkSelection_new () ;
extern "C" void vtkSelection_destructor (vtkNew < vtkSelection > sself) ;
extern "C" void * vtkSelection_get_ptr (vtkNew < vtkSelection > sself) ;
extern "C" vtkNew < vtkSelectionNode > vtkSelectionNode_new () ;
extern "C" void vtkSelectionNode_destructor (vtkNew < vtkSelectionNode > sself) ;
extern "C" void * vtkSelectionNode_get_ptr (vtkNew < vtkSelectionNode > sself) ;
extern "C" vtkNew < vtkSimpleCellTessellator > vtkSimpleCellTessellator_new () ;
extern "C" void vtkSimpleCellTessellator_destructor (vtkNew < vtkSimpleCellTessellator > sself) ;
extern "C" void * vtkSimpleCellTessellator_get_ptr (vtkNew < vtkSimpleCellTessellator > sself) ;
extern "C" vtkNew < vtkSmoothErrorMetric > vtkSmoothErrorMetric_new () ;
extern "C" void vtkSmoothErrorMetric_destructor (vtkNew < vtkSmoothErrorMetric > sself) ;
extern "C" void * vtkSmoothErrorMetric_get_ptr (vtkNew < vtkSmoothErrorMetric > sself) ;
extern "C" vtkNew < vtkSortFieldData > vtkSortFieldData_new () ;
extern "C" void vtkSortFieldData_destructor (vtkNew < vtkSortFieldData > sself) ;
extern "C" void * vtkSortFieldData_get_ptr (vtkNew < vtkSortFieldData > sself) ;
extern "C" vtkNew < vtkSphere > vtkSphere_new () ;
extern "C" void vtkSphere_destructor (vtkNew < vtkSphere > sself) ;
extern "C" void * vtkSphere_get_ptr (vtkNew < vtkSphere > sself) ;
extern "C" vtkNew < vtkSpheres > vtkSpheres_new () ;
extern "C" void vtkSpheres_destructor (vtkNew < vtkSpheres > sself) ;
extern "C" void * vtkSpheres_get_ptr (vtkNew < vtkSpheres > sself) ;
extern "C" vtkNew < vtkSphericalPointIterator > vtkSphericalPointIterator_new () ;
extern "C" void vtkSphericalPointIterator_destructor (vtkNew < vtkSphericalPointIterator > sself) ;
extern "C" void * vtkSphericalPointIterator_get_ptr (vtkNew < vtkSphericalPointIterator > sself) ;
extern "C" vtkNew < vtkStaticCellLinks > vtkStaticCellLinks_new () ;
extern "C" void vtkStaticCellLinks_destructor (vtkNew < vtkStaticCellLinks > sself) ;
extern "C" void * vtkStaticCellLinks_get_ptr (vtkNew < vtkStaticCellLinks > sself) ;
extern "C" vtkNew < vtkStaticCellLocator > vtkStaticCellLocator_new () ;
extern "C" void vtkStaticCellLocator_destructor (vtkNew < vtkStaticCellLocator > sself) ;
extern "C" void * vtkStaticCellLocator_get_ptr (vtkNew < vtkStaticCellLocator > sself) ;
extern "C" vtkNew < vtkStaticPointLocator > vtkStaticPointLocator_new () ;
extern "C" void vtkStaticPointLocator_destructor (vtkNew < vtkStaticPointLocator > sself) ;
extern "C" void * vtkStaticPointLocator_get_ptr (vtkNew < vtkStaticPointLocator > sself) ;
extern "C" vtkNew < vtkStaticPointLocator2D > vtkStaticPointLocator2D_new () ;
extern "C" void vtkStaticPointLocator2D_destructor (vtkNew < vtkStaticPointLocator2D > sself) ;
extern "C" void * vtkStaticPointLocator2D_get_ptr (vtkNew < vtkStaticPointLocator2D > sself) ;
extern "C" vtkNew < vtkStructuredCellArray > vtkStructuredCellArray_new () ;
extern "C" void vtkStructuredCellArray_destructor (vtkNew < vtkStructuredCellArray > sself) ;
extern "C" void * vtkStructuredCellArray_get_ptr (vtkNew < vtkStructuredCellArray > sself) ;
extern "C" vtkNew < vtkStructuredExtent > vtkStructuredExtent_new () ;
extern "C" void vtkStructuredExtent_destructor (vtkNew < vtkStructuredExtent > sself) ;
extern "C" void * vtkStructuredExtent_get_ptr (vtkNew < vtkStructuredExtent > sself) ;
extern "C" vtkNew < vtkStructuredGrid > vtkStructuredGrid_new () ;
extern "C" void vtkStructuredGrid_destructor (vtkNew < vtkStructuredGrid > sself) ;
extern "C" void * vtkStructuredGrid_get_ptr (vtkNew < vtkStructuredGrid > sself) ;
extern "C" vtkNew < vtkStructuredPoints > vtkStructuredPoints_new () ;
extern "C" void vtkStructuredPoints_destructor (vtkNew < vtkStructuredPoints > sself) ;
extern "C" void * vtkStructuredPoints_get_ptr (vtkNew < vtkStructuredPoints > sself) ;
extern "C" vtkNew < vtkStructuredPointsCollection > vtkStructuredPointsCollection_new () ;
extern "C" void vtkStructuredPointsCollection_destructor (vtkNew < vtkStructuredPointsCollection > sself) ;
extern "C" void * vtkStructuredPointsCollection_get_ptr (vtkNew < vtkStructuredPointsCollection > sself) ;
extern "C" vtkNew < vtkSuperquadric > vtkSuperquadric_new () ;
extern "C" void vtkSuperquadric_destructor (vtkNew < vtkSuperquadric > sself) ;
extern "C" void * vtkSuperquadric_get_ptr (vtkNew < vtkSuperquadric > sself) ;
extern "C" vtkNew < vtkTable > vtkTable_new () ;
extern "C" void vtkTable_destructor (vtkNew < vtkTable > sself) ;
extern "C" void * vtkTable_get_ptr (vtkNew < vtkTable > sself) ;
extern "C" vtkNew < vtkTetra > vtkTetra_new () ;
extern "C" void vtkTetra_destructor (vtkNew < vtkTetra > sself) ;
extern "C" void * vtkTetra_get_ptr (vtkNew < vtkTetra > sself) ;
extern "C" vtkNew < vtkTree > vtkTree_new () ;
extern "C" void vtkTree_destructor (vtkNew < vtkTree > sself) ;
extern "C" void * vtkTree_get_ptr (vtkNew < vtkTree > sself) ;
extern "C" vtkNew < vtkTreeBFSIterator > vtkTreeBFSIterator_new () ;
extern "C" void vtkTreeBFSIterator_destructor (vtkNew < vtkTreeBFSIterator > sself) ;
extern "C" void * vtkTreeBFSIterator_get_ptr (vtkNew < vtkTreeBFSIterator > sself) ;
extern "C" vtkNew < vtkTreeDFSIterator > vtkTreeDFSIterator_new () ;
extern "C" void vtkTreeDFSIterator_destructor (vtkNew < vtkTreeDFSIterator > sself) ;
extern "C" void * vtkTreeDFSIterator_get_ptr (vtkNew < vtkTreeDFSIterator > sself) ;
extern "C" vtkNew < vtkTriQuadraticHexahedron > vtkTriQuadraticHexahedron_new () ;
extern "C" void vtkTriQuadraticHexahedron_destructor (vtkNew < vtkTriQuadraticHexahedron > sself) ;
extern "C" void * vtkTriQuadraticHexahedron_get_ptr (vtkNew < vtkTriQuadraticHexahedron > sself) ;
extern "C" vtkNew < vtkTriQuadraticPyramid > vtkTriQuadraticPyramid_new () ;
extern "C" void vtkTriQuadraticPyramid_destructor (vtkNew < vtkTriQuadraticPyramid > sself) ;
extern "C" void * vtkTriQuadraticPyramid_get_ptr (vtkNew < vtkTriQuadraticPyramid > sself) ;
extern "C" vtkNew < vtkTriangle > vtkTriangle_new () ;
extern "C" void vtkTriangle_destructor (vtkNew < vtkTriangle > sself) ;
extern "C" void * vtkTriangle_get_ptr (vtkNew < vtkTriangle > sself) ;
extern "C" vtkNew < vtkTriangleStrip > vtkTriangleStrip_new () ;
extern "C" void vtkTriangleStrip_destructor (vtkNew < vtkTriangleStrip > sself) ;
extern "C" void * vtkTriangleStrip_get_ptr (vtkNew < vtkTriangleStrip > sself) ;
extern "C" vtkNew < vtkUndirectedGraph > vtkUndirectedGraph_new () ;
extern "C" void vtkUndirectedGraph_destructor (vtkNew < vtkUndirectedGraph > sself) ;
extern "C" void * vtkUndirectedGraph_get_ptr (vtkNew < vtkUndirectedGraph > sself) ;
extern "C" vtkNew < vtkUniformGrid > vtkUniformGrid_new () ;
extern "C" void vtkUniformGrid_destructor (vtkNew < vtkUniformGrid > sself) ;
extern "C" void * vtkUniformGrid_get_ptr (vtkNew < vtkUniformGrid > sself) ;
extern "C" vtkNew < vtkUniformGridAMR > vtkUniformGridAMR_new () ;
extern "C" void vtkUniformGridAMR_destructor (vtkNew < vtkUniformGridAMR > sself) ;
extern "C" void * vtkUniformGridAMR_get_ptr (vtkNew < vtkUniformGridAMR > sself) ;
extern "C" vtkNew < vtkUniformGridAMRDataIterator > vtkUniformGridAMRDataIterator_new () ;
extern "C" void vtkUniformGridAMRDataIterator_destructor (vtkNew < vtkUniformGridAMRDataIterator > sself) ;
extern "C" void * vtkUniformGridAMRDataIterator_get_ptr (vtkNew < vtkUniformGridAMRDataIterator > sself) ;
extern "C" vtkNew < vtkUniformHyperTreeGrid > vtkUniformHyperTreeGrid_new () ;
extern "C" void vtkUniformHyperTreeGrid_destructor (vtkNew < vtkUniformHyperTreeGrid > sself) ;
extern "C" void * vtkUniformHyperTreeGrid_get_ptr (vtkNew < vtkUniformHyperTreeGrid > sself) ;
extern "C" vtkNew < vtkUnstructuredGrid > vtkUnstructuredGrid_new () ;
extern "C" void vtkUnstructuredGrid_destructor (vtkNew < vtkUnstructuredGrid > sself) ;
extern "C" void * vtkUnstructuredGrid_get_ptr (vtkNew < vtkUnstructuredGrid > sself) ;
extern "C" vtkNew < vtkUnstructuredGridCellIterator > vtkUnstructuredGridCellIterator_new () ;
extern "C" void vtkUnstructuredGridCellIterator_destructor (vtkNew < vtkUnstructuredGridCellIterator > sself) ;
extern "C" void * vtkUnstructuredGridCellIterator_get_ptr (vtkNew < vtkUnstructuredGridCellIterator > sself) ;
extern "C" vtkNew < vtkVertex > vtkVertex_new () ;
extern "C" void vtkVertex_destructor (vtkNew < vtkVertex > sself) ;
extern "C" void * vtkVertex_get_ptr (vtkNew < vtkVertex > sself) ;
extern "C" vtkNew < vtkVertexListIterator > vtkVertexListIterator_new () ;
extern "C" void vtkVertexListIterator_destructor (vtkNew < vtkVertexListIterator > sself) ;
extern "C" void * vtkVertexListIterator_get_ptr (vtkNew < vtkVertexListIterator > sself) ;
extern "C" vtkNew < vtkVoxel > vtkVoxel_new () ;
extern "C" void vtkVoxel_destructor (vtkNew < vtkVoxel > sself) ;
extern "C" void * vtkVoxel_get_ptr (vtkNew < vtkVoxel > sself) ;
extern "C" vtkNew < vtkWedge > vtkWedge_new () ;
extern "C" void vtkWedge_destructor (vtkNew < vtkWedge > sself) ;
extern "C" void * vtkWedge_get_ptr (vtkNew < vtkWedge > sself) ;
extern "C" vtkNew < vtkXMLDataElement > vtkXMLDataElement_new () ;
extern "C" void vtkXMLDataElement_destructor (vtkNew < vtkXMLDataElement > sself) ;
extern "C" void * vtkXMLDataElement_get_ptr (vtkNew < vtkXMLDataElement > sself) ;
