List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Αδιαφανής κέρσορας σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενη αίτηση. Συνδεδεμένο με το ίδιο `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων. |
| sortBy | string | query | No | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, τα πιο νέα πρώτα), `commentCount` (πρώτα οι περισσότεροι σχολιασμοί), ή `title` (αλφαβητικά). |
| hasComments | boolean | query | No | Αν true, επιστρέφει μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

## Response

Επιστρέφει: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String |
final cursor = cursor_example; // String | Αδιαφανής κέρσορας σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενη αίτηση. Συνδεδεμένο με το ίδιο `sortBy`.
final limit = 56; // int | 1..200, default 50
final q = q_example; // String | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων.
final sortBy = ; // PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, τα πιο νέα πρώτα), `commentCount` (πρώτα οι περισσότεροι σχολιασμοί), ή `title` (αλφαβητικά).
final hasComments = true; // bool | Αν true, επιστρέφει μόνο σελίδες με τουλάχιστον ένα σχόλιο.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]