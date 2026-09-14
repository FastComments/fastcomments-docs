Na stronie indeksu nie renderuj jednego widżetu liczenia komentarzy na wiersz. To powoduje jedno żądanie na każdy post. Użyj liczenia zbiorczego, które wymaga jednego żądania dla całej strony.

Oznacz każdy wiersz `urlId`, którego używa wątek, a następnie załaduj widżet zbiorczy jednorazowo:

[inline-code-attrs-start title = 'Zbiorcze liczenie komentarzy na indeksie'; type='javascript' inline-code-attrs-end]
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

Skrypt znajduje każdy element `.fast-comments-count` na stronie i wstawia jego liczbę.

`data-fast-comments-url-id` musi odpowiadać `urlId`, którego używa widżet komentarzy posta. Jeśli widżet używa slug, znacznik używa slug. Niezgodność powoduje wyświetlenie zera w wątku, który ma komentarze.

Skrypt nasłuchuje `window.FastCommentsBulkCountConfig`, więc nie ma znaczenia, czy ustawisz konfigurację przed, czy po znaczniku skryptu.