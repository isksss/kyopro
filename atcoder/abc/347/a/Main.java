import java.util.ArrayList;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {

        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int k = sc.nextInt();

        ArrayList<Integer> arr = new ArrayList<>();
        
        for(int i=0; i<n; i++){
            int a = sc.nextInt();
            if(a%k==0){
                arr.add(a/k);
            }
        }

        arr.forEach((i) -> System.out.printf("%d ", i));

    }
}
