## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| batchJobId | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationExportStatusResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getApiExportStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta implementującego `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, domyślnie zostanie użyty `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$batch_job_id = 'batch_job_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getApiExportStatus($batch_job_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiExportStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---