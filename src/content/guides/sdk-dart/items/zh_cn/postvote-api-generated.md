## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | path | 是 |  |
| direction | string | query | 否 |  |
| broadcastId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: `VoteResponse`

## 示例

[inline-code-attrs-start title = 'postVote 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final direction = direction_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postVote(tenantId, commentId, PostVoteOptions(direction: direction, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postVote: $e\n');
}
[inline-code-end]