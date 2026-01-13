req
tenantId
urlId

## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| page | integer | query | Nee |  |
| direction | string | query | Nee |  |
| sso | string | query | Nee |  |
| skip | integer | query | Nee |  |
| skipChildren | integer | query | Nee |  |
| limit | integer | query | Nee |  |
| limitChildren | integer | query | Nee |  |
| countChildren | boolean | query | Nee |  |
| fetchPageForCommentId | string | query | Nee |  |
| includeConfig | boolean | query | Nee |  |
| countAll | boolean | query | Nee |  |
| includei10n | boolean | query | Nee |  |
| locale | string | query | Nee |  |
| modules | string | query | Nee |  |
| isCrawler | boolean | query | Nee |  |
| includeNotificationCount | boolean | query | Nee |  |
| asTree | boolean | query | Nee |  |
| maxTreeDepth | integer | query | Nee |  |
| useFullTranslationIds | boolean | query | Nee |  |
| parentId | string | query | Nee |  |
| searchText | string | query | Nee |  |
| hashTags | array | query | Nee |  |
| userId | string | query | Nee |  |
| customConfigStr | string | query | Nee |  |
| afterCommentId | string | query | Nee |  |
| beforeCommentId | string | query | Nee |  |

## Antwoord

Retourneert: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## Voorbeeld

[inline-code-attrs-start title = 'get_comments_public Voorbeeld'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Het definiëren van de host is optioneel en standaard https://fastcomments.com
# Zie configuration.py voor een lijst van alle ondersteunde configuratieparameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Open een context met een instantie van de API-client
with client.ApiClient(configuration) as api_client:
    # Maak een instantie van de API-klasse
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (optioneel)
    direction = client.SortDirections() # SortDirections |  (optioneel)
    sso = 'sso_example' # str |  (optioneel)
    skip = 56 # int |  (optioneel)
    skip_children = 56 # int |  (optioneel)
    limit = 56 # int |  (optioneel)
    limit_children = 56 # int |  (optioneel)
    count_children = True # bool |  (optioneel)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (optioneel)
    include_config = True # bool |  (optioneel)
    count_all = True # bool |  (optioneel)
    includei10n = True # bool |  (optioneel)
    locale = 'locale_example' # str |  (optioneel)
    modules = 'modules_example' # str |  (optioneel)
    is_crawler = True # bool |  (optioneel)
    include_notification_count = True # bool |  (optioneel)
    as_tree = True # bool |  (optioneel)
    max_tree_depth = 56 # int |  (optioneel)
    use_full_translation_ids = True # bool |  (optioneel)
    parent_id = 'parent_id_example' # str |  (optioneel)
    search_text = 'search_text_example' # str |  (optioneel)
    hash_tags = ['hash_tags_example'] # List[str] |  (optioneel)
    user_id = 'user_id_example' # str |  (optioneel)
    custom_config_str = 'custom_config_str_example' # str |  (optioneel)
    after_comment_id = 'after_comment_id_example' # str |  (optioneel)
    before_comment_id = 'before_comment_id_example' # str |  (optioneel)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]