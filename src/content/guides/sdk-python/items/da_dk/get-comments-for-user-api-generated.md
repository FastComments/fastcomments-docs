## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| userId | string | query | No |  |
| direction | string | query | No |  |
| repliesToUserId | string | query | No |  |
| page | number | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| isCrawler | boolean | query | No |  |

## Svar

Returnerer: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_comments_for_user Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetCommentsForUserOptions
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Definering af værten er valgfri og har som standard https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Indtast en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (valgfri)
    direction = client.SortDirections() # SortDirections |  (valgfri)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (valgfri)
    page = 3.4 # float |  (valgfri)
    includei10n = True # bool |  (valgfri)
    locale = 'locale_example' # str |  (valgfri)
    is_crawler = True # bool |  (valgfri)

    try:
        api_response = api_instance.get_comments_for_user(GetCommentsForUserOptions(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler))
        print("Responsen fra PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Undtagelse ved kald af PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]