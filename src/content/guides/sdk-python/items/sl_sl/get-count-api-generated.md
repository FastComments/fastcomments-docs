## Parametri

| Name | Type | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| text-search | string | query | Ne |  |
| byIPFromComment | string | query | Ne |  |
| filter | string | query | Ne |  |
| searchFilters | string | query | Ne |  |
| demo | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_count_comments_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_count'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_count_comments_response import ModerationAPICountCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco API razreda
    api_instance = client.ModerationApi(api_client)
    text_search = 'text_search_example' # str |  (neobvezno)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (neobvezno)
    filter = 'filter_example' # str |  (neobvezno)
    search_filters = 'search_filters_example' # str |  (neobvezno)
    demo = True # bool |  (neobvezno)
    sso = 'sso_example' # str |  (neobvezno)

    try:
        api_response = api_instance.get_count(text_search=text_search, by_ip_from_comment=by_ip_from_comment, filter=filter, search_filters=search_filters, demo=demo, sso=sso)
        print("The response of ModerationApi->get_count:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_count: %s\n" % e)
[inline-code-end]