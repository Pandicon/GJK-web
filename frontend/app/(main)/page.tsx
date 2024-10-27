import Image from "next/image";
import { partnerLogos } from "@/public/logos/";
import headerImage from "@/public/images/header.jpeg";
import { Button } from "@/components/ui/button";
import { getAspectRatio } from "@/lib/utils";
import { getArticles } from "@/lib/actions";
import Link from "next/link";

export default async function Home() {
  const articles = await getArticles(0);
  return (
    <>
      <section className="grid grid-cols-2 items-center max-w-screen-xl mx-auto gap-8">
        <div>
          <h1>Podporujeme rozvoj každého studenta</h1>
          <p className="mb-10">
            Naše výuka podporuje kreativitu, kritické myšlení a zodpovědnost v
            přátelském a podporujícím prostředí. Je to místem, kde se setkávají
            nadaní studenti, kteří společně objevují a rozvíjejí svůj potenciál,
            dosahují vynikajících výsledků a osobního růstu.
          </p>
          <div className="flex gap-4">
            <Button asChild>
              <Link href="#about">ZJISTIT VÍCE</Link>
            </Button>
            <Button variant="outline">PŘIJMACÍ ŘÍZENÍ</Button>
          </div>
        </div>
        <Image
          src={headerImage}
          alt=""
          className="rounded-lg aspect-square object-cover"
        />
      </section>
      <section className="max-w-screen-xl mx-auto py-16">
        <h2 className="mb-2">Aktuality</h2>
        <div className="flex flex-col gap-4 mb-8">
          {articles.slice(0, 3).map((a) => (
            <article key={a.id} className="border p-4 rounded-lg">
              <h3>{a.title}</h3>
              <p>{a.content}</p>
            </article>
          ))}
        </div>
        <Button>VŠECHNY AKTUALITY</Button>
      </section>
      <section
        id="about"
        className="grid grid-cols-2 items-center max-w-screen-xl mx-auto gap-8 py-8"
      >
        <Image
          src={headerImage}
          alt=""
          className="object-cover aspect-square"
        />
        <div>
          <h2>O škole</h2>
          <p className="mb-10">
            Gymnázium Jana Keplera je moderní střední škola zaměřená na rozvoj
            talentů a silných stránek každého studenta. Naše výuka podporuje
            kreativitu, kritické myšlení a zodpovědnost v přátelském a
            podporujícím prostředí. Studenti zde mohou rozvíjet své schopnosti a
            aktivně se zapojovat do života školy i širší komunity. Je to místem,
            kde se setkávají nadaní studenti, kteří objevují a rozvíjejí svůj
            potenciál, dosahují vynikajících výsledků a osobního růstu.
          </p>
          <Button>ŠVP</Button>
        </div>
      </section>
      <section className="max-w-screen-xl mx-auto py-10 grayscale flex justify-between gap-8">
        {partnerLogos.map((logo, index) => (
          <Image
            src={logo}
            alt=""
            key={index}
            style={{ flex: getAspectRatio(logo) }}
            className="w-full h-auto"
          />
        ))}
      </section>
    </>
  );
}
