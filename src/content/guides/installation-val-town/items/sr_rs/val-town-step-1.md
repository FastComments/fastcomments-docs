Виџет је script таг и контејнер елемент, тако да се уклапа у било шта ваш val већ приказује. Овај пример користи Hono JSX, који користе Val Town HTTP шаблони.

[inline-code-attrs-start title = 'Коментар виџет у HTTP val-у'; type='javascript' inline-code-attrs-end]
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

## Изаберите urlId пре објављивања

`urlId` одређује у којој нити се коментар налази. Ако га не подесите, подразумевано се користи очишћена верзија URL-а тренутне странице, што је управо оно што се мења на Val Town: val има дугачак `*.web.val.run` hostname док не заузмете subdomain, гране добијају сопствене URL-ове, а преименовање странице мења путању. Свака варијација тихо постаје посебна, празна нит, а симптом изгледа као „моји коментари су нестали".

Поставите га на нешто стабилно што контролишете, као што је slug објаве или id из базе података, као у примеру изнад. Проследите и `url`, како би имејл нотификације и алати за модерацију могли да повежу на праву страницу.

## Задржавање коментара без JavaScript-а

FastComments приказује комплетну нит на серверу, што val може убацивати у `<noscript>` блок:

[inline-code-attrs-start title = 'Резервна верзија без JavaScript-а'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

URL-енкодирајте параметре. Серверска верзија подржава анонимно и пријављено коментарисање, SSO и угнежђене одговоре.