import java.util.Scanner;

public class Main{
    public static void main(String[] args) {
        // 文字列受付
        Scanner sc = new Scanner(System.in);

        String str = sc.nextLine();

        int n = Integer.parseInt(str.substring(3));

        if(n >0 && n <350 && n != 316){
            System.out.println("Yes");
        }else{
            System.out.println("No");
        }

        sc.close();
    }
}