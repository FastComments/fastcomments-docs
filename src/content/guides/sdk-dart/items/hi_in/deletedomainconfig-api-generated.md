## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| domain | string | path | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: `DeleteDomainConfigResponse`

## उदाहरण

[inline-code-attrs-start title = 'deleteDomainConfig उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API की अधिकृति api_key कॉन्फ़िगर करें
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// यदि आवश्यक हो तो API की के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे की लाइन अनकमेंट करें
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final domain = domain_example; // String | 

try {
    final result = api_instance.deleteDomainConfig(tenantId, domain);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteDomainConfig: $e\n');
}
[inline-code-end]