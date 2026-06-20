## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| forceRecalculate | boolean | query | Non |  |

## Réponse

Renvoie : [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/bulk_aggregate_question_results_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de bulk_aggregate_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.bulk_aggregate_question_results_request import BulkAggregateQuestionResultsRequest
from client.models.bulk_aggregate_question_results_response import BulkAggregateQuestionResultsResponse
from client.rest import ApiException
from pprint import pprint

# Définir l'hôte est optionnel et par défaut c'est https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci‑dessous, utilisez l'exemple qui
# correspond à votre cas d'utilisation d'authentification.
# Configurer l'autorisation par clé API: api_key
# Décommentez ci‑dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    bulk_aggregate_question_results_request = client.BulkAggregateQuestionResultsRequest() # BulkAggregateQuestionResultsRequest | 
    force_recalculate = True # bool |  (optionnel)

    try:
        api_response = api_instance.bulk_aggregate_question_results(tenant_id, bulk_aggregate_question_results_request, force_recalculate=force_recalculate)
        print("The response of DefaultApi->bulk_aggregate_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->bulk_aggregate_question_results: %s\n" % e)
[inline-code-end]