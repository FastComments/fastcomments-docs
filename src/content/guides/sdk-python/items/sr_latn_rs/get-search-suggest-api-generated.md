## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| text-search | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_suggest_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_search_suggest'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import GetSearchSuggestOptions
from client.models.moderation_suggest_response import ModerationSuggestResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    text_search = 'text_search_example' # str |  (opcionalno)
    sso = 'sso_example' # str |  (opcionalno)

    try:
        api_response = api_instance.get_search_suggest(tenant_id, GetSearchSuggestOptions(text_search=text_search, sso=sso))
        print("Odgovor od ModerationApi->get_search_suggest:\n")
        pprint(api_response)
    except Exception as e:
        print("Izuzetak pri pozivanju ModerationApi->get_search_suggest: %s\n" % e)
[inline-code-end]