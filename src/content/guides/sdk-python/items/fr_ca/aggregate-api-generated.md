Agrège des documents en les groupant (si groupBy est fourni) et en appliquant plusieurs opérations.
Différentes opérations (p. ex. sum, countDistinct, avg, etc.) sont prises en charge.

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| parentTenantId | string | query | Non |  |
| includeStats | boolean | query | Non |  |

## Réponse

Renvoie : [`AggregationResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregation_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple pour aggregate'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregation_request import AggregationRequest
from client.models.aggregation_response import AggregationResponse
from client.rest import ApiException
from pprint import pprint

# Définir l'hôte est optionnel et la valeur par défaut est https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur d'API.
# Des exemples pour chaque méthode d'authentification sont fournis ci‑dessous; utilisez l'exemple qui
# correspond à votre cas d'utilisation d'authentification.

# Configurer l'authentification par clé API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommenter ci‑dessous pour définir un préfixe (p. ex. Bearer) pour la clé API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    aggregation_request = client.AggregationRequest() # AggregationRequest | 
    parent_tenant_id = 'parent_tenant_id_example' # str |  (optionnel)
    include_stats = True # bool |  (optionnel)

    try:
        api_response = api_instance.aggregate(tenant_id, aggregation_request, parent_tenant_id=parent_tenant_id, include_stats=include_stats)
        print("The response of DefaultApi->aggregate:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate: %s\n" % e)
[inline-code-end]