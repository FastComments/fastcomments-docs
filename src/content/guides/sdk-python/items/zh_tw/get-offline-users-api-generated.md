過去在該頁面發表過評論但目前不在線上的使用者。依 displayName 排序。
在使用完 /users/online 後使用此來呈現「成員」區段。
對 commenterName 的游標分頁：伺服器會遍歷部分索引 {tenantId, urlId, commenterName}
從 afterName 之後使用 $gt 向前索引，無 $skip 成本。

## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 頁面 URL 識別符（在伺服器端清理）。 |
| afterName | string | query | 否 | 游標：從先前的回應傳入 nextAfterName。 |
| afterUserId | string | query | 否 | 游標決勝鍵：從先前的回應傳入 nextAfterUserId。當設定 afterName 時需要提供，以免同名導致遺失項目。 |

## 回應

回傳：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## 範例

[inline-code-attrs-start title = 'get_offline_users 範例'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# 定義 host 是可選的，預設為 https://fastcomments.com
# 請參閱 configuration.py 以取得所有支援的設定參數清單。
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# 使用 API 客戶端實例進入一個上下文
with client.ApiClient(configuration) as api_client:
    # 建立 API 類別的實例
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 頁面 URL 識別符（在伺服器端清理）。
    after_name = 'after_name_example' # str | 游標：從先前的回應傳入 nextAfterName。 (optional)
    after_user_id = 'after_user_id_example' # str | 游標決勝鍵：從先前的回應傳入 nextAfterUserId。當設定 afterName 時需要提供，以免同名導致遺失項目。 (optional)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]

---