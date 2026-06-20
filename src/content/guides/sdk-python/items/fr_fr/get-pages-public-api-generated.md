---
Liste les pages pour un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salons.
Requiert `enableFChat` to be true on the resolved custom config for each page.
Les pages qui nécessitent SSO sont filtrées en fonction des accès de groupe de l'utilisateur demandeur.

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Curseur de pagination opaque renvoyé comme `nextCursor` par une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | No | 1..200, par défaut 50 |
| q | string | query | No | Filtre optionnel de préfixe de titre insensible à la casse. |
| sortBy | string | query | No | Ordre de tri. `updatedAt` (par défaut, plus récent d'abord), `commentCount` (le plus de commentaires en premier), ou `title` (alphabétique). |
| hasComments | boolean | query | No | Si true, ne renvoyer que les pages avec au moins un commentaire. |

## Réponse

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_public_pages_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_pages_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_public_pages_response import GetPublicPagesResponse
from client.models.pages_sort_by import PagesSortBy
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    cursor = 'cursor_example' # str | Curseur de pagination opaque renvoyé comme `nextCursor` par une requête précédente. Lié au même `sortBy`. (optionnel)
    limit = 56 # int | 1..200, par défaut 50 (optionnel)
    q = 'q_example' # str | Filtre optionnel de préfixe de titre insensible à la casse. (optionnel)
    sort_by = client.PagesSortBy() # PagesSortBy | Ordre de tri. `updatedAt` (par défaut, plus récent d'abord), `commentCount` (le plus de commentaires en premier), ou `title` (alphabétique). (optionnel)
    has_comments = True # bool | Si true, ne renvoyer que les pages avec au moins un commentaire. (optionnel)

    try:
        api_response = api_instance.get_pages_public(tenant_id, cursor=cursor, limit=limit, q=q, sort_by=sort_by, has_comments=has_comments)
        print("The response of PublicApi->get_pages_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_pages_public: %s\n" % e)
[inline-code-end]

---