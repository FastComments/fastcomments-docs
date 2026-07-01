## Parameters

| 名前 | タイプ | ロケーション | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| timeBucket | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |

## Response

返却: `AggregateQuestionResultsResponse`

## Example

[inline-code-attrs-start title = 'aggregateQuestionResults の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final questionId = questionId_example; // String | 
final questionIds = []; // List<String> | 
final urlId = urlId_example; // String | 
final timeBucket = ; // AggregateTimeBucket | 
final startDate = 2013-10-20T19:20:30+01:00; // DateTime | 
final forceRecalculate = true; // bool | 

try {
    final result = api_instance.aggregateQuestionResults(tenantId, AggregateQuestionResultsOptions(questionId: questionId, questionIds: questionIds, urlId: urlId, timeBucket: timeBucket, startDate: startDate, forceRecalculate: forceRecalculate));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->aggregateQuestionResults: $e\n');
}
[inline-code-end]

---