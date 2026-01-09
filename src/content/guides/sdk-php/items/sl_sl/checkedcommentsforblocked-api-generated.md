## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentIds | string | query | Da | Z vejico ločen seznam ID-jev komentarjev. |
| sso | string | query | Ne |  |

## Odgovor

Vrača: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CheckedCommentsForBlocked200Response.php)

## Primer

[inline-code-attrs-start title = 'Primer checkedCommentsForBlocked'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Če želite uporabljati lastnega HTTP odjemalca, posredujte odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // To je izbirno, privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_ids = 'comment_ids_example'; // string | Z vejico ločen seznam ID-jev komentarjev.
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->checkedCommentsForBlocked($tenant_id, $comment_ids, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Izjema pri klicu PublicApi->checkedCommentsForBlocked: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---