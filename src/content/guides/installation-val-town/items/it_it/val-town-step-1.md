Il widget è uno script tag e un elemento contenitore, quindi si inserisce in qualsiasi cosa il tuo val già renderizzi. Questo esempio usa Hono JSX, che è ciò che usano i template HTTP di Val Town.

[inline-code-attrs-start title = 'Widget dei commenti in un HTTP val'; type='javascript' inline-code-attrs-end]
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

## Scegli un urlId prima di pubblicare

`urlId` determina in quale thread finirà un commento. Se lo lasci non impostato, assume come valore predefinito una versione ripulita dell'URL della pagina corrente, che è esattamente la cosa che cambia su Val Town: un val ha un hostname lungo `*.web.val.run` finché non rivendichi un sottodominio, i branch ottengono URL propri, e rinominare una pagina cambia il percorso. Ogni variazione diventa silenziosamente un thread separato e vuoto, e il sintomo si manifesta come "i miei commenti sono spariti".

Impostalo su qualcosa di stabile che controlli tu, come lo slug del post o un id del database, come sopra. Passa anche `url`, così le email di notifica e gli strumenti di moderazione possono collegarsi alla pagina reale.

## Mantenere i commenti senza JavaScript

FastComments renderizza un thread completo lato server, che un val può inserire in un blocco `<noscript>`:

[inline-code-attrs-start title = 'Fallback senza JavaScript'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

Codifica i parametri in URL. La versione lato server supporta commenti anonimi e da utenti autenticati, SSO e risposte annidate.