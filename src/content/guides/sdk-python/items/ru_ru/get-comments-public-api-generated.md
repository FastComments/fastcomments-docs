req
tenantId
urlId

## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|------------|----------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| page | integer | query | No |  |
| direction | string | query | No |  |
| sso | string | query | No |  |
| skip | integer | query | No |  |
| skipChildren | integer | query | No |  |
| limit | integer | query | No |  |
| limitChildren | integer | query | No |  |
| countChildren | boolean | query | No |  |
| fetchPageForCommentId | string | query | No |  |
| includeConfig | boolean | query | No |  |
| countAll | boolean | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| modules | string | query | No |  |
| isCrawler | boolean | query | No |  |
| includeNotificationCount | boolean | query | No |  |
| asTree | boolean | query | No |  |
| maxTreeDepth | integer | query | No |  |
| useFullTranslationIds | boolean | query | No |  |
| parentId | string | query | No |  |
| searchText | string | query | No |  |
| hashTags | array | query | No |  |
| userId | string | query | No |  |
| customConfigStr | string | query | No |  |
| afterCommentId | string | query | No |  |
| beforeCommentId | string | query | No |  |

## Ответ

Returns: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_response_with_presence_public_comment.py)

## Пример

[inline-code-attrs-start title = 'Пример get_comments_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetCommentsPublicOptions
from client.models.get_comments_response_with_presence_public_comment import GetCommentsResponseWithPresencePublicComment
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Определение хоста необязательно и по умолчанию равно https://fastcomments.com
# См. configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (опционально)
    direction = client.SortDirections() # SortDirections |  (опционально)
    sso = 'sso_example' # str |  (опционально)
    skip = 56 # int |  (опционально)
    skip_children = 56 # int |  (опционально)
    limit = 56 # int |  (опционально)
    limit_children = 56 # int |  (опционально)
    count_children = True # bool |  (опционально)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (опционально)
    include_config = True # bool |  (опционально)
    count_all = True # bool |  (опционально)
    includei10n = True # bool |  (опционально)
    locale = 'locale_example' # str |  (опционально)
    modules = 'modules_example' # str |  (опционально)
    is_crawler = True # bool |  (опционально)
    include_notification_count = True # bool |  (опционально)
    as_tree = True # bool |  (опционально)
    max_tree_depth = 56 # int |  (опционально)
    use_full_translation_ids = True # bool |  (опционально)
    parent_id = 'parent_id_example' # str |  (опционально)
    search_text = 'search_text_example' # str |  (опционально)
    hash_tags = ['hash_tags_example'] # List[str] |  (опционально)
    user_id = 'user_id_example' # str |  (опционально)
    custom_config_str = 'custom_config_str_example' # str |  (опционально)
    after_comment_id = 'after_comment_id_example' # str |  (опционально)
    before_comment_id = 'before_comment_id_example' # str |  (опционально)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, GetCommentsPublicOptions(page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id))
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]