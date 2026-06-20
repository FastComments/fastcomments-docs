## Paramètres

| Name | Type | Location | Requis | Description |
|------|------|----------|----------|-------------|
| sso | string | query | Non |  |

## Réponse

Renvoie : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de post_ban_user_undo'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
ban_user_undo_params = FastCommentsClient::BanUserUndoParams.new({changelog: FastCommentsClient::APIBanUserChangeLog.new}) # BanUserUndoParams | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.post_ban_user_undo(ban_user_undo_params, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->post_ban_user_undo: #{e}"
end
[inline-code-end]