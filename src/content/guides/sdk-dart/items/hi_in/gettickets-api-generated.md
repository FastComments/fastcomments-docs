## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| userId | string | query | नहीं |  |
| state | number | query | नहीं |  |
| skip | number | query | नहीं |  |
| limit | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `GetTicketsResponse`

## उदाहरण

[inline-code-attrs-start title = 'getTickets उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API की ऑथराइज़ेशन कॉन्फ़िगर करें: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// आवश्यकता होने पर API की के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे कमेंट हटाएँ
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final state = 1.2; // double | 
final skip = 1.2; // double | 
final limit = 1.2; // double | 

try {
    final result = api_instance.getTickets(tenantId, GetTicketsOptions(userId: userId, state: state, skip: skip, limit: limit));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTickets: $e\n');
}
[inline-code-end]