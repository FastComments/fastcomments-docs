## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| includeByUserIdAndEmail | boolean | query | नहीं |  |
| includeByIP | boolean | query | नहीं |  |
| includeByEmailDomain | boolean | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/bulk_pre_ban_summary.py)

## उदाहरण

[inline-code-attrs-start title = 'post_bulk_pre_ban_summary उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.moderation_api import PostBulkPreBanSummaryOptions
from client.models.bulk_pre_ban_params import BulkPreBanParams
from client.models.bulk_pre_ban_summary import BulkPreBanSummary
from client.rest import ApiException
from pprint import pprint

# होस्ट को परिभाषित करना वैकल्पिक है और इसका डिफ़ॉल्ट https://fastcomments.com है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API क्लाइंट के एक इंस्टेंस के साथ एक कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएँ
    api_instance = client.ModerationApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    bulk_pre_ban_params = client.BulkPreBanParams() # BulkPreBanParams | 
    include_by_user_id_and_email = True # bool |  (वैकल्पिक)
    include_by_ip = True # bool |  (वैकल्पिक)
    include_by_email_domain = True # bool |  (वैकल्पिक)
    sso = 'sso_example' # str |  (वैकल्पिक)

    try:
        api_response = api_instance.post_bulk_pre_ban_summary(tenant_id, bulk_pre_ban_params, PostBulkPreBanSummaryOptions(include_by_user_id_and_email=include_by_user_id_and_email, include_by_ip=include_by_ip, include_by_email_domain=include_by_email_domain, sso=sso))
        print("The response of ModerationApi->post_bulk_pre_ban_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_bulk_pre_ban_summary: %s\n" % e)
[inline-code-end]