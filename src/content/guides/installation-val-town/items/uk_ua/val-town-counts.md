На індексній сторінці не рендерьте один віджет підрахунку коментарів на рядок. Це означає один запит на пост. Використовуйте масовий підрахунок, який робить один запит для всієї сторінки.

Позначте кожен рядок `urlId`, який використовує його тема, потім завантажте масовий віджет один раз:

[inline-code-attrs-start title = 'Масові підрахунки коментарів на індексній сторінці'; type='javascript' inline-code-attrs-end]
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

Скрипт знаходить кожен елемент `.fast-comments-count` на сторінці та заповнює його підрахунок.

`data-fast-comments-url-id` має відповідати `urlId`, який використовує віджет коментарів поста. Якщо віджет використовує slug, маркер використовує slug. Невідповідність показує нуль у темі, яка має коментарі.

Скрипт опитує `window.FastCommentsBulkCountConfig`, тому не має значення, чи ви встановлюєте конфігурацію до чи після тегу скрипта.