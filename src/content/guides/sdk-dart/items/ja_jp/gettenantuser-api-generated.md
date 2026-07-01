## パラメータ

| 名前 | 型 | 場所 | 必要 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## 応答

戻り値: `GetTenantUserResponse`

## 例

[inline-code-attrs-start title = 'getTenantUser 例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 必要に応じてAPIキーのプレフィックス（例: Bearer）を設定するには下のコメントを外してください
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getTenantUser(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenantUser: $e\n');
}
[inline-code-end]