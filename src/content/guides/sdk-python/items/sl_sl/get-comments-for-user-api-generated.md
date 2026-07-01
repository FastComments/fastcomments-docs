## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| userId | string | query | No |  |
| direction | string | query | No |  |
| repliesToUserId | string | query | No |  |
| page | number | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| isCrawler | boolean | query | No |  |

## Odgovor

Vrne: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_comments_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetCommentsForUserOptions
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in privzeto nastavljeno na https://fastcomments.com
# Glej configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco API razreda
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (neobvezno)
    direction = client.SortDirections() # SortDirections |  (neobvezno)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (neobvezno)
    page = 3.4 # float |  (neobvezno)
    includei10n = True # bool |  (neobvezno)
    locale = 'locale_example' # str |  (neobvezno)
    is_crawler = True # bool |  (neobvezno)

    try:
        api_response = api_instance.get_comments_for_user(GetCommentsForUserOptions(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler))
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]