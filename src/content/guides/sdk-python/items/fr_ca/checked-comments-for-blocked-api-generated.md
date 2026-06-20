## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| commentIds | string | query | Oui | Une liste d'identifiants de commentaires séparés par des virgules. |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/check_blocked_comments_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de checked_comments_for_blocked'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.check_blocked_comments_response import CheckBlockedCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Définir l'hôte est optionnel et la valeur par défaut est https://fastcomments.com
# Consultez configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_ids = 'comment_ids_example' # str | Une liste d'identifiants de commentaires séparés par des virgules.
    sso = 'sso_example' # str |  (optionnel)

    try:
        api_response = api_instance.checked_comments_for_blocked(tenant_id, comment_ids, sso=sso)
        print("The response of PublicApi->checked_comments_for_blocked:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->checked_comments_for_blocked: %s\n" % e)
[inline-code-end]