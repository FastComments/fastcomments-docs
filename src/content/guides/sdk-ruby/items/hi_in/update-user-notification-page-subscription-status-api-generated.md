किसी पेज के लिए सूचनाएँ सक्षम या अक्षम करें। जब उपयोगकर्ता किसी पेज की सदस्यता लेते हैं, तो नई रूट टिप्पणियों के लिए सूचनाएँ बनाई जाती हैं, और साथ ही

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| urlId | string | query | हाँ |  |
| url | string | query | हाँ |  |
| pageTitle | string | query | हाँ |  |
| subscribedOrUnsubscribed | string | path | हाँ |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: `UpdateUserNotificationPageSubscriptionStatusResponse`

## उदाहरण

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 
url = 'url_example' # String | 
page_title = 'page_title_example' # String | 
subscribed_or_unsubscribed = 'subscribe' # String | 
opts = {
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.update_user_notification_page_subscription_status(tenant_id, url_id, url, page_title, subscribed_or_unsubscribed, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->update_user_notification_page_subscription_status: #{e}"
end
[inline-code-end]