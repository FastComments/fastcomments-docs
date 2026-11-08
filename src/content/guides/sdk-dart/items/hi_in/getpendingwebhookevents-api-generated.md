## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | query | नहीं |  |
| externalId | string | query | नहीं |  |
| eventType | string | query | नहीं |  |
| type | string | query | नहीं |  |
| domain | string | query | नहीं |  |
| attemptCountGT | number | query | नहीं |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: `GetPendingWebhookEventsResponse`

## उदाहरण

[inline-code-attrs-start title = 'getPendingWebhookEvents उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API की ऑथराइज़ेशन कॉन्फ़िगर करें: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// यदि आवश्यक हो तो API की के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे की लाइन अनकमेंट करें
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final externalId = externalId_example; // String | 
final eventType = eventType_example; // String | 
final type = type_example; // String | 
final domain = domain_example; // String | 
final attemptCountGT = 1.2; // double | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getPendingWebhookEvents(tenantId, GetPendingWebhookEventsOptions(commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getPendingWebhookEvents: $e\n');
}
[inline-code-end]