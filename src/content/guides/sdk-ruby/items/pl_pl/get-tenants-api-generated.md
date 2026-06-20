## Parametry

| Name | Type | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| meta | string | query | Nie |  |
| skip | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tenants_response.rb)

## Przykład

[inline-code-attrs-start title = 'get_tenants Przykład'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# konfiguracja autoryzacji
FastCommentsClient.configure do |config|
  # Skonfiguruj autoryzację klucza API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Odkomentuj następną linię, aby ustawić prefiks dla klucza API, np. 'Bearer' (wartość domyślna: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  meta: 'meta_example', # String | 
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_tenants(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_tenants: #{e}"
end
[inline-code-end]

---