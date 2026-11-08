## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| urlId | string | query | नहीं |  |
| fromCommentId | string | query | नहीं |  |
| viewed | boolean | query | नहीं |  |
| type | string | query | नहीं |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `GetNotificationsResponse`

## उदाहरण

[inline-code-attrs-start title = 'getNotifications उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API की ऑथराइज़ेशन कॉन्फ़िगर करें: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// यदि आवश्यक हो तो API की के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे कमेंट हटाएँ
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final urlId = urlId_example; // String | 
final fromCommentId = fromCommentId_example; // String | 
final viewed = true; // bool | 
final type = type_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getNotifications(tenantId, GetNotificationsOptions(userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getNotifications: $e\n');
}
[inline-code-end]