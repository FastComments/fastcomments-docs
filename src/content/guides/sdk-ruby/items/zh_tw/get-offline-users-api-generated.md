在該頁面上曾發表評論但目前不在線的使用者。依 displayName 排序。
在用盡 /users/online 後使用，以呈現「成員」區段。
對 commenterName 進行游標分頁：伺服器在部分索引 {tenantId, urlId, commenterName} 上從 afterName 向前走，透過 $gt，無 $skip 成本。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | Page URL identifier (cleaned server-side). |
| afterName | string | query | 否 | Cursor：請從先前回應傳入 nextAfterName。 |
| afterUserId | string | query | 否 | 游標平手決勝：請從先前回應傳入 nextAfterUserId。當設定 afterName 時需要提供，避免名稱相同造成條目遺失。 |

## 回應

回傳：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## 範例

[inline-code-attrs-start title = 'get_offline_users 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 頁面 URL 識別碼（伺服器端清理）。
opts = {
  after_name: 'after_name_example', # String | 游標：請從先前回應傳入 nextAfterName。
  after_user_id: 'after_user_id_example' # String | 游標平手決勝：請從先前回應傳入 nextAfterUserId。當設定 afterName 時需要提供，避免名稱相同造成條目遺失。
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]