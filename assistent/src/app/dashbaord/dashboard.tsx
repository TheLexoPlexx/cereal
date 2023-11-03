import { ReactNode } from "react";
import styles from "./dashboard.module.scss";

export function Dashboard(props: { children: ReactNode }) {
  return (
    <>{props.children}</>
  );
}