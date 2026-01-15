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

// Declare exported functions
extern "C" vtkNew < vtkAlgorithm > vtkAlgorithm_new () ;
extern "C" void vtkAlgorithm_destructor (vtkNew < vtkAlgorithm > sself) ;
extern "C" void * vtkAlgorithm_get_ptr (vtkNew < vtkAlgorithm > sself) ;
extern "C" vtkNew < vtkAlgorithmOutput > vtkAlgorithmOutput_new () ;
extern "C" void vtkAlgorithmOutput_destructor (vtkNew < vtkAlgorithmOutput > sself) ;
extern "C" void * vtkAlgorithmOutput_get_ptr (vtkNew < vtkAlgorithmOutput > sself) ;
extern "C" vtkNew < vtkAnnotationLayersAlgorithm > vtkAnnotationLayersAlgorithm_new () ;
extern "C" void vtkAnnotationLayersAlgorithm_destructor (vtkNew < vtkAnnotationLayersAlgorithm > sself) ;
extern "C" void * vtkAnnotationLayersAlgorithm_get_ptr (vtkNew < vtkAnnotationLayersAlgorithm > sself) ;
extern "C" vtkNew < vtkArrayDataAlgorithm > vtkArrayDataAlgorithm_new () ;
extern "C" void vtkArrayDataAlgorithm_destructor (vtkNew < vtkArrayDataAlgorithm > sself) ;
extern "C" void * vtkArrayDataAlgorithm_get_ptr (vtkNew < vtkArrayDataAlgorithm > sself) ;
extern "C" vtkNew < vtkCachedStreamingDemandDrivenPipeline > vtkCachedStreamingDemandDrivenPipeline_new () ;
extern "C" void vtkCachedStreamingDemandDrivenPipeline_destructor (vtkNew < vtkCachedStreamingDemandDrivenPipeline > sself) ;
extern "C" void * vtkCachedStreamingDemandDrivenPipeline_get_ptr (vtkNew < vtkCachedStreamingDemandDrivenPipeline > sself) ;
extern "C" vtkNew < vtkCastToConcrete > vtkCastToConcrete_new () ;
extern "C" void vtkCastToConcrete_destructor (vtkNew < vtkCastToConcrete > sself) ;
extern "C" void * vtkCastToConcrete_get_ptr (vtkNew < vtkCastToConcrete > sself) ;
extern "C" vtkNew < vtkCompositeDataPipeline > vtkCompositeDataPipeline_new () ;
extern "C" void vtkCompositeDataPipeline_destructor (vtkNew < vtkCompositeDataPipeline > sself) ;
extern "C" void * vtkCompositeDataPipeline_get_ptr (vtkNew < vtkCompositeDataPipeline > sself) ;
extern "C" vtkNew < vtkCompositeDataSetAlgorithm > vtkCompositeDataSetAlgorithm_new () ;
extern "C" void vtkCompositeDataSetAlgorithm_destructor (vtkNew < vtkCompositeDataSetAlgorithm > sself) ;
extern "C" void * vtkCompositeDataSetAlgorithm_get_ptr (vtkNew < vtkCompositeDataSetAlgorithm > sself) ;
extern "C" vtkNew < vtkDataObjectAlgorithm > vtkDataObjectAlgorithm_new () ;
extern "C" void vtkDataObjectAlgorithm_destructor (vtkNew < vtkDataObjectAlgorithm > sself) ;
extern "C" void * vtkDataObjectAlgorithm_get_ptr (vtkNew < vtkDataObjectAlgorithm > sself) ;
extern "C" vtkNew < vtkDataSetAlgorithm > vtkDataSetAlgorithm_new () ;
extern "C" void vtkDataSetAlgorithm_destructor (vtkNew < vtkDataSetAlgorithm > sself) ;
extern "C" void * vtkDataSetAlgorithm_get_ptr (vtkNew < vtkDataSetAlgorithm > sself) ;
extern "C" vtkNew < vtkDemandDrivenPipeline > vtkDemandDrivenPipeline_new () ;
extern "C" void vtkDemandDrivenPipeline_destructor (vtkNew < vtkDemandDrivenPipeline > sself) ;
extern "C" void * vtkDemandDrivenPipeline_get_ptr (vtkNew < vtkDemandDrivenPipeline > sself) ;
extern "C" vtkNew < vtkDirectedGraphAlgorithm > vtkDirectedGraphAlgorithm_new () ;
extern "C" void vtkDirectedGraphAlgorithm_destructor (vtkNew < vtkDirectedGraphAlgorithm > sself) ;
extern "C" void * vtkDirectedGraphAlgorithm_get_ptr (vtkNew < vtkDirectedGraphAlgorithm > sself) ;
extern "C" vtkNew < vtkEnsembleSource > vtkEnsembleSource_new () ;
extern "C" void vtkEnsembleSource_destructor (vtkNew < vtkEnsembleSource > sself) ;
extern "C" void * vtkEnsembleSource_get_ptr (vtkNew < vtkEnsembleSource > sself) ;
extern "C" vtkNew < vtkExplicitStructuredGridAlgorithm > vtkExplicitStructuredGridAlgorithm_new () ;
extern "C" void vtkExplicitStructuredGridAlgorithm_destructor (vtkNew < vtkExplicitStructuredGridAlgorithm > sself) ;
extern "C" void * vtkExplicitStructuredGridAlgorithm_get_ptr (vtkNew < vtkExplicitStructuredGridAlgorithm > sself) ;
extern "C" vtkNew < vtkExtentRCBPartitioner > vtkExtentRCBPartitioner_new () ;
extern "C" void vtkExtentRCBPartitioner_destructor (vtkNew < vtkExtentRCBPartitioner > sself) ;
extern "C" void * vtkExtentRCBPartitioner_get_ptr (vtkNew < vtkExtentRCBPartitioner > sself) ;
extern "C" vtkNew < vtkExtentSplitter > vtkExtentSplitter_new () ;
extern "C" void vtkExtentSplitter_destructor (vtkNew < vtkExtentSplitter > sself) ;
extern "C" void * vtkExtentSplitter_get_ptr (vtkNew < vtkExtentSplitter > sself) ;
extern "C" vtkNew < vtkExtentTranslator > vtkExtentTranslator_new () ;
extern "C" void vtkExtentTranslator_destructor (vtkNew < vtkExtentTranslator > sself) ;
extern "C" void * vtkExtentTranslator_get_ptr (vtkNew < vtkExtentTranslator > sself) ;
extern "C" vtkNew < vtkGraphAlgorithm > vtkGraphAlgorithm_new () ;
extern "C" void vtkGraphAlgorithm_destructor (vtkNew < vtkGraphAlgorithm > sself) ;
extern "C" void * vtkGraphAlgorithm_get_ptr (vtkNew < vtkGraphAlgorithm > sself) ;
extern "C" vtkNew < vtkHierarchicalBoxDataSetAlgorithm > vtkHierarchicalBoxDataSetAlgorithm_new () ;
extern "C" void vtkHierarchicalBoxDataSetAlgorithm_destructor (vtkNew < vtkHierarchicalBoxDataSetAlgorithm > sself) ;
extern "C" void * vtkHierarchicalBoxDataSetAlgorithm_get_ptr (vtkNew < vtkHierarchicalBoxDataSetAlgorithm > sself) ;
extern "C" vtkNew < vtkImageToStructuredGrid > vtkImageToStructuredGrid_new () ;
extern "C" void vtkImageToStructuredGrid_destructor (vtkNew < vtkImageToStructuredGrid > sself) ;
extern "C" void * vtkImageToStructuredGrid_get_ptr (vtkNew < vtkImageToStructuredGrid > sself) ;
extern "C" vtkNew < vtkImageToStructuredPoints > vtkImageToStructuredPoints_new () ;
extern "C" void vtkImageToStructuredPoints_destructor (vtkNew < vtkImageToStructuredPoints > sself) ;
extern "C" void * vtkImageToStructuredPoints_get_ptr (vtkNew < vtkImageToStructuredPoints > sself) ;
extern "C" vtkNew < vtkMoleculeAlgorithm > vtkMoleculeAlgorithm_new () ;
extern "C" void vtkMoleculeAlgorithm_destructor (vtkNew < vtkMoleculeAlgorithm > sself) ;
extern "C" void * vtkMoleculeAlgorithm_get_ptr (vtkNew < vtkMoleculeAlgorithm > sself) ;
extern "C" vtkNew < vtkMultiBlockDataSetAlgorithm > vtkMultiBlockDataSetAlgorithm_new () ;
extern "C" void vtkMultiBlockDataSetAlgorithm_destructor (vtkNew < vtkMultiBlockDataSetAlgorithm > sself) ;
extern "C" void * vtkMultiBlockDataSetAlgorithm_get_ptr (vtkNew < vtkMultiBlockDataSetAlgorithm > sself) ;
extern "C" vtkNew < vtkMultiTimeStepAlgorithm > vtkMultiTimeStepAlgorithm_new () ;
extern "C" void vtkMultiTimeStepAlgorithm_destructor (vtkNew < vtkMultiTimeStepAlgorithm > sself) ;
extern "C" void * vtkMultiTimeStepAlgorithm_get_ptr (vtkNew < vtkMultiTimeStepAlgorithm > sself) ;
extern "C" vtkNew < vtkNonOverlappingAMRAlgorithm > vtkNonOverlappingAMRAlgorithm_new () ;
extern "C" void vtkNonOverlappingAMRAlgorithm_destructor (vtkNew < vtkNonOverlappingAMRAlgorithm > sself) ;
extern "C" void * vtkNonOverlappingAMRAlgorithm_get_ptr (vtkNew < vtkNonOverlappingAMRAlgorithm > sself) ;
extern "C" vtkNew < vtkOverlappingAMRAlgorithm > vtkOverlappingAMRAlgorithm_new () ;
extern "C" void vtkOverlappingAMRAlgorithm_destructor (vtkNew < vtkOverlappingAMRAlgorithm > sself) ;
extern "C" void * vtkOverlappingAMRAlgorithm_get_ptr (vtkNew < vtkOverlappingAMRAlgorithm > sself) ;
extern "C" vtkNew < vtkPassInputTypeAlgorithm > vtkPassInputTypeAlgorithm_new () ;
extern "C" void vtkPassInputTypeAlgorithm_destructor (vtkNew < vtkPassInputTypeAlgorithm > sself) ;
extern "C" void * vtkPassInputTypeAlgorithm_get_ptr (vtkNew < vtkPassInputTypeAlgorithm > sself) ;
extern "C" vtkNew < vtkPiecewiseFunctionAlgorithm > vtkPiecewiseFunctionAlgorithm_new () ;
extern "C" void vtkPiecewiseFunctionAlgorithm_destructor (vtkNew < vtkPiecewiseFunctionAlgorithm > sself) ;
extern "C" void * vtkPiecewiseFunctionAlgorithm_get_ptr (vtkNew < vtkPiecewiseFunctionAlgorithm > sself) ;
extern "C" vtkNew < vtkPiecewiseFunctionShiftScale > vtkPiecewiseFunctionShiftScale_new () ;
extern "C" void vtkPiecewiseFunctionShiftScale_destructor (vtkNew < vtkPiecewiseFunctionShiftScale > sself) ;
extern "C" void * vtkPiecewiseFunctionShiftScale_get_ptr (vtkNew < vtkPiecewiseFunctionShiftScale > sself) ;
extern "C" vtkNew < vtkPointSetAlgorithm > vtkPointSetAlgorithm_new () ;
extern "C" void vtkPointSetAlgorithm_destructor (vtkNew < vtkPointSetAlgorithm > sself) ;
extern "C" void * vtkPointSetAlgorithm_get_ptr (vtkNew < vtkPointSetAlgorithm > sself) ;
extern "C" vtkNew < vtkPolyDataAlgorithm > vtkPolyDataAlgorithm_new () ;
extern "C" void vtkPolyDataAlgorithm_destructor (vtkNew < vtkPolyDataAlgorithm > sself) ;
extern "C" void * vtkPolyDataAlgorithm_get_ptr (vtkNew < vtkPolyDataAlgorithm > sself) ;
extern "C" vtkNew < vtkProgressObserver > vtkProgressObserver_new () ;
extern "C" void vtkProgressObserver_destructor (vtkNew < vtkProgressObserver > sself) ;
extern "C" void * vtkProgressObserver_get_ptr (vtkNew < vtkProgressObserver > sself) ;
extern "C" vtkNew < vtkReaderExecutive > vtkReaderExecutive_new () ;
extern "C" void vtkReaderExecutive_destructor (vtkNew < vtkReaderExecutive > sself) ;
extern "C" void * vtkReaderExecutive_get_ptr (vtkNew < vtkReaderExecutive > sself) ;
extern "C" vtkNew < vtkRectilinearGridAlgorithm > vtkRectilinearGridAlgorithm_new () ;
extern "C" void vtkRectilinearGridAlgorithm_destructor (vtkNew < vtkRectilinearGridAlgorithm > sself) ;
extern "C" void * vtkRectilinearGridAlgorithm_get_ptr (vtkNew < vtkRectilinearGridAlgorithm > sself) ;
extern "C" vtkNew < vtkSMPProgressObserver > vtkSMPProgressObserver_new () ;
extern "C" void vtkSMPProgressObserver_destructor (vtkNew < vtkSMPProgressObserver > sself) ;
extern "C" void * vtkSMPProgressObserver_get_ptr (vtkNew < vtkSMPProgressObserver > sself) ;
extern "C" vtkNew < vtkSelectionAlgorithm > vtkSelectionAlgorithm_new () ;
extern "C" void vtkSelectionAlgorithm_destructor (vtkNew < vtkSelectionAlgorithm > sself) ;
extern "C" void * vtkSelectionAlgorithm_get_ptr (vtkNew < vtkSelectionAlgorithm > sself) ;
extern "C" vtkNew < vtkSimpleScalarTree > vtkSimpleScalarTree_new () ;
extern "C" void vtkSimpleScalarTree_destructor (vtkNew < vtkSimpleScalarTree > sself) ;
extern "C" void * vtkSimpleScalarTree_get_ptr (vtkNew < vtkSimpleScalarTree > sself) ;
extern "C" vtkNew < vtkSpanSpace > vtkSpanSpace_new () ;
extern "C" void vtkSpanSpace_destructor (vtkNew < vtkSpanSpace > sself) ;
extern "C" void * vtkSpanSpace_get_ptr (vtkNew < vtkSpanSpace > sself) ;
extern "C" vtkNew < vtkSphereTree > vtkSphereTree_new () ;
extern "C" void vtkSphereTree_destructor (vtkNew < vtkSphereTree > sself) ;
extern "C" void * vtkSphereTree_get_ptr (vtkNew < vtkSphereTree > sself) ;
extern "C" vtkNew < vtkStreamingDemandDrivenPipeline > vtkStreamingDemandDrivenPipeline_new () ;
extern "C" void vtkStreamingDemandDrivenPipeline_destructor (vtkNew < vtkStreamingDemandDrivenPipeline > sself) ;
extern "C" void * vtkStreamingDemandDrivenPipeline_get_ptr (vtkNew < vtkStreamingDemandDrivenPipeline > sself) ;
extern "C" vtkNew < vtkStructuredGridAlgorithm > vtkStructuredGridAlgorithm_new () ;
extern "C" void vtkStructuredGridAlgorithm_destructor (vtkNew < vtkStructuredGridAlgorithm > sself) ;
extern "C" void * vtkStructuredGridAlgorithm_get_ptr (vtkNew < vtkStructuredGridAlgorithm > sself) ;
extern "C" vtkNew < vtkTableAlgorithm > vtkTableAlgorithm_new () ;
extern "C" void vtkTableAlgorithm_destructor (vtkNew < vtkTableAlgorithm > sself) ;
extern "C" void * vtkTableAlgorithm_get_ptr (vtkNew < vtkTableAlgorithm > sself) ;
extern "C" vtkNew < vtkThreadedCompositeDataPipeline > vtkThreadedCompositeDataPipeline_new () ;
extern "C" void vtkThreadedCompositeDataPipeline_destructor (vtkNew < vtkThreadedCompositeDataPipeline > sself) ;
extern "C" void * vtkThreadedCompositeDataPipeline_get_ptr (vtkNew < vtkThreadedCompositeDataPipeline > sself) ;
extern "C" vtkNew < vtkTreeAlgorithm > vtkTreeAlgorithm_new () ;
extern "C" void vtkTreeAlgorithm_destructor (vtkNew < vtkTreeAlgorithm > sself) ;
extern "C" void * vtkTreeAlgorithm_get_ptr (vtkNew < vtkTreeAlgorithm > sself) ;
extern "C" vtkNew < vtkTrivialConsumer > vtkTrivialConsumer_new () ;
extern "C" void vtkTrivialConsumer_destructor (vtkNew < vtkTrivialConsumer > sself) ;
extern "C" void * vtkTrivialConsumer_get_ptr (vtkNew < vtkTrivialConsumer > sself) ;
extern "C" vtkNew < vtkTrivialProducer > vtkTrivialProducer_new () ;
extern "C" void vtkTrivialProducer_destructor (vtkNew < vtkTrivialProducer > sself) ;
extern "C" void * vtkTrivialProducer_get_ptr (vtkNew < vtkTrivialProducer > sself) ;
extern "C" vtkNew < vtkUndirectedGraphAlgorithm > vtkUndirectedGraphAlgorithm_new () ;
extern "C" void vtkUndirectedGraphAlgorithm_destructor (vtkNew < vtkUndirectedGraphAlgorithm > sself) ;
extern "C" void * vtkUndirectedGraphAlgorithm_get_ptr (vtkNew < vtkUndirectedGraphAlgorithm > sself) ;
extern "C" vtkNew < vtkUniformGridAMRAlgorithm > vtkUniformGridAMRAlgorithm_new () ;
extern "C" void vtkUniformGridAMRAlgorithm_destructor (vtkNew < vtkUniformGridAMRAlgorithm > sself) ;
extern "C" void * vtkUniformGridAMRAlgorithm_get_ptr (vtkNew < vtkUniformGridAMRAlgorithm > sself) ;
extern "C" vtkNew < vtkUniformGridPartitioner > vtkUniformGridPartitioner_new () ;
extern "C" void vtkUniformGridPartitioner_destructor (vtkNew < vtkUniformGridPartitioner > sself) ;
extern "C" void * vtkUniformGridPartitioner_get_ptr (vtkNew < vtkUniformGridPartitioner > sself) ;
extern "C" vtkNew < vtkUnstructuredGridAlgorithm > vtkUnstructuredGridAlgorithm_new () ;
extern "C" void vtkUnstructuredGridAlgorithm_destructor (vtkNew < vtkUnstructuredGridAlgorithm > sself) ;
extern "C" void * vtkUnstructuredGridAlgorithm_get_ptr (vtkNew < vtkUnstructuredGridAlgorithm > sself) ;
extern "C" vtkNew < vtkUnstructuredGridBaseAlgorithm > vtkUnstructuredGridBaseAlgorithm_new () ;
extern "C" void vtkUnstructuredGridBaseAlgorithm_destructor (vtkNew < vtkUnstructuredGridBaseAlgorithm > sself) ;
extern "C" void * vtkUnstructuredGridBaseAlgorithm_get_ptr (vtkNew < vtkUnstructuredGridBaseAlgorithm > sself) ;
