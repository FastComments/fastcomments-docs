req
tenantId
urlId

## パラメータ

| 名前 | タイプ | ロケーション | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい |  |
| page | integer | query | いいえ |  |
| direction | string | query | いいえ |  |
| sso | string | query | いいえ |  |
| skip | integer | query | いいえ |  |
| skipChildren | integer | query | いいえ |  |
| limit | integer | query | いいえ |  |
| limitChildren | integer | query | いいえ |  |
| countChildren | boolean | query | いいえ |  |
| fetchPageForCommentId | string | query | いいえ |  |
| includeConfig | boolean | query | いいえ |  |
| countAll | boolean | query | いいえ |  |
| includei10n | boolean | query | いいえ |  |
| locale | string | query | いいえ |  |
| modules | string | query | いいえ |  |
| isCrawler | boolean | query | いいえ |  |
| includeNotificationCount | boolean | query | いいえ |  |
| asTree | boolean | query | いいえ |  |
| maxTreeDepth | integer | query | いいえ |  |
| useFullTranslationIds | boolean | query | いいえ |  |
| parentId | string | query | いいえ |  |
| searchText | string | query | いいえ |  |
| hashTags | array | query | いいえ |  |
| userId | string | query | いいえ |  |
| customConfigStr | string | query | いいえ |  |
| afterCommentId | string | query | いいえ |  |
| beforeCommentId | string | query | いいえ |  |

## レスポンス

Returns: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_response_with_presence_public_comment.py)

## 例

[inline-code-attrs-start title = 'get_comments_public 例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetCommentsPublicOptions
from client.models.get_comments_response_with_presence_public_comment import GetCommentsResponseWithPresencePublicComment
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# configuration.py でサポートされているすべての設定パラメータの一覧をご覧ください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (optional)
    direction = client.SortDirections() # SortDirections |  (optional)
    sso = 'sso_example' # str |  (optional)
    skip = 56 # int |  (optional)
    skip_children = 56 # int |  (optional)
    limit = 56 # int |  (optional)
    limit_children = 56 # int |  (optional)
    count_children = True # bool |  (optional)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (optional)
    include_config = True # bool |  (optional)
    count_all = True # bool |  (optional)
    includei10n = True # bool |  (optional)
    locale = 'locale_example' # str |  (optional)
    modules = 'modules_example' # str |  (optional)
    is_crawler = True # bool |  (optional)
    include_notification_count = True # bool |  (optional)
    as_tree = True # bool |  (optional)
    max_tree_depth = 56 # int |  (optional)
    use_full_translation_ids = True # bool |  (optional)
    parent_id = 'parent_id_example' # str |  (optional)
    search_text = 'search_text_example' # str |  (optional)
    hash_tags = ['hash_tags_example'] # List[str] |  (optional)
    user_id = 'user_id_example' # str |  (optional)
    custom_config_str = 'custom_config_str_example' # str |  (optional)
    after_comment_id = 'after_comment_id_example' # str |  (optional)
    before_comment_id = 'before_comment_id_example' # str |  (optional)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, GetCommentsPublicOptions(page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id))
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]