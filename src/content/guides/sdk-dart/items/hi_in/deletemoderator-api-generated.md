## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |
| sendEmail | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `APIEmptyResponse`

## उदाहरण

[inline-code-attrs-start title = 'deleteModerator उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configure API key authorization: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// आवश्यकता होने पर API key के लिए प्रीफिक्स (जैसे Bearer) सेटअप करने के लिए नीचे की लाइन अनकमेंट करें
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final sendEmail = sendEmail_example; // String | 

try {
    final result = api_instance.deleteModerator(tenantId, id, sendEmail);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteModerator: $e\n');
}
[inline-code-end]