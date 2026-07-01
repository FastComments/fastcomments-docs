Το SDK εκθέτει τρεις κλάσεις πελάτη API:

- **`DefaultApi`** - Μέθοδοι με αυθεντικοποίηση μέσω κλειδιού API για χρήση στην πλευρά του διακομιστή. Διαμορφώστε ένα κλειδί API όπως φαίνεται στην ενότητα [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** - Δημόσιες μέθοδοι που δεν απαιτούν κλειδί API, ασφαλείς για κλήση από προγράμματα περιήγησης και κινητές εφαρμογές.
- **`ModerationApi`** - Ένα εκτενές σύνολο ζωντανών και γρήγορων API μετριασμού. Κάθε μέθοδος `ModerationApi` δέχεται μια παράμετρο `$sso` και μπορεί να αυθεντοποιηθεί μέσω SSO ή ενός cookie συνεδρίας FastComments.com.

### Χρήση του PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Οι δημόσιες μέθοδοι δεν απαιτούν κλειδί API.
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

### Χρήση του ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - Φορτίο SSO που αυθεντικοποιεί τον συντονιστή

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```