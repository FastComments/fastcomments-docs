På en indeksside skal du ikke gengive én comment-count-widget pr. række. Det er én anmodning pr. opslag. Brug bulk-tællingen, som tager en enkelt anmodning for hele siden.

Markér hver række med den `urlId`, dens tråd bruger, og indlæs derefter bulk-widgetten én gang:

[inline-code-attrs-start title = 'Bulk-kommentartællinger på en indeksside'; type='javascript' inline-code-attrs-end]
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

Scriptet finder hvert `.fast-comments-count`-element på siden og udfylder dets tælling.

`data-fast-comments-url-id` skal matche den `urlId`, som opslagets kommentarwidget bruger. Hvis widgetten bruger slug'en, bruger markøren slug'en. En uoverensstemmelse viser nul på en tråd, der har kommentarer.

Scriptet poller efter `window.FastCommentsBulkCountConfig`, så det er ligegyldigt, om du angiver konfigurationen før eller efter script-tagget.