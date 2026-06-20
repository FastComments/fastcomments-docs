## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| page | number | query | Ne |  |
| count | number | query | Ne |  |
| text-search | string | query | Ne |  |
| byIPFromComment | string | query | Ne |  |
| filters | string | query | Ne |  |
| searchFilters | string | query | Ne |  |
| sorts | string | query | Ne |  |
| demo | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrača: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/moderation_api_get_comments_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_api_comments'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.moderation_api_get_comments_response import ModerationAPIGetCommentsResponse
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco odjemalca API
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.ModerationApi(api_client)
    page = 3.4 # float |  (neobvezno)
    count = 3.4 # float |  (neobvezno)
    text_search = 'text_search_example' # str |  (neobvezno)
    by_ip_from_comment = 'by_ip_from_comment_example' # str |  (neobvezno)
    filters = 'filters_example' # str |  (neobvezno)
    search_filters = 'search_filters_example' # str |  (neobvezno)
    sorts = 'sorts_example' # str |  (neobvezno)
    demo = True # bool |  (neobvezno)
    sso = 'sso_example' # str |  (neobvezno)

    try:
        api_response = api_instance.get_api_comments(page=page, count=count, text_search=text_search, by_ip_from_comment=by_ip_from_comment, filters=filters, search_filters=search_filters, sorts=sorts, demo=demo, sso=sso)
        print("The response of ModerationApi->get_api_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_api_comments: %s\n" % e)
[inline-code-end]