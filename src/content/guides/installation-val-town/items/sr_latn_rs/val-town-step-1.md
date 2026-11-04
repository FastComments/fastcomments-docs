Widget je script tag i kontejnerski element, tako da se ubacuje u bilo šta što vaš val već renderuje. Ovaj primer koristi Hono JSX, što je ono što Val Town-ovi HTTP šabloni koriste.

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

## Izaberite urlId pre objavljivanja

`urlId` određuje u koji thread komentari upadaju. Ako ga ostavite nedefinisan, podrazumevano se koristi očišćena verzija URL-a trenutne stranice, što je upravo ono što se menja na Val Town-u: val ima dugačak `*.web.val.run` hostname dok ne preuzmete subdomenu, branch-ovi dobijaju svoje URL-ove, a preimenovanje stranice menja putanju. Svaka varijacija tiho postaje poseban, prazan thread, a simptom se čita kao "moji komentari su nestali".

Postavite ga na nešto stabilno što kontrolišete, kao što je slug objave ili database id, kao u primeru iznad. Prosledite i `url`, kako bi notifikacije mejlom i moderacioni alati mogli da linkuju ka pravoj stranici.

## Zadržavanje komentara bez JavaScripta

FastComments renderuje ceo thread server-side, što val može da ubaci u `<noscript>` blok:

[inline-code-attrs-start title = 'Fallback bez JavaScripta'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

URL-enkodirajte parametre. Server-side verzija podržava anonimno i prijavljeno komentiranje, SSO i ugnježdene odgovore.