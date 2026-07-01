Past commenters on the page who are NOT currently online. Sorted by displayName.  
현재 온라인 상태가 아닌 페이지의 이전 댓글 작성자들. displayName 기준으로 정렬됩니다.  

Use this after exhausting /users/online to render a "Members" section.  
/users/online을 모두 사용한 후, "Members" 섹션을 렌더링할 때 사용합니다.  

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
commenterName에 대한 커서 페이지네이션: 서버는 {tenantId, urlId, commenterName} 부분 인덱스를 afterName 이후부터 $gt를 통해 이동하며, $skip 비용이 없습니다.  

## Parameters

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 페이지 URL 식별자 (서버 측에서 정리됨). |
| afterName | string | query | No | 커서: 이전 응답의 nextAfterName을 전달합니다. |
| afterUserId | string | query | No | 커서 동점 해결: 이전 응답의 nextAfterUserId를 전달합니다. afterName이 설정된 경우 이름이 같은 경우 항목이 누락되지 않도록 필수입니다. |

## Response

Returns: `PageUsersOfflineResponse`  
반환: `PageUsersOfflineResponse`  

## Example

[inline-code-attrs-start title = 'getOfflineUsers 예시'; type = ''; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String |  
final urlId = urlId_example; // String | 페이지 URL 식별자 (서버 측에서 정리됨).
final afterName = afterName_example; // String | 커서: 이전 응답의 nextAfterName을 전달합니다.
final afterUserId = afterUserId_example; // String | 커서 동점 해결: 이전 응답의 nextAfterUserId를 전달합니다. afterName이 설정된 경우 이름이 같은 경우 항목이 누락되지 않도록 필수입니다.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]