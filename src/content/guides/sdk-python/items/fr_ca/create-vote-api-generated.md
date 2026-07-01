## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|-------------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | query | Oui |  |
| direction | string | query | Oui |  |
| userId | string | query | Non |  |
| anonUserId | string | query | Non |  |

## Réponse

Retourne : [`VoteResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/vote_response.py)

## Exemple

[inline-code-attrs-start title = 'create_vote Exemple'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import CreateVoteOptions
from client.models.vote_response import VoteResponse
from client.rest import ApiException
from pprint import pprint

# Définir l'hôte est facultatif et utilise https://fastcomments.com par défaut
# Voir configuration.py pour une liste de tous les paramètres de configuration pris en charge.
# Le client doit configurer les paramètres d'authentification et d'autorisation conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci‑dessous ; utilisez l'exemple qui
# correspond à votre cas d'utilisation d'authentification.

# Configurer l'autorisation par clé API : api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommentez ci‑dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str | 
    direction = 'direction_example' # str | 
    user_id = 'user_id_example' # str |  (facultatif)
    anon_user_id = 'anon_user_id_example' # str |  (facultatif)

    try:
        api_response = api_instance.create_vote(tenant_id, comment_id, direction, CreateVoteOptions(user_id=user_id, anon_user_id=anon_user_id))
        print("La réponse de DefaultApi->create_vote :\n")
        pprint(api_response)
    except Exception as e:
        print("Exception lors de l'appel de DefaultApi->create_vote : %s\n" % e)
[inline-code-end]