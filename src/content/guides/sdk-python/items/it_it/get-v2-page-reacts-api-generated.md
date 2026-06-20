## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | percorso | Sì |  |
| urlId | string | query | Sì |  |

## Risposta

Restituisce: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_v2_page_reacts.py)

## Esempio

[inline-code-attrs-start title = 'Esempio get_v2_page_reacts'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_v2_page_reacts import GetV2PageReacts
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e predefinita su https://fastcomments.com
# Vedere configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_v2_page_reacts(tenant_id, url_id)
        print("The response of PublicApi->get_v2_page_reacts:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_v2_page_reacts: %s\n" % e)
[inline-code-end]

---