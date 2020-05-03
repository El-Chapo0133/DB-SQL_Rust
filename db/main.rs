






trait tManager {
        init() -> u8;
        createTable(name: str) -> u8;
}
trait tDataAdapter {
        adapte_i8() -> Vec<u8>;
        adapte_i16() -> Vec<u8>;
        adapte_i32() -> Vec<u8>;
        adapte_i64() -> Vec<u8>;
        adapte_u8() -> Vec<u8>;
        adapte_u16() -> Vec<u8>;
        adapte_u32() -> Vec<u8>;
        adapte_u64() -> Vec<u8>;
        adapte_string() -> Vec<u8>;
        adapte_bool() -> Vec<u8>;
        adapte_f8() -> Vec<u8>;
        adapte_f16() -> Vec<u8>;
        adapte_f32() -> Vec<u8>;
        adapte_f64() -> Vec<u8>;
        add_id_cell(&mut Vec<Vec<u8>>);
}

trait tDataBase {
        save(output: str) -> u8;
        load(input: str) -> u8;
        insert(data: Vec<Vec<u8>>);
        update(id: u64, data: Vec<Vec<u8>>);
        delete(id: u64);
        get(id: u64) -> Vec<Vec<u8>>;
}
trait tTable {
        init() -> u8;
        insert(data: Vec<Vec<u8>>);
        update(id: u64, data: Vec<Vec<u8>>);
        delete(id: u64);
        get(id: u64) -> Vec<Vec<u8>>;
}
trait tTableHeader {
        init() -> u8;
        check(input: Vec<Vec<u8>>) -> u8;
}
trait tTableCollumnHeader {
        init() -> u8;
}
trait tRow {
        push() -> u8;
}

////////////////////////////
/// TODO: struct lifetime //
////////////////////////////


// Manager for everything related to the DataBase
// you should import only this struct
struct Manager {
        init: u8,
}
// Adapte the data to use in a cell for a Row
struct DataAdaptater {
        
}


// struct that contains every table
#[derive(Clone)]
struct DataBase {
        tables: Vec<Table>,
}
// a table, with the data
struct Table {
        header: TableHeader,
        data: Vec<Row>,
}
// the header of the table, which define the collumn type,name,etc
struct TableHeader {
        data: Vec<TableCollumnHeader>,
}
// define the collumn name and the collumn type
struct TableCollumnHeader {
        name: Vec<u8>,
        type: Vec<u8>,
}
// a row for the table
struct Row {
        data: Vec<Vec<u8>>,
}



impl tManager for Manager {

}
impl tDataAdapter for DataAdaptater {

}
impl tDataBase for DataBase {

}
impl tTable for Table {

}
impl tTableHeader for TableHeader {

}
impl tTableCollumnHeader for TableCollumnHeader {

}
impl tRow for Row {

}