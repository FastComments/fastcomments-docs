## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | パス | はい |  |
| postId | string | パス | はい |  |
| broadcastId | string | クエリ | いいえ |  |
| sso | string | クエリ | いいえ |  |

## Response

戻り値: `CreateFeedPostResponse`

## Example

[inline-code-attrs-start title = 'updateFeedPostPublic の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final postId = postId_example; // String | 
final updateFeedPostParams = UpdateFeedPostParams(); // UpdateFeedPostParams | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.updateFeedPostPublic(tenantId, postId, updateFeedPostParams, UpdateFeedPostPublicOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->updateFeedPostPublic: $e\n');
}
[inline-code-end]