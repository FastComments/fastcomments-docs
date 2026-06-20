## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Oui |  |
| includeByUserIdAndEmail | boolean | query | Non |  |
| includeByIP | boolean | query | Non |  |
| includeByEmailDomain | boolean | query | Non |  |
| sso | string | query | Non |  |

## Response

Renvoie : [`PreBanSummary`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/pre_ban_summary.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_pre_ban_summary'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
comment_id = 'comment_id_example' # Chaîne | 
opts = {
  include_by_user_id_and_email: true, # Booléen | 
  include_by_ip: true, # Booléen | 
  include_by_email_domain: true, # Booléen | 
  sso: 'sso_example' # Chaîne | 
}

begin
  
  result = api_instance.get_pre_ban_summary(comment_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_pre_ban_summary: #{e}"
end
[inline-code-end]

---