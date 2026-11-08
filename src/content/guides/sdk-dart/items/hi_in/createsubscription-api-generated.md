## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: `CreateSubscriptionAPIResponse`

## उदाहरण

[inline-code-attrs-start title = 'createSubscription उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configure API key authorization: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// यदि आवश्यक हो तो API कुंजी के लिए प्रीफिक्स (जैसे Bearer) सेटअप करने के लिए नीचे की लाइन अनकमेंट करें
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createAPIUserSubscriptionData = CreateAPIUserSubscriptionData(); // CreateAPIUserSubscriptionData | 

try {
    final result = api_instance.createSubscription(tenantId, createAPIUserSubscriptionData);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createSubscription: $e\n');
}
[inline-code-end]