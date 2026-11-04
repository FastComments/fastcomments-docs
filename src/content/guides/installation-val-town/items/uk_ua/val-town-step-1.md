Віджет — це тег script та елемент-контейнер, тому він вписується в те, що ваш val вже рендерить. Цей приклад використовує Hono JSX, який використовується в HTTP-шаблонах Val Town.

[inline-code-attrs-start title = 'Віджет коментарів у HTTP val'; type='javascript' inline-code-attrs-end]
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

## Виберіть urlId перед публікацією

`urlId` визначає, у яку гілку потрапить коментар. Якщо його не встановити, за замовчуванням використовується очищена версія URL поточної сторінки, а саме це змінюється на Val Town: val має довгий ім'я хоста `*.web.val.run`, доки ви не прив'яжете субдомен, гілки отримують власні URL, а перейменування сторінки змінює шлях. Кожна варіація мовчки стає окремою порожньою гілкою, і симптом виглядає як «мої коментарі зникли».

Встановіть щось стабільне, що ви контролюєте, наприклад slug публікації або ідентифікатор бази даних, як вище. Передайте також `url`, щоб листи сповіщень та інструменти модерації могли посилатися на реальну сторінку.

## Збереження коментарів без JavaScript

FastComments рендерить повну гілку на сервері, і val може вставити її в блок `<noscript>`:

[inline-code-attrs-start title = 'Резервний варіант без JavaScript'; type='html' inline-code-attrs-end]
[inline-code-start]
<noscript>
  <iframe src="https://fastcomments.com/ssr/comments?tenantId=demo&urlId=POST_SLUG&url=PAGE_URL"
          title="FastComments" width="100%" height="1500px" frameborder="0"
          style="width: 1px !important; min-width: 100% !important; border: none !important;"></iframe>
</noscript>
[inline-code-end]

Кодуйте параметри у форматі URL. Серверна версія підтримує анонімні та авторизовані коментарі, SSO та вкладені відповіді.