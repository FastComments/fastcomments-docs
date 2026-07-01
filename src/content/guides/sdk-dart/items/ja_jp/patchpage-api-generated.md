## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## レスポンス

返却: `PatchPageAPIResponse`

## 例

[inline-code-attrs-start title = 'patchPage の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定してください: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 以下のコメントを解除して、APIキーのプレフィックス（例: Bearer）を設定してください（必要な場合）
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateAPIPageData = UpdateAPIPageData(); // UpdateAPIPageData | 

try {
    final result = api_instance.patchPage(tenantId, id, updateAPIPageData);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchPage: $e\n');
}
[inline-code-end]