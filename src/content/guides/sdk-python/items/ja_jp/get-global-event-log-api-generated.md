req
tenantId
urlId
userIdWS

## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい |  |
| userIdWS | string | query | はい |  |
| startTime | integer | query | はい |  |
| endTime | integer | query | いいえ |  |

## レスポンス

戻り値: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_event_log_response.py)

## 例

[inline-code-attrs-start title = 'get_global_event_log の例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_event_log_response import GetEventLogResponse
from client.rest import ApiException
from pprint import pprint

# ホストの指定は任意で、デフォルトは https://fastcomments.com です
# サポートされているすべての設定パラメータの一覧は configuration.py を参照してください。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API クライアントのインスタンスを使用するコンテキストに入ります
with client.ApiClient(configuration) as api_client:
    # API クラスのインスタンスを作成します
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    user_id_ws = 'user_id_ws_example' # str | 
    start_time = 56 # int | 
    end_time = 56 # int |  (オプション)

    try:
        api_response = api_instance.get_global_event_log(tenant_id, url_id, user_id_ws, start_time, end_time=end_time)
        print("The response of PublicApi->get_global_event_log:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_global_event_log: %s\n" % e)
[inline-code-end]