public class Main {
    public static void main(String[] args) {
        int x = 0;
        x = x + 20; // x = 20
        int y = x - 3; // y = 17
        int z = add(x, y); // z = 37
        System.out.println(z);
    }

    public static int add(int a, int b) {
        return a + b;
    }
}