## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | No |  |

## Respuesta

Devuelve: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIModerateGetUserBanPreferencesResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUserBanPreference'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor reporte a través de http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let sso = "sso_example" // String |  (opcional)

ModerationAPI.getUserBanPreference(sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]