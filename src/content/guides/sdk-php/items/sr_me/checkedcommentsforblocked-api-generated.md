## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentIds | string | query | Da | Lista ID-eva komentara razdvojena zarezom. |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CheckedCommentsForBlocked200Response.php)

## Primjer

[inline-code-attrs-start title = 'checkedCommentsForBlocked Primjer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođen HTTP klijent, proslijedite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, `GuzzleHttp\Client` će biti korišćen kao podrazumijevani.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_ids = 'comment_ids_example'; // string | Lista ID-eva komentara razdvojena zarezom.
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->checkedCommentsForBlocked($tenant_id, $comment_ids, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->checkedCommentsForBlocked: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---