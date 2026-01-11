req
tenantId
urlId
userIdWS

## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| userIdWS | string | query | 是 |  |
| startTime | integer | query | 是 |  |
| endTime | integer | query | 是 |  |

## 回應

回傳: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_event_log200_response.py)

## 範例

[inline-code-attrs-start title = 'get_event_log 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_event_log200_response import GetEventLog200Response
from client.rest import ApiException
from pprint import pprint

# 設定 host 為可選，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API client 的實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    user_id_ws = 'user_id_ws_example' # str | 
    start_time = 56 # int | 
    end_time = 56 # int | 

    try:
        api_response = api_instance.get_event_log(tenant_id, url_id, user_id_ws, start_time, end_time)
        print("The response of PublicApi->get_event_log:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_event_log: %s\n" % e)
[inline-code-end]