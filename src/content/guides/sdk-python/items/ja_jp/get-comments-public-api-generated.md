req
tenantId
urlId

## パラメータ

| Name | Type | Location | Required | Description |
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

戻り値: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## 例

[inline-code-attrs-start title = 'get_comments_public の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# ホストの定義は省略可能で、デフォルトは https://fastcomments.com です
# サポートされている全ての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# APIクライアントのインスタンスでコンテキストを開始します
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (省略可能)
    direction = client.SortDirections() # SortDirections |  (省略可能)
    sso = 'sso_example' # str |  (省略可能)
    skip = 56 # int |  (省略可能)
    skip_children = 56 # int |  (省略可能)
    limit = 56 # int |  (省略可能)
    limit_children = 56 # int |  (省略可能)
    count_children = True # bool |  (省略可能)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (省略可能)
    include_config = True # bool |  (省略可能)
    count_all = True # bool |  (省略可能)
    includei10n = True # bool |  (省略可能)
    locale = 'locale_example' # str |  (省略可能)
    modules = 'modules_example' # str |  (省略可能)
    is_crawler = True # bool |  (省略可能)
    include_notification_count = True # bool |  (省略可能)
    as_tree = True # bool |  (省略可能)
    max_tree_depth = 56 # int |  (省略可能)
    use_full_translation_ids = True # bool |  (省略可能)
    parent_id = 'parent_id_example' # str |  (省略可能)
    search_text = 'search_text_example' # str |  (省略可能)
    hash_tags = ['hash_tags_example'] # List[str] |  (省略可能)
    user_id = 'user_id_example' # str |  (省略可能)
    custom_config_str = 'custom_config_str_example' # str |  (省略可能)
    after_comment_id = 'after_comment_id_example' # str |  (省略可能)
    before_comment_id = 'before_comment_id_example' # str |  (省略可能)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]