import { Route, Routes } from "react-router-dom";
import Home from "./Home";

function ProjectRoutes() {
  return (
    <Routes>
      <Route path="/" element={<Home />} />
    </Routes>
  );
}

export default ProjectRoutes;
