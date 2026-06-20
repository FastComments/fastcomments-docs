Λίστα σελίδων για έναν tenant. Χρησιμοποιείται από τον επιτραπέζιο πελάτη FChat για να γεμίσει τη λίστα δωματίων του.
Απαιτεί `enableFChat` να είναι true στην επιλυμένη προσαρμοσμένη ρύθμιση για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδας του χρήστη που κάνει το αίτημα.

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Αδιαφανές cursor σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Προαιρετικό φίλτρο προθέματος τίτλου ανεξάρτητο από πεζά/κεφαλαία. |
| sortBy | string | query | No | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτα οι σελίδες με τα περισσότερα σχόλια), ή `title` (αλφαβητικά). |
| hasComments | boolean | query | No | Αν είναι true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

## Απόκριση

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Αδιαφανές cursor σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`.
$limit = 56; // int | 1..200, default 50
$q = 'q_example'; // string | Προαιρετικό φίλτρο προθέματος τίτλου ανεξάρτητο από πεζά/κεφαλαία.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτα οι σελίδες με τα περισσότερα σχόλια), ή `title` (αλφαβητικά).
$has_comments = True; // bool | Αν είναι true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]