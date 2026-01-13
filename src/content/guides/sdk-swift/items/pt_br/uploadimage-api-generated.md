Enviar e redimensionar uma imagem

## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Predefinição de tamanho: \"Default\" (1000x1000px) ou \"CrossPlatform\" (cria tamanhos para dispositivos populares) |
| urlId | string | query | No | ID da página de onde o upload está sendo feito, para configurar |

## Resposta

Retorna: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de uploadImage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Predefinição de tamanho: \"Default\" (1000x1000px) ou \"CrossPlatform\" (cria tamanhos para dispositivos populares) (opcional)
let urlId = "urlId_example" // String | ID da página de onde o upload está sendo feito, para configurar (opcional)

PublicAPI.uploadImage(tenantId: tenantId, file: file, sizePreset: sizePreset, urlId: urlId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]