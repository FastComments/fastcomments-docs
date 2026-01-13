## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| urlId | string | query | Non |  |
| userId | string | query | Non |  |
| startDate | string | query | Non |  |
| questionId | string | query | Non |  |
| questionIds | string | query | Non |  |
| skip | number | query | Non |  |

## Réponse

Renvoie: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_results200_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_question_results'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_results200_response import GetQuestionResults200Response
from client.rest import ApiException
from pprint import pprint

# Définir l'hôte est optionnel et par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Le client doit configurer les paramètres d'authentification et d'autorisation
# conformément à la politique de sécurité du serveur API.
# Des exemples pour chaque méthode d'authentification sont fournis ci-dessous, utilisez celui
# qui correspond à votre cas d'utilisation d'authentification.

# Configurer l'autorisation par clé API : api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Décommentez ci-dessous pour régler le préfixe (p.ex. Bearer) pour la clé API, si nécessaire
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créez une instance de la classe API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str |  (optionnel)
    user_id = 'user_id_example' # str |  (optionnel)
    start_date = 'start_date_example' # str |  (optionnel)
    question_id = 'question_id_example' # str |  (optionnel)
    question_ids = 'question_ids_example' # str |  (optionnel)
    skip = 3.4 # float |  (optionnel)

    try:
        api_response = api_instance.get_question_results(tenant_id, url_id=url_id, user_id=user_id, start_date=start_date, question_id=question_id, question_ids=question_ids, skip=skip)
        print("The response of DefaultApi->get_question_results:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_results: %s\n" % e)
[inline-code-end]