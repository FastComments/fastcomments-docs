## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Non |  |
| byIPFromComment | string | query | Non |  |
| filter | string | query | Non |  |
| searchFilters | string | query | Non |  |
| demo | boolean | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_count_comments_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_count_comments_response import ModerationAPICountCommentsResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et a pour valeur par défaut https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrez dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (optionnel)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (optionnel)
    filter = 'filter_example' # str |  (optionnel)
    search_filters = 'search_filters_example' # str |  (optionnel)
    demo = True # bool |  (optionnel)
    sso = 'sso_example' # str |  (optionnel)

    try:
        api_response = api_instance.get_count(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filter=filter, search_filters=search_filters, demo=demo, sso=sso)
        print("The response of ModerationApi->get_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_count: %s\n" % e)
[inline-code-end]

---