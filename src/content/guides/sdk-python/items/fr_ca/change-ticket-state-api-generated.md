## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | Yes |  |
| id | string | path | Yes |  |

## Réponse

Renvoie : [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/change_ticket_state200_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de change_ticket_state'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.change_ticket_state200_response import ChangeTicketState200Response
from client.models.change_ticket_state_body import ChangeTicketStateBody
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut https://fastcomments.com
# Consultez configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci‑dessous, utilisez l'exemple qui
# répond à votre cas d'utilisation d'authentification.

# Configurer l'autorisation par clé API : api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommentez ci‑dessous pour définir un préfixe (p. ex. Bearer) pour la clé API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str | 
    id = 'id_example' # str | 
    change_ticket_state_body = client.ChangeTicketStateBody() # ChangeTicketStateBody | 

    try:
        api_response = api_instance.change_ticket_state(tenant_id, user_id, id, change_ticket_state_body)
        print("The response of DefaultApi->change_ticket_state:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->change_ticket_state: %s\n" % e)
[inline-code-end]