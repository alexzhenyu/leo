/*
namespace: Compile
expectation: Pass
input_file: 
 - inputs/input_unequal.in
 - inputs/input_equal.in
*/

// Conditionally add two u32 integers and log the result to the console.
@program
function main(a: u32, b: u32) -> bool {
    if a == b {
        console.log("{}=={}", a, b); // This line should not fail.
    }
    
    return a == b;
}
