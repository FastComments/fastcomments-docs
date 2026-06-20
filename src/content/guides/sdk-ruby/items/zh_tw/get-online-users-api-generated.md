頁面當前在線的觀眾：目前其 websocket 會話已訂閱該頁面的人。回傳 anonCount + totalCount（整個房間的訂閱者，包括我們未列舉的匿名觀眾）。

## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Page URL identifier (cleaned server-side). |
| afterName | string | query | No | Cursor: pass nextAfterName from the previous response. |
| afterUserId | string | query | No | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |

## 回應

回傳: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## 範例

[inline-code-attrs-start title = 'get_online_users 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 頁面 URL 識別符（伺服器端已清理）。
opts = {
  after_name: 'after_name_example', # String | 游標：將上一次回應的 nextAfterName 傳入。
  after_user_id: 'after_user_id_example' # String | 游標平手決勝參數：將上一次回應的 nextAfterUserId 傳入。當設定了 afterName 時為必要，以免名稱相同時遺失條目。
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]