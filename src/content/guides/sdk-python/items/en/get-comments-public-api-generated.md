
req
tenantId
urlId

## Parameters

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

## Response

Returns: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## Example

[inline-code-attrs-start title = 'get_comments_public Example'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
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
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]
