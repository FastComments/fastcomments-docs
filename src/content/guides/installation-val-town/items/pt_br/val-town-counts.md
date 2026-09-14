Em uma página de índice, não renderize um widget de contagem de comentários por linha. Isso gera uma solicitação por postagem. Use a contagem em massa, que faz uma única solicitação para a página inteira.

Marque cada linha com o `urlId` que seu thread usa e, em seguida, carregue o widget em massa uma única vez:

[inline-code-attrs-start title = 'Contagens de comentários em massa em um índice'; type='javascript' inline-code-attrs-end]
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

O script encontra cada elemento `.fast-comments-count` na página e preenche sua contagem.

`data-fast-comments-url-id` deve corresponder ao `urlId` que o widget de comentários da postagem usa. Se o widget usar o slug, o marcador usará o slug. Uma incompatibilidade exibirá zero em um thread que tem comentários.

O script verifica `window.FastCommentsBulkCountConfig`, portanto não importa se você definir a configuração antes ou depois da tag de script.