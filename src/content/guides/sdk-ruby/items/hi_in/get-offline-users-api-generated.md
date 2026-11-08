पेज पर उन पिछले टिप्पणीकारों की सूची जो अभी ऑनलाइन नहीं हैं। displayName के अनुसार क्रमबद्ध।
"Members" सेक्शन रेंडर करने के लिए /users/online को पूरा करने के बाद इसका उपयोग करें।
commenterName पर कर्सर पेजिनेशन: सर्वर afterName से आगे $gt के माध्यम से आंशिक {tenantId, urlId, commenterName}
इंडेक्स को ट्रैवर्स करता है, कोई $skip लागत नहीं।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)। |
| afterName | string | query | नहीं | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें। |
| afterUserId | string | query | नहीं | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। afterName सेट होने पर आवश्यक ताकि नाम समान होने पर प्रविष्टियाँ न छूटें। |

## प्रतिक्रिया

रिटर्न: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'get_offline_users उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)।
opts = {
  after_name: 'after_name_example', # String | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें।
  after_user_id: 'after_user_id_example' # String | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। afterName सेट होने पर आवश्यक ताकि नाम समान होने पर प्रविष्टियाँ न छूटें।
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]