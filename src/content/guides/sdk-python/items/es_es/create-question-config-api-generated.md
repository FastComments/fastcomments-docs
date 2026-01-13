## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |

## Respuesta

Devuelve: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_question_config200_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de create_question_config'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_question_config200_response import CreateQuestionConfig200Response
from client.models.create_question_config_body import CreateQuestionConfigBody
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto https://fastcomments.com
# Consulte configuration.py para una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# El cliente debe configurar los parámetros de autenticación y autorización
# de acuerdo con la política de seguridad del servidor API.
# Se proporcionan ejemplos para cada método de autenticación a continuación, use el ejemplo que
# satisfaga su caso de uso de autenticación.

# Configure la autorización por clave API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Descomente lo siguiente para configurar un prefijo (p. ej. Bearer) para la clave API, si es necesario
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entre en un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Cree una instancia de la clase API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_question_config_body = client.CreateQuestionConfigBody() # CreateQuestionConfigBody | 

    try:
        api_response = api_instance.create_question_config(tenant_id, create_question_config_body)
        print("The response of DefaultApi->create_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_question_config: %s\n" % e)
[inline-code-end]

---