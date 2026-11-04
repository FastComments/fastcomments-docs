Masovne informacije o korisnicima za najmoprimca (tenant). Za zadane userIds, vraća informacije o prikazu iz User / SSOUser.
Koristi je widget za komentare za obogaćivanje korisnika koji su upravo pojavili putem presence eventa.
Bez konteksta stranice: privatnost se provodi jednoliko (privatni profili su maskirani).

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| ids | string | query | Da | userIds odvojeni zarezima. |

## Odgovor

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Primjer

[inline-code-attrs-start title = 'getUsersInfo primjer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni http klijent, proslijedite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, `GuzzleHttp\Client` će se koristiti kao zadani.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | userIds odvojeni zarezima.


try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]