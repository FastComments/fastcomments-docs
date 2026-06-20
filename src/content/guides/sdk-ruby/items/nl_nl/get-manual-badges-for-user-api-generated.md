## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | Nee |  |
| commentId | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_manual_badges_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'get_manual_badges_for_user Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  badges_user_id: 'badges_user_id_example', # String | 
  comment_id: 'comment_id_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_manual_badges_for_user(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_manual_badges_for_user: #{e}"
end
[inline-code-end]

---