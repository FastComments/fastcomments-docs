Identifies the credential in use: the tenant it belongs to and, for OAuth tokens, the user who authorized it.  
Ідентифікує використані облікові дані: орендар, якому вони належать, та, для OAuth‑токенів, користувача, який їх авторизував.  

Integrations use this to test a connection and label it.  
Інтеграції використовують це для тестування з’єднання та його маркування.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| **Назва** | **Тип** | **Розташування** | **Обов’язково** | **Опис** |
| tenantId | string | query | Yes |  |

## Response

Returns: [`GetMeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMeResponse.swift)  
Повертає: [`GetMeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMeResponse.swift)

## Example

[inline-code-attrs-start title = 'Приклад getMe'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду ще у бета-версії. У разі будь-яких проблем, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 

DefaultAPI.getMe(tenantId: tenantId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]