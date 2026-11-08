## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| userId | string | query | हाँ |  |
| id | string | path | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/change_ticket_state_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'change_ticket_state उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# प्रमाणीकरण सेटअप करें
FastCommentsClient.configure do |config|
  # API कुंजी प्रमाणीकरण कॉन्फ़िगर करें: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API कुंजी के लिए उपसर्ग सेट करने के लिए निम्नलिखित पंक्ति को अनकमेंट करें, जैसे 'Bearer' (डिफ़ॉल्ट nil है)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
user_id = 'user_id_example' # String | 
id = 'id_example' # String | 
change_ticket_state_body = FastCommentsClient::ChangeTicketStateBody.new({state: 37}) # ChangeTicketStateBody | 

begin
  
  result = api_instance.change_ticket_state(tenant_id, user_id, id, change_ticket_state_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->change_ticket_state: #{e}"
end
[inline-code-end]