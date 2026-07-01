Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.
Renvoie anonCount + totalCount (abonnés de la salle, y compris les visionneurs anonymes que nous n’énumérons pas).

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | chemin | Oui |  |
| urlId | string | requête | Oui | Identifiant d’URL de la page (nettoyé côté serveur). |
| afterName | string | requête | Non | Curseur : transmettre nextAfterName de la réponse précédente. |
| afterUserId | string | requête | Non | Déterminant de rupture d’égalité du curseur : transmettre nextAfterUserId de la réponse précédente. Obligatoire lorsque afterName est défini afin que les égalités de nom ne suppriment pas d’entrées. |

## Réponse

Renvoie : [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Définir l'hôte est facultatif et la valeur par défaut est https://fastcomments.com
# Voir configuration.py pour une liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identifiant d’URL de la page (nettoyé côté serveur).
    after_name = 'after_name_example' # str | Curseur : transmettre nextAfterName de la réponse précédente. (optionnel)
    after_user_id = 'after_user_id_example' # str | Déterminant de rupture d’égalité du curseur : transmettre nextAfterUserId de la réponse précédente. Obligatoire lorsque afterName est défini afin que les égalités de nom ne suppriment pas d’entrées. (optionnel)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]