## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## 応答

戻り値: `CreateSubscriptionAPIResponse`

## 例

[inline-code-attrs-start title = 'createSubscription 例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 下記のコメントアウトを外して、APIキーのプレフィックス（例: Bearer）を設定します（必要な場合）
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createAPIUserSubscriptionData = CreateAPIUserSubscriptionData(); // CreateAPIUserSubscriptionData | 

try {
    final result = api_instance.createSubscription(tenantId, createAPIUserSubscriptionData);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createSubscription: $e\n');
}
[inline-code-end]