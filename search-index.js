var searchIndex = JSON.parse('{\
"tiralabra":{"doc":"","t":[3,3,3,8,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,0,11,11,11,11,11,11,11,11,11,11,11,11,0,11,11,3,8,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,3,11,11,11,11,11,11,11,5,5,5,5,11,11,11],"n":["CorrelationMatch","CrossCorrelation","DisplayBuffer","Float","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","compute","compute","compute_truncated","from","from","from","get_buffer_mut","get_display","get_memory","get_offset","get_period","into","into","into","new","new","new","parabolic_interpolation_minimum","ring_buffer","scroll","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","update_display","update_match","util","v","v","Consumer","Item","Producer","borrow","borrow","borrow_mut","borrow_mut","discard_all","from","from","into","into","pop_full","push","try_from","try_from","try_into","try_into","type_id","type_id","with_capacity","IterWindows","borrow","borrow_mut","from","from","into","into_iter","next","shift_left","shift_left_fill","shift_right","shift_right_fill","try_from","try_into","type_id"],"q":["tiralabra","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","tiralabra::ring_buffer","","","","","","","","","","","","","","","","","","","","","tiralabra::util","","","","","","","","","","","","","",""],"d":["Finds the closest match of a shorter piece of audio from a …","Computes cross correlation efficiently, using FFT.","Stores a prepared <code>CorrelationMatch</code> and buffers for display …","","","","","","","","Compute how much <code>b</code> should be shifted (to the right) to …","Compute cross correlation including partially overlapping …","Compute cross correlation excluding partially overlapping …","","","","Get a mutable reference to the input buffer. If it is …","Retrieve the contents of the display buffer.","Retrieve the contents of the memory buffer. This is what …","Get the current offset and residual.","Get current estimated period.","","","","Construct a new <code>DisplayBuffer</code> with given input buffer size …","Allocate and prepare a correlation match algorithm. …","Allocate and prepare a cross correlation. <code>max_size</code> is the …","Finds the approximate minimum point of a function given …","Ring buffer for transferring audio data between threads in …","Scroll all internal buffers by the given signed amount of …","","","","","","","","","","Update the display buffer based on the newest input data …","Update the correlation match position, memory buffer and …","Miscellaneous array utilities.","","","The Consumer party of a ring buffer. Can only pop data off …","Defines the interface that an item stored in the queue …","The Producer party of a ring buffer. Can only push data to …","","","","","Discard all currently available data.","","","","","Tries to fill the entirety of the given <code>data</code> slice. If …","Tries to push the contents of the given slice into the …","","","","","","","Construct a ring buffer with a given capacity, and return …","Iterate over fixed size windows.","","","","Construct an <code>IterWindows</code> from another iterator.","","","","Shift the contents of the given <code>array</code> left, i.e. towards …","Like <code>shift_left</code>, but shifts by a given <code>amount</code> and fills …","Similar to <code>shift_left</code>, but in the opposite direction. The …","Like <code>shift_right</code>, but shifts by a given <code>amount</code> and fills …","","",""],"i":[0,0,0,0,1,2,3,1,2,3,2,3,3,1,2,3,1,1,1,1,1,1,2,3,1,2,3,0,0,1,1,2,3,1,2,3,1,2,3,1,1,0,4,4,0,0,0,5,6,5,6,6,5,6,5,6,6,5,5,6,5,6,5,6,0,0,7,7,7,7,7,7,7,0,0,0,0,7,7,7],"f":[null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["float",8]],["option",4]],null,[[["i32",15]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[["bool",15]]],null,[[["f64",15]]],[[["f64",15]]],null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],[["usize",15],["result",4,["usize"]]]],[[],[["usize",15],["result",4,["usize"]]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[["usize",15]]],null,[[]],[[]],[[]],[[]],[[]],[[]],[[],["option",4]],[[]],[[["usize",15],["copy",8]]],[[]],[[["usize",15],["copy",8]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]]],"p":[[3,"DisplayBuffer"],[3,"CorrelationMatch"],[3,"CrossCorrelation"],[8,"Float"],[3,"Producer"],[3,"Consumer"],[3,"IterWindows"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};