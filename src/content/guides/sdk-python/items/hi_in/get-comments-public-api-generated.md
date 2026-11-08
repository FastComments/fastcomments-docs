req
tenantId
urlId

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| page | integer | query | नहीं |  |
| direction | string | query | नहीं |  |
| sso | string | query | नहीं |  |
| skip | integer | query | नहीं |  |
| skipChildren | integer | query | नहीं |  |
| limit | integer | query | नहीं |  |
| limitChildren | integer | query | नहीं |  |
| countChildren | boolean | query | नहीं |  |
| fetchPageForCommentId | string | query | नहीं |  |
| includeConfig | boolean | query | नहीं |  |
| countAll | boolean | query | नहीं |  |
| includei10n | boolean | query | नहीं |  |
| locale | string | query | नहीं |  |
| modules | string | query | नहीं |  |
| isCrawler | boolean | query | नहीं |  |
| includeNotificationCount | boolean | query | नहीं |  |
| asTree | boolean | query | नहीं |  |
| maxTreeDepth | integer | query | नहीं |  |
| useFullTranslationIds | boolean | query | नहीं |  |
| parentId | string | query | नहीं |  |
| searchText | string | query | नहीं |  |
| hashTags | array | query | नहीं |  |
| userId | string | query | नहीं |  |
| customConfigStr | string | query | नहीं |  |
| afterCommentId | string | query | नहीं |  |
| beforeCommentId | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_response_with_presence_public_comment.py)

## उदाहरण

[inline-code-attrs-start title = 'get_comments_public उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetCommentsPublicOptions
from client.models.get_comments_response_with_presence_public_comment import GetCommentsResponseWithPresencePublicComment
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# होस्ट को परिभाषित करना वैकल्पिक है और यह डिफ़ॉल्ट रूप से https://fastcomments.com होता है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API क्लाइंट के इंस्टेंस के साथ एक कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएं
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (वैकल्पिक)
    direction = client.SortDirections() # SortDirections |  (वैकल्पिक)
    sso = 'sso_example' # str |  (वैकल्पिक)
    skip = 56 # int |  (वैकल्पिक)
    skip_children = 56 # int |  (वैकल्पिक)
    limit = 56 # int |  (वैकल्पिक)
    limit_children = 56 # int |  (वैकल्पिक)
    count_children = True # bool |  (वैकल्पिक)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (वैकल्पिक)
    include_config = True # bool |  (वैकल्पिक)
    count_all = True # bool |  (वैकल्पिक)
    includei10n = True # bool |  (वैकल्पिक)
    locale = 'locale_example' # str |  (वैकल्पिक)
    modules = 'modules_example' # str |  (वैकल्पिक)
    is_crawler = True # bool |  (वैकल्पिक)
    include_notification_count = True # bool |  (वैकल्पिक)
    as_tree = True # bool |  (वैकल्पिक)
    max_tree_depth = 56 # int |  (वैकल्पिक)
    use_full_translation_ids = True # bool |  (वैकल्पिक)
    parent_id = 'parent_id_example' # str |  (वैकल्पिक)
    search_text = 'search_text_example' # str |  (वैकल्पिक)
    hash_tags = ['hash_tags_example'] # List[str] |  (वैकल्पिक)
    user_id = 'user_id_example' # str |  (वैकल्पिक)
    custom_config_str = 'custom_config_str_example' # str |  (वैकल्पिक)
    after_comment_id = 'after_comment_id_example' # str |  (वैकल्पिक)
    before_comment_id = 'before_comment_id_example' # str |  (वैकल्पिक)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, GetCommentsPublicOptions(page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id))
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]