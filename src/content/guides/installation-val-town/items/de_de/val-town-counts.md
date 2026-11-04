Rendern Sie auf einer Indexseite nicht ein Kommentarzahl-Widget pro Zeile. Das wäre eine Anfrage pro Beitrag. Verwenden Sie die Bulk-Zählung, die eine einzige Anfrage für die gesamte Seite benötigt.

Markieren Sie jede Zeile mit der `urlId`, die ihr Thread verwendet, und laden Sie dann das Bulk-Widget einmal:

[inline-code-attrs-start title = 'Bulk-Kommentarzahlen auf einer Indexseite'; type='javascript' inline-code-attrs-end]
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

Das Skript findet jedes `.fast-comments-count`-Element auf der Seite und füllt dessen Anzahl aus.

`data-fast-comments-url-id` muss mit der `urlId` übereinstimmen, die das Kommentar-Widget dieses Beitrags verwendet. Wenn das Widget den Slug verwendet, verwendet der Marker den Slug. Eine Nichtübereinstimmung zeigt null bei einem Thread an, der Kommentare hat.

Das Skript fragt `window.FastCommentsBulkCountConfig` ab, daher spielt es keine Rolle, ob Sie die Konfiguration vor oder nach dem Skript-Tag setzen.