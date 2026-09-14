The widget is a script tag and a container element, so it drops into whatever your val already renders. This example uses Hono JSX, which is what Val Town's HTTP templates use.

[inline-code-attrs-start title = 'Komentarski gradnik v HTTP valu'; type='javascript' inline-code-attrs-end]
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

## Izberite urlId pred objavo

`urlId` določa, v kateri nit komentar pristane. Če ga ne nastavite, se privzeto uporabi očiščena različica trenutnega URL-ja strani, kar je natanko tisto, kar se spreminja v Val Townu: val ima dolgo ime gostitelja `*.web.val.run`, dokler ne zahtevate poddomene, veje dobijo svoje URL-je, in preimenovanje strani spremeni pot. Vsaka različica tiho postane ločena, prazna nit, in simptom se pojavi kot “moji komentarji so izginili”.

Nastavite ga na nekaj stabilnega, kar nadzorujete, kot je slug objave ali ID baze podatkov, kot je prikazano zgoraj. Posredujte tudi `url`, da lahko e‑maili s obvestili in orodja za moderiranje povežejo nazaj na pravo stran.

## Ohranjanje komentarjev brez JavaScripta

FastComments izriše celotno nit na strežniku, kar val lahko vstavi v blok `<noscript>`:

[inline-code-attrs-start title = 'Zasilna rešitev brez JavaScripta'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

Parametre URL‑kodirajte. Strežniška različica podpira anonimno in prijavljeno komentiranje, SSO ter gnezdene odgovore.