## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| updateComments | boolean | query | No |  |

## Response

Returns: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/put_s_s_o_user_a_p_i_response.rb)

## Example

[inline-code-attrs-start title = 'put_sso_user Example'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# setup authorization
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Uncomment the following line to set a prefix for the API key, e.g. 'Bearer' (defaults to nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
update_apisso_user_data = FastCommentsClient::UpdateAPISSOUserData.new # UpdateAPISSOUserData | 
opts = {
  update_comments: true # Boolean | 
}

begin
  
  result = api_instance.put_sso_user(tenant_id, id, update_apisso_user_data, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->put_sso_user: #{e}"
end
[inline-code-end]
