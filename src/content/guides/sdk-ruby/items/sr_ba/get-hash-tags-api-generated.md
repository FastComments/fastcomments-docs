## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| page | number | query | Ne |  |

## Odgovor

Vraća: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_hash_tags200_response.rb)

## Primjer

[inline-code-attrs-start title = 'get_hash_tags Primjer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# postavljanje autorizacije
FastCommentsClient.configure do |config|
  # Podesi autorizaciju API ključa: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Otkomentari sljedeći red da postaviš prefiks za API ključ, npr. 'Bearer' (podrazumijevano nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  page: 1.2 # Float | 
}

begin
  
  result = api_instance.get_hash_tags(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_hash_tags: #{e}"
end
[inline-code-end]