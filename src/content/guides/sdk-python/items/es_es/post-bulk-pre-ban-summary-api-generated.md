## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | No |  |
| includeByIP | boolean | query | No |  |
| includeByEmailDomain | boolean | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/bulk_pre_ban_summary.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de post_bulk_pre_ban_summary'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.bulk_pre_ban_params import BulkPreBanParams
from client.models.bulk_pre_ban_summary import BulkPreBanSummary
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
    api_instance = client.ModerationApi(api_client)
    bulk_pre_ban_params = client.BulkPreBanParams() # BulkPreBanParams | 
    include_by_user_id_and_email = True # bool |  (opcional)
    include_by_ip = True # bool |  (opcional)
    include_by_email_domain = True # bool |  (opcional)
    sso = 'sso_example' # str |  (opcional)

    try:
        api_response = api_instance.post_bulk_pre_ban_summary(bulk_pre_ban_params, include_by_user_id_and_email=include_by_user_id_and_email, include_by_ip=include_by_ip, include_by_email_domain=include_by_email_domain, sso=sso)
        print("The response of ModerationApi->post_bulk_pre_ban_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_bulk_pre_ban_summary: %s\n" % e)
[inline-code-end]