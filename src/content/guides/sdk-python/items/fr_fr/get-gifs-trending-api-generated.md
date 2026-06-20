---
## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| locale | string | query | Non |  |
| rating | string | query | Non |  |
| page | number | query | Non |  |

## Réponse

Renvoie : [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_gifs_trending_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_gifs_trending'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_gifs_trending_response import GetGifsTrendingResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    locale = 'locale_example' # str |  (facultatif)
    rating = 'rating_example' # str |  (facultatif)
    page = 3.4 # float |  (facultatif)

    try:
        api_response = api_instance.get_gifs_trending(tenant_id, locale=locale, rating=rating, page=page)
        print("The response of PublicApi->get_gifs_trending:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gifs_trending: %s\n" % e)
[inline-code-end]

---