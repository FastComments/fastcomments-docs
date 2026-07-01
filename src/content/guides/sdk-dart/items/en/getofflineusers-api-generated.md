Past commenters on the page who are NOT currently online. Sorted by displayName.
Use this after exhausting /users/online to render a "Members" section.
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}
index from afterName forward via $gt, no $skip cost.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Page URL identifier (cleaned server-side). |
| afterName | string | query | No | Cursor: pass nextAfterName from the previous response. |
| afterUserId | string | query | No | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |

## Response

Returns: `PageUsersOfflineResponse`

## Example

[inline-code-attrs-start title = 'getOfflineUsers Example'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Page URL identifier (cleaned server-side).
final afterName = afterName_example; // String | Cursor: pass nextAfterName from the previous response.
final afterUserId = afterUserId_example; // String | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]
