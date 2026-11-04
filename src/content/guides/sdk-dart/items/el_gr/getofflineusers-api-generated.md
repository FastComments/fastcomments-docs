Προηγούμενοι σχολιαστές της σελίδας που ΔΕΝ είναι επί του παρόντος συνδεδεμένοι. Ταξινομημένοι κατά displayName.
Χρησιμοποιήστε το αυτό αφού εξαντλήσετε το /users/online για να αποδώσετε μια ενότητα "Members".
Σελιδοποίηση με δείκτη (cursor) στο commenterName: ο διακομιστής διασχίζει το μερικό ευρετήριο {tenantId, urlId, commenterName}
ξεκινώντας από το afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαραίτητο | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή). |
| afterName | string | query | Όχι | Δείκτης (cursor): περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | Όχι | Συμπληρωματικός δείκτης (tiebreaker): περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαραίτητο όταν ορίζεται το afterName, ώστε οι ισοβαθμίες ονομάτων να μην παραλείψουν εγγραφές. |

## Απάντηση

Επιστρέφει: `PageUsersOfflineResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή).
final afterName = afterName_example; // String | Δείκτης (cursor): περάστε το nextAfterName από την προηγούμενη απάντηση.
final afterUserId = afterUserId_example; // String | Συμπληρωματικός δείκτης (tiebreaker): περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαραίτητο όταν ορίζεται το afterName, ώστε οι ισοβαθμίες ονομάτων να μην παραλείψουν εγγραφές.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]