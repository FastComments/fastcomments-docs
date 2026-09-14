The widget is a script tag and a container element, so it drops into whatever your val already renders. This example uses Hono JSX, which is what Val Town's HTTP templates use.

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

## Elija un urlId antes de lanzar

`urlId` decide en qué hilo cae un comentario. Déjelo sin establecer y por defecto será una versión limpiada de la URL de la página actual, que es exactamente lo que cambia en Val Town: un val tiene un hostname largo `*.web.val.run` hasta que reclama un subdominio, las ramas obtienen sus propias URLs, y renombrar una página cambia la ruta. Cada variación se convierte silenciosamente en un hilo separado y vacío, y el síntoma se lee como “mis comentarios desaparecieron”.

Establézcalo a algo estable que usted controle, como el slug de la publicación o un id de base de datos, como se muestra arriba. Pase también `url`, para que los correos electrónicos de notificación y las herramientas de moderación puedan enlazar de vuelta a la página real.

## Mantener comentarios sin JavaScript

FastComments renderiza un hilo completo del lado del servidor, que un val puede colocar dentro de un bloque `<noscript>`:

[inline-code-attrs-start title = 'Alternativa sin JavaScript'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

Codifique los parámetros en URL. La versión del lado del servidor admite comentarios anónimos y con sesión iniciada, SSO y respuestas anidadas.