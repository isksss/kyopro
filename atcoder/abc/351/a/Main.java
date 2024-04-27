import java.io.PrintWriter;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        // 標準入力
        Scanner sc = new Scanner(System.in);
        // 標準出力
        PrintWriter out = new PrintWriter(System.out);

        // こっから処理開始 ===============
        int omote = 0;
        int ura = 0;

        for(int i=0; i<9; i++){
            omote += sc.nextInt();
        }
        for(int i=0; i<8; i++){
            ura += sc.nextInt();
        }

        out.println(omote - ura + 1);

        // ここまで処理 ===============
        out.flush();
    }
}
