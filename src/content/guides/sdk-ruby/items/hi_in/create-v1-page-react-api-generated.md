## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| title | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_v1_page_react.rb)

## उदाहरण

[inline-code-attrs-start title = 'create_v1_page_react उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 
opts = {
  title: 'title_example' # String | 
}

begin
  
  result = api_instance.create_v1_page_react(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->create_v1_page_react: #{e}"
end
[inline-code-end]