En una página de índice, no renderices un widget de recuento de comentarios por fila. Eso es una solicitud por publicación. Usa el recuento masivo, que realiza una única solicitud para toda la página.

Marca cada fila con el `urlId` que usa su hilo, y luego carga el widget masivo una sola vez:

[inline-code-attrs-start title = 'Recuentos masivos de comentarios en un índice'; type='javascript' inline-code-attrs-end]
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

El script busca cada elemento `.fast-comments-count` en la página y completa su recuento.

`data-fast-comments-url-id` debe coincidir con el `urlId` que usa el widget de comentarios de esa publicación. Si el widget usa el slug, el marcador usa el slug. Una discrepancia muestra cero en un hilo que tiene comentarios.

El script sondea `window.FastCommentsBulkCountConfig`, por lo que no importa si configuras la configuración antes o después de la etiqueta del script.