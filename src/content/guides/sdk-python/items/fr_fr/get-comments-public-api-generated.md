req
tenantId
urlId

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
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

## Réponse

Renvoie : [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_response_with_presence_public_comment.py)

## Exemple

[inline-code-attrs-start title = 'Exemple get_comments_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetCommentsPublicOptions
from client.models.get_comments_response_with_presence_public_comment import GetCommentsResponseWithPresencePublicComment
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Définir l'hôte est facultatif et la valeur par défaut est https://fastcomments.com
# Voir configuration.py pour une liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (facultatif)
    direction = client.SortDirections() # SortDirections |  (facultatif)
    sso = 'sso_example' # str |  (facultatif)
    skip = 56 # int |  (facultatif)
    skip_children = 56 # int |  (facultatif)
    limit = 56 # int |  (facultatif)
    limit_children = 56 # int |  (facultatif)
    count_children = True # bool |  (facultatif)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (facultatif)
    include_config = True # bool |  (facultatif)
    count_all = True # bool |  (facultatif)
    includei10n = True # bool |  (facultatif)
    locale = 'locale_example' # str |  (facultatif)
    modules = 'modules_example' # str |  (facultatif)
    is_crawler = True # bool |  (facultatif)
    include_notification_count = True # bool |  (facultatif)
    as_tree = True # bool |  (facultatif)
    max_tree_depth = 56 # int |  (facultatif)
    use_full_translation_ids = True # bool |  (facultatif)
    parent_id = 'parent_id_example' # str |  (facultatif)
    search_text = 'search_text_example' # str |  (facultatif)
    hash_tags = ['hash_tags_example'] # List[str] |  (facultatif)
    user_id = 'user_id_example' # str |  (facultatif)
    custom_config_str = 'custom_config_str_example' # str |  (facultatif)
    after_comment_id = 'after_comment_id_example' # str |  (facultatif)
    before_comment_id = 'before_comment_id_example' # str |  (facultatif)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, GetCommentsPublicOptions(page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id))
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]