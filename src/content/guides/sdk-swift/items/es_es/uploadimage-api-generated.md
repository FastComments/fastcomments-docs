Subir y redimensionar una imagen

## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Preajuste de tamaño: "Default" (1000x1000px) o "CrossPlatform" (crea tamaños para dispositivos populares) |
| urlId | string | query | No | Id de página desde la cual se realiza la subida, para configurar |

## Respuesta

Devuelve: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de uploadImage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor repórtelo vía http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Preajuste de tamaño: \"Default\" (1000x1000px) o \"CrossPlatform\" (crea tamaños para dispositivos populares) (opcional)
let urlId = "urlId_example" // String | Id de página desde la cual se realiza la subida, para configurar (opcional)

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

---