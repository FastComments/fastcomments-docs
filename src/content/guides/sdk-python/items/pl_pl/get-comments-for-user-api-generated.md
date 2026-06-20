---
## Parametry

| Name | Type | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| userId | string | query | Nie |  |
| direction | string | query | Nie |  |
| repliesToUserId | string | query | Nie |  |
| page | number | query | Nie |  |
| includei10n | boolean | query | Nie |  |
| locale | string | query | Nie |  |
| isCrawler | boolean | query | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia get_comments_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (opcjonalne)
    direction = client.SortDirections() # SortDirections |  (opcjonalne)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (opcjonalne)
    page = 3.4 # float |  (opcjonalne)
    includei10n = True # bool |  (opcjonalne)
    locale = 'locale_example' # str |  (opcjonalne)
    is_crawler = True # bool |  (opcjonalne)

    try:
        api_response = api_instance.get_comments_for_user(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler)
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]

---