## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Tak |  |
| banEmail | boolean | query | Nie |  |
| banEmailDomain | boolean | query | Nie |  |
| banIP | boolean | query | Nie |  |
| deleteAllUsersComments | boolean | query | Nie |  |
| bannedUntil | string | query | Nie |  |
| isShadowBan | boolean | query | Nie |  |
| updateId | string | query | Nie |  |
| banReason | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/ban_user_from_comment_result.rb)

## Przykład

[inline-code-attrs-start title = 'Przykład post_ban_user_from_comment'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # Ciąg znaków | 
opts = {
  ban_email: true, # Wartość logiczna | 
  ban_email_domain: true, # Wartość logiczna | 
  ban_ip: true, # Wartość logiczna | 
  delete_all_users_comments: true, # Wartość logiczna | 
  banned_until: 'banned_until_example', # Ciąg znaków | 
  is_shadow_ban: true, # Wartość logiczna | 
  update_id: 'update_id_example', # Ciąg znaków | 
  ban_reason: 'ban_reason_example', # Ciąg znaków | 
  sso: 'sso_example' # Ciąg znaków | 
}

begin
  
  result = api_instance.post_ban_user_from_comment(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_ban_user_from_comment: #{e}"
end
[inline-code-end]