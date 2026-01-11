req
tenantId
urlId

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| page | integer | query | Nein |  |
| direction | string | query | Nein |  |
| sso | string | query | Nein |  |
| skip | integer | query | Nein |  |
| skipChildren | integer | query | Nein |  |
| limit | integer | query | Nein |  |
| limitChildren | integer | query | Nein |  |
| countChildren | boolean | query | Nein |  |
| fetchPageForCommentId | string | query | Nein |  |
| includeConfig | boolean | query | Nein |  |
| countAll | boolean | query | Nein |  |
| includei10n | boolean | query | Nein |  |
| locale | string | query | Nein |  |
| modules | string | query | Nein |  |
| isCrawler | boolean | query | Nein |  |
| includeNotificationCount | boolean | query | Nein |  |
| asTree | boolean | query | Nein |  |
| maxTreeDepth | integer | query | Nein |  |
| useFullTranslationIds | boolean | query | Nein |  |
| parentId | string | query | Nein |  |
| searchText | string | query | Nein |  |
| hashTags | array | query | Nein |  |
| userId | string | query | Nein |  |
| customConfigStr | string | query | Nein |  |
| afterCommentId | string | query | Nein |  |
| beforeCommentId | string | query | Nein |  |

## Antwort

Gibt zurück: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## Beispiel

[inline-code-attrs-start title = 'get_comments_public Beispiel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Die Angabe des Hosts ist optional und standardmäßig https://fastcomments.com
# Siehe configuration.py für eine Liste aller unterstützten Konfigurationsparameter.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Öffnen Sie einen Kontext mit einer Instanz des API-Clients
with client.ApiClient(configuration) as api_client:
    # Erstellen Sie eine Instanz der API-Klasse
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