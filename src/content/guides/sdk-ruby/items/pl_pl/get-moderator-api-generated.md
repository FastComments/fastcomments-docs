## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| id | string | path | Tak |  |

## Odpowiedź

Zwraca: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_moderator200_response.rb)

## Przykład

[inline-code-attrs-start title = 'Przykład get_moderator'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# konfiguracja autoryzacji
FastCommentsClient.configure do |config|
  # Skonfiguruj autoryzację klucza API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Odkomentuj poniższą linię, aby ustawić prefiks dla klucza API, np. 'Bearer' (domyślnie nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.get_moderator(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_moderator: #{e}"
end
[inline-code-end]