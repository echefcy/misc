import styles from "../styles/Controls.module.css";
import { useState } from "react"

//TODO: what props do i pass into here?
const heaterControl = ({heaterValue, setHeaterValue}) => {

  return (
    <div className={styles.heatControl}>
      <h2> Heater controls</h2>
      <input type="range" min="0" max="100" value={heaterValue} onChange = {(e) => setHeaterValue(Number(e.target.value))} id="heater"></input>{" "}
      {/* //TODO:show the heat level here */}
      <span>{heaterValue}</span>
    </div>
  );
};

export default heaterControl;