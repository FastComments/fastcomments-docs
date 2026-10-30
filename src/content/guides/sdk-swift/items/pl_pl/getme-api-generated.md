Identifies the credential in use: the tenant it belongs to and, for OAuth tokens, the user who authorized it.  
Integrations use this to test a connection and label it.

**Polish translation:**  
Identyfikuje używane poświadczenie: najemcę, do którego należy, oraz, w przypadku tokenów OAuth, użytkownika, który je autoryzował.  
Integracje używają tego do przetestowania połączenia i nadania mu etykiety.

## Parameters

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Tak |  |

## Response

Zwraca: [`GetMeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMeResponse.swift)

## Example

[inline-code-attrs-start title = 'Przykład getMe'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W przypadku jakichkolwiek problemów, proszę zgłosić je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
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