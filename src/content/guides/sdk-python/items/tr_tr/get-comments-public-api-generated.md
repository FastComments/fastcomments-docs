req
tenantId
urlId

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet |  |
| page | integer | query | Hayır |  |
| direction | string | query | Hayır |  |
| sso | string | query | Hayır |  |
| skip | integer | query | Hayır |  |
| skipChildren | integer | query | Hayır |  |
| limit | integer | query | Hayır |  |
| limitChildren | integer | query | Hayır |  |
| countChildren | boolean | query | Hayır |  |
| fetchPageForCommentId | string | query | Hayır |  |
| includeConfig | boolean | query | Hayır |  |
| countAll | boolean | query | Hayır |  |
| includei10n | boolean | query | Hayır |  |
| locale | string | query | Hayır |  |
| modules | string | query | Hayır |  |
| isCrawler | boolean | query | Hayır |  |
| includeNotificationCount | boolean | query | Hayır |  |
| asTree | boolean | query | Hayır |  |
| maxTreeDepth | integer | query | Hayır |  |
| useFullTranslationIds | boolean | query | Hayır |  |
| parentId | string | query | Hayır |  |
| searchText | string | query | Hayır |  |
| hashTags | array | query | Hayır |  |
| userId | string | query | Hayır |  |
| customConfigStr | string | query | Hayır |  |
| afterCommentId | string | query | Hayır |  |
| beforeCommentId | string | query | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_public200_response.py)

## Örnek

[inline-code-attrs-start title = 'get_comments_public Örnek'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_public200_response import GetCommentsPublic200Response
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Sunucu belirtilmesi isteğe bağlıdır ve varsayılan https://fastcomments.com'tur
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisinin bir örneği ile bir bağlama girin
with client.ApiClient(configuration) as api_client:
    # API sınıfının bir örneğini oluşturun
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    page = 56 # int |  (isteğe bağlı)
    direction = client.SortDirections() # SortDirections |  (isteğe bağlı)
    sso = 'sso_example' # str |  (isteğe bağlı)
    skip = 56 # int |  (isteğe bağlı)
    skip_children = 56 # int |  (isteğe bağlı)
    limit = 56 # int |  (isteğe bağlı)
    limit_children = 56 # int |  (isteğe bağlı)
    count_children = True # bool |  (isteğe bağlı)
    fetch_page_for_comment_id = 'fetch_page_for_comment_id_example' # str |  (isteğe bağlı)
    include_config = True # bool |  (isteğe bağlı)
    count_all = True # bool |  (isteğe bağlı)
    includei10n = True # bool |  (isteğe bağlı)
    locale = 'locale_example' # str |  (isteğe bağlı)
    modules = 'modules_example' # str |  (isteğe bağlı)
    is_crawler = True # bool |  (isteğe bağlı)
    include_notification_count = True # bool |  (isteğe bağlı)
    as_tree = True # bool |  (isteğe bağlı)
    max_tree_depth = 56 # int |  (isteğe bağlı)
    use_full_translation_ids = True # bool |  (isteğe bağlı)
    parent_id = 'parent_id_example' # str |  (isteğe bağlı)
    search_text = 'search_text_example' # str |  (isteğe bağlı)
    hash_tags = ['hash_tags_example'] # List[str] |  (isteğe bağlı)
    user_id = 'user_id_example' # str |  (isteğe bağlı)
    custom_config_str = 'custom_config_str_example' # str |  (isteğe bağlı)
    after_comment_id = 'after_comment_id_example' # str |  (isteğe bağlı)
    before_comment_id = 'before_comment_id_example' # str |  (isteğe bağlı)

    try:
        api_response = api_instance.get_comments_public(tenant_id, url_id, page=page, direction=direction, sso=sso, skip=skip, skip_children=skip_children, limit=limit, limit_children=limit_children, count_children=count_children, fetch_page_for_comment_id=fetch_page_for_comment_id, include_config=include_config, count_all=count_all, includei10n=includei10n, locale=locale, modules=modules, is_crawler=is_crawler, include_notification_count=include_notification_count, as_tree=as_tree, max_tree_depth=max_tree_depth, use_full_translation_ids=use_full_translation_ids, parent_id=parent_id, search_text=search_text, hash_tags=hash_tags, user_id=user_id, custom_config_str=custom_config_str, after_comment_id=after_comment_id, before_comment_id=before_comment_id)
        print("The response of PublicApi->get_comments_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_public: %s\n" % e)
[inline-code-end]