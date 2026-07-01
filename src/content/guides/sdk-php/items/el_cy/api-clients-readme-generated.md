---
Το SDK εκθέτει τρεις κλάσεις πελάτη API:

- **`DefaultApi`** - Μέθοδοι που δεν απαιτούν κλειδί API για χρήση στον διακομιστή. Ρυθμίστε ένα κλειδί API όπως φαίνεται στο [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** - Δημόσιες μέθοδοι που δεν απαιτούν κλειδί API, ασφαλείς για κλήση από προγράμματα περιήγησης και κινητές εφαρμογές.
- **`ModerationApi`** - Μία εκτεταμένη σειρά ζωντανών και γρήγορων API διαχείρισης. Κάθε μέθοδος `ModerationApi` δέχεται μια παράμετρο `$sso` και μπορεί να εξουσιοδοτηθεί μέσω SSO ή μέσω cookie συνεδρίας FastComments.com.

### Χρήση PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Public methods do not require an API key.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
```

### Χρήση ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO payload authenticating the moderator

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```
---