
import java.util.Arrays;

class main {

    static Boolean checkQueen(Integer row, Integer[][] board, Integer columnNumber) {
        for (Integer i = 0; i < row; i++) { // Check rows for a queen in this column.
            if (board[i][columnNumber] == 1) {
                return false;
            }
        }

        for (Integer i = row - 1, j = columnNumber - 1; i >= 0 && j >= 0; i--, j--) {
            if (board[i][j] == 1) {
                return false;
            }
        }

        for (Integer i = row - 1, j = columnNumber + 1; j < board.length && i >= 0; i--, j++) {
            if (board[i][j] == 1) {
                return false;
            }
        }
        return true;
    }

    static void findAnswer(Integer row, Integer[][] board) // Place a queen on the board. Then call Check Queen. If check fails, remove queen and move it a row.
    {
        if (row == board.length) {
            printBoard(board);
            return;
        }

        for (int i = 0; i < board.length; i++) // Columns
        {
            Boolean safe = checkQueen(row, board, i);
            if (safe) {
                board[row][i] = 1;
                findAnswer(row + 1, board);
                board[row][i] = 0;
            }
        }
    }

    static void initAlgorithm() {
        Integer[][] board = new Integer[8][8];

        for (Integer[] row : board) {
            Arrays.fill(row, 0);
        }

        findAnswer(0, board); // Start at the 0 row.
    }

    static void printBoard(Integer[][] board) {
        for (Integer i = 0; i <= 7; i++) {
            for (Integer j = 0; j <= 7; j++) {
                System.out.print(board[i][j] + " ");
            }
            System.out.print("\n");
        }
        System.out.println();
    }

    public static void main(String[] args) {
        initAlgorithm();
    }
}
