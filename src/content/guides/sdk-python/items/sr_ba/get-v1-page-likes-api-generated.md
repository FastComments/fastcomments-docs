## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | putanja | Da |  |
| urlId | string | upit | Da |  |

## Odgovor

Vraća: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_v1_page_likes.py)

## Primjer

[inline-code-attrs-start title = 'get_v1_page_likes Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_v1_page_likes import GetV1PageLikes
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opcionalno i podrazumijevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst koristeći instancu API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_v1_page_likes(tenant_id, url_id)
        print("The response of PublicApi->get_v1_page_likes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_v1_page_likes: %s\n" % e)
[inline-code-end]