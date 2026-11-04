Das Widget ist ein Script-Tag und ein Container-Element, daher lässt es sich in alles einfügen, was dein Val bereits rendert. Dieses Beispiel verwendet Hono JSX, was auch die HTTP-Templates von Val Town nutzen.

[inline-code-attrs-start title = 'Kommentar-Widget in einem HTTP val'; type='javascript' inline-code-attrs-end]
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

## Wähle eine urlId, bevor du deployst

`urlId` entscheidet, in welchem Thread ein Kommentar landet. Lässt du es unset, wird standardmäßig eine bereinigte Version der aktuellen Seiten-URL verwendet – genau das, was sich auf Val Town ändert: Ein Val hat einen langen `*.web.val.run` Hostnamen, bis du eine Subdomain beanspruchst, Branches erhalten eigene URLs, und das Umbenennen einer Seite ändert den Pfad. Jede Variante wird stillschweigend zu einem separaten, leeren Thread, und das Symptom liest sich als „meine Kommentare sind verschwunden".

Setze es auf etwas Stabiles, das du kontrollierst, wie den Post-Slug oder eine Datenbank-ID, wie oben. Übergib auch `url`, damit Benachrichtigungs-E-Mails und die Moderations-Tools auf die echte Seite verlinken können.

## Kommentare ohne JavaScript beibehalten

FastComments rendert einen vollständigen Thread serverseitig, den ein Val in einen `<noscript>`-Block einfügen kann:

[inline-code-attrs-start title = 'Fallback ohne JavaScript'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

URL-encode die Parameter. Die serverseitige Version unterstützt anonymes und eingeloggtes Kommentieren, SSO sowie verschachtelte Antworten.