Op een indexpagina moet je niet per rij één comment-count widget renderen. Dat is één verzoek per bericht. Gebruik de bulk‑telling, die één enkel verzoek voor de hele pagina doet.

Markeer elke rij met de `urlId` die de thread gebruikt, en laad vervolgens de bulk‑widget één keer:

[inline-code-attrs-start title = 'Bulk commentaartellingen op een index'; type='javascript' inline-code-attrs-end]
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

Het script vindt elk `.fast-comments-count`‑element op de pagina en vult de telling in.

`data-fast-comments-url-id` moet overeenkomen met de `urlId` die de commentaarwidget van het bericht gebruikt. Als de widget de slug gebruikt, gebruikt de marker de slug. Een mismatch toont nul op een thread die wel reacties heeft.

Het script pollt naar `window.FastCommentsBulkCountConfig`, dus het maakt niet uit of je de configuratie vóór of na de script‑tag instelt.