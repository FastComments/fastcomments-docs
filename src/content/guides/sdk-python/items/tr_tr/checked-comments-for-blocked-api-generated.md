## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| commentIds | string | query | Evet | Virgülle ayrılmış yorum kimlikleri listesi. |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/checked_comments_for_blocked200_response.py)

## Örnek

[inline-code-attrs-start title = 'checked_comments_for_blocked Örneği'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.checked_comments_for_blocked200_response import CheckedCommentsForBlocked200Response
from client.rest import ApiException
from pprint import pprint

# Host tanımlamak isteğe bağlıdır ve varsayılanı https://fastcomments.com'dur
# Desteklenen tüm yapılandırma parametrelerinin listesi için configuration.py dosyasına bakın.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API istemcisi örneği ile bir bağlam başlatın
with client.ApiClient(configuration) as api_client:
    # API sınıfından bir örnek oluşturun
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    comment_ids = 'comment_ids_example' # str | A comma separated list of comment ids.
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.checked_comments_for_blocked(tenant_id, comment_ids, sso=sso)
        print("The response of PublicApi->checked_comments_for_blocked:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->checked_comments_for_blocked: %s\n" % e)
[inline-code-end]