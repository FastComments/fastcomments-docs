## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Non |  |
| byIPFromComment | string | query | Non |  |
| filters | string | query | Non |  |
| searchFilters | string | query | Non |  |
| sorts | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_export_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple de post_api_export'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_export_response import ModerationExportResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut à https://fastcomments.com
# Voir configuration.py pour une liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (facultatif)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (facultatif)
    filters = 'filters_example' # str |  (facultatif)
    search_filters = 'search_filters_example' # str |  (facultatif)
    sorts = 'sorts_example' # str |  (facultatif)
    sso = 'sso_example' # str |  (facultatif)

    try:
        api_response = api_instance.post_api_export(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, sso=sso)
        print("The response of ModerationApi->post_api_export:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_api_export: %s\n" % e)
[inline-code-end]