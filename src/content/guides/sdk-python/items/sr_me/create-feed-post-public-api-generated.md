## Parametri

| Naziv | Tip | Lokacija | Obligatorno | Opis |
|------|------|----------|-------------|------|
| tenantId | string | path | Da |  |
| broadcastId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_post_response.py)

## Primjer

[inline-code-attrs-start title = 'create_feed_post_public Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import CreateFeedPostPublicOptions
from client.models.create_feed_post_params import CreateFeedPostParams
from client.models.create_feed_post_response import CreateFeedPostResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opcionalno i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_feed_post_params = client.CreateFeedPostParams() # CreateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  (opcionalno)
    sso = 'sso_example' # str |  (opcionalno)

    try:
        api_response = api_instance.create_feed_post_public(tenant_id, create_feed_post_params, CreateFeedPostPublicOptions(broadcast_id=broadcast_id, sso=sso))
        print("The response of PublicApi->create_feed_post_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->create_feed_post_public: %s\n" % e)
[inline-code-end]