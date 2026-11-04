Lista las páginas de un tenant. Usado por el cliente de escritorio FChat para poblar su lista de salas.
Requiere que `enableFChat` sea true en la configuración personalizada resuelta de cada página.
Las páginas que requieren SSO se filtran según el acceso de grupo del usuario solicitante.

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | path | Sí |  |
| cursor | string | query | No | Cursor de paginación opaco devuelto como `nextCursor` en una solicitud anterior. Vinculado al mismo `sortBy`. |
| limit | integer | query | No | 1..200, por defecto 50 |
| q | string | query | No | Filtro opcional de prefijo de título, insensible a mayúsculas. |
| sortBy | string | query | No | Orden de clasificación. `updatedAt` (por defecto, más recientes primero), `commentCount` (más comentarios primero) o `title` (alfabético). |
| hasComments | boolean | query | No | Si es true, solo devuelve páginas con al menos un comentario. |

## Respuesta

Devuelve: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Cursor de paginación opaco devuelto como `nextCursor` en una solicitud anterior. Vinculado al mismo `sortBy`. (optional)
    limit = 56 # int | 1..200, por defecto 50 (optional)
    q = 'q_example' # str | Filtro opcional de prefijo de título, insensible a mayúsculas. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Orden de clasificación. `updatedAt` (por defecto, más recientes primero), `commentCount` (más comentarios primero) o `title` (alfabético). (optional)
    has_comments = True # bool | Si es true, solo devuelve páginas con al menos un comentario. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]