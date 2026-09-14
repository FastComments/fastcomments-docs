Su una pagina indice, non visualizzare un widget di conteggio dei commenti per riga. Questo comporta una richiesta per ogni post. Usa il conteggio in blocco, che richiede una singola richiesta per l'intera pagina.

Marca ogni riga con il `urlId` che utilizza il suo thread, quindi carica il widget in blocco una sola volta:

[inline-code-attrs-start title = 'Conteggi dei commenti in blocco su una pagina indice'; type='javascript' inline-code-attrs-end]
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

Lo script trova ogni elemento `.fast-comments-count` nella pagina e ne riempie il conteggio.

`data-fast-comments-url-id` deve corrispondere al `urlId` utilizzato dal widget dei commenti del post. Se il widget utilizza lo slug, il marcatore utilizza lo slug. Una mancata corrispondenza mostra zero su un thread che ha commenti.

Lo script controlla `window.FastCommentsBulkCountConfig`, quindi non importa se imposti la configurazione prima o dopo il tag script.