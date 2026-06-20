## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_export_response.py)

## Ejemplo

[inline-code-attrs-start title = 'post_api_export Ejemplo'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_export_response import ModerationExportResponse
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulte configuration.py para una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Abra un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Crear una instancia de la clase API
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (optional)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (optional)
    filters = 'filters_example' # str |  (optional)
    search_filters = 'search_filters_example' # str |  (optional)
    sorts = 'sorts_example' # str |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_api_export(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, sso=sso)
        print("The response of ModerationApi->post_api_export:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_api_export: %s\n" % e)
[inline-code-end]

---