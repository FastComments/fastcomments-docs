## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| userId | string | query | Non |  |
| badgeId | string | query | Non |  |
| type | number | query | Non |  |
| displayedOnComments | boolean | query | Non |  |
| limit | number | query | Non |  |
| skip | number | query | Non |  |

## Réponse

Retourne : [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badges_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_user_badges'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badges_response import APIGetUserBadgesResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et a pour valeur par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci-dessous; utilisez celui
# qui correspond à votre cas d'utilisation d'authentification.

# Configurer l'authentification par clé API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommentez ci-dessous pour configurer un préfixe (e.g. Bearer) pour la clé API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (optionnel)
    badge_id = 'badge_id_example' # str |  (optionnel)
    type = 3.4 # float |  (optionnel)
    displayed_on_comments = True # bool |  (optionnel)
    limit = 3.4 # float |  (optionnel)
    skip = 3.4 # float |  (optionnel)

    try:
        api_response = api_instance.get_user_badges(tenant_id, user_id=user_id, badge_id=badge_id, type=type, displayed_on_comments=displayed_on_comments, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badges:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badges: %s\n" % e)
[inline-code-end]