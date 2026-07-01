## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Returns: `APIEmptyResponse`

## Example

[inline-code-attrs-start title = 'updateTenantPackage の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 必要に応じて API キーのプレフィックス (例: Bearer) を設定するには、以下のコメントを解除してください
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateTenantPackageBody = UpdateTenantPackageBody(); // UpdateTenantPackageBody | 

try {
    final result = api_instance.updateTenantPackage(tenantId, id, updateTenantPackageBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateTenantPackage: $e\n');
}
[inline-code-end]

---