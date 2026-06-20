---
## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentIds | string | query | Da | Zarezom odvojeni popis ID-jeva komentara. |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CheckBlockedCommentsResponse.php)

## Primjer

[inline-code-attrs-start title = 'Primjer checkedCommentsForBlocked'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni HTTP klijent, proslijedite klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, kao zadani bit će korišten `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_ids = 'comment_ids_example'; // string | Zarezom odvojeni popis ID-jeva komentara.
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->checkedCommentsForBlocked($tenant_id, $comment_ids, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->checkedCommentsForBlocked: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---