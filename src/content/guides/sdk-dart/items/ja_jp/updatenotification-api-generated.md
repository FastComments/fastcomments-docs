## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |

## レスポンス

戻り値: `APIEmptyResponse`

## 例

[inline-code-attrs-start title = 'updateNotification の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO: APIキー認証を設定してください: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 下のコメントアウトを解除して、必要に応じてAPIキーのプレフィックス（例: Bearer）を設定してください
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateNotificationBody = UpdateNotificationBody(); // UpdateNotificationBody | 
final userId = userId_example; // String | 

try {
    final result = api_instance.updateNotification(tenantId, id, updateNotificationBody, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateNotification: $e\n');
}
[inline-code-end]