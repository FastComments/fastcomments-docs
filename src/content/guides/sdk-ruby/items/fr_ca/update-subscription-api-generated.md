## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |
| userId | string | query | Non |  |

## Réponse

Retourne : [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_subscription_a_p_i_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de update_subscription'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configuration de l'autorisation
FastCommentsClient.configure do |config|
  # Configuration de l'autorisation par clé API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Décommentez la ligne suivante pour définir un préfixe pour la clé API, p. ex. 'Bearer' (par défaut nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_api_user_subscription_data = FastCommentsClient::UpdateAPIUserSubscriptionData.new # UpdateAPIUserSubscriptionData | 
opts = {
  user_id: 'user_id_example' # String | 
}

begin
  
  result = api_instance.update_subscription(tenant_id, id, update_api_user_subscription_data, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->update_subscription: #{e}"
end
[inline-code-end]