Lista las páginas de un tenant. Usado por el cliente de escritorio FChat para poblar su lista de salas.
Requiere que `enableFChat` sea `true` en la configuración personalizada resuelta para cada página.
Las páginas que requieren SSO se filtran según el acceso de grupo del usuario solicitante.

## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursor de paginación opaco devuelto como `nextCursor` de una petición anterior. Vinculado al mismo `sortBy`. |
| limit | integer | query | No | 1..200, por defecto 50 |
| q | string | query | No | Filtro opcional por prefijo de título sin distinguir mayúsculas/minúsculas. |
| sortBy | string | query | No | Orden de clasificación. `updatedAt` (por defecto, más recientes primero), `commentCount` (más comentarios primero), o `title` (alfabético). |
| hasComments | boolean | query | No | Si es true, solo devuelve páginas con al menos un comentario. |

## Respuesta

Devuelve: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para ver la lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Abra un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Cursor de paginación opaco devuelto como `nextCursor` de una petición anterior. Vinculado al mismo `sortBy`. (opcional)
    limit = 56 # int | 1..200, por defecto 50 (opcional)
    q = 'q_example' # str | Filtro opcional por prefijo de título sin distinguir mayúsculas/minúsculas. (opcional)
    sort_by = client.PagesSortBy() # PagesSortBy | Orden de clasificación. `updatedAt` (por defecto, más recientes primero), `commentCount` (más comentarios primero) o `title` (alfabético). (opcional)
    has_comments = True # bool | Si es true, solo devuelve páginas con al menos un comentario. (opcional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]