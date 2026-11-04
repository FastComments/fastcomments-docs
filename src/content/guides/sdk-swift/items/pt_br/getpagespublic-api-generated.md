Lista páginas para um tenant. Usada pelo cliente desktop FChat para popular sua lista de salas.
Requer que `enableFChat` seja true na configuração customizada resolvida para cada página.
Páginas que exigem SSO são filtradas de acordo com o acesso a grupos do usuário solicitante.

## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| cursor | string | query | Não | Cursor opaco de paginação retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`. |
| limit | integer | query | Não | 1..200, padrão 50 |
| q | string | query | Não | Filtro opcional de prefixo de título, sem diferenciar maiúsculas de minúsculas. |
| sortBy | string | query | Não | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentados primeiro) ou `title` (alfabética). |
| hasComments | boolean | query | Não | Se true, retorna apenas páginas com pelo menos um comentário. |

## Resposta

Retorna: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getPagesPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Cursor opaco de paginação retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`. (optional)
let limit = 987 // Int | 1..200, padrão 50 (optional)
let q = "q_example" // String | Filtro opcional de prefixo de título, sem diferenciar maiúsculas de minúsculas. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentados primeiro) ou `title` (alfabética). (optional)
let hasComments = true // Bool | Se true, retorna apenas páginas com pelo menos um comentário. (optional)

PublicAPI.getPagesPublic(tenantId: tenantId, options: PublicAPI.GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]