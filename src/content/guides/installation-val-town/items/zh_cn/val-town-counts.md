在索引页面上，不要为每一行渲染一个 comment-count 小部件。这会对每篇文章产生一次请求。使用批量计数，它只需要对整页发起一次请求。

为每一行标记其线程使用的 `urlId`，然后一次性加载批量小部件：

[inline-code-attrs-start title = '索引页面的批量评论计数'; type='javascript' inline-code-attrs-end]
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

脚本会查找页面上所有 `.fast-comments-count` 元素并填充其计数。

`data-fast-comments-url-id` 必须与帖子评论小部件使用的 `urlId` 相匹配。如果小部件使用 slug，则标记也使用 slug。若不匹配，则会在已有评论的线程上显示为零。

脚本会轮询 `window.FastCommentsBulkCountConfig`，因此在脚本标签之前或之后设置配置都没有关系。