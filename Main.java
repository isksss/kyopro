import java.io.BufferedReader;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        try(Scanner sc = new Scanner(System.in);
            PrintWriter pw = new PrintWriter(System.out);){

                int n = sc.nextInt();

                pw.println(n);
        }
    }
    
}
