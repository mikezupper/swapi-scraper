use indradb::Datastore;
use indradb::Transaction;
use indradb::{MemoryDatastore, MemoryTransaction, SpecificVertexQuery, Type};
use tempfile::NamedTempFile;

type next_url = Option<String>;

trait Factor {
    fn factorial_tail_rec(val: next_url) -> Self;
    fn factorial(num: next_url) -> Self;
}

impl Factor for next_url {
    fn factorial_tail_rec(val: next_url) -> Self {
        val
    }

    fn factorial(url: next_url) -> Self {
        //fetch the next results
        //check pagination "next", match Some/None
        match url {
            None => None,
            Some(next_url_to_fetch) =>{
                
                Some(current_url * Self::factorial_tail_rec(current_url - 1)),
            }
        }
    }
}

fn main() {
    println!("hello!");

    let result: next_url = Factor::factorial(3);
    assert_eq!(6, result);
    /*

    let path = NamedTempFile::new().unwrap();

    let id = {
        let datastore = MemoryDatastore::create(path.path()).unwrap();
        let trans = datastore.transaction().unwrap();
        let id = trans.create_vertex_from_type(Type::default()).unwrap();
        datastore.sync().unwrap();
        id
    };

    let datastore = MemoryDatastore::read(path.path()).unwrap();
    let trans = datastore.transaction().unwrap();
    println!("{}{}",trans.get_vertex_count().unwrap(), 1);
    let vertices = trans
        .get_vertices(SpecificVertexQuery::new(vec![id]))
        .unwrap();
    println!("{}{}",vertices.len(), 1);
    println!("{}{}",vertices[0].id, id);
    println!("{:?}{:?}",vertices[0].t, Type::default());
     */
}
