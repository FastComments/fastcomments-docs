The widget is a script tag and a container element, so it drops into whatever your val already renders. This example uses Hono JSX, which is what Val Town's HTTP templates use.

[inline-code-attrs-start title = 'Widget dei commenti in un valore HTTP'; type='javascript' inline-code-attrs-end]
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

`urlId` decide in quale thread atterra un commento. Se lo lasci non impostato, di default usa una versione pulita dell'URL della pagina corrente, che è esattamente la cosa che cambia su Val Town: un val ha un hostname lungo `*.web.val.run` finché non rivendichi un sottodominio, i rami ottengono i propri URL e rinominare una pagina cambia il percorso. Ogni variazione diventa silenziosamente un thread separato e vuoto, e il sintomo appare come "i miei commenti sono scomparsi".

Impostalo su qualcosa di stabile che controlli, come lo slug del post o un ID del database, come mostrato sopra. Passa anche `url`, così le email di notifica e gli strumenti di moderazione possono collegarsi alla pagina reale.

## Mantenere i commenti senza JavaScript

FastComments rende un thread completo lato server, che un val può inserire in un blocco `<noscript>`:

[inline-code-attrs-start title = 'Fallback senza JavaScript'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

Codifica URL i parametri. La versione lato server supporta i commenti anonimi e autenticati, SSO e risposte nidificate.

---