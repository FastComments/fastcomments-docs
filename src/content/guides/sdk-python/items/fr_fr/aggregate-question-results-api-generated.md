## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| questionId | string | query | Non |  |
| questionIds | array | query | Non |  |
| urlId | string | query | Non |  |
| timeBucket | string | query | Non |  |
| startDate | string | query | Non |  |
| forceRecalculate | boolean | query | Non |  |

## Réponse

Renvoie : [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/aggregate_question_results200_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de aggregate_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.aggregate_question_results200_response import AggregateQuestionResults200Response
from client.models.aggregate_time_bucket import AggregateTimeBucket
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci-dessous ; utilisez l'exemple qui
# correspond à votre cas d'utilisation d'authentification.

# Configurer l'autorisation par clé API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommentez ci-dessous pour configurer un préfixe (e.g. Bearer) pour la clé API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    question_id = 'question_id_example' # str |  (optionnel)
    question_ids = ['question_ids_example'] # List[str] |  (optionnel)
    url_id = 'url_id_example' # str |  (optionnel)
    time_bucket = client.AggregateTimeBucket() # AggregateTimeBucket |  (optionnel)
    start_date = '2013-10-20T19:20:30+01:00' # datetime |  (optionnel)
    force_recalculate = True # bool |  (optionnel)

    try:
        api_response = api_instance.aggregate_question_results(tenant_id, question_id=question_id, question_ids=question_ids, url_id=url_id, time_bucket=time_bucket, start_date=start_date, force_recalculate=force_recalculate)
        print("The response of DefaultApi->aggregate_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->aggregate_question_results: %s\n" % e)
[inline-code-end]