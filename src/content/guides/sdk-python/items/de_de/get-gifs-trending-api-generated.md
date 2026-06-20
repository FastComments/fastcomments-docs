## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| locale | string | query | Nein |  |
| rating | string | query | Nein |  |
| page | number | query | Nein |  |

## Antwort

Gibt zurück: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_gifs_trending_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_gifs_trending Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_gifs_trending_response import GetGifsTrendingResponse
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    locale = 'locale_example' # str |  (optional)
    rating = 'rating_example' # str |  (optional)
    page = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_gifs_trending(tenant_id, locale=locale, rating=rating, page=page)
        print("The response of PublicApi->get_gifs_trending:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gifs_trending: %s\n" % e)
[inline-code-end]

---