## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| userId | string | query | No |  |
| direction | string | query | No |  |
| repliesToUserId | string | query | No |  |
| page | number | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| isCrawler | boolean | query | No |  |

## Odgovor

Vraća: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## Primjer

[inline-code-attrs-start title = 'get_comments_for_user Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetCommentsForUserOptions
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Unesite kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Stvaranje instance API klase
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (opcionalno)
    direction = client.SortDirections() # SortDirections |  (opcionalno)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (opcionalno)
    page = 3.4 # float |  (opcionalno)
    includei10n = True # bool |  (opcionalno)
    locale = 'locale_example' # str |  (opcionalno)
    is_crawler = True # bool |  (opcionalno)

    try:
        api_response = api_instance.get_comments_for_user(GetCommentsForUserOptions(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler))
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]