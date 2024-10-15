import Image from "next/image";
import { partnerLogos } from "@/public/logos/";
import headerImage from "@/public/images/header.jpeg";
import styles from "./page.module.css";
import Button from "@/components/Button";
import { getAspectRatio } from "@/lib/utils";
import { getArticles } from "@/lib/actions";

export default async function Home() {
  const articles = await getArticles(0);
  return (
    <>
      <section
        className={`${styles.twoColumn} ${styles.hero} content-max-width`}
      >
        <div>
          <h1>
            Podporujeme rozvoj <span className={styles.highlight}>každého</span>{" "}
            studenta
          </h1>
          <p>
            Naše výuka podporuje kreativitu, kritické myšlení a zodpovědnost v
            přátelském a podporujícím prostředí. Je to místem, kde se setkávají
            nadaní studenti, kteří společně objevují a rozvíjejí svůj potenciál,
            dosahují vynikajících výsledků a osobního růstu.
          </p>
          <Button href="#about">ZJISTIT VÍCE</Button>
          <Button href="/" outline>
            PŘIJMACÍ ŘÍZENÍ
          </Button>
        </div>
        <Image src={headerImage} alt="" />
      </section>
      <section
        className={`bg-blue ${styles.articles} ${styles.sectionPadding}`}
      >
        <div className="content-max-width">
          <h2>Aktuality</h2>
          <div className={styles.articleWrapper}>
            {articles.slice(0, 3).map((a) => (
              <article key={a.id}>
                <h3>{a.title}</h3>
                <p>{a.content}</p>
              </article>
            ))}
          </div>
          <Button href="/" outline>
            VŠECHNY AKTUALITY
          </Button>
        </div>
      </section>
      <section
        id="about"
        className={`${styles.twoColumn} ${styles.sectionPadding} content-max-width`}
      >
        <Image src={headerImage} alt="" />
        <div>
          <h2>O škole</h2>
          <p>
            Gymnázium Jana Keplera je moderní střední škola zaměřená na rozvoj
            talentů a silných stránek každého studenta. Naše výuka podporuje
            kreativitu, kritické myšlení a zodpovědnost v přátelském a
            podporujícím prostředí. Studenti zde mohou rozvíjet své schopnosti a
            aktivně se zapojovat do života školy i širší komunity. Je to místem,
            kde se setkávají nadaní studenti, kteří objevují a rozvíjejí svůj
            potenciál, dosahují vynikajících výsledků a osobního růstu.
          </p>
          <Button href="https://sites.google.com/a/gjk.cz/svp/home" outline>
            ŠVP
          </Button>
        </div>
      </section>
      <section className={`${styles.partners}`}>
        <div className={`content-max-width ${styles.partnerLogos}`}>
          {partnerLogos.map((logo, index) => (
            <Image
              src={logo}
              alt=""
              key={index}
              style={{ flex: getAspectRatio(logo) }}
            />
          ))}
        </div>
      </section>
    </>
  );
}
