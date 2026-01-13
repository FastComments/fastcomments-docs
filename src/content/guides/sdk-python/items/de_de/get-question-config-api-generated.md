## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Antwort

Gibt zurück: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_config200_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_question_config Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_config200_response import GetQuestionConfig200Response
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig auf https://fastcomments.com gesetzt
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Der Client muss die Authentifizierungs- und Autorisierungsparameter
# in Übereinstimmung mit der Sicherheitsrichtlinie des API-Servers konfigurieren.
# Beispiele für jede Auth-Methode sind unten aufgeführt, verwenden Sie das Beispiel, das
# Ihren Auth-Anwendungsfall erfüllt.

# Konfigurieren Sie die API-Schlüssel-Authentifizierung: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Entfernen Sie unten die Kommentarzeichen, um ein Präfix (z. B. Bearer) für den API-Schlüssel zu setzen, falls nötig
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_question_config(tenant_id, id)
        print("The response of DefaultApi->get_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_config: %s\n" % e)
[inline-code-end]

---