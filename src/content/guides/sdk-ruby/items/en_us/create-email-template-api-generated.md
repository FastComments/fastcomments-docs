## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_email_template200_response.rb)

## Example

[inline-code-attrs-start title = 'create_email_template Example'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Set up authorization
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Uncomment the following line to set a prefix for the API key, e.g., 'Bearer' (defaults to nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_email_template_body = FastCommentsClient::CreateEmailTemplateBody.new({email_template_id: 'email_template_id_example', display_name: 'display_name_example', ejs: 'ejs_example'}) # CreateEmailTemplateBody | 

begin
  
  result = api_instance.create_email_template(tenant_id, create_email_template_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_email_template: #{e}"
end
[inline-code-end]

---