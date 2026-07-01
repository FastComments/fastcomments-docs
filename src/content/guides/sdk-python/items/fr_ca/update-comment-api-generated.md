## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|-------------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |
| contextUserId | string | query | Non |  |
| doSpamCheck | boolean | query | Non |  |
| isLive | boolean | query | Non |  |

## Réponse

Retourne : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de update_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import UpdateCommentOptions
from client.models.api_empty_response import APIEmptyResponse
from client.models.updatable_comment_params import UpdatableCommentParams
from client.rest import ApiException
from pprint import pprint

# Définir l'hôte est facultatif et par défaut https://fastcomments.com
# Voir configuration.py pour une liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci‑dessous, utilisez l'exemple qui
# répond à votre cas d'utilisation d'authentification.

# Configurer l'autorisation par clé d'API : api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommentez ci‑dessous pour configurer le préfixe (p. ex. Bearer) de la clé d'API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    updatable_comment_params = client.UpdatableCommentParams() # UpdatableCommentParams | 
    context_user_id = 'context_user_id_example' # str |  (optional)
    do_spam_check = True # bool |  (optional)
    is_live = True # bool |  (optional)

    try:
        api_response = api_instance.update_comment(tenant_id, id, updatable_comment_params, UpdateCommentOptions(context_user_id=context_user_id, do_spam_check=do_spam_check, is_live=is_live))
        print("The response of DefaultApi->update_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_comment: %s\n" % e)
[inline-code-end]