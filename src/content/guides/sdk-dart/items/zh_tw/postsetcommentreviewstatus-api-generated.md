---
## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 查詢 | 是 |  |
| commentId | string | 路徑 | 是 |  |
| reviewed | boolean | 查詢 | 否 |  |
| broadcastId | string | 查詢 | 否 |  |
| sso | string | 查詢 | 否 |  |

## 回應

返回： `APIEmptyResponse`

## 範例

[inline-code-attrs-start title = 'postSetCommentReviewStatus 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final reviewed = true; // bool | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postSetCommentReviewStatus(tenantId, commentId, PostSetCommentReviewStatusOptions(reviewed: reviewed, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postSetCommentReviewStatus: $e\n');
}
[inline-code-end]

---