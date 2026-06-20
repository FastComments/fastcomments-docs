## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| commentId | string | путања | Да |  |
| sso | string | упит | Не |  |

## Одговор

Враћа: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AdjustVotesResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример postAdjustCommentVotes'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако желите да користите прилагођен HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционо, `GuzzleHttp\Client` ће се користити као подразумевани.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // низ
$adjust_comment_votes_params = new \FastComments\Client\Model\AdjustCommentVotesParams(); // \FastComments\Client\Model\AdjustCommentVotesParams
$sso = 'sso_example'; // низ

try {
    $result = $apiInstance->postAdjustCommentVotes($comment_id, $adjust_comment_votes_params, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postAdjustCommentVotes: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---