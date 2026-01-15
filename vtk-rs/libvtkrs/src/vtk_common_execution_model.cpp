// Include header file
#include<vtk_common_execution_model.h>

// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkAlgorithm.h>
#include<vtkAlgorithmOutput.h>
#include<vtkAnnotationLayersAlgorithm.h>
#include<vtkArrayDataAlgorithm.h>
#include<vtkCachedStreamingDemandDrivenPipeline.h>
#include<vtkCastToConcrete.h>
#include<vtkCompositeDataPipeline.h>
#include<vtkCompositeDataSetAlgorithm.h>
#include<vtkDataObjectAlgorithm.h>
#include<vtkDataSetAlgorithm.h>
#include<vtkDemandDrivenPipeline.h>
#include<vtkDirectedGraphAlgorithm.h>
#include<vtkEnsembleSource.h>
#include<vtkExecutive.h>
#include<vtkExplicitStructuredGridAlgorithm.h>
#include<vtkExtentRCBPartitioner.h>
#include<vtkExtentSplitter.h>
#include<vtkExtentTranslator.h>
#include<vtkFilteringInformationKeyManager.h>
#include<vtkGraphAlgorithm.h>
#include<vtkHierarchicalBoxDataSetAlgorithm.h>
#include<vtkHyperTreeGridAlgorithm.h>
#include<vtkImageAlgorithm.h>
#include<vtkImageInPlaceFilter.h>
#include<vtkImageProgressIterator.h>
#include<vtkImageToStructuredGrid.h>
#include<vtkImageToStructuredPoints.h>
#include<vtkInformationDataObjectMetaDataKey.h>
#include<vtkInformationExecutivePortKey.h>
#include<vtkInformationExecutivePortVectorKey.h>
#include<vtkInformationIntegerRequestKey.h>
#include<vtkMoleculeAlgorithm.h>
#include<vtkMultiBlockDataSetAlgorithm.h>
#include<vtkMultiTimeStepAlgorithm.h>
#include<vtkNonOverlappingAMRAlgorithm.h>
#include<vtkOverlappingAMRAlgorithm.h>
#include<vtkParallelReader.h>
#include<vtkPartitionedDataSetAlgorithm.h>
#include<vtkPartitionedDataSetCollectionAlgorithm.h>
#include<vtkPassInputTypeAlgorithm.h>
#include<vtkPiecewiseFunctionAlgorithm.h>
#include<vtkPiecewiseFunctionShiftScale.h>
#include<vtkPointSetAlgorithm.h>
#include<vtkPolyDataAlgorithm.h>
#include<vtkProgressObserver.h>
#include<vtkReaderAlgorithm.h>
#include<vtkReaderExecutive.h>
#include<vtkRectilinearGridAlgorithm.h>
#include<vtkSMPProgressObserver.h>
#include<vtkScalarTree.h>
#include<vtkSelectionAlgorithm.h>
#include<vtkSimpleImageToImageFilter.h>
#include<vtkSimpleReader.h>
#include<vtkSimpleScalarTree.h>
#include<vtkSpanSpace.h>
#include<vtkSphereTree.h>
#include<vtkStreamingDemandDrivenPipeline.h>
#include<vtkStructuredGridAlgorithm.h>
#include<vtkTableAlgorithm.h>
#include<vtkThreadedCompositeDataPipeline.h>
#include<vtkThreadedImageAlgorithm.h>
#include<vtkTreeAlgorithm.h>
#include<vtkTrivialConsumer.h>
#include<vtkTrivialProducer.h>
#include<vtkUndirectedGraphAlgorithm.h>
#include<vtkUniformGridAMRAlgorithm.h>
#include<vtkUniformGridPartitioner.h>
#include<vtkUnstructuredGridAlgorithm.h>
#include<vtkUnstructuredGridBaseAlgorithm.h>

