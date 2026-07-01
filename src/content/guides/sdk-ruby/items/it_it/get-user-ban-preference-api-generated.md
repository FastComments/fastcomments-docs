## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## Response

Restituisce: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_moderate_get_user_ban_preferences_response.rb)

## Esempio

[inline-code-attrs-start title = 'Esempio get_user_ban_preference'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Stringa | 
opts = {
  sso: 'sso_example' # Stringa | 
}

begin
  
  result = api_instance.get_user_ban_preference(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_user_ban_preference: #{e}"
end
[inline-code-end]