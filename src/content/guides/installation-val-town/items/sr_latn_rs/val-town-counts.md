Na indeksnoj stranici, ne prikazujte po jedan widget za brojanje komentara po redu. To je jedan zahtev po objavi. Koristite grupno brojanje, koje zahteva jedan zahtev za celu stranicu.

Označite svaki red sa `urlId` koji njegova nit koristi, zatim učitajte bulk widget jednom:

[inline-code-attrs-start title = 'Grupno brojanje komentara na indeksnoj stranici'; type='javascript' inline-code-attrs-end]
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

Skript pronalazi svaki element `.fast-comments-count` na stranici i popunjava njegov broj.

`data-fast-comments-url-id` mora da se podudara sa `urlId` koji widget za komentare objave koristi. Ako widget koristi slug, marker koristi slug. Nepodudaranje prikazuje nulu na niti koja ima komentare.

Skript periodično proverava `window.FastCommentsBulkCountConfig`, pa nije važno da li konfiguraciju postavite pre ili posle `<script>` taga.