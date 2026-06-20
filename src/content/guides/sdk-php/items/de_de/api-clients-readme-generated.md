Das SDK stellt drei API-Client-Klassen bereit:

- **`DefaultApi`** — Methoden, die per API-Schlüssel authentifiziert sind, für die serverseitige Verwendung. Konfigurieren Sie einen API-Schlüssel wie in [Getting Started](#getting-started-readme-generated) beschrieben.
- **`PublicApi`** — öffentliche Methoden, die keinen API-Schlüssel benötigen und sicher aus Browsern und mobilen Apps aufgerufen werden können.
- **`ModerationApi`** — Methoden für das Moderations-Dashboard: Auflisten, Zählen, Suchen, Protokollieren und Exportieren von Kommentaren; Moderationsaktionen (entfernen/wiederherstellen, markieren, Review/Spam/Freigabe-Status setzen, Stimmen, Thread wieder öffnen/schließen); Sperren (vom Kommentieren ausschließen, rückgängig machen, Vor-Sperr-Zusammenfassungen, Sperrstatus und -einstellungen, Anzahl gesperrter Benutzer); sowie Abzeichen & Vertrauen (Abzeichen vergeben/entfernen, manuelle Abzeichen, Trust-Faktor abrufen/setzen, internes Benutzerprofil). Jede `ModerationApi`-Methode akzeptiert einen `$sso`-Parameter, um den handelnden Moderator via SSO zu authentifizieren.

### Verwendung von PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Öffentliche Methoden benötigen keinen API-Schlüssel.
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

### Verwendung von ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO-Payload zur Authentifizierung des Moderators

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```