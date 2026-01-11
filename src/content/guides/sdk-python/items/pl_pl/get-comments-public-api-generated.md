żądanie
tenantId
urlId

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | ścieżka | Tak |  |
| urlId | string | zapytanie | Tak |  |
| page | integer | zapytanie | Nie |  |
| direction | string | zapytanie | Nie |  |
| sso | string | zapytanie | Nie |  |
| skip | integer | zapytanie | Nie |  |
| skipChildren | integer | zapytanie | Nie |  |
| limit | integer | zapytanie | Nie |  |
| limitChildren | integer | zapytanie | Nie |  |
| countChildren | boolean | zapytanie | Nie |  |
| fetchPageForCommentId | string | zapytanie | Nie |  |
| includeConfig | boolean | zapytanie | Nie |  |
| countAll | boolean | zapytanie | Nie |  |
| includei10n | boolean | zapytanie | Nie |  |
| locale | string | zapytanie | Nie |  |
| modules | string | zapytanie | Nie |  |
| isCrawler | boolean | zapytanie | Nie |  |
| includeNotificationCount | boolean | zapytanie | Nie |  |
| asTree | boolean | zapytanie | Nie |  |
| maxTreeDepth | integer | zapytanie | Nie |  |
| useFullTranslationIds | boolean | zapytanie | Nie |  |
| parentId | string | zapytanie | Nie |  |
| searchText | string | zapytanie | Nie |  |
| hashTags | array | zapytanie | Nie |  |
| userId | string | zapytanie | Nie |  |
| customConfigStr | string | zapytanie | Nie |  |
| afterCommentId | string | zapytanie | Nie |  |
| beforeCommentId | string | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_comments_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Otwórz kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (opcjonalne)
    direction = client.SortDirections() # SortDirections |  (opcjonalne)
    sso = 'sso_example' # str |  (opcjonalne)
    skip = 56 # int |  (opcjonalne)
    skip_children = 56 # int |  (opcjonalne)
    limit = 56 # int |  (opcjonalne)
    limit_children = 56 # int |  (opcjonalne)
    count_children = True # bool |  (opcjonalne)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (opcjonalne)
    include_config = True # bool |  (opcjonalne)
    count_all = True # bool |  (opcjonalne)
    includei10n = True # bool |  (opcjonalne)
    locale = 'locale_example' # str |  (opcjonalne)
    modules = 'modules_example' # str |  (opcjonalne)
    is_crawler = True # bool |  (opcjonalne)
    include_notification_count = True # bool |  (opcjonalne)
    as_tree = True # bool |  (opcjonalne)
    max_tree_depth = 56 # int |  (opcjonalne)
    use_full_translation_ids = True # bool |  (opcjonalne)
    parent_id = 'parent_id_example' # str |  (opcjonalne)
    search_text = 'search_text_example' # str |  (opcjonalne)
    hash_tags = ['hash_tags_example'] # List[str] |  (opcjonalne)
    user_id = 'user_id_example' # str |  (opcjonalne)
    custom_config_str = 'custom_config_str_example' # str |  (opcjonalne)
    after_comment_id = 'after_comment_id_example' # str |  (opcjonalne)
    before_comment_id = 'before_comment_id_example' # str |  (opcjonalne)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]