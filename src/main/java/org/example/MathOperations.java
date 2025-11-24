package org.example;

public class MathOperations {
    public static double countPI(long n) {
        double sum = 0.0;
        for (long k = 0; k < n; k++) {
            sum += ((k % 2 == 0) ? 1 : (-1)) / (double) (2 * k + 1);
        }
        return 4 * sum;
    }

}
