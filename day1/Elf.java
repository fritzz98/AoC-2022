import java.util.ArrayList;
import java.util.List;

public class Elf {
    private List<Integer> ownedCalories;

    public Elf() {
        ownedCalories = new ArrayList<>();
    }

    public void addCalorieValue(int val) {
        ownedCalories.add(val);
    }

    public int getSumOfCalories() {
        int sum = 0;

        for (int calorieValue : ownedCalories)
            sum += calorieValue;
        return sum;
    }

    @Override
    public String toString() {
        return "Elf{" +
                "ownedCalories=" + ownedCalories +
                '}';
    }
}
