The widget is a script tag and a container element, so it drops into whatever your val already renders. This example uses Hono JSX, which is what Val Town's HTTP templates use.

[inline-code-attrs-start title = 'Widget za komentare u HTTP valu'; type='javascript' inline-code-attrs-end]
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

`urlId` određuje u koju nit komentar ulazi. Ako ga ne postavite, zadano je očišćena verzija trenutnog URL-a stranice, što je upravo ono što se mijenja na Val Townu: val ima dugo `*.web.val.run` ime hosta dok ne zatražite poddomen, grane dobivaju svoje URL-ove, a preimenovanje stranice mijenja put. Svaka varijacija tiho postaje zasebna, prazna nit, a simptom se očituje kao "moji komentari su nestali".

Postavite ga na nešto stabilno što kontrolirate, poput slug-a posta ili ID-a iz baze podataka, kao u gornjem primjeru. Proslijedite i `url`, kako bi e‑mail obavijesti i alati za moderaciju mogli voditi natrag na stvarnu stranicu.

## Zadržavanje komentara bez JavaScripta

FastComments prikazuje cijelu nit na poslužitelju, što val može ubaciti u `<noscript>` blok:

[inline-code-attrs-start title = 'Rezervna opcija bez JavaScripta'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

URL‑kodirajte parametre. Verzija na poslužitelju podržava anonimno i prijavljeno komentiranje, SSO i ugniježdene odgovore.