Виджет — это тег script и элемент-контейнер, поэтому он встраивается в то, что ваш val уже рендерит. В этом примере используется Hono JSX — именно его применяют HTTP-шаблоны Val Town.

[inline-code-attrs-start title = 'Виджет комментариев в HTTP val'; type='javascript' inline-code-attrs-end]
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

## Выберите urlId перед публикацией

`urlId` определяет, в какую ветку попадёт комментарий. Если оставить его неустановленным, по умолчанию используется очищенная версия URL текущей страницы — а именно это меняется на Val Town: val имеет длинный hostname `*.web.val.run`, пока вы не закрепите поддомен, ветки получают собственные URL, а переименование страницы меняет путь. Каждая вариация молча становится отдельной пустой веткой, и симптом выглядит как «мои комментарии исчезли».

Задайте стабильное значение, которое вы контролируете, например slug поста или id из базы данных, как показано выше. Также передайте `url`, чтобы письма с уведомлениями и инструменты модерации могли ссылаться на реальную страницу.

## Сохранение комментариев без JavaScript

FastComments рендерит полную ветку на стороне сервера, и val может вставить её в блок `<noscript>`:

[inline-code-attrs-start title = 'Резервный вариант без JavaScript'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

Кодируйте параметры в URL-формате. Серверная версия поддерживает анонимное комментирование и комментирование под залогиненным пользователем, SSO и вложенные ответы.