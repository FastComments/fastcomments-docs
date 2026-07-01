Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial `{tenantId, urlId, commenterName}` index from afterName forward via `$gt`, no `$skip` cost.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Αναγνωριστικό URL σελίδας (καθαρισμένο από τον διακομιστή). |
| afterName | string | query | No | Cursor: περάστε το nextAfterName από την προηγούμενη ανταπόκριση. |
| afterUserId | string | query | No | Cursor tiebreaker: περάστε το nextAfterUserId από την προηγούμενη ανταπόκριση. Απαιτείται όταν ο afterName έχει οριστεί ώστε οι ισότητες ονομάτων να μην αφαιρούν εγγραφές. |

## Response

Returns: `PageUsersOfflineResponse`

## Example

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Αναγνωριστικό URL σελίδας (καθαρισμένο από τον διακομιστή).
final afterName = afterName_example; // String | Cursor: περάστε το nextAfterName από την προηγούμενη ανταπόκριση.
final afterUserId = afterUserId_example; // String | Cursor tiebreaker: περάστε το nextAfterUserId από την προηγούμενη ανταπόκριση. Απαιτείται όταν ο afterName έχει οριστεί ώστε οι ισότητες ονομάτων να μην αφαιρούν εγγραφές.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]