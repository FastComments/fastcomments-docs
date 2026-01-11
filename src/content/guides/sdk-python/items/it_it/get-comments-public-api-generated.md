req
tenantId
urlId

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì |  |
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

## Risposta

Restituisce: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_comments_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# La definizione dell'host è opzionale e il valore predefinito è https://fastcomments.com
# Vedi configuration.py per un elenco di tutti i parametri di configurazione supportati.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entra in un contesto con un'istanza del client API
with client.ApiClient(configuration) as api_client:
    # Crea un'istanza della classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (opzionale)
    direction = client.SortDirections() # SortDirections |  (opzionale)
    sso = 'sso_example' # str |  (opzionale)
    skip = 56 # int |  (opzionale)
    skip_children = 56 # int |  (opzionale)
    limit = 56 # int |  (opzionale)
    limit_children = 56 # int |  (opzionale)
    count_children = True # bool |  (opzionale)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (opzionale)
    include_config = True # bool |  (opzionale)
    count_all = True # bool |  (opzionale)
    includei10n = True # bool |  (opzionale)
    locale = 'locale_example' # str |  (opzionale)
    modules = 'modules_example' # str |  (opzionale)
    is_crawler = True # bool |  (opzionale)
    include_notification_count = True # bool |  (opzionale)
    as_tree = True # bool |  (opzionale)
    max_tree_depth = 56 # int |  (opzionale)
    use_full_translation_ids = True # bool |  (opzionale)
    parent_id = 'parent_id_example' # str |  (opzionale)
    search_text = 'search_text_example' # str |  (opzionale)
    hash_tags = ['hash_tags_example'] # List[str] |  (opzionale)
    user_id = 'user_id_example' # str |  (opzionale)
    custom_config_str = 'custom_config_str_example' # str |  (opzionale)
    after_comment_id = 'after_comment_id_example' # str |  (opzionale)
    before_comment_id = 'before_comment_id_example' # str |  (opzionale)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]