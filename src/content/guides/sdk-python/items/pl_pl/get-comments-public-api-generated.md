req
tenantId
urlId

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak |  |
| page | integer | query | Nie |  |
| direction | string | query | Nie |  |
| sso | string | query | Nie |  |
| skip | integer | query | Nie |  |
| skipChildren | integer | query | Nie |  |
| limit | integer | query | Nie |  |
| limitChildren | integer | query | Nie |  |
| countChildren | boolean | query | Nie |  |
| fetchPageForCommentId | string | query | Nie |  |
| includeConfig | boolean | query | Nie |  |
| countAll | boolean | query | Nie |  |
| includei10n | boolean | query | Nie |  |
| locale | string | query | Nie |  |
| modules | string | query | Nie |  |
| isCrawler | boolean | query | Nie |  |
| includeNotificationCount | boolean | query | Nie |  |
| asTree | boolean | query | Nie |  |
| maxTreeDepth | integer | query | Nie |  |
| useFullTranslationIds | boolean | query | Nie |  |
| parentId | string | query | Nie |  |
| searchText | string | query | Nie |  |
| hashTags | array | query | Nie |  |
| userId | string | query | Nie |  |
| customConfigStr | string | query | Nie |  |
| afterCommentId | string | query | Nie |  |
| beforeCommentId | string | query | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_response_with_presence_public_comment.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_comments_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_response_with_presence_public_comment import GetCommentsResponseWithPresencePublicComment
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, żeby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
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