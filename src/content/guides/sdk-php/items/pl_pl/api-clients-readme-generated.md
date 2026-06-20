The SDK udostępnia trzy klasy klienta API:

- **`DefaultApi`** — metody uwierzytelnione kluczem API do użycia po stronie serwera. Skonfiguruj klucz API, jak pokazano w [Pierwsze kroki](#getting-started-readme-generated).
- **`PublicApi`** — metody publiczne, które nie wymagają klucza API, bezpieczne do wywoływania z przeglądarek i aplikacji mobilnych.
- **`ModerationApi`** — metody dla panelu moderatora: listowanie, zliczanie, wyszukiwanie, rejestrowanie i eksportowanie komentarzy; akcje moderacyjne (usuń/przywróć, zgłoś, ustaw status do przeglądu/spam/akceptacja, głosy, ponowne otwarcie/zamknięcie wątku); bany (zbanowanie z komentowania, cofnięcie, podsumowania przed zbanowaniem, status i preferencje bana, liczby zbanowanych użytkowników); oraz odznaki i zaufanie (przyznawanie/usuwanie odznaki, odznaki ręczne, pobierz/ustaw współczynnik zaufania, wewnętrzny profil użytkownika). Każda metoda `ModerationApi` akceptuje parametr `$sso` w celu uwierzytelnienia działającego moderatora przez SSO.

### Używanie PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Metody publiczne nie wymagają klucza API.
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

### Używanie ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - Dane SSO uwierzytelniające moderatora

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```