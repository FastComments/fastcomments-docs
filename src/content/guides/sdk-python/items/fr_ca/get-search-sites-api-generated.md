## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|-------------|-------------|
| tenantId | string | query | Oui |  |
| value | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Retourne : [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_site_search_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple get_search_sites'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import GetSearchSitesOptions
from client.models.moderation_site_search_response import ModerationSiteSearchResponse
from client.rest import ApiException
from pprint import pprint

# Définir l'hôte est facultatif et la valeur par défaut est https://fastcomments.com
# Voir configuration.py pour une liste de tous les paramètres de configuration supportés.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    value = 'value_example' # str | (facultatif)
    sso = 'sso_example' # str | (facultatif)

    try:
        api_response = api_instance.get_search_sites(tenant_id, GetSearchSitesOptions(value=value, sso=sso))
        print("The response of ModerationApi->get_search_sites:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_search_sites: %s\n" % e)
[inline-code-end]

---