## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| urlId | string | query | Oui |  |
| userId | string | query | Non |  |
| anonUserId | string | query | Non |  |

## Réponse

Renvoie : [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_votes_for_user_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple get_votes_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_votes_for_user_response import GetVotesForUserResponse
from client.rest import ApiException
from pprint import pprint

# Définir l'hôte est optionnel et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci‑dessous, utilisez celui qui
# correspond à votre cas d'utilisation d'authentification.

# Configuration de l'autorisation par clé API : api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommentez ci‑dessous pour définir un préfixe (ex. Bearer) pour la clé API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    user_id = 'user_id_example' # str |  (optionnel)
    anon_user_id = 'anon_user_id_example' # str |  (optionnel)

    try:
        api_response = api_instance.get_votes_for_user(tenant_id, url_id, user_id=user_id, anon_user_id=anon_user_id)
        print("The response of DefaultApi->get_votes_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_votes_for_user: %s\n" % e)
[inline-code-end]