req
tenantId
urlId

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
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

## Отговор

Връща: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_comments_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Задаването на хоста е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък с всички поддържани параметри на конфигурацията.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте инстанция на класа API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (по избор)
    direction = client.SortDirections() # SortDirections |  (по избор)
    sso = 'sso_example' # str |  (по избор)
    skip = 56 # int |  (по избор)
    skip_children = 56 # int |  (по избор)
    limit = 56 # int |  (по избор)
    limit_children = 56 # int |  (по избор)
    count_children = True # bool |  (по избор)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (по избор)
    include_config = True # bool |  (по избор)
    count_all = True # bool |  (по избор)
    includei10n = True # bool |  (по избор)
    locale = 'locale_example' # str |  (по избор)
    modules = 'modules_example' # str |  (по избор)
    is_crawler = True # bool |  (по избор)
    include_notification_count = True # bool |  (по избор)
    as_tree = True # bool |  (по избор)
    max_tree_depth = 56 # int |  (по избор)
    use_full_translation_ids = True # bool |  (по избор)
    parent_id = 'parent_id_example' # str |  (по избор)
    search_text = 'search_text_example' # str |  (по избор)
    hash_tags = ['hash_tags_example'] # List[str] |  (по избор)
    user_id = 'user_id_example' # str |  (по избор)
    custom_config_str = 'custom_config_str_example' # str |  (по избор)
    after_comment_id = 'after_comment_id_example' # str |  (по избор)
    before_comment_id = 'before_comment_id_example' # str |  (по избор)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]