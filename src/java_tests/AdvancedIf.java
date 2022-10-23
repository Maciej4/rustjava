public class Main {
    public static void main(String[] args) {
        int x = 0;
        x = x + 20; // x = 20
        int y = x - 3; // y = 17
        int z = 0;
        // NEW CODE
        int a = 0;
//         boolean a;
//         if(a) {
//             z = y;
//         }
//         if((y==0 || y==0) && (y==0 || y==0)){
//             z = y;
//         }
        if((a==0 && a==0 && a==0) || (a==0 && a==0 && a==0) || (a==0 && a==0 && a==0)){
            a = 0;
        }
//         if(a==0 && (a==0 || a==0) && a==0){
//             a = 0;
//         }
//         if((a==0 || a==0 || a==0) && (a==0 || a==0 || a==0) && (a==0 || a==0 || a==0)){
//             a = 0;
//         }
//         if(a==0 && a==0 || a==0 && a==0){
//             a = 0;
//         }
//         if(a==0 || a==0 && a==0 || a==0){
//             a = 0;
//         }
        // END NEW CODE
        if (y > (z + x - 20) && (y < 100 || y == 17)) {
            z = y;
        }
        System.out.println(z);
    }
}