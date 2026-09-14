---
На странице индекса не рендерьте один виджет подсчёта комментариев на каждую строку. Это приводит к одному запросу на каждый пост. Используйте массовый подсчёт, который делает один запрос для всей страницы.

Отметьте каждую строку с помощью `urlId`, который использует её ветка, затем загрузите массовый виджет один раз:

[inline-code-attrs-start title = 'Массовый подсчёт комментариев на индексе'; type='javascript' inline-code-attrs-end]
[inline-code-start]
<ul>
  {posts.map((post) => (
    <li>
      <a href={post.slug}>{post.title}</a>{" "}
      <span class="fast-comments-count" data-fast-comments-url-id={post.slug}></span>
    </li>
  ))}
</ul>

<script
  dangerouslySetInnerHTML={{
    __html: `window.FastCommentsBulkCountConfig = ${
      JSON.stringify({ tenantId: TENANT_ID })
    };`,
  }}
/>
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>
[inline-code-end]

Скрипт ищет каждый элемент `.fast-comments-count` на странице и заполняет его счётчиком.

`data-fast-comments-url-id` должен соответствовать `urlId`, который использует виджет комментариев поста. Если виджет использует slug, маркер использует slug. Несоответствие приводит к отображению нуля в ветке, где есть комментарии.

Скрипт опрашивает `window.FastCommentsBulkCountConfig`, поэтому не имеет значения, задаёте вы конфигурацию до или после тега script.
---