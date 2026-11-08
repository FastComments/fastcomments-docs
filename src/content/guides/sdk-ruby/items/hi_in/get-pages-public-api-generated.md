किसी टेनेंट के लिए पेजों की सूची लाता है। FChat डेस्कटॉप क्लाइंट द्वारा अपनी रूम सूची भरने के लिए उपयोग किया जाता है।
प्रत्येक पेज के लिए resolved custom config में `enableFChat` true होना आवश्यक है।
SSO की आवश्यकता वाले पेज अनुरोध करने वाले उपयोगकर्ता के group access के आधार पर फ़िल्टर किए जाते हैं।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| cursor | string | query | नहीं | पिछले अनुरोध से `nextCursor` के रूप में लौटाया गया अपारदर्शी पेजिनेशन कर्सर। उसी `sortBy` से बंधा होता है। |
| limit | integer | query | नहीं | 1..200, डिफ़ॉल्ट 50 |
| q | string | query | नहीं | वैकल्पिक, केस-असंवेदनशील शीर्षक प्रीफ़िक्स फ़िल्टर। |
| sortBy | string | query | नहीं | क्रमबद्ध क्रम। `updatedAt` (डिफ़ॉल्ट, नवीनतम पहले), `commentCount` (सर्वाधिक कमेंट पहले), या `title` (वर्णानुक्रम)। |
| hasComments | boolean | query | नहीं | यदि true, तो केवल कम से कम एक कमेंट वाले पेज लौटाएं। |

## प्रतिक्रिया

लौटाता है: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'get_pages_public उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | पिछले अनुरोध से `nextCursor` के रूप में लौटाया गया अपारदर्शी पेजिनेशन कर्सर। उसी `sortBy` से बंधा होता है।
  limit: 56, # Integer | 1..200, डिफ़ॉल्ट 50
  q: 'q_example', # String | वैकल्पिक, केस-असंवेदनशील शीर्षक प्रीफ़िक्स फ़िल्टर।
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | क्रमबद्ध क्रम। `updatedAt` (डिफ़ॉल्ट, नवीनतम पहले), `commentCount` (सर्वाधिक कमेंट पहले), या `title` (वर्णानुक्रम)।
  has_comments: true # Boolean | यदि true, तो केवल कम से कम एक कमेंट वाले पेज लौटाएं।
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]