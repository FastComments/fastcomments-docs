## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| userId | string | query | Όχι |  |
| trustFactor | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SetUserTrustFactorResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα setTrustFactor'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, δώστε τον client σας που υλοποιεί το `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);
$user_id = 'user_id_example'; // string
$trust_factor = 'trust_factor_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->setTrustFactor($user_id, $trust_factor, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->setTrustFactor: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---