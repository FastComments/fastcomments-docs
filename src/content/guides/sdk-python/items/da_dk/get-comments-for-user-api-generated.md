## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| userId | string | query | Nej |  |
| direction | string | query | Nej |  |
| repliesToUserId | string | query | Nej |  |
| page | number | query | Nej |  |
| includei10n | boolean | query | Nej |  |
| locale | string | query | Nej |  |
| isCrawler | boolean | query | Nej |  |

## Svar

Returnerer: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_comments_for_user Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Definering af host er valgfri og standardværdien er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Opret en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (optional)
    direction = client.SortDirections() # SortDirections |  (optional)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (optional)
    page = 3.4 # float |  (optional)
    includei10n = True # bool |  (optional)
    locale = 'locale_example' # str |  (optional)
    is_crawler = True # bool |  (optional)

    try:
        api_response = api_instance.get_comments_for_user(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler)
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]