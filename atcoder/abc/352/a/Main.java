
import java.io.PrintWriter;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        try(Scanner sc = new Scanner(System.in);
            PrintWriter pw = new PrintWriter(System.out);){

                int n = sc.nextInt();
                int x = sc.nextInt();
                int y = sc.nextInt();
                int z = sc.nextInt();

                if(x > y){
                    for(int i=x; i>y; i--){
                        if(z == i){
                            System.out.println("Yes");
                            return;
                        }
                    }
                }else if(x < y){
                    for(int i=x; i<y; i++){
                        if(z == i){
                            System.out.println("Yes");
                            return;
                        }
                    }
                }

                System.out.println("No");
        }
    }
    
}
