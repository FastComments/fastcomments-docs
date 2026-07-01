req
tenantId
urlId

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן |  |
| page | integer | query | לא |  |
| direction | string | query | לא |  |
| sso | string | query | לא |  |
| skip | integer | query | לא |  |
| skipChildren | integer | query | לא |  |
| limit | integer | query | לא |  |
| limitChildren | integer | query | לא |  |
| countChildren | boolean | query | לא |  |
| fetchPageForCommentId | string | query | לא |  |
| includeConfig | boolean | query | לא |  |
| countAll | boolean | query | לא |  |
| includei10n | boolean | query | לא |  |
| locale | string | query | לא |  |
| modules | string | query | לא |  |
| isCrawler | boolean | query | לא |  |
| includeNotificationCount | boolean | query | לא |  |
| asTree | boolean | query | לא |  |
| maxTreeDepth | integer | query | לא |  |
| useFullTranslationIds | boolean | query | לא |  |
| parentId | string | query | לא |  |
| searchText | string | query | לא |  |
| hashTags | array | query | לא |  |
| userId | string | query | לא |  |
| customConfigStr | string | query | לא |  |
| afterCommentId | string | query | לא |  |
| beforeCommentId | string | query | לא |  |

## תגובה

מחזיר: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_response_with_presence_public_comment.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_comments_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetCommentsPublicOptions
from client.models.get_comments_response_with_presence_public_comment import GetCommentsResponseWithPresencePublicComment
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# הגדרת המארח היא אופציונלית וערכה המחדל הוא https://fastcomments.com
# ראו configuration.py לקבלת רשימה של כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# היכנסו להקשר עם מופע של לקוח ה‑API
with client.ApiClient(configuration) as api_client:
    # יצירת מופע של מחלקת ה‑API
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