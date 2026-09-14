На индексна страница не рендерирайте по един widget за брой коментари на ред. Това е една заявка за всяка публикация. Използвайте груповия брой, който изпраща една единствена заявка за цялата страница.

Маркирайте всеки ред с `urlId`, който използва нишката му, след което заредете груповия widget еднократно:

[inline-code-attrs-start title = 'Групови броячи на коментари в индекс'; type='javascript' inline-code-attrs-end]
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

Скриптът намира всеки елемент `.fast-comments-count` на страницата и попълва неговия брой.

`data-fast-comments-url-id` трябва да съвпада с `urlId`, който използва widget‑ът за коментари на публикацията. Ако widget‑ът използва slug, маркерът използва slug. При несъответствие се показва нула за нишка, която има коментари.

Скриптът проверява наличието на `window.FastCommentsBulkCountConfig`, затова няма значение дали конфигурацията е зададена преди или след етикета на скрипта.