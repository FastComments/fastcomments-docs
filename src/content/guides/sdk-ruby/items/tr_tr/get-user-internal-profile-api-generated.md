## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_user_internal_profile_response.rb)

## Örnek

[inline-code-attrs-start title = 'get_user_internal_profile Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  comment_id: 'comment_id_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_user_internal_profile(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_user_internal_profile: #{e}"
end
[inline-code-end]