Lister les pages pour un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salons.
Requiert que `enableFChat` soit vrai dans la configuration personnalisée résolue pour chaque page.
Les pages qui nécessitent SSO sont filtrées en fonction de l'accès de groupe de l'utilisateur demandeur.

## Parameters

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| cursor | string | query | Non | Curseur de pagination opaque retourné comme `nextCursor` d'une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | Non | 1..200, par défaut 50 |
| q | string | query | Non | Filtre optionnel de préfixe de titre insensible à la casse. |
| sortBy | string | query | Non | Ordre de tri. `updatedAt` (par défaut, les plus récents en premier), `commentCount` (les plus commentés en premier), ou `title` (alphabétique). |
| hasComments | boolean | query | Non | Si vrai, ne retourner que les pages avec au moins un commentaire. |

## Response

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Example

[inline-code-attrs-start title = 'Exemple de get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et la valeur par défaut est https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Curseur de pagination opaque retourné comme `nextCursor` d'une requête précédente. Lié au même `sortBy`. (optionnel)
    limit = 56 # int | 1..200, par défaut 50 (optionnel)
    q = 'q_example' # str | Filtre optionnel de préfixe de titre insensible à la casse. (optionnel)
    sort_by = client.PagesSortBy() # PagesSortBy | Ordre de tri. `updatedAt` (par défaut, les plus récents en premier), `commentCount` (les plus commentés en premier), ou `title` (alphabétique). (optionnel)
    has_comments = True # bool | Si vrai, ne retourner que les pages avec au moins un commentaire. (optionnel)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]