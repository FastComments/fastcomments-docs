## パラメータ

| 名前 | 種類 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| forceRecalculate | boolean | query | いいえ |  |

## 応答

戻り値: `BulkAggregateQuestionResultsResponse`

## 例

[inline-code-attrs-start title = 'bulkAggregateQuestionResults の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API キー認証を設定: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 以下のコメントを外して、API キーのプレフィックス（例: Bearer）を設定してください（必要な場合）
 //defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final bulkAggregateQuestionResultsRequest = BulkAggregateQuestionResultsRequest(); // BulkAggregateQuestionResultsRequest | 
final forceRecalculate = true; // bool | 

try {
    final result = api_instance.bulkAggregateQuestionResults(tenantId, bulkAggregateQuestionResultsRequest, forceRecalculate);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->bulkAggregateQuestionResults: $e\n');
}
[inline-code-end]