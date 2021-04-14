var searchIndex = JSON.parse('{\
"tiralabra":{"doc":"","i":[[0,"correlation_match","tiralabra","Design sketch in Finnish:",null,null],[3,"CorrelationMatch","tiralabra::correlation_match","",null,null],[12,"max_size","","",0,null],[12,"cross_correlation","","",0,null],[12,"f_buffer","","",0,null],[12,"g_buffer","","",0,null],[12,"result_buffer","","",0,null],[11,"new","","Allocate and prepare a correlation match algorithm. …",0,[[["usize",15]]]],[11,"compute","","Compute how much <code>b</code> should be shifted (to the right) to …",0,[[],["f32",15]]],[0,"cross_correlation","tiralabra","Computes cross correlation efficiently, using FFT.",null,null],[3,"CrossCorrelation","tiralabra::cross_correlation","",null,null],[12,"base_size","","",1,null],[12,"fft_size","","",1,null],[12,"fft","","",1,null],[12,"buffer","","",1,null],[11,"new","","Allocate and prepare a cross correlation. <code>max_size</code> is the …",1,[[["usize",15]]]],[11,"compute","","Compute cross correlation including partially overlapping …",1,[[]]],[11,"compute_truncated","","Compute cross correlation excluding partially overlapping …",1,[[]]],[11,"compute_raw","","Performs the computation without extracting results from …",1,[[]]],[0,"fft","tiralabra","This module implements the FFT, i.e. Fast Fourier …",null,null],[3,"Prepared","tiralabra::fft","A structure that is initialized beforehand, and contains …",null,null],[12,"size","","",2,null],[11,"new","","Prepare FFT. Size has to be a power of two.",2,[[["usize",15]]]],[11,"fft","","Perform the transform. The size of the array has to be …",2,[[]]],[11,"ifft","","Perform the inverse transform. The size of the array has …",2,[[]]],[0,"math","tiralabra","This module defines mathematical structures and …",null,null],[6,"Num","tiralabra::math","The primary type of real numbers. This may be switched to …",null,null],[17,"PI","","The pi constant. If necessary, can be defined by hand …",null,null],[17,"IMAG_UNIT","","The imaginary unit \\\"i\\\".",null,null],[3,"Complex","","A complex number.",null,null],[12,"real","","",3,null],[12,"imag","","",3,null],[11,"conj","","The complex conjugate.",3,[[]]],[11,"abs2","","Square of the absolute value.",3,[[],["f32",15]]],[11,"abs","","Absolute value.",3,[[],["f32",15]]],[11,"euler","","Euler\'s formula, <code>e^(ix) = cos x + i sin x</code>.",3,[[["f32",15]]]],[11,"from","tiralabra::correlation_match","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","tiralabra::cross_correlation","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","tiralabra::fft","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","tiralabra::math","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","","",3,[[],["complex",3]]],[11,"clone","","",3,[[],["complex",3]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"div","","",3,[[["f32",15]]]],[11,"div","","",3,[[]]],[11,"sub","","",3,[[]]],[11,"add","","",3,[[]]],[11,"mul","","",3,[[["f32",15]]]],[11,"mul","","",3,[[]]],[11,"neg","","",3,[[]]]],"p":[[3,"CorrelationMatch"],[3,"CrossCorrelation"],[3,"Prepared"],[3,"Complex"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);