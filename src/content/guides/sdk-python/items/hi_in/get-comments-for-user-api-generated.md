## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| userId | string | query | नहीं |  |
| direction | string | query | नहीं |  |
| repliesToUserId | string | query | नहीं |  |
| page | number | query | नहीं |  |
| includei10n | boolean | query | नहीं |  |
| locale | string | query | नहीं |  |
| isCrawler | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## उदाहरण

[inline-code-attrs-start title = 'get_comments_for_user उदाहरण'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetCommentsForUserOptions
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# होस्ट को परिभाषित करना वैकल्पिक है और यह डिफ़ॉल्ट रूप से https://fastcomments.com है
# सभी समर्थित कॉन्फ़िगरेशन पैरामीटर की सूची के लिए configuration.py देखें।
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# API क्लाइंट के एक इंस्टेंस के साथ एक कॉन्टेक्स्ट दर्ज करें
with client.ApiClient(configuration) as api_client:
    # API क्लास का एक इंस्टेंस बनाएं
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (वैकल्पिक)
    direction = client.SortDirections() # SortDirections |  (वैकल्पिक)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (वैकल्पिक)
    page = 3.4 # float |  (वैकल्पिक)
    includei10n = True # bool |  (वैकल्पिक)
    locale = 'locale_example' # str |  (वैकल्पिक)
    is_crawler = True # bool |  (वैकल्पिक)

    try:
        api_response = api_instance.get_comments_for_user(GetCommentsForUserOptions(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler))
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]