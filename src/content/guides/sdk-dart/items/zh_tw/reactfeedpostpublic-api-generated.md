## 參數

| 名稱 | 類型 | 位置 | 必須 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| postId | string | path | Yes |  |
| isUndo | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## 回應

返回: `ReactFeedPostResponse`

## 範例

[inline-code-attrs-start title = 'reactFeedPostPublic 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final postId = postId_example; // String | 
final reactBodyParams = ReactBodyParams(); // ReactBodyParams | 
final isUndo = true; // bool | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.reactFeedPostPublic(tenantId, postId, reactBodyParams, ReactFeedPostPublicOptions(isUndo: isUndo, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->reactFeedPostPublic: $e\n');
}
[inline-code-end]