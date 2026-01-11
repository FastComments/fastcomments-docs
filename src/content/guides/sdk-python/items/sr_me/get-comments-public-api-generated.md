req
tenantId
urlId

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| page | integer | query | Не |  |
| direction | string | query | Не |  |
| sso | string | query | Не |  |
| skip | integer | query | Не |  |
| skipChildren | integer | query | Не |  |
| limit | integer | query | Не |  |
| limitChildren | integer | query | Не |  |
| countChildren | boolean | query | Не |  |
| fetchPageForCommentId | string | query | Не |  |
| includeConfig | boolean | query | Не |  |
| countAll | boolean | query | Не |  |
| includei10n | boolean | query | Не |  |
| locale | string | query | Не |  |
| modules | string | query | Не |  |
| isCrawler | boolean | query | Не |  |
| includeNotificationCount | boolean | query | Не |  |
| asTree | boolean | query | Не |  |
| maxTreeDepth | integer | query | Не |  |
| useFullTranslationIds | boolean | query | Не |  |
| parentId | string | query | Не |  |
| searchText | string | query | Не |  |
| hashTags | array | query | Не |  |
| userId | string | query | Не |  |
| customConfigStr | string | query | Не |  |
| afterCommentId | string | query | Не |  |
| beforeCommentId | string | query | Не |  |

## Одговор

Враћа: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_comments_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Подешавање host-а је опционално и подразумевано је на https://fastcomments.com
# Погледајте configuration.py за список свих подржаних параметара конфигурације.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (опционално)
    direction = client.SortDirections() # SortDirections |  (опционално)
    sso = 'sso_example' # str |  (опционално)
    skip = 56 # int |  (опционално)
    skip_children = 56 # int |  (опционално)
    limit = 56 # int |  (опционално)
    limit_children = 56 # int |  (опционално)
    count_children = True # bool |  (опционално)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (опционално)
    include_config = True # bool |  (опционално)
    count_all = True # bool |  (опционално)
    includei10n = True # bool |  (опционално)
    locale = 'locale_example' # str |  (опционално)
    modules = 'modules_example' # str |  (опционално)
    is_crawler = True # bool |  (опционално)
    include_notification_count = True # bool |  (опционално)
    as_tree = True # bool |  (опционално)
    max_tree_depth = 56 # int |  (опционално)
    use_full_translation_ids = True # bool |  (опционално)
    parent_id = 'parent_id_example' # str |  (опционално)
    search_text = 'search_text_example' # str |  (опционално)
    hash_tags = ['hash_tags_example'] # List[str] |  (опционално)
    user_id = 'user_id_example' # str |  (опционално)
    custom_config_str = 'custom_config_str_example' # str |  (опционално)
    after_comment_id = 'after_comment_id_example' # str |  (опционално)
    before_comment_id = 'before_comment_id_example' # str |  (опционално)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]