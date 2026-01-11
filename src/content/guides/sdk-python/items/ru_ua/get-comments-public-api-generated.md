обязательно
tenantId
urlId

## Параметры

| Имя | Тип | Location | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| page | integer | query | Нет |  |
| direction | string | query | Нет |  |
| sso | string | query | Нет |  |
| skip | integer | query | Нет |  |
| skipChildren | integer | query | Нет |  |
| limit | integer | query | Нет |  |
| limitChildren | integer | query | Нет |  |
| countChildren | boolean | query | Нет |  |
| fetchPageForCommentId | string | query | Нет |  |
| includeConfig | boolean | query | Нет |  |
| countAll | boolean | query | Нет |  |
| includei10n | boolean | query | Нет |  |
| locale | string | query | Нет |  |
| modules | string | query | Нет |  |
| isCrawler | boolean | query | Нет |  |
| includeNotificationCount | boolean | query | Нет |  |
| asTree | boolean | query | Нет |  |
| maxTreeDepth | integer | query | Нет |  |
| useFullTranslationIds | boolean | query | Нет |  |
| parentId | string | query | Нет |  |
| searchText | string | query | Нет |  |
| hashTags | array | query | Нет |  |
| userId | string | query | Нет |  |
| customConfigStr | string | query | Нет |  |
| afterCommentId | string | query | Нет |  |
| beforeCommentId | string | query | Нет |  |

## Ответ

Возвращает: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## Пример

[inline-code-attrs-start title = 'Пример get_comments_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Задание хоста необязательно, по умолчанию используется https://fastcomments.com
# Смотрите configuration.py для списка всех поддерживаемых параметров конфигурации.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Откройте контекст с экземпляром API-клиента
with client.ApiClient(configuration) as api_client:
    # Создайте экземпляр класса API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (необязательно)
    direction = client.SortDirections() # SortDirections |  (необязательно)
    sso = 'sso_example' # str |  (необязательно)
    skip = 56 # int |  (необязательно)
    skip_children = 56 # int |  (необязательно)
    limit = 56 # int |  (необязательно)
    limit_children = 56 # int |  (необязательно)
    count_children = True # bool |  (необязательно)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (необязательно)
    include_config = True # bool |  (необязательно)
    count_all = True # bool |  (необязательно)
    includei10n = True # bool |  (необязательно)
    locale = 'locale_example' # str |  (необязательно)
    modules = 'modules_example' # str |  (необязательно)
    is_crawler = True # bool |  (необязательно)
    include_notification_count = True # bool |  (необязательно)
    as_tree = True # bool |  (необязательно)
    max_tree_depth = 56 # int |  (необязательно)
    use_full_translation_ids = True # bool |  (необязательно)
    parent_id = 'parent_id_example' # str |  (необязательно)
    search_text = 'search_text_example' # str |  (необязательно)
    hash_tags = ['hash_tags_example'] # List[str] |  (необязательно)
    user_id = 'user_id_example' # str |  (необязательно)
    custom_config_str = 'custom_config_str_example' # str |  (необязательно)
    after_comment_id = 'after_comment_id_example' # str |  (необязательно)
    before_comment_id = 'before_comment_id_example' # str |  (необязательно)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]