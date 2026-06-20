---
租戶的批次使用者資訊。給定 userIds，從 User / SSOUser 回傳顯示資訊。
由評論小工具用來豐富剛透過 presence 事件出現的使用者。
無頁面上下文：隱私一致性強制執行（私人資料會被遮蔽）。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| ids | string | query | 是 | 以逗號分隔的 userIds. |

## 回應

回傳：[`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## 範例

[inline-code-attrs-start title = 'get_users_info 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | 以逗號分隔的 userIds.

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]

---