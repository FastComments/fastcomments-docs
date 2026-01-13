req
tenantId
urlId

## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí |  |
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

## Respuesta

Devuelve: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_comments_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Definir el host es opcional y por defecto es https://fastcomments.com
# Consulta configuration.py para obtener una lista de todos los parámetros de configuración compatibles.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entra en un contexto con una instancia del cliente de la API
with client.ApiClient(configuration) as api_client:
    # Crea una instancia de la clase API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (opcional)
    direction = client.SortDirections() # SortDirections |  (opcional)
    sso = 'sso_example' # str |  (opcional)
    skip = 56 # int |  (opcional)
    skip_children = 56 # int |  (opcional)
    limit = 56 # int |  (opcional)
    limit_children = 56 # int |  (opcional)
    count_children = True # bool |  (opcional)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (opcional)
    include_config = True # bool |  (opcional)
    count_all = True # bool |  (opcional)
    includei10n = True # bool |  (opcional)
    locale = 'locale_example' # str |  (opcional)
    modules = 'modules_example' # str |  (opcional)
    is_crawler = True # bool |  (opcional)
    include_notification_count = True # bool |  (opcional)
    as_tree = True # bool |  (opcional)
    max_tree_depth = 56 # int |  (opcional)
    use_full_translation_ids = True # bool |  (opcional)
    parent_id = 'parent_id_example' # str |  (opcional)
    search_text = 'search_text_example' # str |  (opcional)
    hash_tags = ['hash_tags_example'] # List[str] |  (opcional)
    user_id = 'user_id_example' # str |  (opcional)
    custom_config_str = 'custom_config_str_example' # str |  (opcional)
    after_comment_id = 'after_comment_id_example' # str |  (opcional)
    before_comment_id = 'before_comment_id_example' # str |  (opcional)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]