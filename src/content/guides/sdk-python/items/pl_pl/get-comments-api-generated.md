## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| page | integer | query | Nie |  |
| limit | integer | query | Nie |  |
| skip | integer | query | Nie |  |
| asTree | boolean | query | Nie |  |
| skipChildren | integer | query | Nie |  |
| limitChildren | integer | query | Nie |  |
| maxTreeDepth | integer | query | Nie |  |
| urlId | string | query | Nie |  |
| userId | string | query | Nie |  |
| anonUserId | string | query | Nie |  |
| contextUserId | string | query | Nie |  |
| hashTag | string | query | Nie |  |
| parentId | string | query | Nie |  |
| direction | string | query | Nie |  |

## Odpowiedź

Zwraca: [`GetComments200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments200_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_comments'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments200_response import GetComments200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Zdefiniowanie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klient musi skonfigurować parametry uwierzytelniania i autoryzacji
# zgodnie z polityką bezpieczeństwa serwera API.
# Przykłady dla każdej metody uwierzytelniania znajdują się poniżej, użyj przykładu który
# odpowiada Twojemu przypadkowi użycia uwierzytelniania.

# Skonfiguruj autoryzację kluczem API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli jest to potrzebne
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Wejdź w kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (opcjonalne)
    limit = 56 # int |  (opcjonalne)
    skip = 56 # int |  (opcjonalne)
    as_tree = True # bool |  (opcjonalne)
    skip_children = 56 # int |  (opcjonalne)
    limit_children = 56 # int |  (opcjonalne)
    max_tree_depth = 56 # int |  (opcjonalne)
    url_id = 'url_id_example' # str |  (opcjonalne)
    user_id = 'user_id_example' # str |  (opcjonalne)
    anon_user_id = 'anon_user_id_example' # str |  (opcjonalne)
    context_user_id = 'context_user_id_example' # str |  (opcjonalne)
    hash_tag = 'hash_tag_example' # str |  (opcjonalne)
    parent_id = 'parent_id_example' # str |  (opcjonalne)
    direction = client.SortDirections() # SortDirections |  (opcjonalne)

    try:
        api_response = api_instance.get_comments(tenant_id, page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction)
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]