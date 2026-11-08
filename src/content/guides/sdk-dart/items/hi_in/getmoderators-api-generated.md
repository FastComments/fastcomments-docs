## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `GetModeratorsResponse`

## उदाहरण

[inline-code-attrs-start title = 'getModerators उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API की अधिकृति कॉन्फ़िगर करें: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// यदि आवश्यक हो तो API की के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे की पंक्ति अनकमेंट करें
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getModerators(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getModerators: $e\n');
}
[inline-code-end]