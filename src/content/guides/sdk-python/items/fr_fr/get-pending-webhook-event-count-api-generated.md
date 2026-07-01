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

Renvoie : [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_pending_webhook_event_count_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple get_pending_webhook_event_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetPendingWebhookEventCountOptions
from client.models.get_pending_webhook_event_count_response import GetPendingWebhookEventCountResponse
from client.rest import ApiException
from pprint import pprint

# Définir l'hôte est optionnel et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci-dessous, utilisez l'exemple qui
# répond à votre cas d'utilisation d'authentification.

# Configurer l'autorisation par clé API : api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommentez ci-dessous pour configurer le préfixe (p. ex. Bearer) pour la clé API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_id = 'comment_id_example' # str |  (optionnel)
    external_id = 'external_id_example' # str |  (optionnel)
    event_type = 'event_type_example' # str |  (optionnel)
    type = 'type_example' # str |  (optionnel)
    domain = 'domain_example' # str |  (optionnel)
    attempt_count_gt = 3.4 # float |  (optionnel)

    try:
        api_response = api_instance.get_pending_webhook_event_count(tenant_id, GetPendingWebhookEventCountOptions(comment_id=comment_id, external_id=external_id, event_type=event_type, type=type, domain=domain, attempt_count_gt=attempt_count_gt))
        print("The response of DefaultApi->get_pending_webhook_event_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_pending_webhook_event_count: %s\n" % e)
[inline-code-end]