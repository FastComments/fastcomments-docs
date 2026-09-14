The widget is a script tag and a container element, so it drops into whatever your val already renders. This example uses Hono JSX, which is what Val Town's HTTP templates use.

[inline-code-attrs-start title = 'Видгет коментара у HTTP валу'; type='javascript' inline-code-attrs-end]
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

## Одаберите urlId пре него што објавите

`urlId` одређује у који се разговор (тема) ставља коментар. Ако га оставите непостављеним, подразумева се чиста верзија тренутног URL‑а странице, што је управо оно што се мења на Val Town: вал има дуго `*.web.val.run` име хоста док не заузмете поддомени, гране добијају сопствене URL‑ове, а преименовање странице мења путању. Свака варијација тихо постаје одвојена, празна тема, а симптом се манифестује као „моји коментари су нестали“.

Поставите га на нешто стабилно што контролишете, као што је slug поста или ID у бази података, као у горе наведеном. Проследите и `url`, како би е‑мејлови за обавештење и алати за модерацију могли да врате везу ка стварној страници.

## Чување коментара без JavaScript‑а

FastComments рендерује целу тему на серверу, што вал може да убаци у `<noscript>` блок:

[inline-code-attrs-start title = 'Резервно решење без JavaScript-а'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

URL‑кодујте параметре. Верзија на серверу подржава анонимно и пријављено коментарисање, SSO и угнежђене одговоре.