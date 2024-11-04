////////// DO NOT CHANGE BELOW HERE /////////
fn print_vec<V: std::fmt::Debug>(vec: &Vec<V>) {
    println!("{vec:#?}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! graph {
    ( $( $n1:literal -> ( $( $n2:literal ), * ) ); + ;) => {
        {
            let mut ret = vec![];
            $( {
                let _n1 = $n1;
                let mut row = vec![ $( ( _n1, $n2 ) ), * ];
                ret.append(&mut row);
            } ) * ;
            ret
        }
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

#[allow(clippy::vec_init_then_push)]
fn main() {
    let my_graph = graph!(
        1 -> (2, 3, 4, 5);
        2 -> (1, 3);
        3 -> (2);
        4 -> ();
        5 -> (1, 2, 3);
    );

    print_vec(&my_graph);
}
