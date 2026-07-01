Past commenters on the page who are NOT currently online. Sorted by displayName.  
Χρησιμοποιήστε το αυτό μετά την εξάντληση του /users/online για να εμφανίσετε μια ενότητα "Members".  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
Σελιδοποίηση με κέρσορα στο commenterName: ο διακομιστής διασχίζει το μερικό {tenantId, urlId, commenterName} δείκτη από το afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|-----------|-----------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Αναγνωριστικό URL της σελίδας (καθαρισμένο από τον διακομιστή). |
| afterName | string | query | No | Κέρσορας: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | No | Διαχωριστής κέρσορα: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν έχει οριστεί afterName ώστε τα ομώνυμα ονόματα να μην αφαιρεθούν. |

## Απόκριση

Returns: `PageUsersOfflineResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Αναγνωριστικό URL της σελίδας (καθαρισμένο από τον διακομιστή).
final afterName = afterName_example; // String | Κέρσορας: περάστε το nextAfterName από την προηγούμενη απάντηση.
final afterUserId = afterUserId_example; // String | Διαχωριστής κέρσορα: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν έχει οριστεί afterName ώστε τα ομώνυμα ονόματα να μην αφαιρεθούν.

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]

---