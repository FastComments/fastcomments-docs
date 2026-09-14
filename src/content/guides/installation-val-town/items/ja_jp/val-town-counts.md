インデックスページでは、行ごとにコメントカウントウィジェットを1つずつレンダリングしないでください。これは投稿ごとに1リクエストになります。ページ全体で1回のリクエストで済む一括カウントを使用してください。

`urlId`（スレッドが使用する）で各行にマークし、まとめてウィジェットを1回だけロードします:

[inline-code-attrs-start title = 'インデックスページの一括コメント数'; type='javascript' inline-code-attrs-end]
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

このスクリプトはページ上のすべての `.fast-comments-count` 要素を検出し、そのカウントを埋め込みます。

`data-fast-comments-url-id` は、投稿のコメントウィジェットが使用する `urlId` と一致する必要があります。ウィジェットがスラッグを使用する場合、マーカーもスラッグを使用します。不一致の場合、コメントがあるスレッドでも0が表示されます。

スクリプトは `window.FastCommentsBulkCountConfig` をポーリングするため、設定をスクリプトタグの前に置くか後に置くかは関係ありません。