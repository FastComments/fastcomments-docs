## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |

## 応答

戻り値: `CreateTenantPackageResponse`

## 例

[inline-code-attrs-start title = 'createTenantPackage 例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキーの認証を設定: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 以下のコメントを外して、APIキーのプレフィックス（例: Bearer）を設定（必要に応じて）
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createTenantPackageBody = CreateTenantPackageBody(); // CreateTenantPackageBody | 

try {
    final result = api_instance.createTenantPackage(tenantId, createTenantPackageBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTenantPackage: $e\n');
}
[inline-code-end]