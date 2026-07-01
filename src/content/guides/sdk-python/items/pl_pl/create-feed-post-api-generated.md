## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|------------|----------|------|
| tenantId | string | query | Tak |  |
| broadcastId | string | query | Nie |  |
| isLive | boolean | query | Nie |  |
| doSpamCheck | boolean | query | Nie |  |
| skipDupCheck | boolean | query | Nie |  |

## Odpowiedź

Zwraca: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_posts_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład create_feed_post'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import CreateFeedPostOptions
from client.models.create_feed_post_params import CreateFeedPostParams
from client.models.create_feed_posts_response import CreateFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# Definiowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz plik configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracyjnych.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji zgodnie z polityką bezpieczeństwa serwera API.
# Przykłady dla każdej metody uwierzytelniania są podane poniżej; użyj przykładu, który spełnia Twój przypadek użycia.

# Skonfiguruj autoryzację kluczem API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli jest potrzebny
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_feed_post_params = client.CreateFeedPostParams() # CreateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  (optional)
    is_live = True # bool |  (optional)
    do_spam_check = True # bool |  (optional)
    skip_dup_check = True # bool |  (optional)

    try:
        api_response = api_instance.create_feed_post(tenant_id, create_feed_post_params, CreateFeedPostOptions(broadcast_id=broadcast_id, is_live=is_live, do_spam_check=do_spam_check, skip_dup_check=skip_dup_check))
        print("The response of DefaultApi->create_feed_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_feed_post: %s\n" % e)
[inline-code-end]