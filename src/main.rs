mod linear_search;
mod linked_list;
mod binary_search;
mod bubble_sort;
mod quick_sort;
mod dijkstras;
mod queue;
mod stack;
mod binary_tree;
mod min_heap;
mod weighted_adj_list;
mod data_structures;
fn linear_search_demo(){
    println!("-------------------");
    println!("LINEAR SEARCH DEMO:");
    println!("-------------------");
    let haystack = [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];
    println!("Array being searched: {:?}",haystack);
    let needle = 81;
    let search = linear_search::linear_search(&haystack, needle);
    match search {
        Some((value, index)) => {
            if value {
                println!("Value {} found at index {}", needle, index);
            } else {
                println!("Value {} not found", needle);
            }
        }
        None => println!("Error: Unable to perform linear search."),
    }
}
fn binary_search_demo(){
    println!("-------------------");
    println!("BINARY SEARCH DEMO:");
    println!("-------------------");
    let haystack = [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];
    println!("Array being searched: {:?}",haystack);
    let needle = 69;
    let search = binary_search::binary_search(&haystack,needle);
    match search {
        Some((value, index)) => {
            if value {
                println!("Value {} found at index {}", needle, index);
            } else {
                println!("Value {} not found", needle);
            }
        }
        None => println!("Error: Unable to perform linear search."),
    }
}
fn bubble_sort_demo(){
    println!("-------------------");
    println!("BUBBLE SORT DEMO:");
    println!("-------------------");
    let mut arr = [9, 3, 7, 4, 69, 420, 42];
    println!("Array before quick sort: {:?}",arr);
    bubble_sort::bubble_sort(&mut arr);
    println!("Array after quick sort: {:?}",arr);
}
fn quick_sort_demo(){
    println!("-------------------");
    println!("QUICK SORT DEMO:");
    println!("-------------------");
    let mut arr = [9, 3, 7, 4, 69, 420, 42];
    println!("Array before quick sort: {:?}",arr);
    quick_sort::quick_sort(&mut arr);
    println!("Array after quick sort: {:?}",arr);
}
fn main() {
    println!("");
    println!("|~~~~~~~~~~~~~~~|");
    println!("SEARCHING DEMOS:");
    println!("|~~~~~~~~~~~~~~~|");
    println!("");
    linear_search_demo();
    binary_search_demo();
    println!("");
    println!("|~~~~~~~~~~~~~~~|");
    println!("SORTING DEMOS:");
    println!("|~~~~~~~~~~~~~~~|");
    println!("");
    quick_sort_demo();
    bubble_sort_demo();
}
