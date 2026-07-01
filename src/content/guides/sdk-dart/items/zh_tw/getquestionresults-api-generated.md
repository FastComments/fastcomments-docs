## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 否 |  |
| userId | string | query | 否 |  |
| startDate | string | query | 否 |  |
| questionId | string | query | 否 |  |
| questionIds | string | query | 否 |  |
| skip | number | query | 否 |  |

## 回應

返回： `GetQuestionResultsResponse`

## 範例

[inline-code-attrs-start title = 'getQuestionResults 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 設定 API 金鑰授權: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消註解以下以設定前綴（例如 Bearer）給 API 金鑰，如有需要
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final userId = userId_example; // String | 
final startDate = startDate_example; // String | 
final questionId = questionId_example; // String | 
final questionIds = questionIds_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getQuestionResults(tenantId, GetQuestionResultsOptions(urlId: urlId, userId: userId, startDate: startDate, questionId: questionId, questionIds: questionIds, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getQuestionResults: $e\n');
}
[inline-code-end]