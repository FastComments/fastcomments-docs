## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| questionId | string | query | नहीं |  |
| questionIds | array | query | नहीं |  |
| urlId | string | query | नहीं |  |
| startDate | string | query | नहीं |  |
| forceRecalculate | boolean | query | नहीं |  |
| minValue | number | query | नहीं |  |
| maxValue | number | query | नहीं |  |
| limit | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `CombineQuestionResultsWithCommentsResponse`

## उदाहरण

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configure API key authorization: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// आवश्यक होने पर API कुंजी के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे की लाइन को अनकमेंट करें
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final questionId = questionId_example; // String | 
final questionIds = []; // List<String> | 
final urlId = urlId_example; // String | 
final startDate = 2013-10-20T19:20:30+01:00; // DateTime | 
final forceRecalculate = true; // bool | 
final minValue = 1.2; // double | 
final maxValue = 1.2; // double | 
final limit = 1.2; // double | 

try {
    final result = api_instance.combineCommentsWithQuestionResults(tenantId, CombineCommentsWithQuestionResultsOptions(questionId: questionId, questionIds: questionIds, urlId: urlId, startDate: startDate, forceRecalculate: forceRecalculate, minValue: minValue, maxValue: maxValue, limit: limit));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->combineCommentsWithQuestionResults: $e\n');
}
[inline-code-end]