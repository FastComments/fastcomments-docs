## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| sendEmails | boolean | query | No |  |
| populateNotifications | boolean | query | No |  |

## Respuesta

Devuelve: [`SaveCommentsBulkResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/save_comments_bulk_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de save_comments_bulk'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_comment_params import CreateCommentParams
from client.models.save_comments_bulk_response import SaveCommentsBulkResponse
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# Consulte configuration.py para ver la lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# The client must configure the authentication and authorization parameters
# de acuerdo con la política de seguridad del servidor API.
# A continuación se proporcionan ejemplos para cada método de autenticación, use el ejemplo que
# satisfaga su caso de uso de autenticación.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente lo siguiente para configurar un prefijo (p. ej. Bearer) para la clave API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = [client.CreateCommentParams()] # List[CreateCommentParams] | 
    is_live = True # bool |  (opcional)
    do_spam_check = True # bool |  (opcional)
    send_emails = True # bool |  (opcional)
    populate_notifications = True # bool |  (opcional)

    try:
        api_response = api_instance.save_comments_bulk(tenant_id, create_comment_params, is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications)
        print("The response of DefaultApi->save_comments_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comments_bulk: %s\n" % e)
[inline-code-end]