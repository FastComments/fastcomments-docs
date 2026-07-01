---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |

## レスポンス

返却値: `CreateTenantUserResponse`

## 例

[inline-code-attrs-start title = 'createTenantUser の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 下記のコメントを解除して、APIキーのプレフィックス（例: Bearer）を設定します（必要な場合）
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createTenantUserBody = CreateTenantUserBody(); // CreateTenantUserBody | 

try {
    final result = api_instance.createTenantUser(tenantId, createTenantUserBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTenantUser: $e\n');
}
[inline-code-end]

---