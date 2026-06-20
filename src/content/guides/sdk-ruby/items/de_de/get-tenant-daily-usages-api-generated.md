## Parameter

| Name | Type | Location | Erforderlich | Beschreibung |
|------|------|----------|-------------|-------------|
| tenantId | string | query | Ja |  |
| yearNumber | number | query | Nein |  |
| monthNumber | number | query | Nein |  |
| dayNumber | number | query | Nein |  |
| skip | number | query | Nein |  |

## Antwort

Gibt zurück: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tenant_daily_usages_response.rb)

## Beispiel

[inline-code-attrs-start title = 'get_tenant_daily_usages Beispiel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Einrichtung der Autorisierung
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Entfernen Sie das Kommentarzeichen der folgenden Zeile, um ein Präfix für den API-Schlüssel festzulegen, z. B. 'Bearer' (Standard ist nil)
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

---