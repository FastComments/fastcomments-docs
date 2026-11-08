## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |
| userId | string | query | नहीं |  |
| anonUserId | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: `FlagCommentResponse`

## उदाहरण

[inline-code-attrs-start title = 'flagComment उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API की ऑथराइज़ेशन: api_key कॉन्फ़िगर करें
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// यदि आवश्यक हो तो API की के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे की लाइन अनकमेंट करें
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final userId = userId_example; // String | 
final anonUserId = anonUserId_example; // String | 

try {
    final result = api_instance.flagComment(tenantId, id, FlagCommentOptions(userId: userId, anonUserId: anonUserId));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->flagComment: $e\n');
}
[inline-code-end]