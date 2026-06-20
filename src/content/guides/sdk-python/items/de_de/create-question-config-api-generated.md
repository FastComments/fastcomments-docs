## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Antwort

Gibt zurück: [`CreateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_question_config_response.py)

## Beispiel

[inline-code-attrs-start title = 'create_question_config Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_question_config_body import CreateQuestionConfigBody
from client.models.create_question_config_response import CreateQuestionConfigResponse
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# gemäß der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Beispiele für jede Authentifizierungsmethode sind unten angegeben, verwenden Sie das
# Beispiel, das Ihren Authentifizierungsanforderungen entspricht.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Einen Kontext mit einer Instanz des API-Clients öffnen
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
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