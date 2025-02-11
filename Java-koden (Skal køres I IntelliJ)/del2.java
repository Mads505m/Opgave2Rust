/*
import java.util.HashMap;
import java.util.Random;

public class Main {

    // Nested Sensor class
    static class Sensor {
        int id;
        float value;

        public Sensor(int id, float value) {
            this.id = id;
            this.value = value;
        }
    }

    public static void main(String[] args) {
        int n = 1_000_000;
        HashMap<Integer, Sensor> sensors = new HashMap<>(n);

        Random rng = new Random();
        for (int id = 0; id < n; id++) {
            float value = rng.nextFloat() * 100;
            sensors.put(id, new Sensor(id, value));
        }

        long startTime = System.nanoTime();
        int searchId = 500_000;
        Sensor foundSensor = sensors.get(searchId);
        if (foundSensor != null) {
            System.out.println("Fundet sensor med id " + foundSensor.id + " og værdi " + foundSensor.value);
        } else {
            System.out.println("Sensor med id " + searchId + " ikke fundet.");
        }
        long duration = System.nanoTime() - startTime;

        System.out.println("Tid brugt til at søge efter sensor i Java (i nanosekunder): " + duration);
    }
}
