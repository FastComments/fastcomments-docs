req
tenantId
urlId

## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
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

## レスポンス

返却: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_response_with_presence_public_comment.py)

## 例

[inline-code-attrs-start title = 'get_comments_public の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_response_with_presence_public_comment import GetCommentsResponseWithPresencePublicComment
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# ホストの定義は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスでコンテキストに入る
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (オプション)
    direction = client.SortDirections() # SortDirections |  (オプション)
    sso = 'sso_example' # str |  (オプション)
    skip = 56 # int |  (オプション)
    skip_children = 56 # int |  (オプション)
    limit = 56 # int |  (オプション)
    limit_children = 56 # int |  (オプション)
    count_children = True # bool |  (オプション)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (オプション)
    include_config = True # bool |  (オプション)
    count_all = True # bool |  (オプション)
    includei10n = True # bool |  (オプション)
    locale = 'locale_example' # str |  (オプション)
    modules = 'modules_example' # str |  (オプション)
    is_crawler = True # bool |  (オプション)
    include_notification_count = True # bool |  (オプション)
    as_tree = True # bool |  (オプション)
    max_tree_depth = 56 # int |  (オプション)
    use_full_translation_ids = True # bool |  (オプション)
    parent_id = 'parent_id_example' # str |  (オプション)
    search_text = 'search_text_example' # str |  (オプション)
    hash_tags = ['hash_tags_example'] # List[str] |  (オプション)
    user_id = 'user_id_example' # str |  (オプション)
    custom_config_str = 'custom_config_str_example' # str |  (オプション)
    after_comment_id = 'after_comment_id_example' # str |  (オプション)
    before_comment_id = 'before_comment_id_example' # str |  (オプション)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]