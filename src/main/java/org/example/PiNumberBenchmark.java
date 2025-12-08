package org.example;

import org.openjdk.jmh.annotations.*;

import java.util.concurrent.TimeUnit;

@BenchmarkMode(Mode.AverageTime)
@OutputTimeUnit(TimeUnit.MILLISECONDS)
@Warmup(iterations = 5, time = 500, timeUnit = TimeUnit.MILLISECONDS)
@Measurement(iterations = 10, time = 500, timeUnit = TimeUnit.MILLISECONDS)
@Fork(2)
@State(Scope.Benchmark)
public class PiNumberBenchmark {

    @Threads(1)
    @Benchmark
    public void benchmarkPiOneThread() {
        MathOperations.countPI(100_000_000);
    }

    @Threads(2)
    @Benchmark
    public void benchmarkPiTwoThreads() {
        MathOperations.countPI(100_000_000);
    }

    @Threads(4)
    @Benchmark
    public void benchmarkPiFourThreads() {
        MathOperations.countPI(100_000_000);
    }

    @Threads(6)
    @Benchmark
    public void benchmarkPiSixThreads() {
        MathOperations.countPI(100_000_000);
    }

    @Threads(50)
    @Benchmark
    public void benchmarkPiFiftyThreads() {
        MathOperations.countPI(100_000_000);
    }

}
