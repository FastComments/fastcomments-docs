req
tenantId
urlId

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| page | integer | query | 否 |  |
| direction | string | query | 否 |  |
| sso | string | query | 否 |  |
| skip | integer | query | 否 |  |
| skipChildren | integer | query | 否 |  |
| limit | integer | query | 否 |  |
| limitChildren | integer | query | 否 |  |
| countChildren | boolean | query | 否 |  |
| fetchPageForCommentId | string | query | 否 |  |
| includeConfig | boolean | query | 否 |  |
| countAll | boolean | query | 否 |  |
| includei10n | boolean | query | 否 |  |
| locale | string | query | 否 |  |
| modules | string | query | 否 |  |
| isCrawler | boolean | query | 否 |  |
| includeNotificationCount | boolean | query | 否 |  |
| asTree | boolean | query | 否 |  |
| maxTreeDepth | integer | query | 否 |  |
| useFullTranslationIds | boolean | query | 否 |  |
| parentId | string | query | 否 |  |
| searchText | string | query | 否 |  |
| hashTags | array | query | 否 |  |
| userId | string | query | 否 |  |
| customConfigStr | string | query | 否 |  |
| afterCommentId | string | query | 否 |  |
| beforeCommentId | string | query | 否 |  |

## 回應

回傳: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## 範例

[inline-code-attrs-start title = 'get_comments_public 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客戶端實例開啟一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (可選)
    direction = client.SortDirections() # SortDirections |  (可選)
    sso = 'sso_example' # str |  (可選)
    skip = 56 # int |  (可選)
    skip_children = 56 # int |  (可選)
    limit = 56 # int |  (可選)
    limit_children = 56 # int |  (可選)
    count_children = True # bool |  (可選)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (可選)
    include_config = True # bool |  (可選)
    count_all = True # bool |  (可選)
    includei10n = True # bool |  (可選)
    locale = 'locale_example' # str |  (可選)
    modules = 'modules_example' # str |  (可選)
    is_crawler = True # bool |  (可選)
    include_notification_count = True # bool |  (可選)
    as_tree = True # bool |  (可選)
    max_tree_depth = 56 # int |  (可選)
    use_full_translation_ids = True # bool |  (可選)
    parent_id = 'parent_id_example' # str |  (可選)
    search_text = 'search_text_example' # str |  (可選)
    hash_tags = ['hash_tags_example'] # List[str] |  (可選)
    user_id = 'user_id_example' # str |  (可選)
    custom_config_str = 'custom_config_str_example' # str |  (可選)
    after_comment_id = 'after_comment_id_example' # str |  (可選)
    before_comment_id = 'before_comment_id_example' # str |  (可選)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]