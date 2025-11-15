import React, { useCallback, useEffect, useState } from "react";
import {
  getFoodItems,
  getFoodItemsByDate,
} from "../services/foodItemDataFetcher";
import { FoodItem } from "../models/FoodItem";
import { useParams } from "react-router-dom";
import { AgGridReact } from "ag-grid-react";
import { AllCommunityModule, ColDef, ModuleRegistry } from "ag-grid-community";
import { testFoodItems } from "./testData";

ModuleRegistry.registerModules([AllCommunityModule]);

const toTwoDecimalPlaces = (params: any): string => {
  if (params.value === null || params.value === undefined) {
    return "";
  }
  return parseFloat(params.value).toFixed(2);
};

const colDefs: ColDef<FoodItem>[] = [
  { field: "date", headerName: "Date" },
  { field: "time", headerName: "Time" },
  { field: "name", headerName: "Name" },
  {
    cellStyle: { textAlign: "right" },
    field: "weight",
    headerClass: "ag-right-aligned-header",
    headerName: "Weight (g)",
    valueFormatter: toTwoDecimalPlaces,
  },
  {
    cellStyle: { textAlign: "right" },
    field: "calories",
    headerClass: "ag-right-aligned-header",
    headerName: "Calories",
    valueFormatter: toTwoDecimalPlaces,
  },
  { field: "notes", headerName: "Notes" },
  { field: "url", headerName: "URL" },
];

const FoodItems = () => {
  const params = useParams();
  const date = params.date || "";
  const gridRef = React.useRef<AgGridReact<FoodItem>>(null);
  const [data, setData] = useState<Array<FoodItem>>([]);

  const loadData = useCallback(async () => {
    let foodItems: FoodItem[] = [];

    try {
      if (date !== "") {
        foodItems = await getFoodItemsByDate(date);
      } else {
        foodItems = await getFoodItems();
      }
    } catch (error) {
      console.error("Failed to load food items:", error);
      foodItems = testFoodItems;
    }
    setData(foodItems);
  }, [date]);

  useEffect(() => {
    loadData();
  }, [loadData]);

  return (
    <div>
      <AgGridReact<FoodItem>
        columnDefs={colDefs}
        domLayout="autoHeight"
        pagination={true}
        paginationPageSize={25}
        paginationPageSizeSelector={[25, 50, 100]}
        ref={gridRef}
        rowData={data}
      />
    </div>
  );
};

export default FoodItems;
