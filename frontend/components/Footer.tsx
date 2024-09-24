import React from "react";
import styles from "./Footer.module.css";

const Footer = () => {
  return (
    <footer className={`bg-blue ${styles.footer}`}>
      <div className={`content-max-width ${styles.footerContent}`}>
        <h3>Gymnázium Jana Keplera</h3>
        <div className={styles.footerLinks}>
          <div>
            <span className={styles.title}> Kontakt</span>
            <ul>
              <li>Parléřova 2, 169 00, Praha 6</li>
              <li>+420 233 352 546</li>
              <li>
                <a href="mailto:gjk@gjk.cz">gjk@gjk.cz</a>
              </li>
            </ul>
          </div>
          <div>
            <span className={styles.title}>Zřizovatel</span>
            <ul>
              <li>Hlavní město Praha</li>
            </ul>
          </div>
          <div>
            <span className={styles.title}>Informace</span>
            <ul>
              <li>GDPR</li>
              <li>Informace o povinném subjektu</li>
            </ul>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
