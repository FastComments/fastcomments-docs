## 參數

| 名稱 | 類型 | 位置 | 必要 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | 字串 | 查詢 | 是 |  |
| commentId | 字串 | 路徑 | 是 |  |
| voteId | 字串 | 路徑 | 是 |  |
| broadcastId | 字串 | 查詢 | 否 |  |
| sso | 字串 | 查詢 | 否 |  |

## 回應

返回： `VoteDeleteResponse`

## 範例

[inline-code-attrs-start title = 'deleteModerationVote 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final voteId = voteId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.deleteModerationVote(tenantId, commentId, voteId, DeleteModerationVoteOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->deleteModerationVote: $e\n');
}
[inline-code-end]