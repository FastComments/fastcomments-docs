Na indeksnoj stranici, nemojte prikazivati jedan widget za broj komentara po retku. To je jedan zahtjev po objavi. Koristite grupni broj, koji uzima jedan zahtjev za cijelu stranicu.

Označite svaki redak s `urlId` koji njegova nit koristi, zatim učitajte grupni widget jednom:

[inline-code-attrs-start title = 'Broj komentara u grupi na indeksnoj stranici'; type='javascript' inline-code-attrs-end]
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

`data-fast-comments-url-id` mora odgovarati `urlId` koji widget za komentare objave koristi. Ako widget koristi slug, marker koristi slug. Neslaganje prikazuje nulu na niti koja ima komentare.

Skript provjerava `window.FastCommentsBulkCountConfig`, pa nije važno postaviti li konfiguraciju prije ili nakon skript taga.