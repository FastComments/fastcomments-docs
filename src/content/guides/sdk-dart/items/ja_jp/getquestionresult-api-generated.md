## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## レスポンス

戻り値: `GetQuestionResultResponse`

## 例

[inline-code-attrs-start title = 'getQuestionResult 例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO: APIキー認証を設定する: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定するには、以下のコメントを外してください
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getQuestionResult(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getQuestionResult: $e\n');
}
[inline-code-end]

---