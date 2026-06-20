## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |

## Odpowiedź

Zwraca: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/add_domain_config_response.rb)

## Przykład

[inline-code-attrs-start title = 'Przykład add_domain_config'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# konfiguracja autoryzacji
FastCommentsClient.configure do |config|
  # Skonfiguruj autoryzację klucza API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Odkomentuj następującą linię, aby ustawić prefiks dla klucza API, np. 'Bearer' (domyślnie nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
add_domain_config_params = FastCommentsClient::AddDomainConfigParams.new({domain: 'domain_example'}) # AddDomainConfigParams | 

begin
  
  result = api_instance.add_domain_config(tenant_id, add_domain_config_params)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->add_domain_config: #{e}"
end
[inline-code-end]