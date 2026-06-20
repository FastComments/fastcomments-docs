SDK'et udstiller tre API-klientklasser:

- **`DefaultApi`** — Metoder, der er autentificeret med API-nøgle til server-side brug. Konfigurer en API-nøgle som vist i [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** — Offentlige metoder, der ikke kræver en API-nøgle og som er sikre at kalde fra browsere og mobilapps.
- **`ModerationApi`** — Metoder til moderator-dashboardet: oplistning, optælling, søgning, logning og eksport af kommentarer; moderationhandlinger (fjern/gendan, flag, indstil review/spam/godkendelsesstatus, stemmer, genåbn/luk tråd); bans (udeluk fra at kommentere, fortryd, forudgående ban-oversigter, banestatus og præferencer, antal bandlyste brugere); og badges & tillid (tildel/fjern badge, manuelle badges, få/opsæt tillidsfaktor, bruger intern profil). Hver `ModerationApi`-metode accepterer en `$sso`-parameter for at autentificere den handlende moderator via SSO.

### Brug af PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Offentlige metoder kræver ikke en API-nøgle.
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

### Brug af ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO-payload, der autentificerer moderatoren

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```