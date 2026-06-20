---
## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| urlIdWS | string | query | はい |  |
| userIds | string | query | はい |  |

## レスポンス

戻り値: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_user_presence_statuses_response.py)

## 例

[inline-code-attrs-start title = 'get_user_presence_statuses の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_user_presence_statuses_response import GetUserPresenceStatusesResponse
from client.rest import ApiException
from pprint import pprint

# ホストの定義はオプションで、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# APIクライアントのインスタンスでコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # APIクラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id_ws = 'url_id_ws_example' # str | 
    user_ids = 'user_ids_example' # str | 

    try:
        api_response = api_instance.get_user_presence_statuses(tenant_id, url_id_ws, user_ids)
        print("The response of PublicApi->get_user_presence_statuses:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_user_presence_statuses: %s\n" % e)
[inline-code-end]

---