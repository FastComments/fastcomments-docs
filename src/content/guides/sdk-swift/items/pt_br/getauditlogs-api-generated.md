## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Sim | O tenant para ler logs de auditoria. |
| limit | integer | query | Não | Quantos logs retornar. Máximo 10000, padrão 1000. |
| skip | integer | query | Não | Quantos logs pular, para paginação. |
| order | string | query | Não | Direção de ordenação no horário do evento. Padrão DESC (mais recente primeiro). |
| after | integer | query | Não |  |
| before | integer | query | Não |  |
| username | string | query | Não | Retornar apenas eventos realizados por este nome de usuário. |
| ip | string | query | Não | Retornar apenas eventos deste endereço IP. |
| crudType | string | query | Não | Retornar apenas eventos deste tipo: c, r, u, d, ou login. |
| resourceName | string | query | Não | Retornar apenas eventos para este recurso, por exemplo User ou Moderator. |
| targetId | string | query | Não | Retornar apenas eventos cujo objeto afetado tem este id. |
| target | string | query | Não | Correspondência de substring sem distinção entre maiúsculas e minúsculas no rótulo do objeto afetado, por exemplo um nome de usuário ou email. |
| includeManagedTenants | boolean | query | Não | Também retornar eventos de tenants que este tenant gerencia. |

## Resposta

Retorna: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | O tenant para ler logs de auditoria.
let limit = 987 // Int | Quantos logs retornar. Máximo 10000, padrão 1000. (optional)
let skip = 987 // Int | Quantos logs pular, para paginação. (optional)
let order = SORT_DIR() // SORTDIR | Direção de ordenação no horário do evento. Padrão DESC (mais recente primeiro). (optional)
let after = 987 // Int64 |  (optional)
let before = 987 // Int64 |  (optional)
let username = "username_example" // String | Retornar apenas eventos realizados por este nome de usuário. (optional)
let ip = "ip_example" // String | Retornar apenas eventos deste endereço IP. (optional)
let crudType = "crudType_example" // String | Retornar apenas eventos deste tipo: c, r, u, d, ou login. (optional)
let resourceName = "resourceName_example" // String | Retornar apenas eventos para este recurso, por exemplo User ou Moderator. (optional)
let targetId = "targetId_example" // String | Retornar apenas eventos cujo objeto afetado tem este id. (optional)
let target = "target_example" // String | Correspondência de substring sem distinção entre maiúsculas e minúsculas no rótulo do objeto afetado, por exemplo um nome de usuário ou email. (optional)
let includeManagedTenants = true // Bool | Também retornar eventos de tenants que este tenant gerencia. (optional)

DefaultAPI.getAuditLogs(tenantId: tenantId, options: DefaultAPI.GetAuditLogsOptions(limit: limit, skip: skip, order: order, after: after, before: before, username: username, ip: ip, crudType: crudType, resourceName: resourceName, targetId: targetId, target: target, includeManagedTenants: includeManagedTenants)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---