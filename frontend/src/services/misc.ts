import { FoodItem } from "../models/dataTypes";

const calculateTotalCalories = (items: FoodItem[]): string => {
  return items.reduce((total, item) => total + item.calories, 0).toFixed(2);
};

const calculateTotalIntake = (data: FoodItem[]): string => {
  return data.reduce((total, item) => total + item.weight, 0).toFixed(2);
};

export { calculateTotalCalories, calculateTotalIntake };