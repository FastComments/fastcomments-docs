Upload and resize an image
==========================

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | path | Sim |  |
| sizePreset | string | query | Não | Size preset: \"Default\" (1000x1000px) ou \"CrossPlatform\" (cria tamanhos para dispositivos populares) |
| urlId | string | query | Não | ID da página de onde o upload está ocorrendo, para configurar |

## Resposta

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'uploadImage Exemplo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Size preset: \"Default\" (1000x1000px) ou \"CrossPlatform\" (cria tamanhos para dispositivos populares) (opcional)
let urlId = "urlId_example" // String | ID da página de onde o upload está ocorrendo, para configurar (opcional)

PublicAPI.uploadImage(tenantId: tenantId, file: file, options: PublicAPI.UploadImageOptions(sizePreset: sizePreset, urlId: urlId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]