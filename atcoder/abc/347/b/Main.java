import java.util.HashSet;
import java.util.Scanner;
import java.util.Set;

public class Main{
    public static void main(String[] args) {
        // 文字列受付
        Scanner sc = new Scanner(System.in);

        String s = sc.nextLine();
        
        Set<String> set = new HashSet<>();

        for(int i=0; i< s.length(); i++){
            for(int j=i; j<s.length(); j++){
                set.add(s.substring(i, j+1));
            }
        }
        System.out.println(set.size());
        sc.close();
    }
}