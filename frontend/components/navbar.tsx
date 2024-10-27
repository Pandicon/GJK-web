import React from "react";
import Link from "next/link";
import Image from "next/image";
import { schoolLogo } from "@/public/logos";
import eSborovnaIcon from "@/public/icons/fa-solid--chalkboard-teacher.svg";
import { MagnifyingGlassIcon } from "@radix-ui/react-icons";
import { Button } from "@/components/ui/button";
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  NavigationMenuContent,
  NavigationMenuTrigger,
  navigationMenuTriggerStyle,
} from "@/components/ui/navigation-menu";

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
  {
    name: "eSborovna",
    href: "https://sites.google.com/a/gjk.cz/sborovna",
  },
];

const Navbar = () => {
  return (
    <div className="max-w-screen-xl flex justify-between mx-auto py-4">
      <Link href="/">
        <Image src={schoolLogo} alt="Gymnázium Jana Keplera" />
      </Link>
      <NavigationMenu>
        <NavigationMenuList>
          {navLinks.map((group, index) => {
            if (group.children != null && group.children.length > 0) {
              return (
                <NavigationMenuItem key={index}>
                  <NavigationMenuTrigger>{group.name}</NavigationMenuTrigger>
                  <NavigationMenuContent>
                    <ul className="grid w-96 p-4">
                      {group.children.map((navLink, index) => (
                        <NavigationMenuLink href={navLink.href} key={index}>
                          {navLink.title}
                        </NavigationMenuLink>
                      ))}
                    </ul>
                  </NavigationMenuContent>
                </NavigationMenuItem>
              );
            } else {
              return (
                <NavigationMenuItem key={index}>
                  <Link href={group.href ?? ""} passHref legacyBehavior>
                    <NavigationMenuLink
                      className={navigationMenuTriggerStyle()}
                    >
                      {group.name}
                    </NavigationMenuLink>
                  </Link>
                </NavigationMenuItem>
              );
            }
          })}
        </NavigationMenuList>
      </NavigationMenu>
      <div>
        <Button variant="ghost" size="icon">
          <MagnifyingGlassIcon />
        </Button>
      </div>
    </div>
  );
};

export default Navbar;
