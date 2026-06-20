## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | chemin | Oui |  |
| search | string | paramètre de requête | Oui |  |
| locale | string | paramètre de requête | Non |  |
| rating | string | paramètre de requête | Non |  |
| page | number | paramètre de requête | Non |  |

## Réponse

Renvoie : [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_gifs_search_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_gifs_search'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_gifs_search_response import GetGifsSearchResponse
from client.rest import ApiException
from pprint import pprint

# Définir l'hôte est optionnel et la valeur par défaut est https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    search = 'search_example' # str | 
    locale = 'locale_example' # str |  (optionnel)
    rating = 'rating_example' # str |  (optionnel)
    page = 3.4 # float |  (optionnel)

    try:
        api_response = api_instance.get_gifs_search(tenant_id, search, locale=locale, rating=rating, page=page)
        print("The response of PublicApi->get_gifs_search:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gifs_search: %s\n" % e)
[inline-code-end]