## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| text-search | string | query | Nee |  |
| byIPFromComment | string | query | Nee |  |
| filters | string | query | Nee |  |
| searchFilters | string | query | Nee |  |
| afterId | string | query | Nee |  |
| demo | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Response

Retourneert: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comment_ids_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_api_ids Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_comment_ids_response import ModerationAPIGetCommentIdsResponse
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en staat standaard op https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (optioneel)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (optioneel)
    filters = 'filters_example' # str |  (optioneel)
    search_filters = 'search_filters_example' # str |  (optioneel)
    after_id = 'after_id_example' # str |  (optioneel)
    demo = True # bool |  (optioneel)
    sso = 'sso_example' # str |  (optioneel)

    try:
        api_response = api_instance.get_api_ids(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, after_id=after_id, demo=demo, sso=sso)
        print("The response of ModerationApi->get_api_ids:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_ids: %s\n" % e)
[inline-code-end]