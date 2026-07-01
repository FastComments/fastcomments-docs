Συγκεντρωτικές πληροφορίες χρήστη για έναν ενοικιαστή. Δοθέντων userIds, επιστρέφει πληροφορίες εμφάνισης από User / SSOUser.  
Χρησιμοποιείται από το widget σχολίων για να εμπλουτίσει τους χρήστες που εμφανίστηκαν μόλις μέσω ενός γεγονότος παρουσίας.  
Χωρίς πλαίσιο σελίδας: η ιδιωτικότητα επιβάλλεται ομοιόμορφα (τα ιδιωτικά προφίλ καλύπτονται).

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | path | Ναι |  |
| ids | string | query | Ναι | UserIds διαχωρισμένα με κόμμα. |

## Απόκριση

Επιστρέφει: `PageUsersInfoResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUsersInfo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | UserIds διαχωρισμένα με κόμμα.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]