O widget é uma tag de script e um elemento contêiner, portanto ele se insere em qualquer coisa que seu val já renderiza. Este exemplo usa Hono JSX, que é o que os templates HTTP do Val Town utilizam.

[inline-code-attrs-start title = 'Widget de comentário em um val HTTP'; type='javascript' inline-code-attrs-end]
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

## Escolha um urlId antes de enviar

`urlId` decide em qual thread um comentário será colocado. Se deixá-lo sem definição, ele padrão para uma versão limpa da URL da página atual, que é exatamente o que muda no Val Town: um val tem um hostname longo `*.web.val.run` até que você reivindique um subdomínio, ramificações recebem suas próprias URLs, e renomear uma página altera o caminho. Cada variação silenciosamente se torna um thread separado e vazio, e o sintoma aparece como "meus comentários desapareceram".

Defina-o para algo estável que você controla, como o slug da postagem ou um ID de banco de dados, como acima. Também passe `url`, para que os e‑mails de notificação e as ferramentas de moderação possam linkar de volta à página real.

## Mantendo comentários sem JavaScript

FastComments renderiza um thread completo no lado do servidor, que um val pode inserir em um bloco `<noscript>`:

[inline-code-attrs-start title = 'Fallback sem JavaScript'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

Codifique os parâmetros em URL. A versão server‑side suporta comentários anônimos e autenticados, SSO e respostas aninhadas.