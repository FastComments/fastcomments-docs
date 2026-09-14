On an index page, don't render one comment-count widget per row. That is one request per post. Use the bulk count, which takes a single request for the whole page.

Mark each row with the `urlId` its thread uses, then load the bulk widget once:

[inline-code-attrs-start title = 'Пакетні підрахунки коментарів на індексній сторінці'; type='javascript' inline-code-attrs-end]
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

The script finds every `.fast-comments-count` element on the page and fills in its count.

`data-fast-comments-url-id` has to match the `urlId` that post's comment widget uses. If the widget uses the slug, the marker uses the slug. A mismatch shows zero on a thread that has comments.

The script polls for `window.FastCommentsBulkCountConfig`, so it does not matter whether you set the config before or after the script tag.