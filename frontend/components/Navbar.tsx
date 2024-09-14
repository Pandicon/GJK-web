import React from "react";
import styles from "./Navbar.module.css";
import Link from "next/link";
import Image from "next/image";
import { schoolLogo } from "@/public/logos";
import eSborovnaIcon from "@/public/icons/fa-solid--chalkboard-teacher.svg";
import searchIcon from "@/public/icons/line-md--search.svg";
import * as NavigationMenu from "@radix-ui/react-navigation-menu";

type NavLink = {
  title: string;
  href?: string;
};

type NavItem = {
  name: string;
  href?: string;
  children?: Array<NavLink>;
};

const navLinks: Array<NavItem> = [
  {
    name: "Pro studenty",
    children: [
      { title: "Maturita" },
      { title: "Rozvrhy" },
      { title: "Suplování" },
      { title: "Volitelné předměty" },
      { title: "Bakaláři", href: "https://dochazka.gjk.cz/" },
    ],
  },
  {
    name: "Pro uchazeče",
    children: [
      { title: "Přijmací řízení" },
      { title: "Dny otevřených dveří" },
      { title: "Přestup na GJK" },
    ],
  },
  {
    name: "O škole",
    children: [
      { title: "Pedagogický sbor" },
      { title: "Dokumenty a formuláře" },
      { title: "Pronájmy" },
      { title: "Pracovní místa" },
    ],
  },
  {
    name: "Fotogalerie",
  },
];

const Navbar = () => {
  return (
    <div className={`${styles.navbar} content-max-width`}>
      <Link href="/">
        <Image
          src={schoolLogo}
          alt="Gymnázium Jana Keplera"
          className={styles.logo}
        />
      </Link>
      <NavigationMenu.Root className={styles.root}>
        <NavigationMenu.List className={styles.list}>
          {navLinks.map((group, index) => {
            if (group.children != null && group.children.length > 0) {
              return (
                <NavigationMenu.Item key={index} className={styles.item}>
                  <NavigationMenu.Trigger className={styles.trigger}>
                    {group.name}
                  </NavigationMenu.Trigger>
                  <NavigationMenu.Content className={styles.content}>
                    {group.children.map((navLink, index) => (
                      <NavigationMenu.Link
                        className={styles.link}
                        href={navLink.href}
                        key={index}
                      >
                        {navLink.title}
                      </NavigationMenu.Link>
                    ))}
                  </NavigationMenu.Content>
                </NavigationMenu.Item>
              );
            } else {
              return (
                <NavigationMenu.Item key={index} className={styles.item}>
                  <NavigationMenu.Link
                    href={group.href}
                    className={styles.link}
                  >
                    {group.name}
                  </NavigationMenu.Link>
                </NavigationMenu.Item>
              );
            }
          })}
        </NavigationMenu.List>
      </NavigationMenu.Root>
      <div className={styles.buttons}>
        <a href="https://sites.google.com/a/gjk.cz/sborovna/">
          <Image src={eSborovnaIcon} alt="" />
        </a>
        <Image src={searchIcon} alt="" />
      </div>
    </div>
  );
};

export default Navbar;
