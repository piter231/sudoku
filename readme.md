# Sudoku Solver 🎮🌐

Welcome to the **Sudoku Solver** project! ✨ This is a simple command-line application written in **Rust** ⚛️ that solves Sudoku puzzles. It's fast, lightweight, and easy to use.

## Features 🚀

- ✨ **Solve any valid Sudoku puzzle** in milliseconds.
- 🔄 **Input-friendly format:** Enter puzzles in a compact single-line format.
- ⚡ **Efficient algorithm** to find solutions (or determine if none exist).
- 🔒 **Zero dependencies:** Built entirely in Rust, no external libraries needed.

## Input Format 🌐

The program accepts puzzles as a single string of 81 characters:

- Use **digits (1-9)** for known values.
- Use **`x`** for unknown cells.
- Example:

```
816245x79
47x819526
25963x418
x649x2753
79235x864
38x476192
631728x45
x28x9x637
94x563281
```

This represents the following Sudoku board:

```
816 245 x79
47x 819 526
259 63x 418
x64 9x2 753
792 35x 864
38x 476 192
631 728 x45
x28 x9x 637
94x 563 281
```

## Output Format 🔗

- The program will print the solved Sudoku in the **same format** as the input.
- If no solution exists, the program will print **nothing**.

### Example

Input:
```
816245x79
47x819526
25963x418
x649x2753
79235x864
38x476192
631728x45
x28x9x637
94x563281
```

Output:
```
816245379
473819526
259637418
164982753
792354864
385476192
631728945
528491637
947563281
```

## Installation 🌟

1. Make sure you have **Rust** installed.

2. Clone this repository:
   ```bash
   git clone https://github.com/piter231/sudoku.git
   cd sudoku
   ```

3. Run the program:
   ```bash
   ./cargo run
   ```

## Contributing 🎓

We welcome contributions! Feel free to:

- 🔨 Fork the repo and submit pull requests.
- 🔍 Report issues or suggest features via [GitHub Issues](https://github.com/piter231/sudoku/issues).
- 🎨 Improve the code or add new features.

## License 🏞️

This project is licensed under the MIT License. 

---

Happy solving! 😎🎉

