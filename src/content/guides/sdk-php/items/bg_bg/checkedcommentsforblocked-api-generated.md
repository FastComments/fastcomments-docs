## Параметри

| Име | Тип | Location | Задължителен | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentIds | string | query | Да | Списък от id-та на коментари, разделени със запетаи. |
| sso | string | query | Не |  |

## Отговор

Връща: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CheckedCommentsForBlocked200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример за checkedCommentsForBlocked'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате собствен HTTP клиент, подайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е незадължително, по подразбиране ще се използва `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_ids = 'comment_ids_example'; // string | Списък от id-та на коментари, разделени със запетаи.
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->checkedCommentsForBlocked($tenant_id, $comment_ids, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->checkedCommentsForBlocked: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---