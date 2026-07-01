## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |

## レスポンス

返却: `APICreateUserBadgeResponse`

## 例

[inline-code-attrs-start title = 'createUserBadge の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定してください: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定するには、以下のコメントを解除してください
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createUserBadgeParams = CreateUserBadgeParams(); // CreateUserBadgeParams | 

try {
    final result = api_instance.createUserBadge(tenantId, createUserBadgeParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createUserBadge: $e\n');
}
[inline-code-end]

---