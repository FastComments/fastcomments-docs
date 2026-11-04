Traduce los comentarios dados al idioma del widget. Solo disponible cuando la regla de personalización del widget correspondiente tiene Comment Translation configurado en Automatic o Manual.

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí |  |
| sso | string | query | No |  |
| locale | string | query | No |  |

## Respuesta

Devuelve: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CommentTranslationsResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de translateComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let commentTranslationsRequest = CommentTranslationsRequest(commentIds: ["commentIds_example"]) // CommentTranslationsRequest | 
let sso = "sso_example" // String |  (opcional)
let locale = "locale_example" // String |  (opcional)

PublicAPI.translateComments(tenantId: tenantId, urlId: urlId, commentTranslationsRequest: commentTranslationsRequest, options: PublicAPI.TranslateCommentsOptions(sso: sso, locale: locale)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]