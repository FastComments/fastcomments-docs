## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Réponse

Retourne : [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_email_template_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de create_email_template'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# configuration de l'authentification
FastCommentsClient.configure do |config|
  # Configurer l'authentification par clé API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Décommentez la ligne suivante pour définir un préfixe pour la clé API, par ex. 'Bearer' (par défaut nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # Chaîne | 
create_email_template_body = FastCommentsClient::CreateEmailTemplateBody.new({email_template_id: 'email_template_id_example', display_name: 'display_name_example', ejs: 'ejs_example'}) # CreateEmailTemplateBody | 

begin
  
  result = api_instance.create_email_template(tenant_id, create_email_template_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_email_template: #{e}"
end
[inline-code-end]

---