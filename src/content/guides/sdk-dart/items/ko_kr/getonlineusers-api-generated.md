현재 페이지에 온라인인 뷰어: 현재 웹소켓 세션이 페이지에 구독된 사람들입니다.  
anonCount + totalCount(방 전체 구독자, 열거하지 않는 익명 뷰어 포함)를 반환합니다.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 페이지 URL 식별자 (서버 측에서 정리됨). |
| afterName | string | query | No | 커서: 이전 응답에서 nextAfterName을 전달합니다. |
| afterUserId | string | query | No | 커서 타이브레이커: 이전 응답에서 nextAfterUserId을 전달합니다. afterName이 설정된 경우 이름이 같은 경우 항목이 누락되지 않도록 필요합니다. |

## Response

Returns: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'getOnlineUsers 예시'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 페이지 URL 식별자 (서버 측에서 정리됨).
final afterName = afterName_example; // String | 커서: 이전 응답에서 nextAfterName을 전달합니다.
final afterUserId = afterUserId_example; // String | 커서 타이브레이커: 이전 응답에서 nextAfterUserId을 전달합니다. afterName이 설정된 경우 이름이 같은 경우 항목이 누락되지 않도록 필요합니다.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]