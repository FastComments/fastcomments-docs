## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## 範例

[inline-code-attrs-start title = 'logout_public 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# 定義主機為選用，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 以 API 用戶端實例開啟一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)

    try:
        api_response = api_instance.logout_public()
        print("The response of PublicApi->logout_public:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->logout_public: %s\n" % e)
[inline-code-end]

---