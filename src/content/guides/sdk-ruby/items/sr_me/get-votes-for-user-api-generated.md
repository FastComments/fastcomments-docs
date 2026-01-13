## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Да |  |
| userId | string | query | Не |  |
| anonUserId | string | query | Не |  |

## Одговор

Враћа: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_votes_for_user200_response.rb)

## Пример

[inline-code-attrs-start title = 'get_votes_for_user Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# подешавање ауторизације
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Откоментирајте следећи ред да бисте поставили префикс за API кључ, нпр. 'Bearer' (подразумевано nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 
opts = {
  user_id: 'user_id_example', # String | 
  anon_user_id: 'anon_user_id_example' # String | 
}

begin
  
  result = api_instance.get_votes_for_user(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_votes_for_user: #{e}"
end
[inline-code-end]