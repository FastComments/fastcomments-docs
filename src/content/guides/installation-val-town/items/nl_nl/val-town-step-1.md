The widget is een script‑tag en een container‑element, dus hij wordt ingevoegd waar jouw val al rendert. Dit voorbeeld gebruikt Hono JSX, wat de HTTP‑templates van Val Town gebruiken.

[inline-code-attrs-start title = 'Commentaarwidget in een HTTP val'; type='javascript' inline-code-attrs-end]
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

## Kies een urlId voordat je publiceert

`urlId` bepaalt in welke thread een opmerking terechtkomt. Laat je het leeg, dan wordt standaard een opgeschoonde versie van de huidige pagin URL gebruikt, wat precies het element is dat verandert op Val Town: een val heeft een lange `*.web.val.run` hostnaam totdat je een subdomein claimt, takken krijgen hun eigen URL’s, en het hernoemen van een pagina wijzigt het pad. Elke variatie wordt stilletjes een aparte, lege thread, en het symptoom leest als “mijn opmerkingen zijn verdwenen”.

Stel het in op iets stabiels dat je controleert, zoals de post‑slug of een database‑id, zoals hierboven. Geef ook `url` door, zodat notificatie‑e‑mails en de moderatietools kunnen linken naar de echte pagina.

## Reacties behouden zonder JavaScript

FastComments rendert een volledige thread server‑side, die een val kan invoegen in een `<noscript>`‑blok:

[inline-code-attrs-start title = 'Fallback zonder JavaScript'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

URL‑encode de parameters. De server‑side versie ondersteunt anonieme en ingelogde opmerkingen, SSO, en geneste antwoorden.