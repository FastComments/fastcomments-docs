## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| postIds | array | query | No |  |
| sso | string | query | No |  |

## 回應

回傳：[`GetUserReactsPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_reacts_public200_response.py)

## 範例

[inline-code-attrs-start title = 'get_user_reacts_public 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_reacts_public200_response import GetUserReactsPublic200Response
from client.rest import ApiException
from pprint import pprint

# 定義 host 是選用的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 與 ApiClient 實例一起進入上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    post_ids = ['post_ids_example'] # List[str] |  （可選）
    sso = 'sso_example' # str |  （可選）

    try:
        api_response = api_instance.get_user_reacts_public(tenant_id, post_ids=post_ids, sso=sso)
        print("The response of PublicApi->get_user_reacts_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_reacts_public: %s\n" % e)
[inline-code-end]