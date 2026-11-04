Λίστα σελίδων για έναν tenant. Χρησιμοποιείται από το FChat desktop client για να συμπληρώσει τη λίστα δωματίων του.
Απαιτεί το `enableFChat` να είναι true στο resolved custom config για κάθε σελίδα.
Σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδας του χρήστη που κάνει το αίτημα.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| cursor | string | query | Όχι | Αδιαφανές pagination cursor που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδέεται με το ίδιο `sortBy`. |
| limit | integer | query | Όχι | 1..200, προεπιλογή 50 |
| q | string | query | Όχι | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων. |
| sortBy | string | query | Όχι | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (περισσότερα σχόλια πρώτα), ή `title` (αλφαβητικά). |
| hasComments | boolean | query | Όχι | Αν true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

## Απάντηση

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε το δικό σας client που υλοποιεί το `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Αδιαφανές pagination cursor που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδέεται με το ίδιο `sortBy`.
    'limit' => 56, // int | 1..200, προεπιλογή 50
    'q' => 'q_example', // string | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (περισσότερα σχόλια πρώτα), ή `title` (αλφαβητικά).
    'has_comments' => True, // bool | Αν true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]