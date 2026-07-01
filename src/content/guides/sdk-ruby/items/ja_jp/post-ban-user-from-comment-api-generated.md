## パラメータ

| 名前 | 種類 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| commentId | string | path | はい |  |
| banEmail | boolean | query | いいえ |  |
| banEmailDomain | boolean | query | いいえ |  |
| banIP | boolean | query | いいえ |  |
| deleteAllUsersComments | boolean | query | いいえ |  |
| bannedUntil | string | query | いいえ |  |
| isShadowBan | boolean | query | いいえ |  |
| updateId | string | query | いいえ |  |
| banReason | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/ban_user_from_comment_result.rb)

## 例

[inline-code-attrs-start title = 'post_ban_user_from_comment の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # 文字列 | 
comment_id = 'comment_id_example' # 文字列 | 
opts = {
  ban_email: true, # 真偽値 | 
  ban_email_domain: true, # 真偽値 | 
  ban_ip: true, # 真偽値 | 
  delete_all_users_comments: true, # 真偽値 | 
  banned_until: 'banned_until_example', # 文字列 | 
  is_shadow_ban: true, # 真偽値 | 
  update_id: 'update_id_example', # 文字列 | 
  ban_reason: 'ban_reason_example', # 文字列 | 
  sso: 'sso_example' # 文字列 | 
}

begin
  
  result = api_instance.post_ban_user_from_comment(tenant_id, comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_ban_user_from_comment: #{e}"
end
[inline-code-end]