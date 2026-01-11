req
tenantId
urlId

## 参数

| 名称 | 类型 | 位置 | 是否必需 | 描述 |
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

## 响应

返回: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## 示例

[inline-code-attrs-start title = 'get_comments_public 示例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# 定义主机是可选的，默认值为 https://fastcomments.com
# 请参阅 configuration.py 以获取所有支持的配置参数列表。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客户端实例进入上下文
with client.ApiClient(configuration) as api_client:
    # 创建 API 类的一个实例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  （可选）
    direction = client.SortDirections() # SortDirections |  （可选）
    sso = 'sso_example' # str |  （可选）
    skip = 56 # int |  （可选）
    skip_children = 56 # int |  （可选）
    limit = 56 # int |  （可选）
    limit_children = 56 # int |  （可选）
    count_children = True # bool |  （可选）
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  （可选）
    include_config = True # bool |  （可选）
    count_all = True # bool |  （可选）
    includei10n = True # bool |  （可选）
    locale = 'locale_example' # str |  （可选）
    modules = 'modules_example' # str |  （可选）
    is_crawler = True # bool |  （可选）
    include_notification_count = True # bool |  （可选）
    as_tree = True # bool |  （可选）
    max_tree_depth = 56 # int |  （可选）
    use_full_translation_ids = True # bool |  （可选）
    parent_id = 'parent_id_example' # str |  （可选）
    search_text = 'search_text_example' # str |  （可选）
    hash_tags = ['hash_tags_example'] # List[str] |  （可选）
    user_id = 'user_id_example' # str |  （可选）
    custom_config_str = 'custom_config_str_example' # str |  （可选）
    after_comment_id = 'after_comment_id_example' # str |  （可选）
    before_comment_id = 'before_comment_id_example' # str |  （可选）

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]