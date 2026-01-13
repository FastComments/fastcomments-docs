req
tenantId
urlId

## פרמטרים

| שם | Type | Location | נדרש | תיאור |
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

מחזיר: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## דוגמה

[inline-code-attrs-start title = 'get_comments_public דוגמה'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host אופציונלית והברירת מחדל היא https://fastcomments.com
# ראה את configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנס להקשר עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צור מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (אופציונלי)
    direction = client.SortDirections() # SortDirections |  (אופציונלי)
    sso = 'sso_example' # str |  (אופציונלי)
    skip = 56 # int |  (אופציונלי)
    skip_children = 56 # int |  (אופציונלי)
    limit = 56 # int |  (אופציונלי)
    limit_children = 56 # int |  (אופציונלי)
    count_children = True # bool |  (אופציונלי)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (אופציונלי)
    include_config = True # bool |  (אופציונלי)
    count_all = True # bool |  (אופציונלי)
    includei10n = True # bool |  (אופציונלי)
    locale = 'locale_example' # str |  (אופציונלי)
    modules = 'modules_example' # str |  (אופציונלי)
    is_crawler = True # bool |  (אופציונלי)
    include_notification_count = True # bool |  (אופציונלי)
    as_tree = True # bool |  (אופציונלי)
    max_tree_depth = 56 # int |  (אופציונלי)
    use_full_translation_ids = True # bool |  (אופציונלי)
    parent_id = 'parent_id_example' # str |  (אופציונלי)
    search_text = 'search_text_example' # str |  (אופציונלי)
    hash_tags = ['hash_tags_example'] # List[str] |  (אופציונלי)
    user_id = 'user_id_example' # str |  (אופציונלי)
    custom_config_str = 'custom_config_str_example' # str |  (אופציונלי)
    after_comment_id = 'after_comment_id_example' # str |  (אופציונלי)
    before_comment_id = 'before_comment_id_example' # str |  (אופציונלי)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]