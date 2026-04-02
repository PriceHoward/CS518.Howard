enum SortingEffects{
    SaveIndexElement(usize),
    Compare(usize, usize),
    Move(usize, usize),
    PlaceIndexElement(usize, i32),
}

fn insertion_sort_pure(arr: &[i32]) -> Vec<SortingEffects> {
    let mut effects = Vec::new();

    for i in 1..arr.len()
    {
        effects.push(SortingEffects::SaveIndexElement(i));
        let index_element = arr[i];
        let mut j = i;

        while j > 0{
        effects.push(SortingEffects::Compare(j-1, i));
        if arr[j-1] > index_element {
            effects.push(SortingEffects::Move(j-1, j));
            j -= 1;
        }else{
            break;
        }
    }
        effects.push(SortingEffects::PlaceIndexElement(j, index_element));
    }
    effects
}


fn insertion_sort_impure(arr: &mut Vec<i32>, effects: Vec<SortingEffects>) {
    for effect in effects {
        match effect {
            SortingEffects::SaveIndexElement(i) => {
                println!("Saving Element {} at index {}.", arr[i], i);
            }
            SortingEffects::Move(minus, j) => {
                arr[j] = arr[minus];
            }
            SortingEffects::PlaceIndexElement(j, index_element) => {
                arr[j] = index_element;
            }
            SortingEffects::Compare(index_one, index_two) => {
                println!("Comparing index {} vs index {}.", index_one, index_two);
            }
        }   
    }   
}


fn insertion_sort_io(arr: &mut Vec<i32>) {
    let effects = insertion_sort_pure(arr);           
    insertion_sort_impure(arr, effects);                          
}


fn main(){
    let mut arr = vec![99, 18, 22, 6, 75, 5, 4, 11, 89, 1];

    insertion_sort_io(&mut arr);
}