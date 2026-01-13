req
tenantId
urlId

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| page | integer | query | Ne |  |
| direction | string | query | Ne |  |
| sso | string | query | Ne |  |
| skip | integer | query | Ne |  |
| skipChildren | integer | query | Ne |  |
| limit | integer | query | Ne |  |
| limitChildren | integer | query | Ne |  |
| countChildren | boolean | query | Ne |  |
| fetchPageForCommentId | string | query | Ne |  |
| includeConfig | boolean | query | Ne |  |
| countAll | boolean | query | Ne |  |
| includei10n | boolean | query | Ne |  |
| locale | string | query | Ne |  |
| modules | string | query | Ne |  |
| isCrawler | boolean | query | Ne |  |
| includeNotificationCount | boolean | query | Ne |  |
| asTree | boolean | query | Ne |  |
| maxTreeDepth | integer | query | Ne |  |
| useFullTranslationIds | boolean | query | Ne |  |
| parentId | string | query | Ne |  |
| searchText | string | query | Ne |  |
| hashTags | array | query | Ne |  |
| userId | string | query | Ne |  |
| customConfigStr | string | query | Ne |  |
| afterCommentId | string | query | Ne |  |
| beforeCommentId | string | query | Ne |  |

## Odgovor

Vraća: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## Primer

[inline-code-attrs-start title = 'get_comments_public Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (opciono)
    direction = client.SortDirections() # SortDirections |  (opciono)
    sso = 'sso_example' # str |  (opciono)
    skip = 56 # int |  (opciono)
    skip_children = 56 # int |  (opciono)
    limit = 56 # int |  (opciono)
    limit_children = 56 # int |  (opciono)
    count_children = True # bool |  (opciono)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (opciono)
    include_config = True # bool |  (opciono)
    count_all = True # bool |  (opciono)
    includei10n = True # bool |  (opciono)
    locale = 'locale_example' # str |  (opciono)
    modules = 'modules_example' # str |  (opciono)
    is_crawler = True # bool |  (opciono)
    include_notification_count = True # bool |  (opciono)
    as_tree = True # bool |  (opciono)
    max_tree_depth = 56 # int |  (opciono)
    use_full_translation_ids = True # bool |  (opciono)
    parent_id = 'parent_id_example' # str |  (opciono)
    search_text = 'search_text_example' # str |  (opciono)
    hash_tags = ['hash_tags_example'] # List[str] |  (opciono)
    user_id = 'user_id_example' # str |  (opciono)
    custom_config_str = 'custom_config_str_example' # str |  (opciono)
    after_comment_id = 'after_comment_id_example' # str |  (opciono)
    before_comment_id = 'before_comment_id_example' # str |  (opciono)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]