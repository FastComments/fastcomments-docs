## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |

## Απάντηση

Επιστρέφει: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_email_template_definitions200_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_email_template_definitions'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# ρύθμιση εξουσιοδότησης
FastCommentsClient.configure do |config|
  # Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Αποσχολιάστε την παρακάτω γραμμή για να ορίσετε ένα πρόθεμα για το API key, π.χ. 'Bearer' (εξ ορισμού nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 

begin
  
  result = api_instance.get_email_template_definitions(tenant_id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_email_template_definitions: #{e}"
end
[inline-code-end]