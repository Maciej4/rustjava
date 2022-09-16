public class Main {
    public static void main(String[] args) {
        Point p1 = new Point(10, 20);

        int z = Point.a + Point.b + p1.x + p1.y + p1.sum();

        System.out.println(z);
    }
}