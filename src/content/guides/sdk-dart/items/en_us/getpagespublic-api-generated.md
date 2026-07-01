List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Optional case-insensitive title prefix filter. |
| sortBy | string | query | No | Sort order. `updatedAt` (default, newest first), `commentCount` (most comments first), or `title` (alphabetical). |
| hasComments | boolean | query | No | If true, only return pages with at least one comment. |

## Response

Returns: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'getPagesPublic Example'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`.
final limit = 56; // int | 1..200, default 50
final q = q_example; // String | Optional case-insensitive title prefix filter.
final sortBy = ; // PagesSortBy | Sort order. `updatedAt` (default, newest first), `commentCount` (most comments first), or `title` (alphabetical).
final hasComments = true; // bool | If true, only return pages with at least one comment.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]
