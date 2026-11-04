Na strani z indeksom ne upodabljajte enega gradnika za štetje komentarjev na vsako vrstico. To je ena zahteva na objavo. Uporabite množično štetje, ki za celotno stran vzame eno samo zahtevo.

Vsako vrstico označite z `urlId`, ki ga uporablja njena nit, nato pa gradnik za množično štetje naložite enkrat:

[inline-code-attrs-start title = 'Množično štetje komentarjev na indeksu'; type='javascript' inline-code-attrs-end]
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

Skript poišče vse elemente `.fast-comments-count` na strani in izpolni njihove številke.

`data-fast-comments-url-id` se mora ujemati z `urlId`, ki ga uporablja gradnik komentarjev te objave. Če gradnik uporablja slug, oznaka uporablja slug. Neujemljenost prikaže nič pri niti, ki ima komentarje.

Skript anketira `window.FastCommentsBulkCountConfig`, zato ni pomembno, ali nastavite konfiguracijo pred ali za oznako skripta.