// Implement declared functions
extern "C" vtkNew < vtkAlgorithm > vtkAlgorithm_new () {return vtkNew < vtkAlgorithm > () ;}
extern "C" void vtkAlgorithm_destructor (vtkNew < vtkAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkAlgorithm_get_ptr (vtkNew < vtkAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkAlgorithmOutput > vtkAlgorithmOutput_new () {return vtkNew < vtkAlgorithmOutput > () ;}
extern "C" void vtkAlgorithmOutput_destructor (vtkNew < vtkAlgorithmOutput > sself) {sself . Reset () ; return ;}
extern "C" void * vtkAlgorithmOutput_get_ptr (vtkNew < vtkAlgorithmOutput > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkAnnotationLayersAlgorithm > vtkAnnotationLayersAlgorithm_new () {return vtkNew < vtkAnnotationLayersAlgorithm > () ;}
extern "C" void vtkAnnotationLayersAlgorithm_destructor (vtkNew < vtkAnnotationLayersAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkAnnotationLayersAlgorithm_get_ptr (vtkNew < vtkAnnotationLayersAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkArrayDataAlgorithm > vtkArrayDataAlgorithm_new () {return vtkNew < vtkArrayDataAlgorithm > () ;}
extern "C" void vtkArrayDataAlgorithm_destructor (vtkNew < vtkArrayDataAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkArrayDataAlgorithm_get_ptr (vtkNew < vtkArrayDataAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCachedStreamingDemandDrivenPipeline > vtkCachedStreamingDemandDrivenPipeline_new () {return vtkNew < vtkCachedStreamingDemandDrivenPipeline > () ;}
extern "C" void vtkCachedStreamingDemandDrivenPipeline_destructor (vtkNew < vtkCachedStreamingDemandDrivenPipeline > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCachedStreamingDemandDrivenPipeline_get_ptr (vtkNew < vtkCachedStreamingDemandDrivenPipeline > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCastToConcrete > vtkCastToConcrete_new () {return vtkNew < vtkCastToConcrete > () ;}
extern "C" void vtkCastToConcrete_destructor (vtkNew < vtkCastToConcrete > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCastToConcrete_get_ptr (vtkNew < vtkCastToConcrete > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCompositeDataPipeline > vtkCompositeDataPipeline_new () {return vtkNew < vtkCompositeDataPipeline > () ;}
extern "C" void vtkCompositeDataPipeline_destructor (vtkNew < vtkCompositeDataPipeline > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCompositeDataPipeline_get_ptr (vtkNew < vtkCompositeDataPipeline > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCompositeDataSetAlgorithm > vtkCompositeDataSetAlgorithm_new () {return vtkNew < vtkCompositeDataSetAlgorithm > () ;}
extern "C" void vtkCompositeDataSetAlgorithm_destructor (vtkNew < vtkCompositeDataSetAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCompositeDataSetAlgorithm_get_ptr (vtkNew < vtkCompositeDataSetAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDataObjectAlgorithm > vtkDataObjectAlgorithm_new () {return vtkNew < vtkDataObjectAlgorithm > () ;}
extern "C" void vtkDataObjectAlgorithm_destructor (vtkNew < vtkDataObjectAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDataObjectAlgorithm_get_ptr (vtkNew < vtkDataObjectAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDataSetAlgorithm > vtkDataSetAlgorithm_new () {return vtkNew < vtkDataSetAlgorithm > () ;}
extern "C" void vtkDataSetAlgorithm_destructor (vtkNew < vtkDataSetAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDataSetAlgorithm_get_ptr (vtkNew < vtkDataSetAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDemandDrivenPipeline > vtkDemandDrivenPipeline_new () {return vtkNew < vtkDemandDrivenPipeline > () ;}
extern "C" void vtkDemandDrivenPipeline_destructor (vtkNew < vtkDemandDrivenPipeline > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDemandDrivenPipeline_get_ptr (vtkNew < vtkDemandDrivenPipeline > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDirectedGraphAlgorithm > vtkDirectedGraphAlgorithm_new () {return vtkNew < vtkDirectedGraphAlgorithm > () ;}
extern "C" void vtkDirectedGraphAlgorithm_destructor (vtkNew < vtkDirectedGraphAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDirectedGraphAlgorithm_get_ptr (vtkNew < vtkDirectedGraphAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkEnsembleSource > vtkEnsembleSource_new () {return vtkNew < vtkEnsembleSource > () ;}
extern "C" void vtkEnsembleSource_destructor (vtkNew < vtkEnsembleSource > sself) {sself . Reset () ; return ;}
extern "C" void * vtkEnsembleSource_get_ptr (vtkNew < vtkEnsembleSource > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkExplicitStructuredGridAlgorithm > vtkExplicitStructuredGridAlgorithm_new () {return vtkNew < vtkExplicitStructuredGridAlgorithm > () ;}
extern "C" void vtkExplicitStructuredGridAlgorithm_destructor (vtkNew < vtkExplicitStructuredGridAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkExplicitStructuredGridAlgorithm_get_ptr (vtkNew < vtkExplicitStructuredGridAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkExtentRCBPartitioner > vtkExtentRCBPartitioner_new () {return vtkNew < vtkExtentRCBPartitioner > () ;}
extern "C" void vtkExtentRCBPartitioner_destructor (vtkNew < vtkExtentRCBPartitioner > sself) {sself . Reset () ; return ;}
extern "C" void * vtkExtentRCBPartitioner_get_ptr (vtkNew < vtkExtentRCBPartitioner > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkExtentSplitter > vtkExtentSplitter_new () {return vtkNew < vtkExtentSplitter > () ;}
extern "C" void vtkExtentSplitter_destructor (vtkNew < vtkExtentSplitter > sself) {sself . Reset () ; return ;}
extern "C" void * vtkExtentSplitter_get_ptr (vtkNew < vtkExtentSplitter > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkExtentTranslator > vtkExtentTranslator_new () {return vtkNew < vtkExtentTranslator > () ;}
extern "C" void vtkExtentTranslator_destructor (vtkNew < vtkExtentTranslator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkExtentTranslator_get_ptr (vtkNew < vtkExtentTranslator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkGraphAlgorithm > vtkGraphAlgorithm_new () {return vtkNew < vtkGraphAlgorithm > () ;}
extern "C" void vtkGraphAlgorithm_destructor (vtkNew < vtkGraphAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkGraphAlgorithm_get_ptr (vtkNew < vtkGraphAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHierarchicalBoxDataSetAlgorithm > vtkHierarchicalBoxDataSetAlgorithm_new () {return vtkNew < vtkHierarchicalBoxDataSetAlgorithm > () ;}
extern "C" void vtkHierarchicalBoxDataSetAlgorithm_destructor (vtkNew < vtkHierarchicalBoxDataSetAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHierarchicalBoxDataSetAlgorithm_get_ptr (vtkNew < vtkHierarchicalBoxDataSetAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkImageToStructuredGrid > vtkImageToStructuredGrid_new () {return vtkNew < vtkImageToStructuredGrid > () ;}
extern "C" void vtkImageToStructuredGrid_destructor (vtkNew < vtkImageToStructuredGrid > sself) {sself . Reset () ; return ;}
extern "C" void * vtkImageToStructuredGrid_get_ptr (vtkNew < vtkImageToStructuredGrid > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkImageToStructuredPoints > vtkImageToStructuredPoints_new () {return vtkNew < vtkImageToStructuredPoints > () ;}
extern "C" void vtkImageToStructuredPoints_destructor (vtkNew < vtkImageToStructuredPoints > sself) {sself . Reset () ; return ;}
extern "C" void * vtkImageToStructuredPoints_get_ptr (vtkNew < vtkImageToStructuredPoints > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMoleculeAlgorithm > vtkMoleculeAlgorithm_new () {return vtkNew < vtkMoleculeAlgorithm > () ;}
extern "C" void vtkMoleculeAlgorithm_destructor (vtkNew < vtkMoleculeAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMoleculeAlgorithm_get_ptr (vtkNew < vtkMoleculeAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMultiBlockDataSetAlgorithm > vtkMultiBlockDataSetAlgorithm_new () {return vtkNew < vtkMultiBlockDataSetAlgorithm > () ;}
extern "C" void vtkMultiBlockDataSetAlgorithm_destructor (vtkNew < vtkMultiBlockDataSetAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMultiBlockDataSetAlgorithm_get_ptr (vtkNew < vtkMultiBlockDataSetAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMultiTimeStepAlgorithm > vtkMultiTimeStepAlgorithm_new () {return vtkNew < vtkMultiTimeStepAlgorithm > () ;}
extern "C" void vtkMultiTimeStepAlgorithm_destructor (vtkNew < vtkMultiTimeStepAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMultiTimeStepAlgorithm_get_ptr (vtkNew < vtkMultiTimeStepAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkNonOverlappingAMRAlgorithm > vtkNonOverlappingAMRAlgorithm_new () {return vtkNew < vtkNonOverlappingAMRAlgorithm > () ;}
extern "C" void vtkNonOverlappingAMRAlgorithm_destructor (vtkNew < vtkNonOverlappingAMRAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkNonOverlappingAMRAlgorithm_get_ptr (vtkNew < vtkNonOverlappingAMRAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkOverlappingAMRAlgorithm > vtkOverlappingAMRAlgorithm_new () {return vtkNew < vtkOverlappingAMRAlgorithm > () ;}
extern "C" void vtkOverlappingAMRAlgorithm_destructor (vtkNew < vtkOverlappingAMRAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkOverlappingAMRAlgorithm_get_ptr (vtkNew < vtkOverlappingAMRAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPassInputTypeAlgorithm > vtkPassInputTypeAlgorithm_new () {return vtkNew < vtkPassInputTypeAlgorithm > () ;}
extern "C" void vtkPassInputTypeAlgorithm_destructor (vtkNew < vtkPassInputTypeAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPassInputTypeAlgorithm_get_ptr (vtkNew < vtkPassInputTypeAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPiecewiseFunctionAlgorithm > vtkPiecewiseFunctionAlgorithm_new () {return vtkNew < vtkPiecewiseFunctionAlgorithm > () ;}
extern "C" void vtkPiecewiseFunctionAlgorithm_destructor (vtkNew < vtkPiecewiseFunctionAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPiecewiseFunctionAlgorithm_get_ptr (vtkNew < vtkPiecewiseFunctionAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPiecewiseFunctionShiftScale > vtkPiecewiseFunctionShiftScale_new () {return vtkNew < vtkPiecewiseFunctionShiftScale > () ;}
extern "C" void vtkPiecewiseFunctionShiftScale_destructor (vtkNew < vtkPiecewiseFunctionShiftScale > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPiecewiseFunctionShiftScale_get_ptr (vtkNew < vtkPiecewiseFunctionShiftScale > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPointSetAlgorithm > vtkPointSetAlgorithm_new () {return vtkNew < vtkPointSetAlgorithm > () ;}
extern "C" void vtkPointSetAlgorithm_destructor (vtkNew < vtkPointSetAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPointSetAlgorithm_get_ptr (vtkNew < vtkPointSetAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPolyDataAlgorithm > vtkPolyDataAlgorithm_new () {return vtkNew < vtkPolyDataAlgorithm > () ;}
extern "C" void vtkPolyDataAlgorithm_destructor (vtkNew < vtkPolyDataAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPolyDataAlgorithm_get_ptr (vtkNew < vtkPolyDataAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkProgressObserver > vtkProgressObserver_new () {return vtkNew < vtkProgressObserver > () ;}
extern "C" void vtkProgressObserver_destructor (vtkNew < vtkProgressObserver > sself) {sself . Reset () ; return ;}
extern "C" void * vtkProgressObserver_get_ptr (vtkNew < vtkProgressObserver > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkReaderExecutive > vtkReaderExecutive_new () {return vtkNew < vtkReaderExecutive > () ;}
extern "C" void vtkReaderExecutive_destructor (vtkNew < vtkReaderExecutive > sself) {sself . Reset () ; return ;}
extern "C" void * vtkReaderExecutive_get_ptr (vtkNew < vtkReaderExecutive > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkRectilinearGridAlgorithm > vtkRectilinearGridAlgorithm_new () {return vtkNew < vtkRectilinearGridAlgorithm > () ;}
extern "C" void vtkRectilinearGridAlgorithm_destructor (vtkNew < vtkRectilinearGridAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkRectilinearGridAlgorithm_get_ptr (vtkNew < vtkRectilinearGridAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSMPProgressObserver > vtkSMPProgressObserver_new () {return vtkNew < vtkSMPProgressObserver > () ;}
extern "C" void vtkSMPProgressObserver_destructor (vtkNew < vtkSMPProgressObserver > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSMPProgressObserver_get_ptr (vtkNew < vtkSMPProgressObserver > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSelectionAlgorithm > vtkSelectionAlgorithm_new () {return vtkNew < vtkSelectionAlgorithm > () ;}
extern "C" void vtkSelectionAlgorithm_destructor (vtkNew < vtkSelectionAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSelectionAlgorithm_get_ptr (vtkNew < vtkSelectionAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSimpleScalarTree > vtkSimpleScalarTree_new () {return vtkNew < vtkSimpleScalarTree > () ;}
extern "C" void vtkSimpleScalarTree_destructor (vtkNew < vtkSimpleScalarTree > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSimpleScalarTree_get_ptr (vtkNew < vtkSimpleScalarTree > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSpanSpace > vtkSpanSpace_new () {return vtkNew < vtkSpanSpace > () ;}
extern "C" void vtkSpanSpace_destructor (vtkNew < vtkSpanSpace > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSpanSpace_get_ptr (vtkNew < vtkSpanSpace > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSphereTree > vtkSphereTree_new () {return vtkNew < vtkSphereTree > () ;}
extern "C" void vtkSphereTree_destructor (vtkNew < vtkSphereTree > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSphereTree_get_ptr (vtkNew < vtkSphereTree > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkStreamingDemandDrivenPipeline > vtkStreamingDemandDrivenPipeline_new () {return vtkNew < vtkStreamingDemandDrivenPipeline > () ;}
extern "C" void vtkStreamingDemandDrivenPipeline_destructor (vtkNew < vtkStreamingDemandDrivenPipeline > sself) {sself . Reset () ; return ;}
extern "C" void * vtkStreamingDemandDrivenPipeline_get_ptr (vtkNew < vtkStreamingDemandDrivenPipeline > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkStructuredGridAlgorithm > vtkStructuredGridAlgorithm_new () {return vtkNew < vtkStructuredGridAlgorithm > () ;}
extern "C" void vtkStructuredGridAlgorithm_destructor (vtkNew < vtkStructuredGridAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkStructuredGridAlgorithm_get_ptr (vtkNew < vtkStructuredGridAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTableAlgorithm > vtkTableAlgorithm_new () {return vtkNew < vtkTableAlgorithm > () ;}
extern "C" void vtkTableAlgorithm_destructor (vtkNew < vtkTableAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTableAlgorithm_get_ptr (vtkNew < vtkTableAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkThreadedCompositeDataPipeline > vtkThreadedCompositeDataPipeline_new () {return vtkNew < vtkThreadedCompositeDataPipeline > () ;}
extern "C" void vtkThreadedCompositeDataPipeline_destructor (vtkNew < vtkThreadedCompositeDataPipeline > sself) {sself . Reset () ; return ;}
extern "C" void * vtkThreadedCompositeDataPipeline_get_ptr (vtkNew < vtkThreadedCompositeDataPipeline > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTreeAlgorithm > vtkTreeAlgorithm_new () {return vtkNew < vtkTreeAlgorithm > () ;}
extern "C" void vtkTreeAlgorithm_destructor (vtkNew < vtkTreeAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTreeAlgorithm_get_ptr (vtkNew < vtkTreeAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTrivialConsumer > vtkTrivialConsumer_new () {return vtkNew < vtkTrivialConsumer > () ;}
extern "C" void vtkTrivialConsumer_destructor (vtkNew < vtkTrivialConsumer > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTrivialConsumer_get_ptr (vtkNew < vtkTrivialConsumer > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTrivialProducer > vtkTrivialProducer_new () {return vtkNew < vtkTrivialProducer > () ;}
extern "C" void vtkTrivialProducer_destructor (vtkNew < vtkTrivialProducer > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTrivialProducer_get_ptr (vtkNew < vtkTrivialProducer > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUndirectedGraphAlgorithm > vtkUndirectedGraphAlgorithm_new () {return vtkNew < vtkUndirectedGraphAlgorithm > () ;}
extern "C" void vtkUndirectedGraphAlgorithm_destructor (vtkNew < vtkUndirectedGraphAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUndirectedGraphAlgorithm_get_ptr (vtkNew < vtkUndirectedGraphAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUniformGridAMRAlgorithm > vtkUniformGridAMRAlgorithm_new () {return vtkNew < vtkUniformGridAMRAlgorithm > () ;}
extern "C" void vtkUniformGridAMRAlgorithm_destructor (vtkNew < vtkUniformGridAMRAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUniformGridAMRAlgorithm_get_ptr (vtkNew < vtkUniformGridAMRAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUniformGridPartitioner > vtkUniformGridPartitioner_new () {return vtkNew < vtkUniformGridPartitioner > () ;}
extern "C" void vtkUniformGridPartitioner_destructor (vtkNew < vtkUniformGridPartitioner > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUniformGridPartitioner_get_ptr (vtkNew < vtkUniformGridPartitioner > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUnstructuredGridAlgorithm > vtkUnstructuredGridAlgorithm_new () {return vtkNew < vtkUnstructuredGridAlgorithm > () ;}
extern "C" void vtkUnstructuredGridAlgorithm_destructor (vtkNew < vtkUnstructuredGridAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUnstructuredGridAlgorithm_get_ptr (vtkNew < vtkUnstructuredGridAlgorithm > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUnstructuredGridBaseAlgorithm > vtkUnstructuredGridBaseAlgorithm_new () {return vtkNew < vtkUnstructuredGridBaseAlgorithm > () ;}
extern "C" void vtkUnstructuredGridBaseAlgorithm_destructor (vtkNew < vtkUnstructuredGridBaseAlgorithm > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUnstructuredGridBaseAlgorithm_get_ptr (vtkNew < vtkUnstructuredGridBaseAlgorithm > sself) {return sself . GetPointer () ;}
