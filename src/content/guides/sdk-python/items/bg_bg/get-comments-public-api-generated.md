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

Връща: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_response_with_presence_public_comment.py)

## Пример

[inline-code-attrs-start title = 'Пример за get_comments_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_response_with_presence_public_comment import GetCommentsResponseWithPresencePublicComment
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Задаването на host е по избор и по подразбиране е https://fastcomments.com
# Вижте configuration.py за списък на всички поддържани параметри за конфигурация.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Влезте в контекст с инстанция на API клиента
with client.ApiClient(configuration) as api_client:
    # Създайте екземпляр на API класа
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (незадължително)
    direction = client.SortDirections() # SortDirections |  (незадължително)
    sso = 'sso_example' # str |  (незадължително)
    skip = 56 # int |  (незадължително)
    skip_children = 56 # int |  (незадължително)
    limit = 56 # int |  (незадължително)
    limit_children = 56 # int |  (незадължително)
    count_children = True # bool |  (незадължително)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (незадължително)
    include_config = True # bool |  (незадължително)
    count_all = True # bool |  (незадължително)
    includei10n = True # bool |  (незадължително)
    locale = 'locale_example' # str |  (незадължително)
    modules = 'modules_example' # str |  (незадължително)
    is_crawler = True # bool |  (незадължително)
    include_notification_count = True # bool |  (незадължително)
    as_tree = True # bool |  (незадължително)
    max_tree_depth = 56 # int |  (незадължително)
    use_full_translation_ids = True # bool |  (незадължително)
    parent_id = 'parent_id_example' # str |  (незадължително)
    search_text = 'search_text_example' # str |  (незадължително)
    hash_tags = ['hash_tags_example'] # List[str] |  (незадължително)
    user_id = 'user_id_example' # str |  (незадължително)
    custom_config_str = 'custom_config_str_example' # str |  (незадължително)
    after_comment_id = 'after_comment_id_example' # str |  (незадължително)
    before_comment_id = 'before_comment_id_example' # str |  (незадължително)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]