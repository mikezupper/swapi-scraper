use indradb::Datastore;
use indradb::Transaction;
use indradb::{MemoryDatastore, MemoryTransaction, SpecificVertexQuery, Type};
use tempfile::NamedTempFile;
#[derive(Debug)]
struct States<'a> {
    a: &'a i32,
    b: &'a i32,
}

trait Currying<T> {
    type ReturnType: Fn(T) -> T;
    fn add(self) -> Self::ReturnType;
}

impl Currying<i32> for States<'static> {
    type ReturnType = Box<dyn Fn(i32) -> i32>;

    fn add(self) -> Self::ReturnType {
        Box::new(move |x| x * self.a)
    }
}

fn fmt(prev_str: &str) -> String {
    let mut new_str = String::new();

    let closure_annotated = |next_str| -> String {
        new_str.push_str(prev_str);
        new_str.push_str(next_str);
        return new_str;
    };

    closure_annotated("dolor sit amet")
}
fn main() {
    ///CLOSURES
    let r_txt = "Lorem ipsum ";
    dbg!("Lorem ipsum dolor sit amet", fmt(r_txt));

    let r_value: States = States { a: &100, b: &100 };

    let r1 = r_value.add();
    let r2 = r1(5);

    dbg!(500, r2);
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
