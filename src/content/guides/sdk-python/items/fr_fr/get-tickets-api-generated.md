## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| state | number | query | No |  |
| skip | number | query | No |  |
| limit | number | query | No |  |

## Réponse

Retourne: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tickets200_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple get_tickets'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tickets200_response import GetTickets200Response
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut à https://fastcomments.com
# Voir configuration.py pour une liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci-dessous, utilisez l'exemple qui
# correspond à votre cas d'utilisation d'authentification.

# Configurer l'autorisation par clé API : api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommentez ci-dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (facultatif)
    state = 3.4 # float |  (facultatif)
    skip = 3.4 # float |  (facultatif)
    limit = 3.4 # float |  (facultatif)

    try:
        api_response = api_instance.get_tickets(tenant_id, user_id=user_id, state=state, skip=skip, limit=limit)
        print("The response of DefaultApi->get_tickets:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tickets: %s\n" % e)
[inline-code-end]