## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## レスポンス

戻り値: `GetDomainConfigsResponse`

## 例

[inline-code-attrs-start title = 'getDomainConfigs の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定する: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// コメント解除して以下を設定し、APIキーのプレフィックス（例: Bearer）を設定します（必要な場合）
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 

try {
    final result = api_instance.getDomainConfigs(tenantId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getDomainConfigs: $e\n');
}
[inline-code-end]