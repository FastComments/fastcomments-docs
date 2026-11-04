테넌트의 페이지 목록을 조회합니다. FChat 데스크톱 클라이언트가 방 목록을 채우는 데 사용됩니다.
각 페이지의 확인된 커스텀 설정에서 `enableFChat`이 true여야 합니다.
SSO가 필요한 페이지는 요청 사용자의 그룹 접근 권한을 기준으로 필터링됩니다.

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| cursor | string | query | 아니요 | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 연결됩니다. |
| limit | integer | query | 아니요 | 1..200, 기본값 50 |
| q | string | query | 아니요 | 선택 사항인 대소문자를 구분하지 않는 제목 접두사 필터입니다. |
| sortBy | string | query | 아니요 | 정렬 순서. `updatedAt` (기본값, 최신순), `commentCount` (댓글 많은 순), 또는 `title` (알파벳순). |
| hasComments | boolean | query | 아니요 | true인 경우, 댓글이 하나 이상 있는 페이지만 반환합니다. |

## 응답

반환값: `GetPublicPagesResponse`

## 예제

[inline-code-attrs-start title = 'getPagesPublic 예제'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 연결됩니다.
final limit = 56; // int | 1..200, 기본값 50
final q = q_example; // String | 선택 사항인 대소문자를 구분하지 않는 제목 접두사 필터입니다.
final sortBy = ; // PagesSortBy | 정렬 순서. `updatedAt` (기본값, 최신순), `commentCount` (댓글 많은 순), 또는 `title` (알파벳순).
final hasComments = true; // bool | true인 경우, 댓글이 하나 이상 있는 페이지만 반환합니다.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]