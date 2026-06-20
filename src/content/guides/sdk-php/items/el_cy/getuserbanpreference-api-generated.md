## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIModerateGetUserBanPreferencesResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserBanPreference'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον πελάτη που υλοποιεί την `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό — η `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // τύπος: string

try {
    $result = $apiInstance->getUserBanPreference($sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getUserBanPreference: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---