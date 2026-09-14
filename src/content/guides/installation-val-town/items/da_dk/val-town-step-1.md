The widget er et script‑tag og et container‑element, så den falder ind i hvad din val allerede rendererer. Dette eksempel bruger Hono JSX, som er hvad Val Towns HTTP‑skabeloner bruger.

[inline-code-attrs-start title = 'Kommentar-widget i en HTTP val'; type='javascript' inline-code-attrs-end]
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

## Vælg et urlId før du udgiver

`urlId` bestemmer, hvilken tråd en kommentar lander i. Lad den stå uindstillet, så falder den tilbage på en renset version af den aktuelle side‑URL, hvilket er præcis det, der ændrer sig på Val Town: en val har et langt `*.web.val.run`‑hostname indtil du gør krav på et subdomæne, grene får deres egne URL’er, og omdøbning af en side ændrer stien. Hver variation bliver stiltiende en separat, tom tråd, og symptomet læses som “mine kommentarer forsvandt”.

Sæt den til noget stabilt, som du kontrollerer, f.eks. post‑slug’en eller en database‑id, som ovenfor. Send også `url` med, så notifikations‑e‑mails og moderationsværktøjerne kan linke tilbage til den rigtige side.

## Bevare kommentarer uden JavaScript

FastComments rendererer en fuld tråd på server‑siden, som en val kan placere i en `<noscript>`‑blok:

[inline-code-attrs-start title = 'Ingen JavaScript-tilbagefald'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

URL‑kod de parametre. Server‑side‑versionen understøtter anonyme og loggede kommentarer, SSO og indlejrede svar.