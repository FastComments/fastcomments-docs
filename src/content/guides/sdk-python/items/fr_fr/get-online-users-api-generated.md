Spectateurs actuellement en ligne d'une page : personnes dont la session websocket est abonnée à la page en ce moment.
Retourne anonCount + totalCount (abonnés de la salle, y compris les spectateurs anonymes que nous n'énumérons pas).

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifiant de l'URL de la page (nettoyé côté serveur). |
| afterName | string | query | No | Curseur : passer nextAfterName depuis la réponse précédente. |
| afterUserId | string | query | No | Départage du curseur : passer nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom ne fassent pas disparaître des entrées. |

## Réponse

Renvoie : [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut vaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identifiant de l'URL de la page (nettoyé côté serveur).
    after_name = 'after_name_example' # str | Curseur : passer nextAfterName depuis la réponse précédente. (optionnel)
    after_user_id = 'after_user_id_example' # str | Départage du curseur : passer nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom ne fassent pas disparaître des entrées. (optionnel)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]