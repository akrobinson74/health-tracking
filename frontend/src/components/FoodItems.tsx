import React, { useCallback, useEffect, useState } from "react";
import {
  getFoodItems,
  getFoodItemsByDate,
} from "../services/foodItemDataFetcher";
import { FoodItem, NestedFoodItem, nestFoodItems } from "../models/dataTypes";
import { useParams } from "react-router-dom";
import { testFoodItems } from "./testData";
import DataTable from "./DataTable";
import { ColumnDef, createColumnHelper } from "@tanstack/react-table";

const columnHelper = createColumnHelper<FoodItem>();

const columns: ColumnDef<FoodItem, any>[] = [
  columnHelper.accessor("date", {
    header: "Date",
    id: "date",
  }),
  columnHelper.accessor("time", {
    header: "Time",
    id: "time",
  }),
  columnHelper.accessor("name", {
    header: "Name",
    id: "name",
  }),
  columnHelper.accessor("weight", {
    header: "Weight",
    cell: (info) => info.getValue().toFixed(2),
  }),
  columnHelper.accessor("calories", {
    header: "Calories",
    cell: (info) => info.getValue().toFixed(2),
  }),
  columnHelper.accessor("notes", {
    header: "Notes",
    id: "notes",
  }),
  columnHelper.accessor("url", {
    header: "URL",
    id: "url",
  }),
];

const nestedColumnHelper = createColumnHelper<NestedFoodItem>();
const nestedColumns: ColumnDef<NestedFoodItem, any>[] = [
  nestedColumnHelper.accessor("date", {
    header: "Date",
    id: "date",
  }),
  nestedColumnHelper.accessor("time", {
    header: "Time",
    id: "time",
  }),
];

const FoodItems = () => {
  const params = useParams();
  const date = params.date || "";
  const [data, setData] = useState<Array<FoodItem>>([]);
  const [nestedData, setNestedData] = useState<Array<NestedFoodItem>>([]);

  const loadData = useCallback(async () => {
    let foodItems: FoodItem[] = [];

    try {
      if (date !== "") {
        foodItems = await getFoodItemsByDate(date);
        console.log("Food items by date:", JSON.stringify(foodItems));
      } else {
        foodItems = await getFoodItems();
      }
    } catch (error) {
      console.error("Failed to load food items:", error);
      foodItems = testFoodItems;
    }
    setData(foodItems);
    setNestedData(nestFoodItems(foodItems));
    console.log("Nested food items:", JSON.stringify(nestFoodItems(foodItems)));
  }, [date]);

  useEffect(() => {
    loadData();
  }, [loadData]);

  if (data.length !== 0) {
    return (
      <div>
        <DataTable data={data} columns={columns} />
      </div>
    );
  } else {
    return <div>Loading...</div>;
  }
};

export default FoodItems;
