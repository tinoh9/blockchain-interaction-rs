// LIST OF COMMON FUNCTIONS IN RUST

assert! // Used for testing. assert! checks a boolean condition and panics if the condition is not met
assert_eq! // Used for testing. assert_eq! compares two values for equality and panics if they are not equal
format! // Used to format a string using a template
println! // Used to print a message to the console
vec! // Used to create a new vector with a given set of elements
and_then // Does the same thing as map, but the function must return another Option or Result type
clone // Used for creating a deep copy of a value
collect // Used to collect values from an iterator into a collection, such as a Vec or a HashMap
expect // Does the same thing as unwrap, but allows you to specify a custom error message to display if the value is None or Err
filter // Used to create a new iterator with only the elements that match a predicate
find // Used to search for an element in a vector and return its position
fold // Used to reduce an iterator to a single value by repeatedly applying a function
insert // Used to insert an element at a specific position in a vector
into_iter // Consumes a collection and returns an iterator over its elements
is_empty // Returns true if a collection is empty, false otherwise
iter // Returns an iterator over the elements of a collection
join // Used to join the elements of a vector with a separator string
len // function returns the length of a string, slice, or array
map // Used to transform values inside Option and Result types. map applies a function to the inner value and returns a new Option or Result with the transformed value
max // Used to find the maximum value in a collection
min // Used to find the minimum value in a collection
pop // Used to remove and return the last element of a vector
push // Used to add an element to the end of a vector
reduce: // Used for reducing a collection to a single value
remove // Used to remove an element at a specific position in a vector
reverse // Reverses the order of the elements in a collection
sort // Used to sort the elements of a vector in ascending order
sort_by // Does the same thing as sort, but allows you to specify a custom comparison function
split_at // Splits a collection at a given index and returns a tuple of the two resulting collections
unwrap // Used to unwrap Option and Result types. unwrap returns the inner value if the Option or Result is Some or Ok, and panics if it is None or Err
unwrap_or  // Does the same thing as unwrap, but returns a default values if it is None or Err
unwrap_or_else: // Does the same thing as unwrap, but calls a function to generate a default value if the variant is None
Box::new // Used to allocate an object on the heap and return a pointer to it
HashMap::new // Used to create a new empty HashMap instance
Iterator::collect // Used to convert an iterator into a collection, such as a vector
Option::unwrap // Used to extract the value from an Option type, panicking if the value is None
Result::unwrap_or_else // Used to extract the value from a Result type, or to provide a default value if the value is an Err
String::from // Used to create a new String from a given string literal
String::new // Used to create a new empty string
iter::filter // Used to filter the elements of an iterator, producing a new iterator with only the elements that match a predicate
iter::map // Used to apply a function to each element of an iterator, producing a new iterator with the result
mem::drop // Used for manually dropping a value
result::unwrap // Used to extract the value from a Result type, panicking if the value is an error
slice::sort // Used to sort the elements of a slice
str::parse // Used to parse a string into a number. It takes a string and a type that implements the FromStr trait, and returns a value of that type if the string can be parsed successfully
thread::spawn // Used to create a new thread and execute a given closure on it