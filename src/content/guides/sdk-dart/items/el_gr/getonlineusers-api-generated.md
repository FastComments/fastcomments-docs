Τρέχοντες online θεατές μιας σελίδας: άτομα των οποίων η συνεδρία websocket είναι εγγεγραμμένη στην σελίδα αυτή τη στιγμή.  
Επιστρέφει anonCount + totalCount (συνδρομητές σε όλο το δωμάτιο, συμπεριλαμβανομένων των ανώνυμων προβολών που δεν απαριθμούμε).

## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Αναγνωριστικό URL σελίδας (καθαρίζεται από την πλευρά του διακομιστή). |
| afterName | string | query | No | Δείκτης: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | No | Δείκτης διαλύτης ισοψηφίας: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν το afterName έχει οριστεί ώστε οι ισοψηφίες ονομάτων να μην αφαιρούν εγγραφές. |

## Response

Επιστρέφει: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'Παράδειγμα getOnlineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Αναγνωριστικό URL σελίδας (καθαρίζεται από την πλευρά του διακομιστή).
final afterName = afterName_example; // String | Δείκτης: περάστε το nextAfterName από την προηγούμενη απάντηση.
final afterUserId = afterUserId_example; // String | Δείκτης διαλύτης ισοψηφίας: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν το afterName έχει οριστεί ώστε οι ισοψηφίες ονομάτων να μην αφαιρούν εγγραφές.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]