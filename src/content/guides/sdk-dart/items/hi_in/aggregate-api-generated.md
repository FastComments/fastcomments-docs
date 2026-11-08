दस्तावेज़ों को समूहित करके (यदि groupBy दिया गया है) और कई ऑपरेशन लागू करके एकत्रित करता है।
विभिन्न ऑपरेशन (जैसे sum, countDistinct, avg, आदि) समर्थित हैं।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| parentTenantId | string | query | नहीं |  |
| includeStats | boolean | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: `AggregateResponse`

## उदाहरण

[inline-code-attrs-start title = 'aggregate उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configure API key authorization: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// आवश्यक हो तो API कुंजी के लिए प्रीफिक्स (जैसे Bearer) सेटअप करने के लिए नीचे की लाइन अनकमेंट करें
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final aggregationRequest = AggregationRequest(); // AggregationRequest | 
final parentTenantId = parentTenantId_example; // String | 
final includeStats = true; // bool | 

try {
    final result = api_instance.aggregate(tenantId, aggregationRequest, AggregateOptions(parentTenantId: parentTenantId, includeStats: includeStats));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->aggregate: $e\n');
}
[inline-code-end]