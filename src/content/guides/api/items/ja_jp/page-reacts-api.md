Page Reacts は、ユーザーがページに「いいね」したり、独自のリアクション画像で反応できるようにします。 [Page Reacts widget](/guide-page-reacts.html) と Floating Likes ウィジェットはこれらのエンドポイント上に構築されており、独自のいいねボタンを作成するために自分で呼び出すこともできます。

このガイドの他の部分とは異なり、Page Reacts のエンドポイントは公開されています。ユーザーのブラウザから直接呼び出され、API キーは不要で、API クレジットも消費しません。すべてのリアクションはリクエストを行ったユーザーに紐付くため、ユーザーは自分のリアクションのみを追加または削除できます。

エンドポイントは 2 つのセットがあります：

- `/page-reacts/v1/likes/:tenantId` - ユーザーごとにページあたり 1 つの「いいね」。いいねボタンに使用します。
- `/page-reacts/v2/:tenantId` - ページごとに複数のリアクションを使用でき、短い `id`（例: `heart` や `laugh`）で識別します。

どちらも SDK の `PublicApi` の一部として利用可能で、例えば [JavaScript SDK](/guide-sdk-javascript.html) では `getV1PageLikes`、`createV1PageReact`、`deleteV1PageReact` があります。

### ユーザーの識別

リアクションはリクエストを行ったユーザーに紐付きます：

- **SSO users:** `sso` クエリパラメータに、コメントウィジェットに渡すのと同じ SSO オブジェクトの URI エンコードされた JSON を設定します。詳細は [SSO](/guide-customizations-and-configuration.html#sso) を参照してください。
- **Anonymous users:** `sso` パラメータがなく FastComments のログインもない場合、サーバーはブラウザに FastComments セッションクッキーで保存された匿名 ID を割り当てます。リクエストには `credentials: 'include'` を付けてクッキーを保持してください。サードパーティクッキーをブロックするブラウザでは匿名 ID が保持されないため、確実にユーザーを認識する必要がある場合は SSO を使用してください。

### urlId

`urlId` はページを識別するもので、コメントと同様の役割を果たします。コメントウィジェットに渡すのと同じ `urlId` を使用すれば、いいねとコメントが同じページでカウントされます。URI エンコードを忘れずに行ってください。

[inline-code-attrs-start title = 'いいねボタンの例'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// Optional, for SSO users. The same object you give the comment widget's "sso" option.
const sso = null;

function likesUrl() {
    let url = `https://fastcomments.com/page-reacts/v1/likes/${tenantId}?urlId=${encodeURIComponent(urlId)}`;
    if (sso) {
        url += '&sso=' + encodeURIComponent(JSON.stringify(sso));
    }
    return url;
}

async function getLikes() {
    const response = await fetch(likesUrl(), {credentials: 'include'});
    return response.json(); // {status, likeCount, didLike, commentCount, urlIdWS}
}

async function like() {
    await fetch(likesUrl(), {method: 'POST', credentials: 'include'});
}

async function unlike() {
    await fetch(likesUrl(), {method: 'DELETE', credentials: 'include'});
}
[inline-code-end]