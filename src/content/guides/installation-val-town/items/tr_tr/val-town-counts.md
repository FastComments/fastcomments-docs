Bir dizin sayfasında her satır için ayrı bir yorum sayacı widget'ı oluşturmayın. Bu, her gönderi için bir istek demektir. Tüm sayfa için tek bir istek alan toplu sayacı (bulk count) kullanın.

Her satırı, ilgili thread'in kullandığı `urlId` ile işaretleyin, ardından toplu widget'ı bir kez yükleyin:

[inline-code-attrs-start title = 'Bir dizin sayfasında toplu yorum sayaçları'; type='javascript' inline-code-attrs-end]
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

Betik, sayfadaki her `.fast-comments-count` öğesini bulur ve sayacını doldurur.

`data-fast-comments-url-id`, ilgili gönderinin yorum widget'ının kullandığı `urlId` ile eşleşmelidir. Widget slug kullanıyorsa, işaretleyici de slug kullanır. Bir uyuşmazlık, yorumları olan bir thread'de sıfır gösterilmesine neden olur.

Betik, `window.FastCommentsBulkCountConfig` için yoklama yapar, bu nedenle yapılandırmayı script etiketinden önce veya sonra ayarlamanız fark etmez.