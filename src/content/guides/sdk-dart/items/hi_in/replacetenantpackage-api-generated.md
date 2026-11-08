## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: `APIEmptyResponse`

## उदाहरण

[inline-code-attrs-start title = 'replaceTenantPackage उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API की ऑथराइज़ेशन कॉन्फ़िगर करें: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// यदि आवश्यक हो तो API की के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे की लाइन को अनकमेंट करें
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final replaceTenantPackageBody = ReplaceTenantPackageBody(); // ReplaceTenantPackageBody | 

try {
    final result = api_instance.replaceTenantPackage(tenantId, id, replaceTenantPackageBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->replaceTenantPackage: $e\n');
}
[inline-code-end]