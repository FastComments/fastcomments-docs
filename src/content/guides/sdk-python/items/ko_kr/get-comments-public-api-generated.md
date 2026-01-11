req
tenantId
urlId

## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| page | integer | query | 아니오 |  |
| direction | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |
| skip | integer | query | 아니오 |  |
| skipChildren | integer | query | 아니오 |  |
| limit | integer | query | 아니오 |  |
| limitChildren | integer | query | 아니오 |  |
| countChildren | boolean | query | 아니오 |  |
| fetchPageForCommentId | string | query | 아니오 |  |
| includeConfig | boolean | query | 아니오 |  |
| countAll | boolean | query | 아니오 |  |
| includei10n | boolean | query | 아니오 |  |
| locale | string | query | 아니오 |  |
| modules | string | query | 아니오 |  |
| isCrawler | boolean | query | 아니오 |  |
| includeNotificationCount | boolean | query | 아니오 |  |
| asTree | boolean | query | 아니오 |  |
| maxTreeDepth | integer | query | 아니오 |  |
| useFullTranslationIds | boolean | query | 아니오 |  |
| parentId | string | query | 아니오 |  |
| searchText | string | query | 아니오 |  |
| hashTags | array | query | 아니오 |  |
| userId | string | query | 아니오 |  |
| customConfigStr | string | query | 아니오 |  |
| afterCommentId | string | query | 아니오 |  |
| beforeCommentId | string | query | 아니오 |  |

## 응답

반환: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## 예제

[inline-code-attrs-start title = 'get_comments_public 예제'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# 호스트 지정은 선택 사항이며 기본값은 https://fastcomments.com 입니다
# 모든 지원 구성 매개변수 목록은 configuration.py를 참조하세요.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API 클라이언트 인스턴스와 함께 컨텍스트에 들어갑니다
with client.ApiClient(configuration) as api_client:
    # API 클래스의 인스턴스를 생성합니다
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (선택 사항)
    direction = client.SortDirections() # SortDirections |  (선택 사항)
    sso = 'sso_example' # str |  (선택 사항)
    skip = 56 # int |  (선택 사항)
    skip_children = 56 # int |  (선택 사항)
    limit = 56 # int |  (선택 사항)
    limit_children = 56 # int |  (선택 사항)
    count_children = True # bool |  (선택 사항)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (선택 사항)
    include_config = True # bool |  (선택 사항)
    count_all = True # bool |  (선택 사항)
    includei10n = True # bool |  (선택 사항)
    locale = 'locale_example' # str |  (선택 사항)
    modules = 'modules_example' # str |  (선택 사항)
    is_crawler = True # bool |  (선택 사항)
    include_notification_count = True # bool |  (선택 사항)
    as_tree = True # bool |  (선택 사항)
    max_tree_depth = 56 # int |  (선택 사항)
    use_full_translation_ids = True # bool |  (선택 사항)
    parent_id = 'parent_id_example' # str |  (선택 사항)
    search_text = 'search_text_example' # str |  (선택 사항)
    hash_tags = ['hash_tags_example'] # List[str] |  (선택 사항)
    user_id = 'user_id_example' # str |  (선택 사항)
    custom_config_str = 'custom_config_str_example' # str |  (선택 사항)
    after_comment_id = 'after_comment_id_example' # str |  (선택 사항)
    before_comment_id = 'before_comment_id_example' # str |  (선택 사항)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]

---