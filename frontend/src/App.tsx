import React from "react";
import { BrowserRouter as Router, Link, Route, Routes } from "react-router-dom";
import FoodItems from "./components/FoodItems";

function App() {
  return (
    <Router>
      <div>
        <nav>
          <ul>
            <li>
              <Link to="/foodItems">All Food Items</Link>
            </li>
            <li>
              <Link to="/foodItemsByDate/2024-11-12">
                Food Items by Date (2024-11-12)
              </Link>
            </li>
          </ul>
        </nav>
      </div>
      <div>
        <Routes>
          <Route path="/foodItems" element={<FoodItems />} />
          <Route path="/foodItemsByDate/:date" element={<FoodItems />} />
        </Routes>
      </div>
    </Router>
  );
}

export default App;
