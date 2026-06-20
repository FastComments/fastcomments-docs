Il SDK espone tre classi client API:

- **`DefaultApi`** — Metodi autenticati tramite API key per uso server-side. Configura una API key come mostrato in [Primi passi](#getting-started-readme-generated).
- **`PublicApi`** — Metodi pubblici che non richiedono una API key, sicuri da chiamare da browser e app mobili.
- **`ModerationApi`** — Metodi per la dashboard di moderazione: elencare, contare, cercare, registrare ed esportare commenti; azioni di moderazione (rimuovi/ripristina, segnala, imposta stato revisione/spam/approvazione, voti, riapri/chiudi thread); ban (ban dai commenti, annulla, riepiloghi pre-ban, stato e preferenze ban, conteggi utenti bannati); e badge & trust (assegna/rimuovi badge, badge manuali, ottenere/impostare fattore di fiducia, profilo interno utente). Ogni metodo di `ModerationApi` accetta un parametro `$sso` per autenticare il moderatore tramite SSO.

### Utilizzo di PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// I metodi pubblici non richiedono una chiave API.
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

### Utilizzo di ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - Payload SSO che autentica il moderatore

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```