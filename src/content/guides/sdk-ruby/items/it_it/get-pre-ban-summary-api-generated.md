## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Sì |  |
| includeByUserIdAndEmail | boolean | query | No |  |
| includeByIP | boolean | query | No |  |
| includeByEmailDomain | boolean | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`PreBanSummary`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/pre_ban_summary.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_pre_ban_summary'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # Stringa | 
opts = {
  include_by_user_id_and_email: true, # Booleano | 
  include_by_ip: true, # Booleano | 
  include_by_email_domain: true, # Booleano | 
  sso: 'sso_example' # Stringa | 
}

begin
  
  result = api_instance.get_pre_ban_summary(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_pre_ban_summary: #{e}"
end
[inline-code-end]