На индекс страници, немојте да рендерујете један widget за број коментара по реду. То је један захтев по посту. Користите групни број, који захтева један захтев за целу страницу.

Означите сваки ред `urlId`‑ом који користи његова тема, а затим учитајте групни widget једном:

[inline-code-attrs-start title = 'Групни број коментара на индекс страници'; type='javascript' inline-code-attrs-end]
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

Скрипта проналази сваки елемент `.fast-comments-count` на страници и попуњава његов број.

`data-fast-comments-url-id` мора да се поклапа са `urlId`‑ом који користи widget за коментаре поста. Ако widget користи slug, маркер користи slug. Неусаглашеност приказује нулу на теми која има коментаре.

Скрипта проверава `window.FastCommentsBulkCountConfig`, па није битно да ли постављате конфигурацију пре или после script таг‑а.