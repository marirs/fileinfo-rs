use exe::PE;
use std::hash::Hash;

pub fn get_pe_file_size<P: PE>(pe: &P) -> usize {
    pe.as_slice().len()
    // pe.get_buffer().as_ref().len()
}

pub fn round_to_pe_sz_with_offset<P: PE>(pe: &P, offset: usize, value: usize) -> usize {
    let pe_sz = get_pe_file_size(pe);
    match offset + value > pe_sz {
        true => pe_sz - offset,
        false => value,
    }
}
pub fn safe_read<P: PE>(pe: &P, offset: usize, size: usize) -> &[u8] {
    let pe_sz = get_pe_file_size(pe);
    if offset >= pe_sz {
        return pe.read(0, 0).unwrap();
    }

    let safe_size = round_to_pe_sz_with_offset(pe, offset, size);
    pe.read(offset, safe_size).unwrap()
}

#[allow(unused)]
pub trait Comparable<T, U>
where
    T: Eq + PartialEq + Hash + IntoIterator,
{
    fn compare(s: Vec<T>) -> Vec<(U, u32)>;
}

#[macro_export]
macro_rules! compare_default_impl {
    { $U:ty, $V:ty } => {
        impl Comparable<$U, $V> for $U {
            fn compare(pe_richs: Vec<$U>) -> Vec<($V, u32)> {

                let mut map: HashMap<$V, u32> = HashMap::<$V, u32>::new();
                for pe_string_vec in pe_richs {
                    let mut found_already: HashMap<$V, bool> = HashMap::<$V, bool>::new();


                    for element in pe_string_vec.into_iter() {
                        if ! found_already.contains_key(&element) {
                            map.entry(element.clone()).and_modify(|val| *val += 1).or_insert(1);
                            found_already.insert(element.to_owned(), true);
                        }
                    }
                }

                let mut sorted_map: Vec<($V, u32)> = map.into_iter().collect::<Vec<_>>();
                sorted_map.sort_by(|a, b| a.1.cmp(&b.1));

                sorted_map
            }
        }
    };
}
