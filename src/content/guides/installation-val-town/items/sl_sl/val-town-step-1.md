Widget je oznaka skripta in vsebnik, zato se umesti v karkoli že izrisuje vaš val. Ta primer uporablja Hono JSX, kar uporabljajo tudi HTTP predloge Val Town.

[inline-code-attrs-start title = 'Komentarski widget v HTTP valu'; type='javascript' inline-code-attrs-end]
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

`urlId` določa, v katero nit pride komentar. Če ga pustite nenastavljen, se privzeto uporabi očiščena različica URL-ja trenutne strani, kar je točno tisto, kar se na Val Town spreminja: val ima dolgo ime gostitelja `*.web.val.run`, dokler ne prevzamete poddomene, veje dobijo svoje URL-je, preimenovanje strani pa spremeni pot. Vsaka različica tiho postane ločena, prazna nit, simptom pa se kaže kot »moji komentarji so izginili«.

Nastavite ga na nekaj stabilnega, kar nadzorujete, na primer slug prispevka ali ID v podatkovni zbirki, kot zgoraj. Podajte tudi `url`, da se e-poštna obvestila in orodja za moderiranje lahko povežejo nazaj na pravo stran.

## Ohranjanje komentarjev brez JavaScripta

FastComments izrisuje celotno nit na strani strežnika, kar val lahko vstavi v blok `<noscript>`:

[inline-code-attrs-start title = 'Rezerva brez JavaScripta'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

Parametre kodirajte v URL. Strežniška različica podpira anonimno in prijavljeno komentiranje, SSO ter ugnezdene odgovore.