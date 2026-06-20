## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| id | string | query | Da |  |

## Response

Vraća: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_v1_page_react.py)

## Primjer

[inline-code-attrs-start title = 'Primjer delete_v2_page_react'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_v1_page_react import CreateV1PageReact
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
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