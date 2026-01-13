req
tenantId
urlId

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |
| page | integer | query | Ні |  |
| direction | string | query | Ні |  |
| sso | string | query | Ні |  |
| skip | integer | query | Ні |  |
| skipChildren | integer | query | Ні |  |
| limit | integer | query | Ні |  |
| limitChildren | integer | query | Ні |  |
| countChildren | boolean | query | Ні |  |
| fetchPageForCommentId | string | query | Ні |  |
| includeConfig | boolean | query | Ні |  |
| countAll | boolean | query | Ні |  |
| includei10n | boolean | query | Ні |  |
| locale | string | query | Ні |  |
| modules | string | query | Ні |  |
| isCrawler | boolean | query | Ні |  |
| includeNotificationCount | boolean | query | Ні |  |
| asTree | boolean | query | Ні |  |
| maxTreeDepth | integer | query | Ні |  |
| useFullTranslationIds | boolean | query | Ні |  |
| parentId | string | query | Ні |  |
| searchText | string | query | Ні |  |
| hashTags | array | query | Ні |  |
| userId | string | query | Ні |  |
| customConfigStr | string | query | Ні |  |
| afterCommentId | string | query | Ні |  |
| beforeCommentId | string | query | Ні |  |

## Відповідь

Повертає: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## Приклад

[inline-code-attrs-start title = 'Приклад get_comments_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Визначення хоста необов'язкове; за замовчуванням використовується https://fastcomments.com
# Див. configuration.py для списку всіх підтримуваних параметрів конфігурації.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Увійдіть у контекст з екземпляром клієнта API
with client.ApiClient(configuration) as api_client:
    # Створіть екземпляр класу API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (необов'язково)
    direction = client.SortDirections() # SortDirections |  (необов'язково)
    sso = 'sso_example' # str |  (необов'язково)
    skip = 56 # int |  (необов'язково)
    skip_children = 56 # int |  (необов'язково)
    limit = 56 # int |  (необов'язково)
    limit_children = 56 # int |  (необов'язково)
    count_children = True # bool |  (необов'язково)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (необов'язково)
    include_config = True # bool |  (необов'язково)
    count_all = True # bool |  (необов'язково)
    includei10n = True # bool |  (необов'язково)
    locale = 'locale_example' # str |  (необов'язково)
    modules = 'modules_example' # str |  (необов'язково)
    is_crawler = True # bool |  (необов'язково)
    include_notification_count = True # bool |  (необов'язково)
    as_tree = True # bool |  (необов'язково)
    max_tree_depth = 56 # int |  (необов'язково)
    use_full_translation_ids = True # bool |  (необов'язково)
    parent_id = 'parent_id_example' # str |  (необов'язково)
    search_text = 'search_text_example' # str |  (необов'язково)
    hash_tags = ['hash_tags_example'] # List[str] |  (необов'язково)
    user_id = 'user_id_example' # str |  (необов'язково)
    custom_config_str = 'custom_config_str_example' # str |  (необов'язково)
    after_comment_id = 'after_comment_id_example' # str |  (необов'язково)
    before_comment_id = 'before_comment_id_example' # str |  (необов'язково)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]