किसी पेज के वर्तमान में ऑनलाइन दर्शक: वे लोग जिनका websocket सेशन इस समय पेज के लिए सब्सक्राइब है।
anonCount + totalCount लौटाता है (पूरे रूम के सब्सक्राइबर, जिनमें वे अनाम दर्शक भी शामिल हैं जिन्हें हम एन्युमरेट नहीं करते)।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)। |
| afterName | string | query | नहीं | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें। |
| afterUserId | string | query | नहीं | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। afterName सेट होने पर आवश्यक ताकि नाम समान होने पर प्रविष्टियाँ न छूटें। |

## प्रतिक्रिया

लौटाता है: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## उदाहरण

[inline-code-attrs-start title = 'get_online_users उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]