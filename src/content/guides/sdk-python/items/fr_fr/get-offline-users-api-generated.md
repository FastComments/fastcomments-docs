Past commenters on the page who are NOT currently online. Sorted by displayName.  
Commentateurs précédents sur la page qui ne sont PAS actuellement en ligne. Triés par displayName.

Use this after exhausting /users/online to render a "Members" section.  
Utilisez cela après avoir épuisé /users/online pour afficher une section "Membres".

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
Pagination par curseur sur commenterName : le serveur parcourt l’index partiel {tenantId, urlId, commenterName} à partir de afterName en avant via $gt, sans coût $skip.

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifiant d'URL de page (nettoyé côté serveur). |
| afterName | string | query | No | Curseur : passer nextAfterName de la réponse précédente. |
| afterUserId | string | query | No | Critère de désaccord du curseur : passer nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom ne suppriment pas d'entrées. |

## Réponse

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Définir l'hôte est facultatif et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identifiant d'URL de page (nettoyé côté serveur).
    after_name = 'after_name_example' # str | Curseur : passer nextAfterName de la réponse précédente. (optional)
    after_user_id = 'after_user_id_example' # str | Critère de désaccord du curseur : passer nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom ne suppriment pas d'entrées. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]