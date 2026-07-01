List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| cursor | string | query | Non | Curseur de pagination opaque renvoyé comme `nextCursor` à partir d’une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | Non | 1..200, par défaut 50 (facultatif) |
| q | string | query | Non | Filtre optionnel de préfixe de titre insensible à la casse. |
| sortBy | string | query | Non | Ordre de tri. `updatedAt` (par défaut, le plus récent d’abord), `commentCount` (le plus de commentaires d’abord), ou `title` (alphabétique). |
| hasComments | boolean | query | Non | Si vrai, ne renvoie que les pages contenant au moins un commentaire. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Example

[inline-code-attrs-start title = 'Exemple get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetPagesPublicOptions
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# La définition de l’hôte est facultative et utilise https://fastcomments.com par défaut
# Voir configuration.py pour une liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Curseur de pagination opaque renvoyé comme `nextCursor` à partir d’une requête précédente. Lié au même `sortBy`. (facultatif)
    limit = 56 # int | 1..200, par défaut 50 (facultatif)
    q = 'q_example' # str | Filtre optionnel de préfixe de titre insensible à la casse. (facultatif)
    sort_by = client.PagesSortBy() # PagesSortBy | Ordre de tri. `updatedAt` (par défaut, le plus récent d’abord), `commentCount` (le plus de commentaires d’abord), ou `title` (alphabétique). (facultatif)
    has_comments = True # bool | Si vrai, ne renvoie que les pages contenant au moins un commentaire. (facultatif)

    try:
        api_response = api_instance.get_pages_public(tenant_id, GetPagesPublicOptions(cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments))
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]