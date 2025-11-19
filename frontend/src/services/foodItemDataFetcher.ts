import { FoodItem } from "../models/dataTypes";

const getFoodItems = async (): Promise<FoodItem[]> => {
  try {
    const response = await fetch(`http://localhost:8888/foodItems`);
    if (!response.ok) {
      throw new Error('Network response was not ok');
    }
    const data = await response.json();

    console.debug('Fetched food items:', JSON.stringify(data));

    return data;
  } catch (error) {
    console.error('Error fetching food items:', error);
    throw error;
  }
};

const getFoodItemsByDate = async (date: string): Promise<FoodItem[]> => {
  try {
    const response = await fetch(`http://localhost:8888/foodItem?date=${date}`);
    if (!response.ok) {
      throw new Error('Network response was not ok');
    }
    const data = await response.json();

    console.debug('Fetched food items:', JSON.stringify(data));

    return data;
  } catch (error) {
    console.error('Error fetching food items:', error);
    throw error;
  }
};

export { getFoodItems, getFoodItemsByDate };