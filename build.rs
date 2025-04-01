use cmake::Config;

fn main() {
    // Exit early without doing anything if we are building for docsrs
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    println!("cargo:rerun-if-changed=libvtkrs");

    let dst = Config::new("libvtkrs").build();

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-search=/usr/lib/");
        println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu/");
    }

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=vtkrs");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    println!("cargo:rustc-link-lib=dylib=vtkAcceleratorsVTKmCore");
    println!("cargo:rustc-link-lib=dylib=vtkAcceleratorsVTKmDataModel");
    println!("cargo:rustc-link-lib=dylib=vtkAcceleratorsVTKmFilters");
    println!("cargo:rustc-link-lib=dylib=vtkChartsCore");
    println!("cargo:rustc-link-lib=dylib=vtkCommonArchive");
    println!("cargo:rustc-link-lib=dylib=vtkCommonColor");
    println!("cargo:rustc-link-lib=dylib=vtkCommonComputationalGeometry");
    println!("cargo:rustc-link-lib=dylib=vtkCommonCore");
    println!("cargo:rustc-link-lib=dylib=vtkCommonDataModel");
    println!("cargo:rustc-link-lib=dylib=vtkCommonExecutionModel");
    println!("cargo:rustc-link-lib=dylib=vtkCommonMath");
    println!("cargo:rustc-link-lib=dylib=vtkCommonMisc");
    println!("cargo:rustc-link-lib=dylib=vtkCommonPython");
    println!("cargo:rustc-link-lib=dylib=vtkCommonSystem");
    println!("cargo:rustc-link-lib=dylib=vtkCommonTransforms");
    println!("cargo:rustc-link-lib=dylib=vtkDICOMParser");
    println!("cargo:rustc-link-lib=dylib=vtkDomainsChemistryOpenGL2");
    println!("cargo:rustc-link-lib=dylib=vtkDomainsChemistry");
    println!("cargo:rustc-link-lib=dylib=vtkDomainsParallelChemistry");
    println!("cargo:rustc-link-lib=dylib=vtkexodusII");
    println!("cargo:rustc-link-lib=dylib=vtkfides");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersAMR");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersCellGrid");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersCore");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersExtraction");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersFlowPaths");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersGeneral");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersGeneric");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersGeometryPreview");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersGeometry");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersHybrid");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersHyperTree");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersImaging");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersModeling");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersParallelDIY2");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersParallelFlowPaths");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersParallelGeometry");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersParallelImaging");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersParallelMPI");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersParallel");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersParallelStatistics");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersParallelVerdict");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersPoints");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersProgrammable");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersPython");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersReduction");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersReebGraph");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersSelection");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersSMP");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersSources");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersStatistics");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersTemporal");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersTensor");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersTexture");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersTopology");
    println!("cargo:rustc-link-lib=dylib=vtkFiltersVerdict");
    println!("cargo:rustc-link-lib=dylib=vtkGeovisCore");
    println!("cargo:rustc-link-lib=dylib=vtkGeovisGDAL");
    println!("cargo:rustc-link-lib=dylib=vtkglad");
    println!("cargo:rustc-link-lib=dylib=vtkGUISupportQtQuick");
    println!("cargo:rustc-link-lib=dylib=vtkGUISupportQt");
    println!("cargo:rustc-link-lib=dylib=vtkGUISupportQtSQL");
    println!("cargo:rustc-link-lib=dylib=vtkh5part");
    println!("cargo:rustc-link-lib=dylib=vtkImagingColor");
    println!("cargo:rustc-link-lib=dylib=vtkImagingCore");
    println!("cargo:rustc-link-lib=dylib=vtkImagingFourier");
    println!("cargo:rustc-link-lib=dylib=vtkImagingGeneral");
    println!("cargo:rustc-link-lib=dylib=vtkImagingHybrid");
    println!("cargo:rustc-link-lib=dylib=vtkImagingMath");
    println!("cargo:rustc-link-lib=dylib=vtkImagingMorphological");
    println!("cargo:rustc-link-lib=dylib=vtkImagingOpenGL2");
    println!("cargo:rustc-link-lib=dylib=vtkImagingSources");
    println!("cargo:rustc-link-lib=dylib=vtkImagingStatistics");
    println!("cargo:rustc-link-lib=dylib=vtkImagingStencil");
    println!("cargo:rustc-link-lib=dylib=vtkInfovisBoostGraphAlgorithms");
    println!("cargo:rustc-link-lib=dylib=vtkInfovisCore");
    println!("cargo:rustc-link-lib=dylib=vtkInfovisLayout");
    println!("cargo:rustc-link-lib=dylib=vtkInteractionImage");
    println!("cargo:rustc-link-lib=dylib=vtkInteractionStyle");
    println!("cargo:rustc-link-lib=dylib=vtkInteractionWidgets");
    println!("cargo:rustc-link-lib=dylib=vtkIOADIOS2");
    println!("cargo:rustc-link-lib=dylib=vtkIOAlembic");
    println!("cargo:rustc-link-lib=dylib=vtkIOAMR");
    println!("cargo:rustc-link-lib=dylib=vtkIOAsynchronous");
    println!("cargo:rustc-link-lib=dylib=vtkIOCellGrid");
    println!("cargo:rustc-link-lib=dylib=vtkIOCesium3DTiles");
    println!("cargo:rustc-link-lib=dylib=vtkIOCGNSReader");
    println!("cargo:rustc-link-lib=dylib=vtkIOChemistry");
    println!("cargo:rustc-link-lib=dylib=vtkIOCityGML");
    println!("cargo:rustc-link-lib=dylib=vtkIOCONVERGECFD");
    println!("cargo:rustc-link-lib=dylib=vtkIOCore");
    println!("cargo:rustc-link-lib=dylib=vtkIOEngys");
    println!("cargo:rustc-link-lib=dylib=vtkIOEnSight");
    println!("cargo:rustc-link-lib=dylib=vtkIOERF");
    println!("cargo:rustc-link-lib=dylib=vtkIOExodus");
    println!("cargo:rustc-link-lib=dylib=vtkIOExportGL2PS");
    println!("cargo:rustc-link-lib=dylib=vtkIOExportPDF");
    println!("cargo:rustc-link-lib=dylib=vtkIOExport");
    println!("cargo:rustc-link-lib=dylib=vtkIOFDS");
    println!("cargo:rustc-link-lib=dylib=vtkIOFFMPEG");
    println!("cargo:rustc-link-lib=dylib=vtkIOFides");
    println!("cargo:rustc-link-lib=dylib=vtkIOFLUENTCFF");
    println!("cargo:rustc-link-lib=dylib=vtkIOGDAL");
    println!("cargo:rustc-link-lib=dylib=vtkIOGeoJSON");
    println!("cargo:rustc-link-lib=dylib=vtkIOGeometry");
    println!("cargo:rustc-link-lib=dylib=vtkIOH5part");
    println!("cargo:rustc-link-lib=dylib=vtkIOH5Rage");
    println!("cargo:rustc-link-lib=dylib=vtkIOHDF");
    println!("cargo:rustc-link-lib=dylib=vtkIOImage");
    println!("cargo:rustc-link-lib=dylib=vtkIOImport");
    println!("cargo:rustc-link-lib=dylib=vtkIOInfovis");
    println!("cargo:rustc-link-lib=dylib=vtkIOIOSS");
    println!("cargo:rustc-link-lib=dylib=vtkIOLAS");
    println!("cargo:rustc-link-lib=dylib=vtkIOLegacy");
    println!("cargo:rustc-link-lib=dylib=vtkIOLSDyna");
    println!("cargo:rustc-link-lib=dylib=vtkIOMINC");
    println!("cargo:rustc-link-lib=dylib=vtkIOMotionFX");
    println!("cargo:rustc-link-lib=dylib=vtkIOMovie");
    println!("cargo:rustc-link-lib=dylib=vtkIOMPIImage");
    println!("cargo:rustc-link-lib=dylib=vtkIOMPIParallel");
    println!("cargo:rustc-link-lib=dylib=vtkIOMySQL");
    println!("cargo:rustc-link-lib=dylib=vtkIONetCDF");
    println!("cargo:rustc-link-lib=dylib=vtkIOOCCT");
    println!("cargo:rustc-link-lib=dylib=vtkIOODBC");
    println!("cargo:rustc-link-lib=dylib=vtkIOOggTheora");
    println!("cargo:rustc-link-lib=dylib=vtkIOOMF");
    println!("cargo:rustc-link-lib=dylib=vtkIOOpenVDB");
    println!("cargo:rustc-link-lib=dylib=vtkIOParallelExodus");
    println!("cargo:rustc-link-lib=dylib=vtkIOParallelLSDyna");
    println!("cargo:rustc-link-lib=dylib=vtkIOParallelNetCDF");
    println!("cargo:rustc-link-lib=dylib=vtkIOParallel");
    println!("cargo:rustc-link-lib=dylib=vtkIOParallelXdmf3");
    println!("cargo:rustc-link-lib=dylib=vtkIOParallelXML");
    println!("cargo:rustc-link-lib=dylib=vtkIOPDAL");
    println!("cargo:rustc-link-lib=dylib=vtkIOPIO");
    println!("cargo:rustc-link-lib=dylib=vtkIOPLY");
    println!("cargo:rustc-link-lib=dylib=vtkIOPostgreSQL");
    println!("cargo:rustc-link-lib=dylib=vtkIOSegY");
    println!("cargo:rustc-link-lib=dylib=vtkIOSQL");
    println!("cargo:rustc-link-lib=dylib=vtkioss");
    println!("cargo:rustc-link-lib=dylib=vtkIOTecplotTable");
    println!("cargo:rustc-link-lib=dylib=vtkIOTRUCHAS");
    println!("cargo:rustc-link-lib=dylib=vtkIOVeraOut");
    println!("cargo:rustc-link-lib=dylib=vtkIOVideo");
    println!("cargo:rustc-link-lib=dylib=vtkIOVPIC");
    println!("cargo:rustc-link-lib=dylib=vtkIOXdmf2");
    println!("cargo:rustc-link-lib=dylib=vtkIOXdmf3");
    println!("cargo:rustc-link-lib=dylib=vtkIOXMLParser");
    println!("cargo:rustc-link-lib=dylib=vtkIOXML");
    println!("cargo:rustc-link-lib=dylib=vtkkissfft");
    println!("cargo:rustc-link-lib=dylib=vtkloguru");
    println!("cargo:rustc-link-lib=dylib=vtkmetaio");
    println!("cargo:rustc-link-lib=dylib=vtkParallelCore");
    println!("cargo:rustc-link-lib=dylib=vtkParallelDIY");
    println!("cargo:rustc-link-lib=dylib=vtkParallelMPI4Py");
    println!("cargo:rustc-link-lib=dylib=vtkParallelMPI");
    println!("cargo:rustc-link-lib=dylib=vtkPythonContext2D");
    println!("cargo:rustc-link-lib=dylib=vtkPythonInterpreter");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingAnari");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingAnnotation");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingCellGrid");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingContext2D");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingContextOpenGL2");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingCore");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingExternal");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingFFMPEGOpenGL2");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingFreeTypeFontConfig");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingFreeType");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingGL2PSOpenGL2");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingHyperTreeGrid");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingImage");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingLabel");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingLICOpenGL2");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingLOD");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingMatplotlib");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingOpenGL2");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingOpenVR");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingOpenXR");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingParallelLIC");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingParallel");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingQt");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingRayTracing");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingSceneGraph");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingTk");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingUI");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingVolumeAMR");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingVolumeOpenGL2");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingVolume");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingVRModels");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingVR");
    println!("cargo:rustc-link-lib=dylib=vtkRenderingVtkJS");
    println!("cargo:rustc-link-lib=dylib=vtksys");
    println!("cargo:rustc-link-lib=dylib=vtkTestingCore");
    println!("cargo:rustc-link-lib=dylib=vtkTestingDataModel");
    println!("cargo:rustc-link-lib=dylib=vtkTestingGenericBridge");
    println!("cargo:rustc-link-lib=dylib=vtkTestingIOSQL");
    println!("cargo:rustc-link-lib=dylib=vtkTestingRendering");
    println!("cargo:rustc-link-lib=dylib=vtktoken");
    println!("cargo:rustc-link-lib=dylib=vtkUtilitiesBenchmarks");
    println!("cargo:rustc-link-lib=dylib=vtkViewsContext2D");
    println!("cargo:rustc-link-lib=dylib=vtkViewsCore");
    println!("cargo:rustc-link-lib=dylib=vtkViewsInfovis");
    println!("cargo:rustc-link-lib=dylib=vtkViewsQt");
    println!("cargo:rustc-link-lib=dylib=vtkvpic");
    println!("cargo:rustc-link-lib=dylib=vtkWebCore");
    println!("cargo:rustc-link-lib=dylib=vtkWebGLExporter");
    println!("cargo:rustc-link-lib=dylib=vtkWrappingPythonCore3.13");
    println!("cargo:rustc-link-lib=dylib=vtkWrappingTools");
    println!("cargo:rustc-link-lib=dylib=vtkxdmf2");
    println!("cargo:rustc-link-lib=dylib=vtkxdmf3");
    println!("cargo:rustc-link-lib=dylib=vtkxdmfcore");
    println!("cargo:rustc-link-lib=dylib=vtkzfp");
}
