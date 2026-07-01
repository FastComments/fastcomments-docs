## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| urlId | string | query | Não | Usado para determinar se a página atual está subscrita. |
| pageSize | integer | query | Não |  |
| afterId | string | query | Não |  |
| includeContext | boolean | query | Não |  |
| afterCreatedAt | integer | query | Não |  |
| unreadOnly | boolean | query | Não |  |
| dmOnly | boolean | query | Não |  |
| noDm | boolean | query | Não |  |
| includeTranslations | boolean | query | Não |  |
| includeTenantNotifications | boolean | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// As amostras de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Usado para determinar se a página atual está subscrita. (opcional)
let pageSize = 987 // Int |  (opcional)
let afterId = "afterId_example" // String |  (opcional)
let includeContext = true // Bool |  (opcional)
let afterCreatedAt = 987 // Int64 |  (opcional)
let unreadOnly = true // Bool |  (opcional)
let dmOnly = true // Bool |  (opcional)
let noDm = true // Bool |  (opcional)
let includeTranslations = true // Bool |  (opcional)
let includeTenantNotifications = true // Bool |  (opcional)
let sso = "sso_example" // String |  (opcional)

PublicAPI.getUserNotifications(tenantId: tenantId, options: PublicAPI.GetUserNotificationsOptions(urlId: urlId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, includeTenantNotifications: includeTenantNotifications, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]