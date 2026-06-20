租戶的批量使用者資訊。給定 userIds，回傳來自 User / SSOUser 的顯示資訊。
此功能供評論小工具使用，以補足剛透過 presence 事件出現的使用者資訊。
無頁面上下文：隱私會一致地套用（私人個人檔案會被遮蔽）。

## 參數

| 名稱 | Type | Location | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| ids | string | query | 是 | 以逗號分隔的 userIds。 |

## 回應

回傳：[`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_info_response.py)

## 範例

[inline-code-attrs-start title = 'get_users_info 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_info_response import PageUsersInfoResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API client 的實例建立一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    ids = 'ids_example' # str | 以逗號分隔的 userIds。

    try:
        api_response = api_instance.get_users_info(tenant_id, ids)
        print("The response of PublicApi->get_users_info:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_users_info: %s\n" % e)
[inline-code-end]