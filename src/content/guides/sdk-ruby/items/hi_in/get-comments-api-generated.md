## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| page | integer | query | नहीं |  |
| limit | integer | query | नहीं |  |
| skip | integer | query | नहीं |  |
| asTree | boolean | query | नहीं |  |
| skipChildren | integer | query | नहीं |  |
| limitChildren | integer | query | नहीं |  |
| maxTreeDepth | integer | query | नहीं |  |
| urlId | string | query | नहीं |  |
| userId | string | query | नहीं |  |
| anonUserId | string | query | नहीं |  |
| contextUserId | string | query | नहीं |  |
| hashTag | string | query | नहीं |  |
| parentId | string | query | नहीं |  |
| direction | string | query | नहीं |  |
| fromDate | integer | query | नहीं |  |
| toDate | integer | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/api_get_comments_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'get_comments उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# प्रमाणीकरण सेटअप करें
FastCommentsClient.configure do |config|
  # API की प्रमाणीकरण कॉन्फ़िगर करें: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API की के लिए प्रीफ़िक्स सेट करने के लिए निम्नलिखित पंक्ति को अनकमेंट करें, जैसे 'Bearer' (डिफ़ॉल्ट nil है)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  page: 56, # Integer | 
  limit: 56, # Integer | 
  skip: 56, # Integer | 
  as_tree: true, # Boolean | 
  skip_children: 56, # Integer | 
  limit_children: 56, # Integer | 
  max_tree_depth: 56, # Integer | 
  url_id: 'url_id_example', # String | 
  user_id: 'user_id_example', # String | 
  anon_user_id: 'anon_user_id_example', # String | 
  context_user_id: 'context_user_id_example', # String | 
  hash_tag: 'hash_tag_example', # String | 
  parent_id: 'parent_id_example', # String | 
  direction: FastCommentsClient::SortDirections::OF, # SortDirections | 
  from_date: 789, # Integer | 
  to_date: 789 # Integer | 
}

begin
  
  result = api_instance.get_comments(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_comments: #{e}"
end
[inline-code-end]