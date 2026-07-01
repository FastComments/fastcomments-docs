## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## レスポンス

戻り値: `GetCachedNotificationCountResponse`

## 例

[inline-code-attrs-start title = 'getCachedNotificationCount の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定してください: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 必要に応じて、以下のコメントを外してAPIキーのプレフィックス（例: Bearer）を設定してください
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getCachedNotificationCount(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getCachedNotificationCount: $e\n');
}
[inline-code-end]

---