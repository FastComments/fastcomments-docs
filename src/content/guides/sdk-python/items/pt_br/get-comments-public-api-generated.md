req
tenantId
urlId

## Parâmetros

| Nome | Type | Location | Obrigatório | Descrição |
|------|------|----------|------------|-----------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim |  |
| page | integer | query | Não |  |
| direction | string | query | Não |  |
| sso | string | query | Não |  |
| skip | integer | query | Não |  |
| skipChildren | integer | query | Não |  |
| limit | integer | query | Não |  |
| limitChildren | integer | query | Não |  |
| countChildren | boolean | query | Não |  |
| fetchPageForCommentId | string | query | Não |  |
| includeConfig | boolean | query | Não |  |
| countAll | boolean | query | Não |  |
| includei10n | boolean | query | Não |  |
| locale | string | query | Não |  |
| modules | string | query | Não |  |
| isCrawler | boolean | query | Não |  |
| includeNotificationCount | boolean | query | Não |  |
| asTree | boolean | query | Não |  |
| maxTreeDepth | integer | query | Não |  |
| useFullTranslationIds | boolean | query | Não |  |
| parentId | string | query | Não |  |
| searchText | string | query | Não |  |
| hashTags | array | query | Não |  |
| userId | string | query | Não |  |
| customConfigStr | string | query | Não |  |
| afterCommentId | string | query | Não |  |
| beforeCommentId | string | query | Não |  |

## Resposta

Retorna: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_comments_public'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Definir o host é opcional e por padrão é https://fastcomments.com
# Veja configuration.py para uma lista de todos os parâmetros de configuração suportados.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entre em um contexto com uma instância do cliente da API
with client.ApiClient(configuration) as api_client:
    # Crie uma instância da classe API
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