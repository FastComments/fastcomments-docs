Λίστα σελίδων για έναν ενοικιαστή. Χρησιμοποιείται από τον πελάτη FChat για επιτραπέζιους υπολογιστές ώστε να γεμίσει τη λίστα δωματίων του. Απαιτεί `enableFChat` να είναι αληθές στην επιλυμένη προσαρμοσμένη ρύθμιση για κάθε σελίδα. Οι σελίδες που απαιτούν SSO φιλτράρονται βάσει της πρόσβασης ομάδας του χρήστη που κάνει το αίτημα.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Αδιαφανής δείκτης σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`. |
| limit | integer | query | No | 1..200, προεπιλογή 50 |
| q | string | query | No | Προαιρετικό φιλτράρισμα προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων. |
| sortBy | string | query | No | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, τα πιο πρόσφατα πρώτα), `commentCount` (τα περισσότερα σχόλια πρώτα), ή `title` (αλφαβητική). |
| hasComments | boolean | query | No | Εάν είναι αληθές, επιστρέφει μόνο σελίδες που έχουν τουλάχιστον ένα σχόλιο. |

## Απόκριση

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'getPagesPublic Παράδειγμα'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// $apiInstance = new FastComments\Client\Api\PublicApi(
$apiInstance = new FastComments\Client\Api\PublicApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον πελάτη σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Αδιαφανής δείκτης σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένο με το ίδιο `sortBy`.
    'limit' => 56, // int | 1..200, προεπιλογή 50
    'q' => 'q_example', // string | Προαιρετικό φιλτράρισμα προθέματος τίτλου χωρίς διάκριση πεζών-κεφαλαίων.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Σειρά ταξινόμησης. `updatedAt` (προεπιλογή, τα πιο πρόσφατα πρώτα), `commentCount` (τα περισσότερα σχόλια πρώτα), ή `title` (αλφαβητική).
    'has_comments' => True, // bool | Εάν είναι αληθές, επιστρέφει μόνο σελίδες που έχουν τουλάχιστον ένα σχόλιο.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]