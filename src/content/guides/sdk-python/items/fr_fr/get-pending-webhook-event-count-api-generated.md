## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | query | Non |  |
| externalId | string | query | Non |  |
| eventType | string | query | Non |  |
| type | string | query | Non |  |
| domain | string | query | Non |  |
| attemptCountGT | number | query | Non |  |

## Réponse

Renvoie: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_event_count200_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_pending_webhook_event_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_pending_webhook_event_count200_response import GetPendingWebhookEventCount200Response
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut à https://fastcomments.com
# Consultez configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci-dessous, utilisez l'exemple qui
# correspond à votre cas d'utilisation d'authentification.

# Configurer l'authentification par clé API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommentez la ligne ci-dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (optional)
    external_id = 'external_id_example' # str |  (optional)
    event_type = 'event_type_example' # str |  (optional)
    type = 'type_example' # str |  (optional)
    domain = 'domain_example' # str |  (optional)
    attempt_count_gt = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_pending_webhook_event_count(tenant_id, comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt)
        print("The response of DefaultApi->get_pending_webhook_event_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_event_count: %s\n" % e)
[inline-code-end]