req  
tenantId  
afterId  

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| afterId | string | query | Hayır |  |
| limit | integer | query | Hayır |  |
| tags | array | query | Hayır |  |
| sso | string | query | Hayır |  |
| isCrawler | boolean | query | Hayır |  |
| includeUserInfo | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/public_feed_posts_response.py)

## Örnek

[inline-code-attrs-start title = 'get_feed_posts_public Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
import client  
from client.api.public_api import GetFeedPostsPublicOptions  
from client.models.public_feed_posts_response import PublicFeedPostsResponse  
from client.rest import ApiException  
from pprint import pprint  

# Sunucunun tanımlanması isteğe bağlıdır ve varsayılan https://fastcomments.com adresidir  
# Tüm desteklenen yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.  
configuration = client.Configuration(  
    host = "https://fastcomments.com"  
)  


# API istemcisinin bir örneğiyle bir bağlam girin  
with client.ApiClient(configuration) as api_client:  
    # API sınıfının bir örneğini oluşturun  
    api_instance = client.PublicApi(api_client)  
    tenant_id = 'tenant_id_example' # str |  
    after_id = 'after_id_example' # str |  (optional)  
    limit = 56 # int |  (optional)  
    tags = ['tags_example'] # List[str] |  (optional)  
    sso = 'sso_example' # str |  (optional)  
    is_crawler = True # bool |  (optional)  
    include_user_info = True # bool |  (optional)  

    try:  
        api_response = api_instance.get_feed_posts_public(tenant_id, GetFeedPostsPublicOptions(after_id=after_id, limit=limit, tags=tags, sso=sso, is_crawler=is_crawler, include_user_info=include_user_info))  
        print("PublicApi->get_feed_posts_public yanıtı:\n")  
        pprint(api_response)  
    except Exception as e:  
        print("PublicApi->get_feed_posts_public çağrılırken istisna: %s\n" % e)  
[inline-code-end]