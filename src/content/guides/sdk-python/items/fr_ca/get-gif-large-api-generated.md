## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| largeInternalURLSanitized | string | query | Oui |  |

## Réponse

Renvoie : [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/gif_get_large_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple get_gif_large'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.gif_get_large_response import GifGetLargeResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut à https://fastcomments.com
# Voir configuration.py pour une liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    large_internal_url_sanitized = 'large_internal_url_sanitized_example' # str | 

    try:
        api_response = api_instance.get_gif_large(tenant_id, large_internal_url_sanitized)
        print("The response of PublicApi->get_gif_large:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_gif_large: %s\n" % e)
[inline-code-end]

---