// Include header file
#include <vtk_common_execution_model.h>

// Default include in all modules
#include <vtkNew.h>
#include <vtkObjectBase.h>

// Include objects of this module
#include <vtkAggregateToPartitionedDataSetCollection.h>
#include <vtkAlgorithm.h>
#include <vtkAlgorithmOutput.h>
#include <vtkAnnotationLayersAlgorithm.h>
#include <vtkArrayDataAlgorithm.h>
#include <vtkCachedStreamingDemandDrivenPipeline.h>
#include <vtkCastToConcrete.h>
#include <vtkCellGridAlgorithm.h>
#include <vtkCompositeDataPipeline.h>
#include <vtkCompositeDataSetAlgorithm.h>
#include <vtkDataObjectAlgorithm.h>
#include <vtkDataSetAlgorithm.h>
#include <vtkDemandDrivenPipeline.h>
#include <vtkDirectedGraphAlgorithm.h>
#include <vtkEndFor.h>
#include <vtkEnsembleSource.h>
#include <vtkExecutionAggregator.h>
#include <vtkExecutionRange.h>
#include <vtkExecutive.h>
#include <vtkExplicitStructuredGridAlgorithm.h>
#include <vtkExtentRCBPartitioner.h>
#include <vtkExtentSplitter.h>
#include <vtkExtentTranslator.h>
#include <vtkFilteringInformationKeyManager.h>
#include <vtkForEach.h>
#include <vtkGraphAlgorithm.h>
#include <vtkHierarchicalBoxDataSetAlgorithm.h>
#include <vtkHyperTreeGridAlgorithm.h>
#include <vtkImageAlgorithm.h>
#include <vtkImageInPlaceFilter.h>
#include <vtkImageProgressIterator.h>
#include <vtkImageToStructuredGrid.h>
#include <vtkImageToStructuredPoints.h>
#include <vtkInformationDataObjectMetaDataKey.h>
#include <vtkInformationExecutivePortKey.h>
#include <vtkInformationExecutivePortVectorKey.h>
#include <vtkInformationIntegerRequestKey.h>
#include <vtkMoleculeAlgorithm.h>
#include <vtkMultiBlockDataSetAlgorithm.h>
#include <vtkMultiTimeStepAlgorithm.h>
#include <vtkNonOverlappingAMRAlgorithm.h>
#include <vtkOverlappingAMRAlgorithm.h>
#include <vtkParallelReader.h>
#include <vtkPartitionedDataSetAlgorithm.h>
#include <vtkPartitionedDataSetCollectionAlgorithm.h>
#include <vtkPassInputTypeAlgorithm.h>
#include <vtkPiecewiseFunctionAlgorithm.h>
#include <vtkPiecewiseFunctionShiftScale.h>
#include <vtkPointSetAlgorithm.h>
#include <vtkPolyDataAlgorithm.h>
#include <vtkProgressObserver.h>
#include <vtkReaderAlgorithm.h>
#include <vtkRectilinearGridAlgorithm.h>
#include <vtkSMPProgressObserver.h>
#include <vtkScalarTree.h>
#include <vtkSelectionAlgorithm.h>
#include <vtkSimpleImageToImageFilter.h>
#include <vtkSimpleReader.h>
#include <vtkSimpleScalarTree.h>
#include <vtkSpanSpace.h>
#include <vtkSphereTree.h>
#include <vtkStreamingDemandDrivenPipeline.h>
#include <vtkStructuredGridAlgorithm.h>
#include <vtkTableAlgorithm.h>
#include <vtkTemporalAlgorithm.h>
#include <vtkThreadedCompositeDataPipeline.h>
#include <vtkThreadedImageAlgorithm.h>
#include <vtkTimeRange.h>
#include <vtkTreeAlgorithm.h>
#include <vtkTrivialConsumer.h>
#include <vtkTrivialProducer.h>
#include <vtkUndirectedGraphAlgorithm.h>
#include <vtkUniformGridAMRAlgorithm.h>
#include <vtkUniformGridPartitioner.h>
#include <vtkUnstructuredGridAlgorithm.h>
#include <vtkUnstructuredGridBaseAlgorithm.h>

