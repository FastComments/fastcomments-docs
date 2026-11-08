## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |
| contextUserId | string | query | नहीं |  |
| doSpamCheck | boolean | query | नहीं |  |
| isLive | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `APIEmptyResponse`

## उदाहरण

[inline-code-attrs-start title = 'updateComment उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configure API key authorization: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// आवश्यकता होने पर API कुंजी के लिए प्रीफिक्स (जैसे Bearer) सेटअप करने के लिए नीचे की लाइन को अनकमेंट करें
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updatableCommentParams = UpdatableCommentParams(); // UpdatableCommentParams | 
final contextUserId = contextUserId_example; // String | 
final doSpamCheck = true; // bool | 
final isLive = true; // bool | 

try {
    final result = api_instance.updateComment(tenantId, id, updatableCommentParams, UpdateCommentOptions(contextUserId: contextUserId, doSpamCheck: doSpamCheck, isLive: isLive));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateComment: $e\n');
}
[inline-code-end]