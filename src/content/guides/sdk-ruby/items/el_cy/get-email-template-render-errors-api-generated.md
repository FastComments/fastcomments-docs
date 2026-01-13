## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |
| skip | number | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetEmailTemplateRenderErrors200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_email_template_render_errors200_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_email_template_render_errors'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# ρύθμιση εξουσιοδότησης
FastCommentsClient.configure do |config|
  # Διαμόρφωση εξουσιοδότησης με κλειδί API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Καταργήστε το σχόλιο στην παρακάτω γραμμή για να ορίσετε ένα πρόθεμα για το κλειδί API, π.χ. 'Bearer' (προεπιλογή nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_email_template_render_errors(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_email_template_render_errors: #{e}"
end
[inline-code-end]

---