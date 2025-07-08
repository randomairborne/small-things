import java.util.Random;

class SlimeChunks {
    private static boolean isSlimeChunk(long seed, int x, int z)
    {
        Random random = new Random(seed + (long)(x * x * 4987142) + (long)(x * 5947611) + (long)(z * z) * 4392871L + (long)(z * 389711) ^ 987234911L);
        return random.nextInt(10) == 0;
    }
    public static void main(String[] args) {
        Long seed = 100l;
        Integer searchSpace = 100000;
        for (Integer i = -searchSpace; i <= searchSpace; i++) {
            for (Integer j = -searchSpace; j <= searchSpace; j++) {
                Boolean valid = true;
                for (Integer x_off = 0; x_off < 4; x_off++) {
                    for (Integer y_off = 0; y_off < 4; y_off++) {
                        valid = valid && isSlimeChunk(seed, i + x_off, j+ y_off);
                    }
                }
                if (valid) {
                    System.out.println(String.format("chunk %d %d block %d %d", i, j, i * 16, j * 16));
                }
            }
        }
    }
}