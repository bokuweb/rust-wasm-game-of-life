class Game {

    constructor(buf, row_size, col_size) {
        this.field = this.create(buf, row_size, col_size);
        this.a = [];
    }

    create(buf, row_size, col_size) {
        return [...Array(row_size).keys()]
            .map((i) => {
                const start = i * col_size;
                const end = start + col_size;
                return buf.slice(start, end);
            })
    }

    next() {
        const a = [];
        this.field.forEach((r, y) => [].push.apply(a, this.next_row(r, y)));
        return a;
    }

    next_row(row, y) {
        return row.map((c, x) => this.next_cell(y, x));
    }

    next_cell(y, x) {
        const alive_num = [
            [y - 1, x - 1],
            [y, x - 1],
            [y + 1, x - 1],
            [y - 1, x],
            [y + 1, x],
            [y - 1, x + 1],
            [y, x + 1],
            [y + 1, x + 1],
        ]
            .map(([y, x]) => this.get_cell_state(y, x))
            .filter(cell => cell)
            .length;

        if (alive_num === 3) return true;
        if (alive_num === 2 && this.is_alive(y, x)) return true;
        return false;

    }

    is_alive(y, x) {
        return !!this.field[y][x];
    }


    get_cell_state(row, column) {
        if (row < 0 || column < 0) return false;
        if (row >= this.field.length || column >= this.field[row].length) return false;
        return this.field[row][column];
    }
}
