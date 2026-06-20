---
Información en bloque de usuarios para un tenant. Dado userIds, devuelve información para mostrar desde User / SSOUser.
Usado por el widget de comentarios para enriquecer usuarios que acaban de aparecer vía un evento de presencia.
Sin contexto de página: la privacidad se aplica de forma uniforme (los perfiles privados están enmascarados).

## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| ids | string | query | Sí | userIds delimitados por comas. |

## Respuesta

Devuelve: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersInfoResponse.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUsersInfo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor repórtelo vía http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let ids = "ids_example" // String | userIds delimitados por comas.

PublicAPI.getUsersInfo(tenantId: tenantId, ids: ids) { (response, error) in
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