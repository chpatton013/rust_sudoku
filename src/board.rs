enum Cell {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

const ROW_SIZE: u8 = 9;
const NUM_ROWS: u8 = 9;

const COLUMN_SIZE: u8 = 9;
const NUM_COLUMNS: u8 = 9;

const HOUSE_ROW_SIZE: u8 = 3;
const NUM_HOUSE_ROWS: u8 = 3;
const HOUSE_COLUMN_SIZE: u8 = 3;
const NUM_HOUSE_COLUMNS: u8 = 3;
const HOUSE_SIZE: u8 = HOUSE_ROW_SIZE * HOUSE_COLUMN_SIZE;
const NUM_HOUSES: u8 = NUM_HOUSE_ROWS * NUM_HOUSE_COLUMNS;

type CellGroup = Vec<Option<Cell>>;
type CellRefGroup = Vec<Option<&Cell>>;

struct Row {
    cells: CellRefGroup
}

struct Column {
    cells: CellRefGroup
}

struct House {
    cells: CellRefGroup
}

struct Board {
    cells: CellGroup,
    rows: Vec<Row>,
    columns: Vec<Column>,
    houses: Vec<House>,
}

impl Board {
    pub fn new(cells: CellGroup) -> Board {
        Board {
            cells: cells,
            rows: init_rows(cells),
            columns: init_columns(cells),
            houses: init_houses(cells),
        }
    }

    fn init_rows(cells: CellGroup) -> Vec<Row> {
        let mut rows = Vec::with_capacity(NUM_ROWS);
        rows.resize(NUM_ROWS);
        for row_index in 0..NUM_ROWS {
            let mut row = Vec::with_capacity(ROW_SIZE);
            row.resize(ROW_SIZE);
            for column_index in 0..COLUMN_SIZE {
                let cell_index = row_index * ROW_SIZE + column_index;
                row[column_index] = cells[cell_index];
            }
            rows[row_index] = Row { cells: row };
        }
        return rows;
    }

    fn init_columns(cells: CellGroup) -> Vec<Column> {
        let mut columns = Vec::with_capacity(NUM_COLUMNS);
        columns.resize(NUM_COLUMNS);
        for column_index in 0..NUM_COLUMNS {
            let mut column = Vec::with_capacity(COLUMN_SIZE);
            column.resize(COLUMN_SIZE);
            for row_index in 0..ROW_SIZE {
                let cell_index = row_index * ROW_SIZE + column_index;
                column[column_index] = cells[cell_index];
            }
            columns[column_index] = Column { cells: column };
        }
        return columns;
    }

    fn init_house(cells: CellGroup) -> Vec<House> {
        let mut houses = Vec::with_capacity(NUM_HOUSES);
        houses.resize(NUM_HOUSES);
        for house_row_index in 0..NUM_HOUSE_ROWS {
            for house_column_index in 0..NUM_HOUSE_COLUMNS {
                let house_index = house_row_index * NUM_HOUSE_ROWS + house_column_index;
                let mut house = Vec::with_capacity(HOUSE_SIZE);
                house.resize(HOUSE_SIZE);
                for house_cell_row_index in 0..HOUSE_ROW_SIZE {
                    for house_cell_column_index in 0..HOUSE_COLUMN_SIZE {
                        let house_cell_index =
                                house_cell_row_index * HOUSE_ROW_SIZE +
                                house_cell_column_index;
                        let cell_row_index =
                                house_row_index * NUM_HOUSE_ROWS +
                                house_cell_row_index;
                        let cell_column_index =
                                house_column_index * NUM_HOUSE_COLUMNS +
                                house_cell_column_index;
                        let cell_index =
                                cell_row_index * ROW_SIZE + cell_column_index;
                        house[house_cell_index] = cells[cell_index];
                    }
                }
                houses[house_index] = House { cells: house };
            }
        }
        return houses;
    }

    pub fn get_row(&self, index: u8) -> &Row {
        return self.rows[i];
    }

    pub fn get_column(&self, index: u8) -> &Column {
        return self.columns[i];
    }

    pub fn get_house(&self, index: u8) -> &House {
        return self.houses[i];
    }
}
