export interface FoodItem {
  id: number;
  date: string;
  time: string;
  name: string;
  weight: number;
  calories: number;
  notes: string;
  url: string;
}

export interface WeighIn {
  id: number;
  date: string;
  time: string;
  weight: number;
  notes: string;
}

export type SingleFoodItem = {
  name: string;
  weight: number;
  calories: number;
  notes: string;
  url: string;
}

export type NestedFoodItem = {
  id: number;
  date: string;
  time: string;
  subRows?: SingleFoodItem[];
}

export const nestFoodItems = (foodItems: FoodItem[]): NestedFoodItem[] => {
  let map = new Map<string, NestedFoodItem>();

  foodItems.forEach((item) => {
    const key = `${item.date}|${item.time}`;

    if (!map.has(key)) {
      let nestedItem: NestedFoodItem = {
        id: item.id,
        date: item.date,
        time: item.time,
        subRows: [{ name: item.name, weight: item.weight, calories: item.calories, notes: item.notes, url: item.url }],
      };
      map.set(key, nestedItem);
    } else {
      let nestedItem = map.get(key);
      nestedItem?.subRows?.push({
        name: item.name,
        weight: item.weight,
        calories: item.calories,
        notes: item.notes,
        url: item.url,
      });
    }
  });

  return Array.from(map.values());
}