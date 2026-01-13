## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| id | string | path | Tak |  |
| sure | string | query | Nie |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## Przykład

[inline-code-attrs-start title = 'delete_tenant Przykład'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# konfiguracja autoryzacji
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Odkomentuj następującą linię, aby ustawić prefiks dla klucza API, np. 'Bearer' (domyślnie nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  sure: 'sure_example' # String | 
}

begin
  
  result = api_instance.delete_tenant(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_tenant: #{e}"
end
[inline-code-end]

---