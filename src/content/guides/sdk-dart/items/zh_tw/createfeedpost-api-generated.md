## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| broadcastId | string | query | No |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| skipDupCheck | boolean | query | No |  |

## 回應

返回: `CreateFeedPostsResponse`

## 範例

[inline-code-attrs-start title = 'createFeedPost 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 設定 API 金鑰授權: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消註解以下以設定 API 金鑰的前綴 (例如 Bearer)，如果需要的話
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