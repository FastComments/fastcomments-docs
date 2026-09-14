Sur une page d'index, n'affichez pas un widget de compteur de commentaires par ligne. Cela représente une requête par article. Utilisez le comptage en masse, qui ne nécessite qu'une seule requête pour toute la page.

Marquez chaque ligne avec le `urlId` utilisé par son fil, puis chargez le widget en masse une seule fois :

[inline-code-attrs-start title = 'Comptes de commentaires en masse sur un index'; type='javascript' inline-code-attrs-end]
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

Le script trouve chaque élément `.fast-comments-count` sur la page et remplit son compteur.

`data-fast-comments-url-id` doit correspondre au `urlId` utilisé par le widget de commentaires du post. Si le widget utilise le slug, le marqueur utilise le slug. Un décalage affiche zéro sur un fil qui possède des commentaires.

Le script interroge `window.FastCommentsBulkCountConfig`, il n'importe donc pas si vous définissez la configuration avant ou après la balise script.