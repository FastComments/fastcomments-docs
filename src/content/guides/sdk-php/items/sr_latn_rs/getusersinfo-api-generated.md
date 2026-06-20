---
Masovni podaci o korisnicima za tenant. Za date userIds, vraća prikazne informacije iz User / SSOUser.
Koristi se u widgetu komentara za obogaćivanje korisnika koji su se upravo pojavili putem događaja prisutnosti.
Bez konteksta stranice: privatnost se sprovodi jednako (privatni profili su maskirani).

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | putanja | Da |  |
| ids | string | upit | Da | userIds odvojeni zarezima. |

## Odgovor

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Primer

[inline-code-attrs-start title = 'getUsersInfo Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite da koristite prilagođeni http klijent, prosledite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, `GuzzleHttp\Client` će biti korišćen kao podrazumevani.
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

---