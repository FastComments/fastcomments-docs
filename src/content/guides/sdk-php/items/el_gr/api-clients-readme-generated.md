Το SDK παρέχει τρεις κλάσεις πελατών API:

- **`DefaultApi`** — μέθοδοι αυθεντικοποιημένες με API key για χρήση στο server-side. Διαμορφώστε ένα API key όπως φαίνεται στο [Εισαγωγή](#getting-started-readme-generated).
- **`PublicApi`** — δημόσιες μέθοδοι που δεν απαιτούν API key, ασφαλείς για κλήση από προγράμματα περιήγησης και εφαρμογές για κινητά.
- **`ModerationApi`** — μέθοδοι για τον πίνακα ελέγχου των διαχειριστών: λίστα, καταμέτρηση, αναζήτηση, καταγραφή και εξαγωγή σχολίων; ενέργειες εποπτείας (αφαίρεση/επαναφορά, σημαία, ορισμός κατάστασης ανασκόπησης/spam/έγκρισης, ψήφοι, επανανοίγμα/κλείσιμο νήματος); αποκλεισμοί (αποκλεισμός χρήστη από το σχολιασμό, αναίρεση αποκλεισμού, προ-συνοπτικές αναφορές πριν από αποκλεισμό, κατάσταση και προτιμήσεις αποκλεισμού, πλήθος αποκλεισμένων χρηστών); και σήματα & εμπιστοσύνη (απονομή/αφαίρεση σήματος, χειροκίνητα σήματα, λήψη/ορισμός παράγοντα εμπιστοσύνης, εσωτερικό προφίλ χρήστη). Κάθε μέθοδος του `ModerationApi` δέχεται παράμετρο `$sso` για να αυθεντικοποιήσει τον ενεργούντα διαχειριστή μέσω SSO.

### Χρήση του PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Οι δημόσιες μέθοδοι δεν απαιτούν API key.
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
$sso = 'sso_example'; // string - δεδομένα SSO που αυθεντικοποιούν τον διαχειριστή

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```