import java.io.PrintWriter;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        // 標準入力
        @SuppressWarnings("resource")
        Scanner sc = new Scanner(System.in);

        // こっから処理開始 ===============
        
        int n = sc.nextInt();
        String[] a = new String[n];
        String[] b = new String[n];
        String space = sc.nextLine();
        for(int i=0; i<n; i++){
            a[i] = sc.nextLine();
        }
        for(int i=0; i<n; i++){
            b[i] = sc.nextLine();
        }

        for(int i=0; i<n; i++){
            if(!a[i].equals(b[i])){
                for(int j=0; j<a[i].length(); j++){
                    char ca = a[i].charAt(j);
                    char cb = b[i].charAt(j);

                    if(ca != cb){
                        System.out.println(i+1 + " " + (j+1));
                    }
                }
            }
        }

        // ここまで処理 ===============
    }
}
