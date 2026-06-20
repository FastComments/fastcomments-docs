Lista páginas para um tenant. Usado pelo cliente desktop FChat para preencher sua lista de salas. Requer que `enableFChat` seja true na configuração personalizada resolvida para cada página. Páginas que requerem SSO são filtradas com base no acesso de grupo do usuário solicitante.

## Parameters

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| cursor | string | query | Não | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`. |
| limit | integer | query | Não | 1..200, default 50 |
| q | string | query | Não | Optional case-insensitive title prefix filter. |
| sortBy | string | query | Não | Sort order. `updatedAt` (default, newest first), `commentCount` (most comments first), or `title` (alphabetical). |
| hasComments | boolean | query | Não | If true, only return pages with at least one comment. |

## Response

Retorna: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Cursor de paginação opaco retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`. (opcional)
let limit = 987 // Int | 1..200, padrão 50 (opcional)
let q = "q_example" // String | Filtro opcional de prefixo de título, sem diferenciar maiúsculas/minúsculas. (opcional)
let sortBy = PagesSortBy() // PagesSortBy | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentários primeiro), ou `title` (alfabética). (opcional)
let hasComments = true // Bool | Se true, retorne apenas páginas com pelo menos um comentário. (opcional)

PublicAPI.getPagesPublic(tenantId: tenantId, cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]