Λίστα σελίδων για έναν tenant. Χρησιμοποιείται από το FChat desktop client για τη συμπλήρωση της λίστας δωματίων του.
Απαιτεί το `enableFChat` να είναι true στο resolved custom config για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδας του χρήστη που κάνει το αίτημα.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαραίτητο | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| cursor | string | query | Όχι | Αδιαφανές pagination cursor που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδέεται με το ίδιο `sortBy`. |
| limit | integer | query | Όχι | 1..200, προεπιλογή 50 |
| q | string | query | Όχι | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων. |
| sortBy | string | query | Όχι | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (περισσότερα σχόλια πρώτα) ή `title` (αλφαβητικά). |
| hasComments | boolean | query | Όχι | Αν true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

## Απάντηση

Επιστρέφει: `GetPublicPagesResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Αδιαφανές pagination cursor που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδέεται με το ίδιο `sortBy`.
final limit = 56; // int | 1..200, προεπιλογή 50
final q = q_example; // String | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων.
final sortBy = ; // PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (περισσότερα σχόλια πρώτα) ή `title` (αλφαβητικά).
final hasComments = true; // bool | Αν true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]