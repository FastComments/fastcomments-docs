Page Reacts(ページリアクション)を使用すると、ユーザーがページに「いいね」をしたり、独自のリアクション画像セットで反応したりできるようになります。[Page Reactsウィジェット](/guide-page-reacts.html)とFloating Likesウィジェットはこれらのエンドポイントを基盤としており、独自の「いいね」ボタンを構築するために自分で呼び出すこともできます。

このガイドの他の部分とは異なり、Page Reactsのエンドポイントは公開されています。これらはユーザーのブラウザから呼び出され、APIキーを必要とせず、APIクレジットも消費しません。すべてのリアクションはリクエストを行ったユーザーに紐付けられるため、ユーザーは自分自身のリアクションのみ追加または削除できます。

エンドポイントには2種類あります:

- `/page-reacts/v1/likes/:tenantId` - ユーザーごと・ページごとに1つの「いいね」。いいねボタンに使用します。
- `/page-reacts/v2/:tenantId` - ページごとに複数のリアクション。それぞれ、あなたが選んだ短い`id`(例:`heart`や`laugh`)で識別されます。

どちらもSDKの`PublicApi`の一部として利用可能です。例えば、[JavaScript SDK](/guide-sdk-javascript.html)の`getV1PageLikes`、`createV1PageReact`、`deleteV1PageReact`などがあります。

### リアクションしたユーザーの表示

`GET`エンドポイントに`includeUserInfo=public`を渡すと、いいねやリアクションをしたユーザーの情報(ユーザー名とアバター、新しい順)も取得できます。プロフィールアクティビティを非公開にしているユーザーや、匿名ユーザーは除外されます。最大100件のリアクションが返されます。

SSOユーザーは、SSOペイロードで`isProfileActivityPrivate`を`false`に設定しない限り、非公開のプロフィールアクティビティとして作成されます。これらのリストに表示したいユーザーに対しては、この設定を行ってください。

SSOが必要なページでは、ページを閲覧できるSSOユーザーのみがリアクションしたユーザーを一覧表示できます。それ以外のユーザーは、`sso-required`または`page-forbidden`というコードを含む`403`を受け取ります。

サーバーからは、APIキーを使用して[`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get)と[`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get)で同じリストを取得でき、さらに`includeUserInfo=include-private`を指定して非公開プロフィールのユーザーを含めることもできます。

### ユーザーの識別

リアクションはリクエストを行ったユーザーに紐付けられます:

- **SSOユーザー:** `sso`クエリパラメータに、コメントウィジェットに渡すのと同じSSOオブジェクトのURIエンコードされたJSONを設定して渡します。[SSO](/guide-customizations-and-configuration.html#sso)を参照してください。
- **匿名ユーザー:** `sso`パラメータがなく、FastCommentsへのログインもない場合、サーバーはFastCommentsのセッションCookieに保存された匿名IDをブラウザに割り当てます。Cookieをリクエスト間で保持するため、`credentials: 'include'`を付けてリクエストを送信してください。サードパーティCookieをブロックするブラウザでは匿名IDが保持されないため、各ユーザーを確実に識別する必要がある場合はSSOを使用してください。

### urlIdについて

`urlId`はページを識別するもので、コメントの場合と同じです。いいねとコメントが同じページでカウントされるよう、コメントウィジェットに渡すのと同じ`urlId`を使用してください。URIエンコードすることを忘れないでください。

[inline-code-attrs-start title = 'いいねボタンの例'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId = 'demo';
const urlId = 'https://example.com/my-article';
// 任意、SSOユーザーの場合。コメントウィジェットの"sso"オプションに渡すのと同じオブジェクト。
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