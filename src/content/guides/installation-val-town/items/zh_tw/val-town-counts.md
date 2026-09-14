在索引頁面上，不要為每一列渲染單一的 comment-count 小部件。這會對每篇文章產生一次請求。請使用批量計數，它只需要對整個頁面發出一次請求。

在每一列上標記其線程使用的 `urlId`，然後一次性載入批量小部件：

[inline-code-attrs-start title = '索引頁面的批量評論計數'; type='javascript' inline-code-attrs-end]
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

此腳本會在頁面上尋找所有 `.fast-comments-count` 元素，並填入相應的計數。

`data-fast-comments-url-id` 必須與文章評論小部件使用的 `urlId` 相匹配。如果小部件使用 slug，標記也使用 slug。若不匹配，則即使線程中有評論也會顯示為零。

此腳本會輪詢 `window.FastCommentsBulkCountConfig`，因此無論您在 script 標籤之前或之後設定配置都沒關係。