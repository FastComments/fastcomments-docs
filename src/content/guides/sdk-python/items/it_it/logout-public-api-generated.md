## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di logout_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e per impostazione predefinita è https://fastcomments.com
# Consulta configuration.py per l'elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.PublicApi(api_client)

    try:
        api_response = api_instance.logout_public()
        print("The response of PublicApi->logout_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->logout_public: %s\n" % e)
[inline-code-end]

---