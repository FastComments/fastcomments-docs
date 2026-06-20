Καταλόγος σελίδων για έναν tenant. Χρησιμοποιείται από τον desktop πελάτη FChat για να συμπληρώσει τη λίστα δωματίων του.
Απαιτείται το `enableFChat` να έχει τιμή true στην επιλυμένη προσαρμοσμένη διαμόρφωση για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται βάσει της πρόσβασης ομάδων του χρήστη που κάνει το αίτημα.

## Παράμετροι

| Όνομα | Τύπος | Location | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| cursor | string | query | Όχι | Αδιαφανές cursor σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`. |
| limit | integer | query | Όχι | 1..200, προεπιλογή 50 |
| q | string | query | Όχι | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών/κεφαλαίων. |
| sortBy | string | query | Όχι | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτα οι σελίδες με τα περισσότερα σχόλια), ή `title` (αλφαβητικά). |
| hasComments | boolean | query | Όχι | Αν είναι true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

## Απόκριση

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον client σας που υλοποιεί την `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, θα χρησιμοποιηθεί η `GuzzleHttp\Client` ως προεπιλογή.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Αδιαφανές cursor σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`.
$limit = 56; // int | 1..200, προεπιλογή 50
$q = 'q_example'; // string | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών/κεφαλαίων.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτα οι σελίδες με τα περισσότερα σχόλια), ή `title` (αλφαβητικά).
$has_comments = True; // bool | Εάν είναι true, να επιστραφούν μόνο σελίδες με τουλάχιστον ένα σχόλιο.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]