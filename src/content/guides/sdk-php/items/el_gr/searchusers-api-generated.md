## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| usernameStartsWith | string | query | No |  |
| mentionGroupIds | array | query | No |  |
| sso | string | query | No |  |
| searchSection | string | query | No |  |

## Απόκριση

Επιστρέφει: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SearchUsers200Response.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα searchUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον client σας που υλοποιεί το `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, θα χρησιμοποιηθεί το `GuzzleHttp\Client` ως προεπιλογή.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // συμβολοσειρά
$url_id = 'url_id_example'; // συμβολοσειρά
$username_starts_with = 'username_starts_with_example'; // συμβολοσειρά
$mention_group_ids = array('mention_group_ids_example'); // πίνακας συμβολοσειρών
$sso = 'sso_example'; // συμβολοσειρά
$search_section = 'search_section_example'; // συμβολοσειρά

try {
    $result = $apiInstance->searchUsers($tenant_id, $url_id, $username_starts_with, $mention_group_ids, $sso, $search_section);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->searchUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---