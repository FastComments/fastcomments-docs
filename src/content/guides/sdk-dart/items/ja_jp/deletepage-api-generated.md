## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |

## Response

Returns: `DeletePageAPIResponse`

## Example

[inline-code-attrs-start title = 'deletePage の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定してください: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定するには以下のコメントを解除してください
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.deletePage(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deletePage: $e\n');
}
[inline-code-end]