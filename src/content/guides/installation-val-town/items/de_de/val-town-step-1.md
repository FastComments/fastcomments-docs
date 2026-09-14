The widget is a script tag and a container element, so it drops into whatever your val already renders. This example uses Hono JSX, which is what Val Town's HTTP templates use.

[inline-code-attrs-start title = 'Kommentar-Widget in einem HTTP-Val'; type='javascript' inline-code-attrs-end]
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

## Wählen Sie eine urlId, bevor Sie veröffentlichen

`urlId` entscheidet, in welchem Thread ein Kommentar landet. Lassen Sie es unverändert, wird es standardmäßig auf eine bereinigte Version der aktuellen Seiten‑URL gesetzt, was genau das ist, was sich bei Val Town ändert: Ein Val hat einen langen `*.web.val.run` Hostnamen, bis Sie eine Subdomain beanspruchen, Zweige erhalten eigene URLs, und das Umbenennen einer Seite ändert den Pfad. Jede Variation wird stillschweigend zu einem separaten, leeren Thread, und das Symptom lautet „meine Kommentare sind verschwunden“.

Setzen Sie es auf etwas Stabiles, das Sie kontrollieren, wie den Beitrags‑Slug oder eine Datenbank‑ID, wie oben gezeigt. Übergeben Sie auch `url`, damit Benachrichtigungs‑E‑Mails und die Moderations‑Tools zurück zur echten Seite verlinken können.

## Kommentare ohne JavaScript beibehalten

FastComments rendert einen vollständigen Thread serverseitig, den ein Val in einen `<noscript>`‑Block einfügen kann:

[inline-code-attrs-start title = 'Kein-JavaScript-Fallback'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

URL‑kodieren Sie die Parameter. Die serverseitige Version unterstützt anonyme und angemeldete Kommentare, SSO und verschachtelte Antworten.