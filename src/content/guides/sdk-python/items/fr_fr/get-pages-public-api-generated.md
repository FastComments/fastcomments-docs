List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Nécessite que `enableFChat` soit vrai dans la configuration personnalisée résolue pour chaque page.  
Les pages nécessitant le SSO sont filtrées en fonction de l’accès aux groupes de l'utilisateur demandeur.

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Curseur de pagination opaque retourné comme `nextCursor` d'une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | No | 1..200, par défaut 50 |
| q | string | query | No | Filtre de préfixe de titre optionnel insensible à la casse. |
| sortBy | string | query | No | Ordre de tri. `updatedAt` (par défaut, le plus récent d'abord), `commentCount` (les plus de commentaires d'abord), ou `title` (alphabétique). |
| hasComments | boolean | query | No | Si vrai, ne retourne que les pages contenant au moins un commentaire. |

## Réponse

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Curseur de pagination opaque retourné comme `nextCursor` d'une requête précédente. Lié au même `sortBy`. (optional)
    limit = 56 # int | 1..200, par défaut 50 (optional)
    q = 'q_example' # str | Filtre de préfixe de titre optionnel insensible à la casse. (optional)
    sort_by = client.PagesSortBy() # PagesSortBy | Ordre de tri. `updatedAt` (par défaut, le plus récent d'abord), `commentCount` (les plus de commentaires d'abord), ou `title` (alphabétique). (optional)
    has_comments = True # bool | Si vrai, ne retourne que les pages contenant au moins un commentaire. (optional)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]