Cargar y redimensionar una imagen

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| sizePreset | string | query | No | Preajuste de tamaño: "Default" (1000x1000px) o "CrossPlatform" (crea tamaños para dispositivos populares) |
| urlId | string | query | No | ID de página desde la que ocurre la carga, para configurar |

## Respuesta

Devuelve: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo uploadImage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código están todavía en beta. Para cualquier problema, informe a través de http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Preajuste de tamaño: \"Default\" (1000x1000px) o \"CrossPlatform\" (crea tamaños para dispositivos populares) (opcional)
let urlId = "urlId_example" // String | ID de página desde la que ocurre la carga, para configurar (opcional)

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

---