---
## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## レスポンス

返却: `APIEmptyResponse`

## 例

[inline-code-attrs-start title = 'updateTenant の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定するには、以下のコメントアウトを解除してください
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateTenantBody = UpdateTenantBody(); // UpdateTenantBody | 

try {
    final result = api_instance.updateTenant(tenantId, id, updateTenantBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateTenant: $e\n');
}
[inline-code-end]

---