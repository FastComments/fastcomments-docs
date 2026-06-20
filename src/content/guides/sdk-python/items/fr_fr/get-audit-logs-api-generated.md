## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| limit | number | query | No |  |
| skip | number | query | No |  |
| order | string | query | No |  |
| after | number | query | No |  |
| before | number | query | No |  |

## Réponse

Renvoie : [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_audit_logs_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple get_audit_logs'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_audit_logs_response import GetAuditLogsResponse
from client.models.sortdir import SORTDIR
from client.rest import ApiException
from pprint import pprint

# Définir l'hôte est optionnel et la valeur par défaut est https://fastcomments.com
# Voir configuration.py pour une liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci-dessous, utilisez l'exemple qui
# correspond à votre cas d'utilisation d'authentification.

# Configurer l'autorisation par clé API : api_key
# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# Décommentez ci-dessous pour configurer un préfixe (p. ex. Bearer) pour la clé API, si nécessaire

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    limit = 3.4 # float |  (optionnel)
    skip = 3.4 # float |  (optionnel)
    order = client.SORTDIR() # SORTDIR |  (optionnel)
    after = 3.4 # float |  (optionnel)
    before = 3.4 # float |  (optionnel)

    try:
        api_response = api_instance.get_audit_logs(tenant_id, limit=limit, skip=skip, order=order, after=after, before=before)
        print("The response of DefaultApi->get_audit_logs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_audit_logs: %s\n" % e)
[inline-code-end]