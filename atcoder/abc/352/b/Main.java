import java.io.PrintWriter;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        try(Scanner sc = new Scanner(System.in);
            PrintWriter pw = new PrintWriter(System.out);){

                String s = sc.nextLine();
                String t = sc.nextLine();

                int tn = 0;

                for(int i=0; i<s.length(); i++){
                    // System.out.print(s.charAt(i));
                    for(int j=tn;j<t.length() && tn < t.length(); j++){
                        if(s.charAt(i) == t.charAt(j)){
                            System.out.print((j+1) + " ");
                            tn = j + 1;
                            break;
                        }
                    }
                }
        }
    }
    
}
