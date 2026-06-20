## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| domainToUpdate | string | path | Так |  |

## Відповідь

Повертає: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PutDomainConfigResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад putDomainConfig'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду ще перебувають у бета-версії. Якщо виникне проблема, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let domainToUpdate = "domainToUpdate_example" // String | 
let updateDomainConfigParams = UpdateDomainConfigParams(domain: "domain_example", emailFromName: "emailFromName_example", emailFromEmail: "emailFromEmail_example", logoSrc: "logoSrc_example", logoSrc100px: "logoSrc100px_example", footerUnsubscribeURL: "footerUnsubscribeURL_example", emailHeaders: "TODO") // UpdateDomainConfigParams | 

DefaultAPI.putDomainConfig(tenantId: tenantId, domainToUpdate: domainToUpdate, updateDomainConfigParams: updateDomainConfigParams) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]