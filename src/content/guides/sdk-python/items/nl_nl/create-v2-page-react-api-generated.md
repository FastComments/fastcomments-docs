## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| id | string | query | Ja |  |
| title | string | query | Nee |  |

## Respons

Retourneert: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_v1_page_react.py)

## Voorbeeld

[inline-code-attrs-start title = 'create_v2_page_react Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_v1_page_react import CreateV1PageReact
from client.rest import ApiException
from pprint import pprint

# Het opgeven van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    id = 'id_example' # str | 
    title = 'title_example' # str |  (optional)

    try:
        api_response = api_instance.create_v2_page_react(tenant_id, url_id, id, title=title)
        print("The response of PublicApi->create_v2_page_react:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->create_v2_page_react: %s\n" % e)
[inline-code-end]