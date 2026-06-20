## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| yearNumber | number | query | Ne |  |
| monthNumber | number | query | Ne |  |
| dayNumber | number | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vraća: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tenant_daily_usages_response.rb)

## Primer

[inline-code-attrs-start title = 'Primer get_tenant_daily_usages'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# postavljanje autorizacije
FastCommentsClient.configure do |config|
  # Konfigurišite autorizaciju API ključa: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Otkomentarišite sledeći red da biste postavili prefiks za API ključ, npr. 'Bearer' (podrazumevano nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  year_number: 1.2, # Float | 
  month_number: 1.2, # Float | 
  day_number: 1.2, # Float | 
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_tenant_daily_usages(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_tenant_daily_usages: #{e}"
end
[inline-code-end]