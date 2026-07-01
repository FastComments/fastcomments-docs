Τρέχοντες online θεατές μιας σελίδας: άτομα των οποίων η συνεδρία websocket είναι εγγεγραμμένη στη σελίδα αυτή τη στιγμή.  
Επιστρέφει anonCount + totalCount (συνδρομητές σε όλο το δωμάτιο, συμπεριλαμβανομένων ανώνυμων θεατών που δεν καταχωρούμε).

## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Αναγνωριστικό URL της σελίδας (καθαρισμένο στο διακομιστή). |
| afterName | string | query | No | Κέρσορας: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | No | Αντι-ζυγίαση κέρσορα: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν το afterName είναι ορισμένο ώστε οι ισοτιμίες ονομάτων να μην παραλείπουν εγγραφές. |

## Response

Επιστρέφει: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'getOnlineUsers Παράδειγμα'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Αναγνωριστικό URL της σελίδας (καθαρισμένο στο διακομιστή).
final afterName = afterName_example; // String | Κέρσορας: περάστε το nextAfterName από την προηγούμενη απάντηση.
final afterUserId = afterUserId_example; // String | Αντι-ζυγίαση κέρσορα: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν το afterName είναι ορισμένο ώστε οι ισοτιμίες ονομάτων να μην παραλείπουν εγγραφές.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]