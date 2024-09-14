import React from "react";
import styles from "./Button.module.css";
import Link from "next/link";

enum Colorscheme {
  Light,
  Dark,
}

interface ButtonProps {
  children: React.ReactNode;
  href: string;
  outline?: boolean;
  colorscheme?: Colorscheme;
}

const Button: React.FC<ButtonProps> = ({
  children,
  href,
  outline = false,
  colorscheme = Colorscheme.Light,
}) => {
  return (
    <Link
      href={href}
      className={`${styles.button} ${outline ? styles.outline : ""} ${getColorscheme(colorscheme)}`}
    >
      {children}
    </Link>
  );
};

export default Button;

function getColorscheme(colorscheme: Colorscheme) {
  switch (colorscheme) {
    case Colorscheme.Light:
      return styles.light;
    case Colorscheme.Dark:
      return styles.dark;
  }
}
