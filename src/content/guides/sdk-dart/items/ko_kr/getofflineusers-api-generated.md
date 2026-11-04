현재 온라인 상태가 아닌, 해당 페이지의 과거 댓글 작성자들입니다. displayName 기준으로 정렬됩니다.
/users/online을 모두 조회한 후 "멤버" 섹션을 렌더링할 때 이 엔드포인트를 사용하세요.
commenterName 기준 커서 페이지네이션: 서버는 부분 {tenantId, urlId, commenterName}
인덱스를 afterName부터 $gt로 순회하며, $skip 비용이 없습니다.

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|------|------|------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 | 페이지 URL 식별자(서버에서 정리됨). |
| afterName | string | query | 아니요 | 커서: 이전 응답의 nextAfterName을 전달하세요. |
| afterUserId | string | query | 아니요 | 커서 타이브레이커: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 필수이며, 이름이 같은 항목이 누락되지 않도록 합니다. |

## 응답

반환값: `PageUsersOfflineResponse`

## 예제

[inline-code-attrs-start title = 'getOfflineUsers 예제'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 페이지 URL 식별자(서버에서 정리됨).
final afterName = afterName_example; // String | 커서: 이전 응답의 nextAfterName을 전달하세요.
final afterUserId = afterUserId_example; // String | 커서 타이브레이커: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 필수이며, 이름이 같은 항목이 누락되지 않도록 합니다.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]