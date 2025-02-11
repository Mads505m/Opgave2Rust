/*import java.util.ArrayList;
import java.util.Random;

public class Main {

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
        ArrayList<Sensor> sensors = new ArrayList<>(n);

        // Tidtagning for at måle tid til at indlæse data
        long startTime = System.nanoTime();

        Random rng = new Random();
        for (int id = 0; id < n; id++) {
            float value = rng.nextFloat() * 100;
            sensors.add(new Sensor(id, value));
        }

        long duration = System.nanoTime() - startTime;
        System.out.println("Tid brugt til at indlæse data i Java (i nanosekunder): " + duration);

        // Søgning efter specifik sensor-ID
        int searchId = 3;
        //int searchId = 500000;
        startTime = System.nanoTime();

        Sensor foundSensor = null;
        for (Sensor sensor : sensors) {
            if (sensor.id == searchId) {
                foundSensor = sensor;
                break;
            }
        }

        duration = System.nanoTime() - startTime;
        if (foundSensor != null) {
            System.out.println("Fundet sensor med ID: " + foundSensor.id + " og værdi: " + foundSensor.value);
        } else {
            System.out.println("Sensor med ID " + searchId + " ikke fundet.");
        }

        System.out.println("Tid brugt til at søge efter sensor i Java (i nanosekunder): " + duration);
    }
}
