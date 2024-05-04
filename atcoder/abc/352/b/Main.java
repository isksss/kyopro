
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();

        int[] a = new int[n];
        int[] b = new int[n];

        for(int i=0; i<n; i++){
            a[i] = sc.nextInt();
            b[i] = sc.nextInt();

            // System.out.println(a[i] + " " + b[i]);
        }
        int max = 0;
        int c = a[0];
        int m = b[0]-a[0]; // 頭最高の人
        int mi = 0;

        for(int i=1; i<n; i++){
            int h = b[i]- a[i];
            if(h>m){
                max += c;

                //
                c = a[i];
                m = h;
                mi = i;
                continue;
            }else{
                max += a[i];
            }
        }

        max += m;
        System.out.println(max);
        
    }
    
}
