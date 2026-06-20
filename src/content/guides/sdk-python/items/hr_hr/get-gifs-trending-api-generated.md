## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| locale | string | query | Ne |  |
| rating | string | query | Ne |  |
| page | number | query | Ne |  |

## Odgovor

Vraća: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_gifs_trending_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer get_gifs_trending'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_gifs_trending_response import GetGifsTrendingResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    locale = 'locale_example' # str |  (neobavezno)
    rating = 'rating_example' # str |  (neobavezno)
    page = 3.4 # float |  (neobavezno)

    try:
        api_response = api_instance.get_gifs_trending(tenant_id, locale=locale, rating=rating, page=page)
        print("The response of PublicApi->get_gifs_trending:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gifs_trending: %s\n" % e)
[inline-code-end]