The widget is a script tag and a container element, so it drops into whatever your val already renders. This example uses Hono JSX, which is what Val Town's HTTP templates use.

[inline-code-attrs-start title = 'Widget za komentare u HTTP val-u'; type='javascript' inline-code-attrs-end]
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

## Izaberite urlId pre nego što objavite

`urlId` određuje u koju nit komentar ulazi. Ako ga ostavite nepostavljenim, podrazumeva se očišćena verzija trenutnog URL-a stranice, što je upravo ono što se menja na Val Town-u: val ima dugačko `*.web.val.run` ime hosta dok ne rezervišete poddomen, grane dobijaju svoje URL-ove, a preimenovanje stranice menja put. Svaka varijacija tiho postaje zasebna, prazna nit, i simptom se manifestuje kao „moji komentari su nestali“.

Postavite ga na nešto stabilno što vi kontrolišete, poput slug-a posta ili ID-a iz baze podataka, kao što je gore prikazano. Takođe prosledite `url`, kako bi e‑mail obaveštenja i alati za moderaciju mogli da se povežu nazad na pravu stranicu.

## Čuvanje komentara bez JavaScripta

FastComments renderuje kompletnu nit na serveru, što val može ubaciti u `<noscript>` blok:

[inline-code-attrs-start title = 'Fallback bez JavaScripta'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

URL‑enkodujte parametre. Server‑side verzija podržava anonimno i prijavljeno komentarisanje, SSO i ugnježdene odgovore.

---