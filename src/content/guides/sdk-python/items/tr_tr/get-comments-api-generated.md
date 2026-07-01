## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| page | integer | query | Hayır |  |
| limit | integer | query | Hayır |  |
| skip | integer | query | Hayır |  |
| asTree | boolean | query | Hayır |  |
| skipChildren | integer | query | Hayır |  |
| limitChildren | integer | query | Hayır |  |
| maxTreeDepth | integer | query | Hayır |  |
| urlId | string | query | Hayır |  |
| userId | string | query | Hayır |  |
| anonUserId | string | query | Hayır |  |
| contextUserId | string | query | Hayır |  |
| hashTag | string | query | Hayır |  |
| parentId | string | query | Hayır |  |
| direction | string | query | Hayır |  |
| fromDate | integer | query | Hayır |  |
| toDate | integer | query | Hayır |  |

## Yanıt

Döndürür: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_comments_response.py)

## Örnek

[inline-code-attrs-start title = 'get_comments Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import GetCommentsOptions
from client.models.api_get_comments_response import APIGetCommentsResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Host tanımlaması isteğe bağlıdır ve varsayılan olarak https://fastcomments.com adresine yönlendirilir
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# İstemci, kimlik doğrulama ve yetkilendirme parametrelerini API sunucusunun güvenlik politikasına göre yapılandırmalıdır.
# Aşağıda her kimlik doğrulama yöntemi için örnekler sağlanmıştır; kimlik doğrulama senaryonuza uyan örneği kullanın.

# API anahtarı yetkilendirmesini yapılandır: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak için aşağıdaki satırın yorumunu kaldırın
# configuration.api_key_prefix['api_key'] = 'Bearer'

# API istemcisi bir örnekle bir bağlam içine girin
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    page = 56 # int |  (optional)
    limit = 56 # int |  (optional)
    skip = 56 # int |  (optional)
    as_tree = True # bool |  (optional)
    skip_children = 56 # int |  (optional)
    limit_children = 56 # int |  (optional)
    max_tree_depth = 56 # int |  (optional)
    url_id = 'url_id_example' # str |  (optional)
    user_id = 'user_id_example' # str |  (optional)
    anon_user_id = 'anon_user_id_example' # str |  (optional)
    context_user_id = 'context_user_id_example' # str |  (optional)
    hash_tag = 'hash_tag_example' # str |  (optional)
    parent_id = 'parent_id_example' # str |  (optional)
    direction = client.SortDirections() # SortDirections |  (optional)
    from_date = 56 # int |  (optional)
    to_date = 56 # int |  (optional)

    try:
        api_response = api_instance.get_comments(tenant_id, GetCommentsOptions(page=page, limit=limit, skip=skip, as_tree=as_tree, skip_children=skip_children, limit_children=limit_children, max_tree_depth=max_tree_depth, url_id=url_id, user_id=user_id, anon_user_id=anon_user_id, context_user_id=context_user_id, hash_tag=hash_tag, parent_id=parent_id, direction=direction, from_date=from_date, to_date=to_date))
        print("The response of DefaultApi->get_comments:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_comments: %s\n" % e)
[inline-code-end]