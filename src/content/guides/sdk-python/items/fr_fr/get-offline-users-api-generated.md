Commentateurs passés sur la page qui ne sont PAS actuellement en ligne. Triés par displayName.
À utiliser après avoir épuisé /users/online pour afficher une section « Membres ».
Pagination par curseur sur commenterName : le serveur parcourt l'index partiel {tenantId, urlId, commenterName}
à partir de afterName via $gt, sans coût de $skip.

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Identifiant de l'URL de la page (nettoyé côté serveur). |
| afterName | string | query | Non | Curseur : passez nextAfterName de la réponse précédente. |
| afterUserId | string | query | Non | Départage du curseur : passez nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les homonymies ne fassent pas perdre d'entrées. |

## Réponse

Renvoie : [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOfflineUsersOptions
from client.models.page_users_offline_response import PageUsersOfflineResponse
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
    url_id = 'url_id_example' # str | Identifiant de l'URL de la page (nettoyé côté serveur).
    after_name = 'after_name_example' # str | Curseur : passez nextAfterName de la réponse précédente. (optional)
    after_user_id = 'after_user_id_example' # str | Départage du curseur : passez nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les homonymies ne fassent pas perdre d'entrées. (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, GetOfflineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]