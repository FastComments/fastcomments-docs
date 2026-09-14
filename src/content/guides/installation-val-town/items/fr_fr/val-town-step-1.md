Le widget est une balise script et un élément conteneur, il s’insère donc où que votre val rende déjà. Cet exemple utilise Hono JSX, qui est ce que les modèles HTTP de Val Town utilisent.

[inline-code-attrs-start title = 'Widget de commentaire dans un val HTTP'; type='javascript' inline-code-attrs-end]
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

## Choisissez un urlId avant de déployer

`urlId` détermine dans quel fil un commentaire atterrit. Laissez‑le non défini et il prendra par défaut une version nettoyée de l’URL de la page actuelle, ce qui correspond exactement à ce qui change sur Val Town : un val possède un long nom d’hôte `*.web.val.run` jusqu’à ce que vous réclamiez un sous‑domaine, les branches obtiennent leurs propres URL, et le renommage d’une page modifie le chemin. Chaque variation devient silencieusement un fil séparé et vide, et le symptôme apparaît comme « mes commentaires ont disparu ».

Attribuez‑lui quelque chose de stable que vous contrôlez, comme le slug du post ou un identifiant de base de données, comme ci‑dessus. Transmettez également `url`, afin que les e‑mails de notification et les outils de modération puissent renvoyer à la vraie page.

## Conserver les commentaires sans JavaScript

FastComments rend un fil complet côté serveur, qu’un val peut placer dans un bloc `<noscript>` :

[inline-code-attrs-start title = 'Solution de secours sans JavaScript'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

Encodez les paramètres dans l’URL. La version côté serveur prend en charge les commentaires anonymes et connectés, le SSO et les réponses imbriquées.