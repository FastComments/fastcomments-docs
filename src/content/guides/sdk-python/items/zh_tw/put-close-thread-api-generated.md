## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| urlId | string | query | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 範例

[inline-code-attrs-start title = 'put_close_thread 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 有關所有支援的組態參數列表，請參閱 configuration.py。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 用戶端實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.ModerationApi(api_client)
    url_id = 'url_id_example' # str | 
    sso = 'sso_example' # str | (可選)

    try:
        api_response = api_instance.put_close_thread(url_id, sso=sso)
        print("The response of ModerationApi->put_close_thread:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_close_thread: %s\n" % e)
[inline-code-end]