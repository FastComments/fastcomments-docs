req
tenantId
urlId

## Παράμετροι

| Name | Type | Location | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |
| page | integer | query | Όχι |  |
| direction | string | query | Όχι |  |
| sso | string | query | Όχι |  |
| skip | integer | query | Όχι |  |
| skipChildren | integer | query | Όχι |  |
| limit | integer | query | Όχι |  |
| limitChildren | integer | query | Όχι |  |
| countChildren | boolean | query | Όχι |  |
| fetchPageForCommentId | string | query | Όχι |  |
| includeConfig | boolean | query | Όχι |  |
| countAll | boolean | query | Όχι |  |
| includei10n | boolean | query | Όχι |  |
| locale | string | query | Όχι |  |
| modules | string | query | Όχι |  |
| isCrawler | boolean | query | Όχι |  |
| includeNotificationCount | boolean | query | Όχι |  |
| asTree | boolean | query | Όχι |  |
| maxTreeDepth | integer | query | Όχι |  |
| useFullTranslationIds | boolean | query | Όχι |  |
| parentId | string | query | Όχι |  |
| searchText | string | query | Όχι |  |
| hashTags | array | query | Όχι |  |
| userId | string | query | Όχι |  |
| customConfigStr | string | query | Όχι |  |
| afterCommentId | string | query | Όχι |  |
| beforeCommentId | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_comments_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και από προεπιλογή είναι https://fastcomments.com
# Δείτε το configuration.py για λίστα όλων των υποστηριζόμενων παραμέτρων ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Μπείτε σε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (προαιρετικό)
    direction = client.SortDirections() # SortDirections |  (προαιρετικό)
    sso = 'sso_example' # str |  (προαιρετικό)
    skip = 56 # int |  (προαιρετικό)
    skip_children = 56 # int |  (προαιρετικό)
    limit = 56 # int |  (προαιρετικό)
    limit_children = 56 # int |  (προαιρετικό)
    count_children = True # bool |  (προαιρετικό)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (προαιρετικό)
    include_config = True # bool |  (προαιρετικό)
    count_all = True # bool |  (προαιρετικό)
    includei10n = True # bool |  (προαιρετικό)
    locale = 'locale_example' # str |  (προαιρετικό)
    modules = 'modules_example' # str |  (προαιρετικό)
    is_crawler = True # bool |  (προαιρετικό)
    include_notification_count = True # bool |  (προαιρετικό)
    as_tree = True # bool |  (προαιρετικό)
    max_tree_depth = 56 # int |  (προαιρετικό)
    use_full_translation_ids = True # bool |  (προαιρετικό)
    parent_id = 'parent_id_example' # str |  (προαιρετικό)
    search_text = 'search_text_example' # str |  (προαιρετικό)
    hash_tags = ['hash_tags_example'] # List[str] |  (προαιρετικό)
    user_id = 'user_id_example' # str |  (προαιρετικό)
    custom_config_str = 'custom_config_str_example' # str |  (προαιρετικό)
    after_comment_id = 'after_comment_id_example' # str |  (προαιρετικό)
    before_comment_id = 'before_comment_id_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]