## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Nee |  |
| direction | string | query | Nee |  |
| repliesToUserId | string | query | Nee |  |
| page | number | query | Nee |  |
| includei10n | boolean | query | Nee |  |
| locale | string | query | Nee |  |
| isCrawler | boolean | query | Nee |  |

## Antwoord

Retourneert: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_comments_for_user Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Het instellen van de host is optioneel en standaard is https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (optioneel)
    direction = client.SortDirections() # SortDirections |  (optioneel)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (optioneel)
    page = 3.4 # float |  (optioneel)
    includei10n = True # bool |  (optioneel)
    locale = 'locale_example' # str |  (optioneel)
    is_crawler = True # bool |  (optioneel)

    try:
        api_response = api_instance.get_comments_for_user(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler)
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]