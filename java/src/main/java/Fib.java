/**
 * build: mvn package
 * run:   java -cp target/fibonacci-1.0-SNAPSHOT.jar Fib [iterations]
 */
public class Fib {

    private static int fib(final int i) {
        if (i < 2) {
            return 1;
        }
        return fib(i - 1) + fib(i - 2);
    }

    public static void main(String[] args) {
        int input = 34;
        if (args[0] != null) {
            input = Integer.parseInt(args[0]);
        }
        System.out.printf("%d\n", fib(input));
    }

}
