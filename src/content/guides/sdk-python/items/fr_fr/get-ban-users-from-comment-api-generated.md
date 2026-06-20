---
## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Oui |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_banned_users_from_comment_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple pour get_ban_users_from_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_banned_users_from_comment_response import GetBannedUsersFromCommentResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et la valeur par défaut est https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ouvrez un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (optionnel)

    try:
        api_response = api_instance.get_ban_users_from_comment(comment_id, sso=sso)
        print("The response of ModerationApi->get_ban_users_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_ban_users_from_comment: %s\n" % e)
[inline-code-end]

---