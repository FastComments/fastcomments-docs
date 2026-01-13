Existem alguns endpoints para obter as contagens, dependendo do que voce quer e se deseja obte-las de um navegador, servidor ou usando o SDK da API.

## Contagens Publicas de Comentarios

Voce pode obter as contagens publicas de comentarios usando os widgets acima ou usando as APIs que eles usam. Essas APIs permanecem inalteradas desde 2019 e nunca mudarao.

[inline-code-attrs-start title = 'Single Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count/:tenantId?urlId=page_id_or_url
[inline-code-end]

Isso retornara uma estrutura como:

[inline-code-attrs-start title = 'Single Count Endpoint Response'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{"count":0,"postfix":"comments"}
[inline-code-end]

A propriedade `postfix` e sempre incluida.

[inline-code-attrs-start title = 'Bulk Count Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/widgets/comment-count-bulk/:tenantId?urlIds=x,y,z
[inline-code-end]

Isso retornara uma estrutura como:

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

O objeto `counts` so e preenchido para paginas que tem contagens. O mapa `translations` esta sempre presente pois e usado para o widget.

### Comportamento dos Endpoints Publicos / Cache

Os endpoints publicos tem um mecanismo de cache de 60 segundos para lidar com picos de trafego. Internamente, isso e um cache LRU por thread na memoria do servidor, entao voce pode ver as contagens mudarem ligeiramente (subir e depois descer temporariamente) quando as pessoas estao deixando muitos comentarios.

Os endpoints publicos sempre retornam a contagem *total* de comentarios, nao a contagem de comentarios raiz.

### APIs do Lado do Servidor / SDK

A maneira de obter comentarios do seu servidor e chamar a [API de Paginas](/guide-api.html#page-structure) e obter o objeto de pagina, que contem a contagem total de comentarios e a contagem de comentarios raiz. Fornecemos SDKs que permitem chamar esta API sem construir manualmente a solicitacao da API e fornecem valores de retorno tipados.
