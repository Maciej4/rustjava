public class Point {
    public static int a = 10;
    public static int b = 20;

    // x and y coordinates of the point
    public int x;
    public int y;

    // Constructor
    public Point(int x, int y) {
        this.x = x;
        this.y = y;
    }

    // Return the sum of the x and y coordinates
    public int sum() {
        return x + y;
    }
}
