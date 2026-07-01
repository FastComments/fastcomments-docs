List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 이전 요청에서 `nextCursor` 로 반환된 불투명한 페이지네이션 커서. 동일한 `sortBy` 와 연결됩니다. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | 대소문자를 구분하지 않는 선택적 제목 접두어 필터. |
| sortBy | string | query | No | 정렬 순서. `updatedAt` (기본값, 최신 순), `commentCount` (댓글 수 많은 순), 또는 `title` (알파벳 순). |
| hasComments | boolean | query | No | true인 경우, 최소 하나의 댓글이 있는 페이지만 반환합니다. |

## Response

Returns: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'getPagesPublic 예제'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | 이전 요청에서 `nextCursor` 로 반환된 불투명한 페이지네이션 커서. 동일한 `sortBy` 와 연결됩니다.
final limit = 56; // int | 1..200, default 50
final q = q_example; // String | 대소문자를 구분하지 않는 선택적 제목 접두어 필터.
final sortBy = ; // PagesSortBy | 정렬 순서. `updatedAt` (기본값, 최신 순), `commentCount` (댓글 수 많은 순), 또는 `title` (알파벳 순).
final hasComments = true; // bool | true인 경우, 최소 하나의 댓글이 있는 페이지만 반환합니다.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]