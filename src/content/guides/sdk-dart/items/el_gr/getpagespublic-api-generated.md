List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

Λίστα σελίδων για έναν ενοικιαστή. Χρησιμοποιείται από τον πελάτη επιφάνειας εργασίας FChat για την πληρότητα της λίστας δωματίων του.  
Απαιτείται η ρύθμιση `enableFChat` να είναι true στην επιλυμένη προσαρμοσμένη διαμόρφωση για κάθε σελίδα.  
Οι σελίδες που απαιτούν SSO φιλτράρονται βάσει της πρόσβασης ομάδας του χρήστη που κάνει το αίτημα.

## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Αδιαφανής δρομέας σελίδων που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων. |
| sortBy | string | query | No | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτα τα περισσότερα σχόλια), ή `title` (αλφαβητικά). |
| hasComments | boolean | query | No | Εάν είναι true, επιστρέφει μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

## Response

Επιστρέφει: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String |
final cursor = cursor_example; // String | Αδιαφανής δρομέας σελίδων που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`.
final limit = 56; // int | 1..200, default 50
final q = q_example; // String | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων.
final sortBy = ; // PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτα τα περισσότερα σχόλια), ή `title` (αλφαβητικά).
final hasComments = true; // bool | Εάν είναι true, επιστρέφει μόνο σελίδες με τουλάχιστον ένα σχόλιο.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]