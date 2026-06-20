## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| id | string | query | Yes |  |

## Antwort

Gibt zurück: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_v1_page_react.py)

## Beispiel

[inline-code-attrs-start title = 'delete_v2_page_react Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_v1_page_react import CreateV1PageReact
from client.rest import ApiException
from pprint import pprint

# Das Festlegen des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Einen Kontext mit einer Instanz des API-Clients öffnen
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_v2_page_react(tenant_id, url_id, id)
        print("The response of PublicApi->delete_v2_page_react:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->delete_v2_page_react: %s\n" % e)
[inline-code-end]

---