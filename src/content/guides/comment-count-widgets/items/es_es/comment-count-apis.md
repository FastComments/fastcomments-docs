Hay varios endpoints para obtener los conteos, dependiendo de lo que quieras y si quieres obtenerlos desde un navegador, servidor o usando el SDK de la API.

## Conteos Publicos de Comentarios

Puedes obtener los conteos publicos de comentarios usando los widgets anteriores o usando las APIs que ellos usan. Estas APIs han permanecido sin cambios desde 2019 y nunca cambiaran.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

Esto devolvera una estructura como:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

La propiedad `postfix` siempre esta incluida.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

Esto devolvera una estructura como:

[inline-code-attrs-start title = 'Bulk Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "translations": {
        "t174": "0 Comments",
        "t175": "1 Comment",
        "t176": "[count] Comments"
    },
    "counts": {
        "x": 10
    }
}
[inline-code-end]

El objeto `counts` solo se completa para paginas que tienen conteos. El mapa `translations` siempre esta presente ya que se usa para el widget.

### Comportamiento de Endpoints Publicos / Cache

Los endpoints publicos tienen un mecanismo de cache de 60 segundos para manejar picos de trafico. Internamente, esto es un cache LRU por hilo en memoria en el servidor, por lo que puedes ver que los conteos cambian ligeramente (suben y luego bajan temporalmente) cuando las personas dejan muchos comentarios.

Los endpoints publicos siempre devuelven el conteo *total* de comentarios, no el conteo de comentarios raiz.

### APIs del Lado del Servidor / SDK

La forma de obtener comentarios desde tu servidor es llamar a la [API de Paginas](/guide-api.html#page-structure) y obtener el objeto de pagina, que contiene el conteo total de comentarios y el conteo de comentarios raiz. Proporcionamos SDKs que te permiten llamar a esta API sin construir la solicitud de API manualmente y proporcionan valores de retorno tipados.
