---
## パラメータ

| 名前 | タイプ | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| domainToUpdate | string | path | はい |  |

## レスポンス

返却: `PatchDomainConfigResponse`

## 例

[inline-code-attrs-start title = 'patchDomainConfig 例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定してください: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 必要に応じて、APIキーのプレフィックス（例：Bearer）を設定するために以下のコメントを解除してください
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final domainToUpdate = domainToUpdate_example; // String | 
final patchDomainConfigParams = PatchDomainConfigParams(); // PatchDomainConfigParams | 

try {
    final result = api_instance.patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchDomainConfig: $e\n');
}
[inline-code-end]

---