import React from "react";

const Footer = () => {
  return (
    <footer className="py-4 mx-auto max-w-screen-xl">
      <div className="flex justify-between">
        <h3>Gymnázium Jana Keplera</h3>
        <div className="flex gap-10">
          <div>
            <span className="font-semibold"> Kontakt</span>
            <ul>
              <li>Parléřova 2, 169 00, Praha 6</li>
              <li>+420 233 352 546</li>
              <li>
                <a href="mailto:gjk@gjk.cz">gjk@gjk.cz</a>
              </li>
            </ul>
          </div>
          <div>
            <span className="font-semibold">Zřizovatel</span>
            <ul>
              <li>Hlavní město Praha</li>
            </ul>
          </div>
          <div>
            <span className="font-semibold">Informace</span>
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
