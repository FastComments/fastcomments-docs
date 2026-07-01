## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |
| userId | string | query | いいえ |  |

## 応答

戻り値: `GetTicketResponse`

## 例

[inline-code-attrs-start title = 'getTicket の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 以下のコメントアウトを解除して、APIキーのプレフィックス（例: Bearer）を設定します（必要な場合）
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final userId = userId_example; // String | 

try {
    final result = api_instance.getTicket(tenantId, id, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTicket: $e\n');
}
[inline-code-end]