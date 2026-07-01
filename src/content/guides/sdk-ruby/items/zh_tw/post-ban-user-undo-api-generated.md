## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## 範例

[inline-code-attrs-start title = 'post_ban_user_undo 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # String | 
ban_user_undo_params = FastCommentsClient::BanUserUndoParams.new({changelog: FastCommentsClient::APIBanUserChangeLog.new}) # BanUserUndoParams | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.post_ban_user_undo(tenant_id, ban_user_undo_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_ban_user_undo: #{e}"
end
[inline-code-end]