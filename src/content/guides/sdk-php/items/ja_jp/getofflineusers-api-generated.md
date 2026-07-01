Past commenters on the page who are NOT currently online. Sorted by displayName.  
ページ上で過去にコメントしたが、現在オンラインではないユーザー。displayNameでソートされます。

Use this after exhausting /users/online to render a "Members" section.  
`/users/online` をすべて使い切った後に、"Members" セクションを表示するために使用します。

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
commenterName に対するカーソルページング: サーバーは部分的な `{tenantId, urlId, commenterName}` インデックスを `afterName` 以降へ `$gt` で進めます。`$skip` のコストはかかりません。

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | ページ URL 識別子（サーバー側でクリーンアップされたもの）。 |
| afterName | string | query | No | カーソル: 前回のレスポンスから `nextAfterName` を渡します。 |
| afterUserId | string | query | No | カーソルのタイブレーカー: 前回のレスポンスから `nextAfterUserId` を渡します。`afterName` が設定されている場合に必要で、名前が同じ場合にエントリが落ちないようにします。 |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Example

[inline-code-attrs-start title = 'getOfflineUsers の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタム HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // これはオプションです。デフォルトで `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | ページ URL 識別子（サーバー側でクリーンアップされたもの）。
$options = [
    'after_name' => 'after_name_example', // string | カーソル: 前回のレスポンスから `nextAfterName` を渡します。
    'after_user_id' => 'after_user_id_example', // string | カーソルのタイブレーカー: 前回のレスポンスから `nextAfterUserId` を渡します。`afterName` が設定されている場合に必要で、名前が同じ場合にエントリが落ちないようにします。
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]