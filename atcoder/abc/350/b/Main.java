import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int q = sc.nextInt();
        // 治療する歯
        int[] t = new int[q];
        for(int i=0; i<q; i++){
            t[i] = sc.nextInt() -1;
        }

        // 歯
        // falseで歯が生えている
        boolean[] h = new boolean[n];
        
        // 歯の数
        int x = n;
        for(int k:t){
            h[k] = !h[k];
            if(h[k]){
                x--;
            }else{
                x++;
            }
        }
        System.out.println(x);

    }
}
