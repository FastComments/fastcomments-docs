req
tenantId
urlId

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| page | integer | query | Non |  |
| direction | string | query | Non |  |
| sso | string | query | Non |  |
| skip | integer | query | Non |  |
| skipChildren | integer | query | Non |  |
| limit | integer | query | Non |  |
| limitChildren | integer | query | Non |  |
| countChildren | boolean | query | Non |  |
| fetchPageForCommentId | string | query | Non |  |
| includeConfig | boolean | query | Non |  |
| countAll | boolean | query | Non |  |
| includei10n | boolean | query | Non |  |
| locale | string | query | Non |  |
| modules | string | query | Non |  |
| isCrawler | boolean | query | Non |  |
| includeNotificationCount | boolean | query | Non |  |
| asTree | boolean | query | Non |  |
| maxTreeDepth | integer | query | Non |  |
| useFullTranslationIds | boolean | query | Non |  |
| parentId | string | query | Non |  |
| searchText | string | query | Non |  |
| hashTags | array | query | Non |  |
| userId | string | query | Non |  |
| customConfigStr | string | query | Non |  |
| afterCommentId | string | query | Non |  |
| beforeCommentId | string | query | Non |  |

## Réponse

Renvoie: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## Exemple

[inline-code-attrs-start title = 'Exemple pour get_comments_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Définir l'hôte est optionnel et la valeur par défaut est https://fastcomments.com
# Voir configuration.py pour la liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (optionnel)
    direction = client.SortDirections() # SortDirections |  (optionnel)
    sso = 'sso_example' # str |  (optionnel)
    skip = 56 # int |  (optionnel)
    skip_children = 56 # int |  (optionnel)
    limit = 56 # int |  (optionnel)
    limit_children = 56 # int |  (optionnel)
    count_children = True # bool |  (optionnel)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (optionnel)
    include_config = True # bool |  (optionnel)
    count_all = True # bool |  (optionnel)
    includei10n = True # bool |  (optionnel)
    locale = 'locale_example' # str |  (optionnel)
    modules = 'modules_example' # str |  (optionnel)
    is_crawler = True # bool |  (optionnel)
    include_notification_count = True # bool |  (optionnel)
    as_tree = True # bool |  (optionnel)
    max_tree_depth = 56 # int |  (optionnel)
    use_full_translation_ids = True # bool |  (optionnel)
    parent_id = 'parent_id_example' # str |  (optionnel)
    search_text = 'search_text_example' # str |  (optionnel)
    hash_tags = ['hash_tags_example'] # List[str] |  (optionnel)
    user_id = 'user_id_example' # str |  (optionnel)
    custom_config_str = 'custom_config_str_example' # str |  (optionnel)
    after_comment_id = 'after_comment_id_example' # str |  (optionnel)
    before_comment_id = 'before_comment_id_example' # str |  (optionnel)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]