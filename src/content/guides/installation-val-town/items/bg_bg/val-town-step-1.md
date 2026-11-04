Уиджетът е скрипт таг и контейнерен елемент, така че се вмъква в това, което вашият val вече рендира. Този пример използва Hono JSX, което е това, което HTTP шаблоните на Val Town използват.

[inline-code-attrs-start title = 'Коментар уиджет в HTTP val'; type='javascript' inline-code-attrs-end]
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

## Изберете urlId, преди да пуснете

`urlId` определя в коя нишка попада даден коментар. Ако го оставите непоправен, той по подразбиране е почистена версия на URL адреса на текущата страница, което е точно нещото, което се променя в Val Town: val има дълго `*.web.val.run` име на хост, докато не заявите поддомейн, клоновете получават собствени URL адреси, а преименуването на страница променя пътя. Всяка вариация тихо става отделна, празна нишка, а симптомът изглежда като „коментарите ми изчезнаха".

Задайте го на нещо стабилно, което контролирате, като slug на публикацията или идентификатор от база данни, както по-горе. Подайте и `url`, така че имейлите за известия и инструментите за модериране да могат да линкват обратно към реалната страница.

## Запазване на коментарите без JavaScript

FastComments рендира пълна нишка от страна на сървъра, която val може да вмъкне в блок `<noscript>`:

[inline-code-attrs-start title = 'Резервен вариант без JavaScript'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

Кодирайте параметрите с URL encoding. Версията от страна на сървъра поддържа анонимно коментиране и коментиране с влизане, SSO и вложени отговори.