Widget je script tag i container element, pa se uklapa u ono što vaš val već renderira. Ovaj primjer koristi Hono JSX, što koriste Val Town HTTP predlošci.

[inline-code-attrs-start title = 'Comment widget u HTTP valu'; type='javascript' inline-code-attrs-end]
[inline-code-start]
/** @jsxImportSource npm:hono@4/jsx */
import { Hono } from "npm:hono@4";

const app = new Hono();

app.get("/:slug", (c) => {
  const slug = c.req.param("slug");
  const url = new URL(c.req.path, c.req.url).toString();

  const config = JSON.stringify({
    tenantId: "demo",
    urlId: slug,
    url,
  });

  return c.html(
    <html>
      <body>
        <h1>{slug}</h1>
        <div id="fastcomments-widget"></div>
        <script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
        <script
          dangerouslySetInnerHTML={{
            __html:
              `window.FastCommentsUI(document.getElementById("fastcomments-widget"), ${config});`,
          }}
        />
      </body>
    </html>,
  );
});

export default app.fetch;
[inline-code-end]

## Odaberite urlId prije objave

`urlId` određuje u koju nit komentari padaju. Ako ga ostavite nedefiniranim, zadano se koristi očišćena verzija URL-a trenutne stranice, što je upravo ono što se mijenja na Val Town: val ima dugački `*.web.val.run` hostname dok ne zatražite subdomenu, grane dobivaju vlastite URL-ove, a preimenovanje stranice mijenja putanju. Svaka varijacija tiho postaje zasebna, prazna nit, a simptom se čita kao "moji su komentari nestali".

Postavite ga na nešto stabilno što kontrolirate, poput sluga objave ili ID-a iz baze podataka, kao gore. Proslijedite i `url`, kako bi notifikacijski emailovi i moderacijski alati mogli povezati natrag na stvarnu stranicu.

## Zadržavanje komentara bez JavaScripta

FastComments renderira cijelu nit server-side, što val može ubaciti u `<noscript>` blok:

[inline-code-attrs-start title = 'Fallback bez JavaScripta'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

URL-enkodirajte parametre. Server-side verzija podržava anonimno i prijavljeno komentiranje, SSO i ugniježđene odgovore.