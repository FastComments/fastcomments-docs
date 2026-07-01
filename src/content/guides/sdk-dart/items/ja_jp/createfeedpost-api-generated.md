## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| broadcastId | string | query | No |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| skipDupCheck | boolean | query | No |  |

## 応答

戻り値: `CreateFeedPostsResponse`

## 例

[inline-code-attrs-start title = 'createFeedPost の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定してください: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定するには、以下のコメントを外してください
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createFeedPostParams = CreateFeedPostParams(); // CreateFeedPostParams | 
final broadcastId = broadcastId_example; // String | 
final isLive = true; // bool | 
final doSpamCheck = true; // bool | 
final skipDupCheck = true; // bool | 

try {
    final result = api_instance.createFeedPost(tenantId, createFeedPostParams, CreateFeedPostOptions(broadcastId: broadcastId, isLive: isLive, doSpamCheck: doSpamCheck, skipDupCheck: skipDupCheck));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createFeedPost: $e\n');
}
[inline-code-end]