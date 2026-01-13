## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentId | string | query | Da |  |
| direction | string | query | Da |  |
| userId | string | query | Ne |  |
| anonUserId | string | query | Ne |  |

## Odgovor

Vraća: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/vote_comment200_response.rb)

## Primjer

[inline-code-attrs-start title = 'Primjer create_vote'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Postavljanje autorizacije
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Otkomentarišite sljedeću liniju da postavite prefiks za API ključ, npr. 'Bearer' (zadano: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
comment_id = 'comment_id_example' # String | 
direction = 'up' # String | 
opts = {
  user_id: 'user_id_example', # String | 
  anon_user_id: 'anon_user_id_example' # String | 
}

begin
  
  result = api_instance.create_vote(tenant_id, comment_id, direction, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_vote: #{e}"
end
[inline-code-end]