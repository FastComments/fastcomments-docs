El widget es una etiqueta de script y un elemento contenedor, por lo que se integra en lo que tu val ya renderiza. Este ejemplo usa Hono JSX, que es lo que utilizan las plantillas HTTP de Val Town.

[inline-code-attrs-start title = 'Widget de comentarios en un val HTTP'; type='javascript' inline-code-attrs-end]
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

## Elige un urlId antes de publicar

`urlId` decide en qué hilo aterrizan los comentarios. Si lo dejas sin establecer, se toma por defecto una versión depurada de la URL de la página actual, que es exactamente lo que cambia en Val Town: un val tiene un nombre de host largo `*.web.val.run` hasta que reclamas un subdominio, las ramas obtienen sus propias URLs y renombrar una página cambia la ruta. Cada variación se convierte silenciosamente en un hilo separado y vacío, y el síntoma se manifiesta como "mis comentarios desaparecieron".

Establécelo en algo estable que tú controles, como el slug de la entrada o un id de base de datos, como en el ejemplo anterior. Pasa también `url`, para que los correos de notificación y las herramientas de moderación puedan enlazar a la página real.

## Mantener los comentarios sin JavaScript

FastComments renderiza un hilo completo del lado del servidor, que un val puede insertar en un bloque `<noscript>`:

[inline-code-attrs-start title = 'Alternativa sin JavaScript'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

Codifica los parámetros en URL. La versión del lado del servidor admite comentarios anónimos y de usuarios autenticados, SSO y respuestas anidadas.