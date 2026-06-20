De SDK biedt drie API-clientklassen:

- **`DefaultApi`** — Methoden die authenticatie met API-sleutel vereisen voor server-side gebruik. Configureer een API-sleutel zoals getoond in [Aan de slag](#getting-started-readme-generated).
- **`PublicApi`** — openbare methoden die geen API-sleutel vereisen, veilig om te gebruiken vanuit browsers en mobiele apps.
- **`ModerationApi`** — methoden voor het moderatordashboard: het weergeven, tellen, doorzoeken, loggen en exporteren van opmerkingen; moderatieacties (verwijderen/terugzetten, markeren, review/spam/goedkeuringsstatus instellen, stemmen, thread heropenen/sluiten); bans (verbannen vanaf reageren, ongedaan maken, pre-ban samenvattingen, ban-status en voorkeuren, aantal verbannen gebruikers); en badges & vertrouwen (badge toekennen/verwijderen, handmatige badges, trustfactor ophalen/instellen, intern gebruikersprofiel). Elke `ModerationApi`-methode accepteert een `$sso`-parameter om de handelende moderator via SSO te authenticeren.

### PublicApi gebruiken

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Openbare methoden vereisen geen API-sleutel.
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

### ModerationApi gebruiken

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO-payload die de moderator authenticeert

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```