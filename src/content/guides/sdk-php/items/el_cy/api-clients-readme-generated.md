Το SDK παρέχει τρεις κλάσεις πελατών API:

- **`DefaultApi`** — μέθοδοι με αυθεντικοποίηση μέσω API key για χρήση στο server-side. Διαμορφώστε ένα API key όπως φαίνεται στο [Οδηγός εκκίνησης](#getting-started-readme-generated).
- **`PublicApi`** — δημόσιες μέθοδοι που δεν απαιτούν κλειδί API, ασφαλές να καλούνται από browsers και κινητές εφαρμογές.
- **`ModerationApi`** — μέθοδοι για τον πίνακα εργαλείων του moderator: καταχώρηση, μέτρηση, αναζήτηση, καταγραφή και εξαγωγή σχολίων; ενέργειες moderation (αφαίρεση/επανόρθωση, σηματοδότηση, ορισμός κατάστασης επανεξέτασης/spam/έγκρισης, ψήφοι, επανανοίγμα/κλείσιμο νήματος); απαγορεύσεις (απαγόρευση σχολιασμού, αναίρεση, περιλήψεις πριν την απαγόρευση, κατάσταση και προτιμήσεις απαγόρευσης, μετρήσεις απαγορευμένων χρηστών); και badges & trust (απονομή/αφαίρεση badge, χειροκίνητα badges, ανάγνωση/ρύθμιση trust factor, εσωτερικό προφίλ χρήστη). Κάθε μέθοδος του `ModerationApi` δέχεται μια παράμετρο `$sso` για την αυθεντικοποίηση του ενεργούντος moderator μέσω SSO.

### Χρήση PublicApi

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

### Χρήση ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - περιεχόμενο SSO που πιστοποιεί τον διαχειριστή

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```