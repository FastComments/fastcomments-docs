Masovne informacije o uporabnikih za najemnika. Glede na userIds vrne prikazne informacije iz User / SSOUser.
Uporablja ga vtičnik za komentarje za obogatitev uporabnikov, ki so se pravkar pojavili zaradi dogodka prisotnosti.
Brez konteksta strani: zasebnost se uveljavlja enotno (zasebni profili so skriti).

## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | UserIds ločeni z vejico. |

## Odgovor

Vrne: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getUsersInfo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Če želite uporabiti prilagojen HTTP odjemalec, posredujte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je opcijsko, kot privzeti bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | userIds ločeni z vejico.

try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]