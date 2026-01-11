req
tenantId
urlId

## Parametre

| Navn | Type | Location | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| page | integer | query | Nej |  |
| direction | string | query | Nej |  |
| sso | string | query | Nej |  |
| skip | integer | query | Nej |  |
| skipChildren | integer | query | Nej |  |
| limit | integer | query | Nej |  |
| limitChildren | integer | query | Nej |  |
| countChildren | boolean | query | Nej |  |
| fetchPageForCommentId | string | query | Nej |  |
| includeConfig | boolean | query | Nej |  |
| countAll | boolean | query | Nej |  |
| includei10n | boolean | query | Nej |  |
| locale | string | query | Nej |  |
| modules | string | query | Nej |  |
| isCrawler | boolean | query | Nej |  |
| includeNotificationCount | boolean | query | Nej |  |
| asTree | boolean | query | Nej |  |
| maxTreeDepth | integer | query | Nej |  |
| useFullTranslationIds | boolean | query | Nej |  |
| parentId | string | query | Nej |  |
| searchText | string | query | Nej |  |
| hashTags | array | query | Nej |  |
| userId | string | query | Nej |  |
| customConfigStr | string | query | Nej |  |
| afterCommentId | string | query | Nej |  |
| beforeCommentId | string | query | Nej |  |

## Svar

Returnerer: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## Eksempel

[inline-code-attrs-start title = 'get_comments_public Eksempel'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Det er valgfrit at angive host, standard er https://fastcomments.com
# Se configuration.py for en liste over alle understøttede konfigurationsparametre.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Åbn en kontekst med en instans af API-klienten
with client.ApiClient(configuration) as api_client:
    # Opret en instans af API-klassen
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (valgfri)
    direction = client.SortDirections() # SortDirections |  (valgfri)
    sso = 'sso_example' # str |  (valgfri)
    skip = 56 # int |  (valgfri)
    skip_children = 56 # int |  (valgfri)
    limit = 56 # int |  (valgfri)
    limit_children = 56 # int |  (valgfri)
    count_children = True # bool |  (valgfri)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (valgfri)
    include_config = True # bool |  (valgfri)
    count_all = True # bool |  (valgfri)
    includei10n = True # bool |  (valgfri)
    locale = 'locale_example' # str |  (valgfri)
    modules = 'modules_example' # str |  (valgfri)
    is_crawler = True # bool |  (valgfri)
    include_notification_count = True # bool |  (valgfri)
    as_tree = True # bool |  (valgfri)
    max_tree_depth = 56 # int |  (valgfri)
    use_full_translation_ids = True # bool |  (valgfri)
    parent_id = 'parent_id_example' # str |  (valgfri)
    search_text = 'search_text_example' # str |  (valgfri)
    hash_tags = ['hash_tags_example'] # List[str] |  (valgfri)
    user_id = 'user_id_example' # str |  (valgfri)
    custom_config_str = 'custom_config_str_example' # str |  (valgfri)
    after_comment_id = 'after_comment_id_example' # str |  (valgfri)
    before_comment_id = 'before_comment_id_example' # str |  (valgfri)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]