// Implement declared functions
extern "C" vtkNew<vtkAggregateToPartitionedDataSetCollection> vtkAggregateToPartitionedDataSetCollection_new() {
    return vtkNew<vtkAggregateToPartitionedDataSetCollection>();
}
extern "C" void vtkAggregateToPartitionedDataSetCollection_destructor(
    vtkNew<vtkAggregateToPartitionedDataSetCollection> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkAggregateToPartitionedDataSetCollection_get_ptr(
    vtkNew<vtkAggregateToPartitionedDataSetCollection> sself
) {
    return sself.GetPointer();
}
extern "C" void vtk_aggregate_to_partitioned_data_set_collection_clear(
    vtkNew<vtkAggregateToPartitionedDataSetCollection> self
) {
    return self->Clear();
}
extern "C" vtkNew<vtkAlgorithm> vtkAlgorithm_new() {
    return vtkNew<vtkAlgorithm>();
}
extern "C" void vtkAlgorithm_destructor(vtkNew<vtkAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAlgorithm_get_ptr(vtkNew<vtkAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_algorithm_has_executive(vtkNew<vtkAlgorithm> self) {
    return self->HasExecutive();
}
extern "C" int vtk_algorithm_get_number_of_input_ports(vtkNew<vtkAlgorithm> self) {
    return self->GetNumberOfInputPorts();
}
extern "C" int vtk_algorithm_get_number_of_output_ports(vtkNew<vtkAlgorithm> self) {
    return self->GetNumberOfOutputPorts();
}
extern "C" bool vtk_algorithm_uses_garbage_collector(vtkNew<vtkAlgorithm> self) {
    return self->UsesGarbageCollector();
}
extern "C" void vtk_algorithm_set_abort_execute_and_update_time(vtkNew<vtkAlgorithm> self) {
    return self->SetAbortExecuteAndUpdateTime();
}
extern "C" void vtk_algorithm_set_abort_execute(vtkNew<vtkAlgorithm> self, int _arg) {
    return self->SetAbortExecute(_arg);
}
extern "C" int vtk_algorithm_get_abort_execute(vtkNew<vtkAlgorithm> self) {
    return self->GetAbortExecute();
}
extern "C" void vtk_algorithm_abort_execute_on(vtkNew<vtkAlgorithm> self) {
    return self->AbortExecuteOn();
}
extern "C" void vtk_algorithm_abort_execute_off(vtkNew<vtkAlgorithm> self) {
    return self->AbortExecuteOff();
}
extern "C" double vtk_algorithm_get_progress(vtkNew<vtkAlgorithm> self) {
    return self->GetProgress();
}
extern "C" void vtk_algorithm_update_progress(vtkNew<vtkAlgorithm> self, double amount) {
    return self->UpdateProgress(amount);
}
extern "C" bool vtk_algorithm_check_abort(vtkNew<vtkAlgorithm> self) {
    return self->CheckAbort();
}
extern "C" void vtk_algorithm_set_abort_output(vtkNew<vtkAlgorithm> self, bool _arg) {
    return self->SetAbortOutput(_arg);
}
extern "C" bool vtk_algorithm_get_abort_output(vtkNew<vtkAlgorithm> self) {
    return self->GetAbortOutput();
}
extern "C" void vtk_algorithm_set_progress_shift_scale(
    vtkNew<vtkAlgorithm> self, double shift, double scale
) {
    return self->SetProgressShiftScale(shift, scale);
}
extern "C" double vtk_algorithm_get_progress_shift(vtkNew<vtkAlgorithm> self) {
    return self->GetProgressShift();
}
extern "C" double vtk_algorithm_get_progress_scale(vtkNew<vtkAlgorithm> self) {
    return self->GetProgressScale();
}
extern "C" unsigned long vtk_algorithm_get_error_code(vtkNew<vtkAlgorithm> self) {
    return self->GetErrorCode();
}
extern "C" void vtk_algorithm_set_input_array_to_process_c_int_c_int_c_int_c_int_c_int(
    vtkNew<vtkAlgorithm> self,
    int idx,
    int port,
    int connection,
    int fieldAssociation,
    int fieldAttributeType
) {
    return self->SetInputArrayToProcess(
        idx, port, connection, fieldAssociation, fieldAttributeType
    );
}
extern "C" void vtk_algorithm_set_input_array_to_process_c_int_c_int_c_int_c_int_c_int_c_int(
    vtkNew<vtkAlgorithm> self,
    int idx,
    int port,
    int connection,
    int fieldAssociation,
    int fieldAttributeType,
    int component
) {
    return self->SetInputArrayToProcess(
        idx, port, connection, fieldAssociation, fieldAttributeType, component
    );
}
extern "C" int vtk_algorithm_get_number_of_input_array_specifications(vtkNew<vtkAlgorithm> self) {
    return self->GetNumberOfInputArraySpecifications();
}
extern "C" bool vtk_algorithm_reset_input_array_specifications(vtkNew<vtkAlgorithm> self) {
    return self->ResetInputArraySpecifications();
}
extern "C" void vtk_algorithm_remove_all_inputs(vtkNew<vtkAlgorithm> self) {
    return self->RemoveAllInputs();
}
extern "C" void vtk_algorithm_remove_input_connection_c_int_c_int(
    vtkNew<vtkAlgorithm> self, int port, int idx
) {
    return self->RemoveInputConnection(port, idx);
}
extern "C" void vtk_algorithm_remove_all_input_connections(vtkNew<vtkAlgorithm> self, int port) {
    return self->RemoveAllInputConnections(port);
}
extern "C" int vtk_algorithm_get_number_of_input_connections(vtkNew<vtkAlgorithm> self, int port) {
    return self->GetNumberOfInputConnections(port);
}
extern "C" int vtk_algorithm_get_total_number_of_input_connections(vtkNew<vtkAlgorithm> self) {
    return self->GetTotalNumberOfInputConnections();
}
extern "C" void vtk_algorithm_update_c_int(vtkNew<vtkAlgorithm> self, int port) {
    return self->Update(port);
}
extern "C" void vtk_algorithm_update(vtkNew<vtkAlgorithm> self) {
    return self->Update();
}
extern "C" int vtk_algorithm_update_piece(
    vtkNew<vtkAlgorithm> self, int piece, int numPieces, int ghostLevels, const int* extents
) {
    return self->UpdatePiece(piece, numPieces, ghostLevels, extents);
}
extern "C" int vtk_algorithm_update_extent(vtkNew<vtkAlgorithm> self, const int* extents) {
    return self->UpdateExtent(extents);
}
extern "C" int vtk_algorithm_update_time_step(
    vtkNew<vtkAlgorithm> self,
    double time,
    int piece,
    int numPieces,
    int ghostLevels,
    const int* extents
) {
    return self->UpdateTimeStep(time, piece, numPieces, ghostLevels, extents);
}
extern "C" void vtk_algorithm_update_information(vtkNew<vtkAlgorithm> self) {
    return self->UpdateInformation();
}
extern "C" void vtk_algorithm_update_data_object(vtkNew<vtkAlgorithm> self) {
    return self->UpdateDataObject();
}
extern "C" void vtk_algorithm_propagate_update_extent(vtkNew<vtkAlgorithm> self) {
    return self->PropagateUpdateExtent();
}
extern "C" void vtk_algorithm_update_whole_extent(vtkNew<vtkAlgorithm> self) {
    return self->UpdateWholeExtent();
}
extern "C" void vtk_algorithm_set_release_data_flag(vtkNew<vtkAlgorithm> self, int p0) {
    return self->SetReleaseDataFlag(p0);
}
extern "C" int vtk_algorithm_get_release_data_flag(vtkNew<vtkAlgorithm> self) {
    return self->GetReleaseDataFlag();
}
extern "C" void vtk_algorithm_release_data_flag_on(vtkNew<vtkAlgorithm> self) {
    return self->ReleaseDataFlagOn();
}
extern "C" void vtk_algorithm_release_data_flag_off(vtkNew<vtkAlgorithm> self) {
    return self->ReleaseDataFlagOff();
}
extern "C" void vtk_algorithm_get_update_extent_c_int_6(vtkNew<vtkAlgorithm> self, int* extent) {
    return self->GetUpdateExtent(extent);
}
extern "C" void vtk_algorithm_get_update_extent_c_int_c_int_6(
    vtkNew<vtkAlgorithm> self, int port, int* extent
) {
    return self->GetUpdateExtent(port, extent);
}
extern "C" int vtk_algorithm_get_update_piece(vtkNew<vtkAlgorithm> self) {
    return self->GetUpdatePiece();
}
extern "C" int vtk_algorithm_get_update_piece_c_int(vtkNew<vtkAlgorithm> self, int port) {
    return self->GetUpdatePiece(port);
}
extern "C" int vtk_algorithm_get_update_number_of_pieces(vtkNew<vtkAlgorithm> self) {
    return self->GetUpdateNumberOfPieces();
}
extern "C" int vtk_algorithm_get_update_number_of_pieces_c_int(
    vtkNew<vtkAlgorithm> self, int port
) {
    return self->GetUpdateNumberOfPieces(port);
}
extern "C" int vtk_algorithm_get_update_ghost_level(vtkNew<vtkAlgorithm> self) {
    return self->GetUpdateGhostLevel();
}
extern "C" int vtk_algorithm_get_update_ghost_level_c_int(vtkNew<vtkAlgorithm> self, int port) {
    return self->GetUpdateGhostLevel(port);
}
extern "C" void vtk_algorithm_set_no_prior_temporal_access_information_key_c_int(
    vtkNew<vtkAlgorithm> self, int key
) {
    return self->SetNoPriorTemporalAccessInformationKey(key);
}
extern "C" void vtk_algorithm_set_no_prior_temporal_access_information_key(
    vtkNew<vtkAlgorithm> self
) {
    return self->SetNoPriorTemporalAccessInformationKey();
}
extern "C" void vtk_algorithm_remove_no_prior_temporal_access_information_key(
    vtkNew<vtkAlgorithm> self
) {
    return self->RemoveNoPriorTemporalAccessInformationKey();
}
extern "C" vtkNew<vtkAlgorithmOutput> vtkAlgorithmOutput_new() {
    return vtkNew<vtkAlgorithmOutput>();
}
extern "C" void vtkAlgorithmOutput_destructor(vtkNew<vtkAlgorithmOutput> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAlgorithmOutput_get_ptr(vtkNew<vtkAlgorithmOutput> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_algorithm_output_set_index(vtkNew<vtkAlgorithmOutput> self, int index) {
    return self->SetIndex(index);
}
extern "C" int vtk_algorithm_output_get_index(vtkNew<vtkAlgorithmOutput> self) {
    return self->GetIndex();
}
extern "C" vtkNew<vtkAnnotationLayersAlgorithm> vtkAnnotationLayersAlgorithm_new() {
    return vtkNew<vtkAnnotationLayersAlgorithm>();
}
extern "C" void vtkAnnotationLayersAlgorithm_destructor(
    vtkNew<vtkAnnotationLayersAlgorithm> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkAnnotationLayersAlgorithm_get_ptr(vtkNew<vtkAnnotationLayersAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkArrayDataAlgorithm> vtkArrayDataAlgorithm_new() {
    return vtkNew<vtkArrayDataAlgorithm>();
}
extern "C" void vtkArrayDataAlgorithm_destructor(vtkNew<vtkArrayDataAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkArrayDataAlgorithm_get_ptr(vtkNew<vtkArrayDataAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkCachedStreamingDemandDrivenPipeline> vtkCachedStreamingDemandDrivenPipeline_new() {
    return vtkNew<vtkCachedStreamingDemandDrivenPipeline>();
}
extern "C" void vtkCachedStreamingDemandDrivenPipeline_destructor(
    vtkNew<vtkCachedStreamingDemandDrivenPipeline> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkCachedStreamingDemandDrivenPipeline_get_ptr(
    vtkNew<vtkCachedStreamingDemandDrivenPipeline> sself
) {
    return sself.GetPointer();
}
extern "C" void vtk_cached_streaming_demand_driven_pipeline_set_cache_size(
    vtkNew<vtkCachedStreamingDemandDrivenPipeline> self, int size
) {
    return self->SetCacheSize(size);
}
extern "C" int vtk_cached_streaming_demand_driven_pipeline_get_cache_size(
    vtkNew<vtkCachedStreamingDemandDrivenPipeline> self
) {
    return self->GetCacheSize();
}
extern "C" vtkNew<vtkCastToConcrete> vtkCastToConcrete_new() {
    return vtkNew<vtkCastToConcrete>();
}
extern "C" void vtkCastToConcrete_destructor(vtkNew<vtkCastToConcrete> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCastToConcrete_get_ptr(vtkNew<vtkCastToConcrete> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkCellGridAlgorithm> vtkCellGridAlgorithm_new() {
    return vtkNew<vtkCellGridAlgorithm>();
}
extern "C" void vtkCellGridAlgorithm_destructor(vtkNew<vtkCellGridAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCellGridAlgorithm_get_ptr(vtkNew<vtkCellGridAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkCompositeDataPipeline> vtkCompositeDataPipeline_new() {
    return vtkNew<vtkCompositeDataPipeline>();
}
extern "C" void vtkCompositeDataPipeline_destructor(vtkNew<vtkCompositeDataPipeline> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeDataPipeline_get_ptr(vtkNew<vtkCompositeDataPipeline> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkCompositeDataSetAlgorithm> vtkCompositeDataSetAlgorithm_new() {
    return vtkNew<vtkCompositeDataSetAlgorithm>();
}
extern "C" void vtkCompositeDataSetAlgorithm_destructor(
    vtkNew<vtkCompositeDataSetAlgorithm> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeDataSetAlgorithm_get_ptr(vtkNew<vtkCompositeDataSetAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkDataObjectAlgorithm> vtkDataObjectAlgorithm_new() {
    return vtkNew<vtkDataObjectAlgorithm>();
}
extern "C" void vtkDataObjectAlgorithm_destructor(vtkNew<vtkDataObjectAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDataObjectAlgorithm_get_ptr(vtkNew<vtkDataObjectAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkDataSetAlgorithm> vtkDataSetAlgorithm_new() {
    return vtkNew<vtkDataSetAlgorithm>();
}
extern "C" void vtkDataSetAlgorithm_destructor(vtkNew<vtkDataSetAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDataSetAlgorithm_get_ptr(vtkNew<vtkDataSetAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkDemandDrivenPipeline> vtkDemandDrivenPipeline_new() {
    return vtkNew<vtkDemandDrivenPipeline>();
}
extern "C" void vtkDemandDrivenPipeline_destructor(vtkNew<vtkDemandDrivenPipeline> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDemandDrivenPipeline_get_ptr(vtkNew<vtkDemandDrivenPipeline> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long vtk_demand_driven_pipeline_get_pipeline_m_time(
    vtkNew<vtkDemandDrivenPipeline> self
) {
    return self->GetPipelineMTime();
}
extern "C" int vtk_demand_driven_pipeline_set_release_data_flag(
    vtkNew<vtkDemandDrivenPipeline> self, int port, int n
) {
    return self->SetReleaseDataFlag(port, n);
}
extern "C" int vtk_demand_driven_pipeline_get_release_data_flag(
    vtkNew<vtkDemandDrivenPipeline> self, int port
) {
    return self->GetReleaseDataFlag(port);
}
extern "C" int vtk_demand_driven_pipeline_update_pipeline_m_time(
    vtkNew<vtkDemandDrivenPipeline> self
) {
    return self->UpdatePipelineMTime();
}
extern "C" int vtk_demand_driven_pipeline_update_data_object(vtkNew<vtkDemandDrivenPipeline> self) {
    return self->UpdateDataObject();
}
extern "C" int vtk_demand_driven_pipeline_update_data(
    vtkNew<vtkDemandDrivenPipeline> self, int outputPort
) {
    return self->UpdateData(outputPort);
}
extern "C" vtkNew<vtkDirectedGraphAlgorithm> vtkDirectedGraphAlgorithm_new() {
    return vtkNew<vtkDirectedGraphAlgorithm>();
}
extern "C" void vtkDirectedGraphAlgorithm_destructor(vtkNew<vtkDirectedGraphAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDirectedGraphAlgorithm_get_ptr(vtkNew<vtkDirectedGraphAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkEndFor> vtkEndFor_new() {
    return vtkNew<vtkEndFor>();
}
extern "C" void vtkEndFor_destructor(vtkNew<vtkEndFor> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkEndFor_get_ptr(vtkNew<vtkEndFor> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkEnsembleSource> vtkEnsembleSource_new() {
    return vtkNew<vtkEnsembleSource>();
}
extern "C" void vtkEnsembleSource_destructor(vtkNew<vtkEnsembleSource> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkEnsembleSource_get_ptr(vtkNew<vtkEnsembleSource> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_ensemble_source_remove_all_members(vtkNew<vtkEnsembleSource> self) {
    return self->RemoveAllMembers();
}
extern "C" unsigned int vtk_ensemble_source_get_number_of_members(vtkNew<vtkEnsembleSource> self) {
    return self->GetNumberOfMembers();
}
extern "C" void vtk_ensemble_source_set_current_member(
    vtkNew<vtkEnsembleSource> self, unsigned int _arg
) {
    return self->SetCurrentMember(_arg);
}
extern "C" unsigned int vtk_ensemble_source_get_current_member(vtkNew<vtkEnsembleSource> self) {
    return self->GetCurrentMember();
}
extern "C" vtkNew<vtkExplicitStructuredGridAlgorithm> vtkExplicitStructuredGridAlgorithm_new() {
    return vtkNew<vtkExplicitStructuredGridAlgorithm>();
}
extern "C" void vtkExplicitStructuredGridAlgorithm_destructor(
    vtkNew<vtkExplicitStructuredGridAlgorithm> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkExplicitStructuredGridAlgorithm_get_ptr(
    vtkNew<vtkExplicitStructuredGridAlgorithm> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkExtentRCBPartitioner> vtkExtentRCBPartitioner_new() {
    return vtkNew<vtkExtentRCBPartitioner>();
}
extern "C" void vtkExtentRCBPartitioner_destructor(vtkNew<vtkExtentRCBPartitioner> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkExtentRCBPartitioner_get_ptr(vtkNew<vtkExtentRCBPartitioner> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_extent_rcb_partitioner_set_number_of_partitions(
    vtkNew<vtkExtentRCBPartitioner> self, int N
) {
    return self->SetNumberOfPartitions(N);
}
extern "C" void vtk_extent_rcb_partitioner_set_global_extent_c_int_c_int_c_int_c_int_c_int_c_int(
    vtkNew<vtkExtentRCBPartitioner> self, int imin, int imax, int jmin, int jmax, int kmin, int kmax
) {
    return self->SetGlobalExtent(imin, imax, jmin, jmax, kmin, kmax);
}
extern "C" void vtk_extent_rcb_partitioner_set_global_extent_c_int_6(
    vtkNew<vtkExtentRCBPartitioner> self, int* ext
) {
    return self->SetGlobalExtent(ext);
}
extern "C" void vtk_extent_rcb_partitioner_set_duplicate_nodes(
    vtkNew<vtkExtentRCBPartitioner> self, int _arg
) {
    return self->SetDuplicateNodes(_arg);
}
extern "C" int vtk_extent_rcb_partitioner_get_duplicate_nodes(
    vtkNew<vtkExtentRCBPartitioner> self
) {
    return self->GetDuplicateNodes();
}
extern "C" void vtk_extent_rcb_partitioner_duplicate_nodes_on(
    vtkNew<vtkExtentRCBPartitioner> self
) {
    return self->DuplicateNodesOn();
}
extern "C" void vtk_extent_rcb_partitioner_duplicate_nodes_off(
    vtkNew<vtkExtentRCBPartitioner> self
) {
    return self->DuplicateNodesOff();
}
extern "C" void vtk_extent_rcb_partitioner_set_number_of_ghost_layers(
    vtkNew<vtkExtentRCBPartitioner> self, int _arg
) {
    return self->SetNumberOfGhostLayers(_arg);
}
extern "C" int vtk_extent_rcb_partitioner_get_number_of_ghost_layers(
    vtkNew<vtkExtentRCBPartitioner> self
) {
    return self->GetNumberOfGhostLayers();
}
extern "C" int vtk_extent_rcb_partitioner_get_num_extents(vtkNew<vtkExtentRCBPartitioner> self) {
    return self->GetNumExtents();
}
extern "C" void vtk_extent_rcb_partitioner_partition(vtkNew<vtkExtentRCBPartitioner> self) {
    return self->Partition();
}
extern "C" void vtk_extent_rcb_partitioner_get_partition_extent(
    vtkNew<vtkExtentRCBPartitioner> self, int idx, int* ext
) {
    return self->GetPartitionExtent(idx, ext);
}
extern "C" vtkNew<vtkExtentSplitter> vtkExtentSplitter_new() {
    return vtkNew<vtkExtentSplitter>();
}
extern "C" void vtkExtentSplitter_destructor(vtkNew<vtkExtentSplitter> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkExtentSplitter_get_ptr(vtkNew<vtkExtentSplitter> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_extent_splitter_add_extent_source_c_int_c_int_c_int_c_int_c_int_c_int_c_int_c_int(
    vtkNew<vtkExtentSplitter> self,
    int id,
    int priority,
    int x0,
    int x1,
    int y0,
    int y1,
    int z0,
    int z1
) {
    return self->AddExtentSource(id, priority, x0, x1, y0, y1, z0, z1);
}
extern "C" void vtk_extent_splitter_remove_extent_source(vtkNew<vtkExtentSplitter> self, int id) {
    return self->RemoveExtentSource(id);
}
extern "C" void vtk_extent_splitter_remove_all_extent_sources(vtkNew<vtkExtentSplitter> self) {
    return self->RemoveAllExtentSources();
}
extern "C" void vtk_extent_splitter_add_extent_c_int_c_int_c_int_c_int_c_int_c_int(
    vtkNew<vtkExtentSplitter> self, int x0, int x1, int y0, int y1, int z0, int z1
) {
    return self->AddExtent(x0, x1, y0, y1, z0, z1);
}
extern "C" int vtk_extent_splitter_compute_sub_extents(vtkNew<vtkExtentSplitter> self) {
    return self->ComputeSubExtents();
}
extern "C" int vtk_extent_splitter_get_number_of_sub_extents(vtkNew<vtkExtentSplitter> self) {
    return self->GetNumberOfSubExtents();
}
extern "C" int vtk_extent_splitter_get_sub_extent_source(
    vtkNew<vtkExtentSplitter> self, int index
) {
    return self->GetSubExtentSource(index);
}
extern "C" int vtk_extent_splitter_get_point_mode(vtkNew<vtkExtentSplitter> self) {
    return self->GetPointMode();
}
extern "C" void vtk_extent_splitter_set_point_mode(vtkNew<vtkExtentSplitter> self, int _arg) {
    return self->SetPointMode(_arg);
}
extern "C" void vtk_extent_splitter_point_mode_on(vtkNew<vtkExtentSplitter> self) {
    return self->PointModeOn();
}
extern "C" void vtk_extent_splitter_point_mode_off(vtkNew<vtkExtentSplitter> self) {
    return self->PointModeOff();
}
extern "C" vtkNew<vtkExtentTranslator> vtkExtentTranslator_new() {
    return vtkNew<vtkExtentTranslator>();
}
extern "C" void vtkExtentTranslator_destructor(vtkNew<vtkExtentTranslator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkExtentTranslator_get_ptr(vtkNew<vtkExtentTranslator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_extent_translator_set_whole_extent_c_int_c_int_c_int_c_int_c_int_c_int(
    vtkNew<vtkExtentTranslator> self,
    int _arg1,
    int _arg2,
    int _arg3,
    int _arg4,
    int _arg5,
    int _arg6
) {
    return self->SetWholeExtent(_arg1, _arg2, _arg3, _arg4, _arg5, _arg6);
}
extern "C" void vtk_extent_translator_set_whole_extent_const_c_int_6(
    vtkNew<vtkExtentTranslator> self, const int* _arg
) {
    return self->SetWholeExtent(_arg);
}
extern "C" void vtk_extent_translator_get_whole_extent_c_int_6(
    vtkNew<vtkExtentTranslator> self, int* _arg
) {
    return self->GetWholeExtent(_arg);
}
extern "C" void vtk_extent_translator_set_extent_c_int_c_int_c_int_c_int_c_int_c_int(
    vtkNew<vtkExtentTranslator> self,
    int _arg1,
    int _arg2,
    int _arg3,
    int _arg4,
    int _arg5,
    int _arg6
) {
    return self->SetExtent(_arg1, _arg2, _arg3, _arg4, _arg5, _arg6);
}
extern "C" void vtk_extent_translator_set_extent_const_c_int_6(
    vtkNew<vtkExtentTranslator> self, const int* _arg
) {
    return self->SetExtent(_arg);
}
extern "C" void vtk_extent_translator_get_extent_c_int_6(
    vtkNew<vtkExtentTranslator> self, int* _arg
) {
    return self->GetExtent(_arg);
}
extern "C" void vtk_extent_translator_set_piece(vtkNew<vtkExtentTranslator> self, int _arg) {
    return self->SetPiece(_arg);
}
extern "C" int vtk_extent_translator_get_piece(vtkNew<vtkExtentTranslator> self) {
    return self->GetPiece();
}
extern "C" void vtk_extent_translator_set_number_of_pieces(
    vtkNew<vtkExtentTranslator> self, int _arg
) {
    return self->SetNumberOfPieces(_arg);
}
extern "C" int vtk_extent_translator_get_number_of_pieces(vtkNew<vtkExtentTranslator> self) {
    return self->GetNumberOfPieces();
}
extern "C" void vtk_extent_translator_set_ghost_level(vtkNew<vtkExtentTranslator> self, int _arg) {
    return self->SetGhostLevel(_arg);
}
extern "C" int vtk_extent_translator_get_ghost_level(vtkNew<vtkExtentTranslator> self) {
    return self->GetGhostLevel();
}
extern "C" int vtk_extent_translator_piece_to_extent(vtkNew<vtkExtentTranslator> self) {
    return self->PieceToExtent();
}
extern "C" int vtk_extent_translator_piece_to_extent_by_points(vtkNew<vtkExtentTranslator> self) {
    return self->PieceToExtentByPoints();
}
extern "C" void vtk_extent_translator_set_split_mode_to_block(vtkNew<vtkExtentTranslator> self) {
    return self->SetSplitModeToBlock();
}
extern "C" void vtk_extent_translator_set_split_mode_to_x_slab(vtkNew<vtkExtentTranslator> self) {
    return self->SetSplitModeToXSlab();
}
extern "C" void vtk_extent_translator_set_split_mode_to_y_slab(vtkNew<vtkExtentTranslator> self) {
    return self->SetSplitModeToYSlab();
}
extern "C" void vtk_extent_translator_set_split_mode_to_z_slab(vtkNew<vtkExtentTranslator> self) {
    return self->SetSplitModeToZSlab();
}
extern "C" int vtk_extent_translator_get_split_mode(vtkNew<vtkExtentTranslator> self) {
    return self->GetSplitMode();
}
extern "C" vtkNew<vtkForEach> vtkForEach_new() {
    return vtkNew<vtkForEach>();
}
extern "C" void vtkForEach_destructor(vtkNew<vtkForEach> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkForEach_get_ptr(vtkNew<vtkForEach> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_for_each_is_iterating(vtkNew<vtkForEach> self) {
    return self->IsIterating();
}
extern "C" void vtk_for_each_iter(vtkNew<vtkForEach> self) {
    return self->Iter();
}
extern "C" vtkNew<vtkGraphAlgorithm> vtkGraphAlgorithm_new() {
    return vtkNew<vtkGraphAlgorithm>();
}
extern "C" void vtkGraphAlgorithm_destructor(vtkNew<vtkGraphAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkGraphAlgorithm_get_ptr(vtkNew<vtkGraphAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkHierarchicalBoxDataSetAlgorithm> vtkHierarchicalBoxDataSetAlgorithm_new() {
    return vtkNew<vtkHierarchicalBoxDataSetAlgorithm>();
}
extern "C" void vtkHierarchicalBoxDataSetAlgorithm_destructor(
    vtkNew<vtkHierarchicalBoxDataSetAlgorithm> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkHierarchicalBoxDataSetAlgorithm_get_ptr(
    vtkNew<vtkHierarchicalBoxDataSetAlgorithm> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkImageToStructuredGrid> vtkImageToStructuredGrid_new() {
    return vtkNew<vtkImageToStructuredGrid>();
}
extern "C" void vtkImageToStructuredGrid_destructor(vtkNew<vtkImageToStructuredGrid> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkImageToStructuredGrid_get_ptr(vtkNew<vtkImageToStructuredGrid> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkImageToStructuredPoints> vtkImageToStructuredPoints_new() {
    return vtkNew<vtkImageToStructuredPoints>();
}
extern "C" void vtkImageToStructuredPoints_destructor(vtkNew<vtkImageToStructuredPoints> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkImageToStructuredPoints_get_ptr(vtkNew<vtkImageToStructuredPoints> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkMoleculeAlgorithm> vtkMoleculeAlgorithm_new() {
    return vtkNew<vtkMoleculeAlgorithm>();
}
extern "C" void vtkMoleculeAlgorithm_destructor(vtkNew<vtkMoleculeAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkMoleculeAlgorithm_get_ptr(vtkNew<vtkMoleculeAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkMultiBlockDataSetAlgorithm> vtkMultiBlockDataSetAlgorithm_new() {
    return vtkNew<vtkMultiBlockDataSetAlgorithm>();
}
extern "C" void vtkMultiBlockDataSetAlgorithm_destructor(
    vtkNew<vtkMultiBlockDataSetAlgorithm> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkMultiBlockDataSetAlgorithm_get_ptr(
    vtkNew<vtkMultiBlockDataSetAlgorithm> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkMultiTimeStepAlgorithm> vtkMultiTimeStepAlgorithm_new() {
    return vtkNew<vtkMultiTimeStepAlgorithm>();
}
extern "C" void vtkMultiTimeStepAlgorithm_destructor(vtkNew<vtkMultiTimeStepAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkMultiTimeStepAlgorithm_get_ptr(vtkNew<vtkMultiTimeStepAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkNonOverlappingAMRAlgorithm> vtkNonOverlappingAMRAlgorithm_new() {
    return vtkNew<vtkNonOverlappingAMRAlgorithm>();
}
extern "C" void vtkNonOverlappingAMRAlgorithm_destructor(
    vtkNew<vtkNonOverlappingAMRAlgorithm> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkNonOverlappingAMRAlgorithm_get_ptr(
    vtkNew<vtkNonOverlappingAMRAlgorithm> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkOverlappingAMRAlgorithm> vtkOverlappingAMRAlgorithm_new() {
    return vtkNew<vtkOverlappingAMRAlgorithm>();
}
extern "C" void vtkOverlappingAMRAlgorithm_destructor(vtkNew<vtkOverlappingAMRAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkOverlappingAMRAlgorithm_get_ptr(vtkNew<vtkOverlappingAMRAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkPassInputTypeAlgorithm> vtkPassInputTypeAlgorithm_new() {
    return vtkNew<vtkPassInputTypeAlgorithm>();
}
extern "C" void vtkPassInputTypeAlgorithm_destructor(vtkNew<vtkPassInputTypeAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPassInputTypeAlgorithm_get_ptr(vtkNew<vtkPassInputTypeAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkPiecewiseFunctionAlgorithm> vtkPiecewiseFunctionAlgorithm_new() {
    return vtkNew<vtkPiecewiseFunctionAlgorithm>();
}
extern "C" void vtkPiecewiseFunctionAlgorithm_destructor(
    vtkNew<vtkPiecewiseFunctionAlgorithm> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkPiecewiseFunctionAlgorithm_get_ptr(
    vtkNew<vtkPiecewiseFunctionAlgorithm> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkPiecewiseFunctionShiftScale> vtkPiecewiseFunctionShiftScale_new() {
    return vtkNew<vtkPiecewiseFunctionShiftScale>();
}
extern "C" void vtkPiecewiseFunctionShiftScale_destructor(
    vtkNew<vtkPiecewiseFunctionShiftScale> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkPiecewiseFunctionShiftScale_get_ptr(
    vtkNew<vtkPiecewiseFunctionShiftScale> sself
) {
    return sself.GetPointer();
}
extern "C" void vtk_piecewise_function_shift_scale_set_position_shift(
    vtkNew<vtkPiecewiseFunctionShiftScale> self, double _arg
) {
    return self->SetPositionShift(_arg);
}
extern "C" void vtk_piecewise_function_shift_scale_set_position_scale(
    vtkNew<vtkPiecewiseFunctionShiftScale> self, double _arg
) {
    return self->SetPositionScale(_arg);
}
extern "C" void vtk_piecewise_function_shift_scale_set_value_shift(
    vtkNew<vtkPiecewiseFunctionShiftScale> self, double _arg
) {
    return self->SetValueShift(_arg);
}
extern "C" void vtk_piecewise_function_shift_scale_set_value_scale(
    vtkNew<vtkPiecewiseFunctionShiftScale> self, double _arg
) {
    return self->SetValueScale(_arg);
}
extern "C" double vtk_piecewise_function_shift_scale_get_position_shift(
    vtkNew<vtkPiecewiseFunctionShiftScale> self
) {
    return self->GetPositionShift();
}
extern "C" double vtk_piecewise_function_shift_scale_get_position_scale(
    vtkNew<vtkPiecewiseFunctionShiftScale> self
) {
    return self->GetPositionScale();
}
extern "C" double vtk_piecewise_function_shift_scale_get_value_shift(
    vtkNew<vtkPiecewiseFunctionShiftScale> self
) {
    return self->GetValueShift();
}
extern "C" double vtk_piecewise_function_shift_scale_get_value_scale(
    vtkNew<vtkPiecewiseFunctionShiftScale> self
) {
    return self->GetValueScale();
}
extern "C" vtkNew<vtkPointSetAlgorithm> vtkPointSetAlgorithm_new() {
    return vtkNew<vtkPointSetAlgorithm>();
}
extern "C" void vtkPointSetAlgorithm_destructor(vtkNew<vtkPointSetAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPointSetAlgorithm_get_ptr(vtkNew<vtkPointSetAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkPolyDataAlgorithm> vtkPolyDataAlgorithm_new() {
    return vtkNew<vtkPolyDataAlgorithm>();
}
extern "C" void vtkPolyDataAlgorithm_destructor(vtkNew<vtkPolyDataAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPolyDataAlgorithm_get_ptr(vtkNew<vtkPolyDataAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkProgressObserver> vtkProgressObserver_new() {
    return vtkNew<vtkProgressObserver>();
}
extern "C" void vtkProgressObserver_destructor(vtkNew<vtkProgressObserver> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkProgressObserver_get_ptr(vtkNew<vtkProgressObserver> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_progress_observer_update_progress(
    vtkNew<vtkProgressObserver> self, double amount
) {
    return self->UpdateProgress(amount);
}
extern "C" double vtk_progress_observer_get_progress(vtkNew<vtkProgressObserver> self) {
    return self->GetProgress();
}
extern "C" vtkNew<vtkRectilinearGridAlgorithm> vtkRectilinearGridAlgorithm_new() {
    return vtkNew<vtkRectilinearGridAlgorithm>();
}
extern "C" void vtkRectilinearGridAlgorithm_destructor(vtkNew<vtkRectilinearGridAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkRectilinearGridAlgorithm_get_ptr(vtkNew<vtkRectilinearGridAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkSMPProgressObserver> vtkSMPProgressObserver_new() {
    return vtkNew<vtkSMPProgressObserver>();
}
extern "C" void vtkSMPProgressObserver_destructor(vtkNew<vtkSMPProgressObserver> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSMPProgressObserver_get_ptr(vtkNew<vtkSMPProgressObserver> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_smp_progress_observer_update_progress(
    vtkNew<vtkSMPProgressObserver> self, double progress
) {
    return self->UpdateProgress(progress);
}
extern "C" vtkNew<vtkSelectionAlgorithm> vtkSelectionAlgorithm_new() {
    return vtkNew<vtkSelectionAlgorithm>();
}
extern "C" void vtkSelectionAlgorithm_destructor(vtkNew<vtkSelectionAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSelectionAlgorithm_get_ptr(vtkNew<vtkSelectionAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkSimpleScalarTree> vtkSimpleScalarTree_new() {
    return vtkNew<vtkSimpleScalarTree>();
}
extern "C" void vtkSimpleScalarTree_destructor(vtkNew<vtkSimpleScalarTree> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSimpleScalarTree_get_ptr(vtkNew<vtkSimpleScalarTree> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_simple_scalar_tree_set_branching_factor(
    vtkNew<vtkSimpleScalarTree> self, int _arg
) {
    return self->SetBranchingFactor(_arg);
}
extern "C" int vtk_simple_scalar_tree_get_branching_factor_min_value(
    vtkNew<vtkSimpleScalarTree> self
) {
    return self->GetBranchingFactorMinValue();
}
extern "C" int vtk_simple_scalar_tree_get_branching_factor_max_value(
    vtkNew<vtkSimpleScalarTree> self
) {
    return self->GetBranchingFactorMaxValue();
}
extern "C" int vtk_simple_scalar_tree_get_branching_factor(vtkNew<vtkSimpleScalarTree> self) {
    return self->GetBranchingFactor();
}
extern "C" int vtk_simple_scalar_tree_get_level(vtkNew<vtkSimpleScalarTree> self) {
    return self->GetLevel();
}
extern "C" void vtk_simple_scalar_tree_set_max_level(vtkNew<vtkSimpleScalarTree> self, int _arg) {
    return self->SetMaxLevel(_arg);
}
extern "C" int vtk_simple_scalar_tree_get_max_level_min_value(vtkNew<vtkSimpleScalarTree> self) {
    return self->GetMaxLevelMinValue();
}
extern "C" int vtk_simple_scalar_tree_get_max_level_max_value(vtkNew<vtkSimpleScalarTree> self) {
    return self->GetMaxLevelMaxValue();
}
extern "C" int vtk_simple_scalar_tree_get_max_level(vtkNew<vtkSimpleScalarTree> self) {
    return self->GetMaxLevel();
}
extern "C" void vtk_simple_scalar_tree_build_tree(vtkNew<vtkSimpleScalarTree> self) {
    return self->BuildTree();
}
extern "C" void vtk_simple_scalar_tree_initialize(vtkNew<vtkSimpleScalarTree> self) {
    return self->Initialize();
}
extern "C" void vtk_simple_scalar_tree_init_traversal(
    vtkNew<vtkSimpleScalarTree> self, double scalarValue
) {
    return self->InitTraversal(scalarValue);
}
extern "C" long long vtk_simple_scalar_tree_get_number_of_cell_batches(
    vtkNew<vtkSimpleScalarTree> self, double scalarValue
) {
    return self->GetNumberOfCellBatches(scalarValue);
}
extern "C" vtkNew<vtkSpanSpace> vtkSpanSpace_new() {
    return vtkNew<vtkSpanSpace>();
}
extern "C" void vtkSpanSpace_destructor(vtkNew<vtkSpanSpace> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSpanSpace_get_ptr(vtkNew<vtkSpanSpace> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_span_space_set_scalar_range_double_double(
    vtkNew<vtkSpanSpace> self, double _arg1, double _arg2
) {
    return self->SetScalarRange(_arg1, _arg2);
}
extern "C" void vtk_span_space_set_scalar_range_const_double_2(
    vtkNew<vtkSpanSpace> self, const double* _arg
) {
    return self->SetScalarRange(_arg);
}
extern "C" void vtk_span_space_get_scalar_range_double_2(vtkNew<vtkSpanSpace> self, double* data) {
    return self->GetScalarRange(data);
}
extern "C" void vtk_span_space_set_compute_scalar_range(vtkNew<vtkSpanSpace> self, int _arg) {
    return self->SetComputeScalarRange(_arg);
}
extern "C" int vtk_span_space_get_compute_scalar_range(vtkNew<vtkSpanSpace> self) {
    return self->GetComputeScalarRange();
}
extern "C" void vtk_span_space_compute_scalar_range_on(vtkNew<vtkSpanSpace> self) {
    return self->ComputeScalarRangeOn();
}
extern "C" void vtk_span_space_compute_scalar_range_off(vtkNew<vtkSpanSpace> self) {
    return self->ComputeScalarRangeOff();
}
extern "C" void vtk_span_space_set_resolution(vtkNew<vtkSpanSpace> self, long long _arg) {
    return self->SetResolution(_arg);
}
extern "C" long long vtk_span_space_get_resolution_min_value(vtkNew<vtkSpanSpace> self) {
    return self->GetResolutionMinValue();
}
extern "C" long long vtk_span_space_get_resolution_max_value(vtkNew<vtkSpanSpace> self) {
    return self->GetResolutionMaxValue();
}
extern "C" long long vtk_span_space_get_resolution(vtkNew<vtkSpanSpace> self) {
    return self->GetResolution();
}
extern "C" void vtk_span_space_set_compute_resolution(vtkNew<vtkSpanSpace> self, int _arg) {
    return self->SetComputeResolution(_arg);
}
extern "C" int vtk_span_space_get_compute_resolution(vtkNew<vtkSpanSpace> self) {
    return self->GetComputeResolution();
}
extern "C" void vtk_span_space_compute_resolution_on(vtkNew<vtkSpanSpace> self) {
    return self->ComputeResolutionOn();
}
extern "C" void vtk_span_space_compute_resolution_off(vtkNew<vtkSpanSpace> self) {
    return self->ComputeResolutionOff();
}
extern "C" void vtk_span_space_set_number_of_cells_per_bucket(vtkNew<vtkSpanSpace> self, int _arg) {
    return self->SetNumberOfCellsPerBucket(_arg);
}
extern "C" int vtk_span_space_get_number_of_cells_per_bucket_min_value(vtkNew<vtkSpanSpace> self) {
    return self->GetNumberOfCellsPerBucketMinValue();
}
extern "C" int vtk_span_space_get_number_of_cells_per_bucket_max_value(vtkNew<vtkSpanSpace> self) {
    return self->GetNumberOfCellsPerBucketMaxValue();
}
extern "C" int vtk_span_space_get_number_of_cells_per_bucket(vtkNew<vtkSpanSpace> self) {
    return self->GetNumberOfCellsPerBucket();
}
extern "C" void vtk_span_space_initialize(vtkNew<vtkSpanSpace> self) {
    return self->Initialize();
}
extern "C" void vtk_span_space_build_tree(vtkNew<vtkSpanSpace> self) {
    return self->BuildTree();
}
extern "C" void vtk_span_space_init_traversal(vtkNew<vtkSpanSpace> self, double scalarValue) {
    return self->InitTraversal(scalarValue);
}
extern "C" long long vtk_span_space_get_number_of_cell_batches(
    vtkNew<vtkSpanSpace> self, double scalarValue
) {
    return self->GetNumberOfCellBatches(scalarValue);
}
extern "C" void vtk_span_space_set_batch_size(vtkNew<vtkSpanSpace> self, long long _arg) {
    return self->SetBatchSize(_arg);
}
extern "C" long long vtk_span_space_get_batch_size_min_value(vtkNew<vtkSpanSpace> self) {
    return self->GetBatchSizeMinValue();
}
extern "C" long long vtk_span_space_get_batch_size_max_value(vtkNew<vtkSpanSpace> self) {
    return self->GetBatchSizeMaxValue();
}
extern "C" long long vtk_span_space_get_batch_size(vtkNew<vtkSpanSpace> self) {
    return self->GetBatchSize();
}
extern "C" vtkNew<vtkSphereTree> vtkSphereTree_new() {
    return vtkNew<vtkSphereTree>();
}
extern "C" void vtkSphereTree_destructor(vtkNew<vtkSphereTree> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSphereTree_get_ptr(vtkNew<vtkSphereTree> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_sphere_tree_build(vtkNew<vtkSphereTree> self) {
    return self->Build();
}
extern "C" void vtk_sphere_tree_set_build_hierarchy(vtkNew<vtkSphereTree> self, bool _arg) {
    return self->SetBuildHierarchy(_arg);
}
extern "C" bool vtk_sphere_tree_get_build_hierarchy(vtkNew<vtkSphereTree> self) {
    return self->GetBuildHierarchy();
}
extern "C" void vtk_sphere_tree_build_hierarchy_on(vtkNew<vtkSphereTree> self) {
    return self->BuildHierarchyOn();
}
extern "C" void vtk_sphere_tree_build_hierarchy_off(vtkNew<vtkSphereTree> self) {
    return self->BuildHierarchyOff();
}
extern "C" void vtk_sphere_tree_set_resolution(vtkNew<vtkSphereTree> self, int _arg) {
    return self->SetResolution(_arg);
}
extern "C" int vtk_sphere_tree_get_resolution_min_value(vtkNew<vtkSphereTree> self) {
    return self->GetResolutionMinValue();
}
extern "C" int vtk_sphere_tree_get_resolution_max_value(vtkNew<vtkSphereTree> self) {
    return self->GetResolutionMaxValue();
}
extern "C" int vtk_sphere_tree_get_resolution(vtkNew<vtkSphereTree> self) {
    return self->GetResolution();
}
extern "C" void vtk_sphere_tree_set_max_level(vtkNew<vtkSphereTree> self, int _arg) {
    return self->SetMaxLevel(_arg);
}
extern "C" int vtk_sphere_tree_get_max_level_min_value(vtkNew<vtkSphereTree> self) {
    return self->GetMaxLevelMinValue();
}
extern "C" int vtk_sphere_tree_get_max_level_max_value(vtkNew<vtkSphereTree> self) {
    return self->GetMaxLevelMaxValue();
}
extern "C" int vtk_sphere_tree_get_max_level(vtkNew<vtkSphereTree> self) {
    return self->GetMaxLevel();
}
extern "C" int vtk_sphere_tree_get_number_of_levels(vtkNew<vtkSphereTree> self) {
    return self->GetNumberOfLevels();
}
extern "C" bool vtk_sphere_tree_uses_garbage_collector(vtkNew<vtkSphereTree> self) {
    return self->UsesGarbageCollector();
}
extern "C" vtkNew<vtkStreamingDemandDrivenPipeline> vtkStreamingDemandDrivenPipeline_new() {
    return vtkNew<vtkStreamingDemandDrivenPipeline>();
}
extern "C" void vtkStreamingDemandDrivenPipeline_destructor(
    vtkNew<vtkStreamingDemandDrivenPipeline> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkStreamingDemandDrivenPipeline_get_ptr(
    vtkNew<vtkStreamingDemandDrivenPipeline> sself
) {
    return sself.GetPointer();
}
extern "C" int vtk_streaming_demand_driven_pipeline_update(
    vtkNew<vtkStreamingDemandDrivenPipeline> self
) {
    return self->Update();
}
extern "C" int vtk_streaming_demand_driven_pipeline_update_whole_extent(
    vtkNew<vtkStreamingDemandDrivenPipeline> self
) {
    return self->UpdateWholeExtent();
}
extern "C" int vtk_streaming_demand_driven_pipeline_propagate_update_extent(
    vtkNew<vtkStreamingDemandDrivenPipeline> self, int outputPort
) {
    return self->PropagateUpdateExtent(outputPort);
}
extern "C" int vtk_streaming_demand_driven_pipeline_propagate_time(
    vtkNew<vtkStreamingDemandDrivenPipeline> self, int outputPort
) {
    return self->PropagateTime(outputPort);
}
extern "C" int vtk_streaming_demand_driven_pipeline_update_time_dependent_information(
    vtkNew<vtkStreamingDemandDrivenPipeline> self, int outputPort
) {
    return self->UpdateTimeDependentInformation(outputPort);
}
extern "C" int vtk_streaming_demand_driven_pipeline_set_request_exact_extent(
    vtkNew<vtkStreamingDemandDrivenPipeline> self, int port, int flag
) {
    return self->SetRequestExactExtent(port, flag);
}
extern "C" int vtk_streaming_demand_driven_pipeline_get_request_exact_extent(
    vtkNew<vtkStreamingDemandDrivenPipeline> self, int port
) {
    return self->GetRequestExactExtent(port);
}
extern "C" vtkNew<vtkStructuredGridAlgorithm> vtkStructuredGridAlgorithm_new() {
    return vtkNew<vtkStructuredGridAlgorithm>();
}
extern "C" void vtkStructuredGridAlgorithm_destructor(vtkNew<vtkStructuredGridAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStructuredGridAlgorithm_get_ptr(vtkNew<vtkStructuredGridAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkTableAlgorithm> vtkTableAlgorithm_new() {
    return vtkNew<vtkTableAlgorithm>();
}
extern "C" void vtkTableAlgorithm_destructor(vtkNew<vtkTableAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTableAlgorithm_get_ptr(vtkNew<vtkTableAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkThreadedCompositeDataPipeline> vtkThreadedCompositeDataPipeline_new() {
    return vtkNew<vtkThreadedCompositeDataPipeline>();
}
extern "C" void vtkThreadedCompositeDataPipeline_destructor(
    vtkNew<vtkThreadedCompositeDataPipeline> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkThreadedCompositeDataPipeline_get_ptr(
    vtkNew<vtkThreadedCompositeDataPipeline> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkTimeRange> vtkTimeRange_new() {
    return vtkNew<vtkTimeRange>();
}
extern "C" void vtkTimeRange_destructor(vtkNew<vtkTimeRange> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTimeRange_get_ptr(vtkNew<vtkTimeRange> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkTreeAlgorithm> vtkTreeAlgorithm_new() {
    return vtkNew<vtkTreeAlgorithm>();
}
extern "C" void vtkTreeAlgorithm_destructor(vtkNew<vtkTreeAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTreeAlgorithm_get_ptr(vtkNew<vtkTreeAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkTrivialConsumer> vtkTrivialConsumer_new() {
    return vtkNew<vtkTrivialConsumer>();
}
extern "C" void vtkTrivialConsumer_destructor(vtkNew<vtkTrivialConsumer> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTrivialConsumer_get_ptr(vtkNew<vtkTrivialConsumer> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkTrivialProducer> vtkTrivialProducer_new() {
    return vtkNew<vtkTrivialProducer>();
}
extern "C" void vtkTrivialProducer_destructor(vtkNew<vtkTrivialProducer> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTrivialProducer_get_ptr(vtkNew<vtkTrivialProducer> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long vtk_trivial_producer_get_m_time(vtkNew<vtkTrivialProducer> self) {
    return self->GetMTime();
}
extern "C" void vtk_trivial_producer_set_whole_extent_c_int_c_int_c_int_c_int_c_int_c_int(
    vtkNew<vtkTrivialProducer> self,
    int _arg1,
    int _arg2,
    int _arg3,
    int _arg4,
    int _arg5,
    int _arg6
) {
    return self->SetWholeExtent(_arg1, _arg2, _arg3, _arg4, _arg5, _arg6);
}
extern "C" void vtk_trivial_producer_set_whole_extent_const_c_int_6(
    vtkNew<vtkTrivialProducer> self, const int* _arg
) {
    return self->SetWholeExtent(_arg);
}
extern "C" void vtk_trivial_producer_get_whole_extent_c_int_6(
    vtkNew<vtkTrivialProducer> self, int* _arg
) {
    return self->GetWholeExtent(_arg);
}
extern "C" vtkNew<vtkUndirectedGraphAlgorithm> vtkUndirectedGraphAlgorithm_new() {
    return vtkNew<vtkUndirectedGraphAlgorithm>();
}
extern "C" void vtkUndirectedGraphAlgorithm_destructor(vtkNew<vtkUndirectedGraphAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkUndirectedGraphAlgorithm_get_ptr(vtkNew<vtkUndirectedGraphAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkUniformGridAMRAlgorithm> vtkUniformGridAMRAlgorithm_new() {
    return vtkNew<vtkUniformGridAMRAlgorithm>();
}
extern "C" void vtkUniformGridAMRAlgorithm_destructor(vtkNew<vtkUniformGridAMRAlgorithm> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkUniformGridAMRAlgorithm_get_ptr(vtkNew<vtkUniformGridAMRAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkUniformGridPartitioner> vtkUniformGridPartitioner_new() {
    return vtkNew<vtkUniformGridPartitioner>();
}
extern "C" void vtkUniformGridPartitioner_destructor(vtkNew<vtkUniformGridPartitioner> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkUniformGridPartitioner_get_ptr(vtkNew<vtkUniformGridPartitioner> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_uniform_grid_partitioner_get_number_of_partitions(
    vtkNew<vtkUniformGridPartitioner> self
) {
    return self->GetNumberOfPartitions();
}
extern "C" void vtk_uniform_grid_partitioner_set_number_of_partitions(
    vtkNew<vtkUniformGridPartitioner> self, int _arg
) {
    return self->SetNumberOfPartitions(_arg);
}
extern "C" int vtk_uniform_grid_partitioner_get_number_of_ghost_layers(
    vtkNew<vtkUniformGridPartitioner> self
) {
    return self->GetNumberOfGhostLayers();
}
extern "C" void vtk_uniform_grid_partitioner_set_number_of_ghost_layers(
    vtkNew<vtkUniformGridPartitioner> self, int _arg
) {
    return self->SetNumberOfGhostLayers(_arg);
}
extern "C" int vtk_uniform_grid_partitioner_get_duplicate_nodes(
    vtkNew<vtkUniformGridPartitioner> self
) {
    return self->GetDuplicateNodes();
}
extern "C" void vtk_uniform_grid_partitioner_set_duplicate_nodes(
    vtkNew<vtkUniformGridPartitioner> self, int _arg
) {
    return self->SetDuplicateNodes(_arg);
}
extern "C" void vtk_uniform_grid_partitioner_duplicate_nodes_on(
    vtkNew<vtkUniformGridPartitioner> self
) {
    return self->DuplicateNodesOn();
}
extern "C" void vtk_uniform_grid_partitioner_duplicate_nodes_off(
    vtkNew<vtkUniformGridPartitioner> self
) {
    return self->DuplicateNodesOff();
}
extern "C" vtkNew<vtkUnstructuredGridAlgorithm> vtkUnstructuredGridAlgorithm_new() {
    return vtkNew<vtkUnstructuredGridAlgorithm>();
}
extern "C" void vtkUnstructuredGridAlgorithm_destructor(
    vtkNew<vtkUnstructuredGridAlgorithm> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkUnstructuredGridAlgorithm_get_ptr(vtkNew<vtkUnstructuredGridAlgorithm> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkUnstructuredGridBaseAlgorithm> vtkUnstructuredGridBaseAlgorithm_new() {
    return vtkNew<vtkUnstructuredGridBaseAlgorithm>();
}
extern "C" void vtkUnstructuredGridBaseAlgorithm_destructor(
    vtkNew<vtkUnstructuredGridBaseAlgorithm> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkUnstructuredGridBaseAlgorithm_get_ptr(
    vtkNew<vtkUnstructuredGridBaseAlgorithm> sself
) {
    return sself.GetPointer();
}
