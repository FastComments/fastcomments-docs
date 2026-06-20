## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| locale | string | query | No |  |
| rating | string | query | No |  |
| page | number | query | No |  |

## Risposta

Restituisce: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_gifs_trending_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio get_gifs_trending'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_gifs_trending_response import GetGifsTrendingResponse
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e ha come valore predefinito https://fastcomments.com
# Vedere configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrare in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Creare un'istanza della classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    locale = 'locale_example' # str |  (opzionale)
    rating = 'rating_example' # str |  (opzionale)
    page = 3.4 # float |  (opzionale)

    try:
        api_response = api_instance.get_gifs_trending(tenant_id, locale=locale, rating=rating, page=page)
        print("The response of PublicApi->get_gifs_trending:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gifs_trending: %s\n" % e)
[inline-code-end]