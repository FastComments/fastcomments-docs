## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| yearNumber | number | query | Non |  |
| monthNumber | number | query | Non |  |
| dayNumber | number | query | Non |  |
| skip | number | query | Non |  |

## Réponse

Renvoie : [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_tenant_daily_usages200_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_tenant_daily_usages'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configuration de l'autorisation
FastCommentsClient.configure do |config|
  # Configurer l'autorisation par clé API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Décommentez la ligne suivante pour définir un préfixe pour la clé API, par ex. 'Bearer' (par défaut nil)
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