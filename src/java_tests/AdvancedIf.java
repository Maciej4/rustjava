public class Main {
    public static void main(String[] args) {
        int x = 0;
        x = x + 20; // x = 20
        int y = x - 3; // y = 17
        int z = 0;
        if (y > (z + x - 20) && (y < 100 || y == 17)) {
            z = y;
        }
        System.out.println(z);
    }
}