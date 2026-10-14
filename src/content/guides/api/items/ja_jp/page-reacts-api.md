Page Reactsは、ユーザーがページに「いいね」したり、独自のリアクション画像セットでリアクションできるようにします。The [Page Reacts widget](/guide-page-reacts.html) と the Floating Likes widget はこれらのエンドポイント上に構築されており、独自のいいねボタンを作成するために自分で呼び出すこともできます。

このガイドの他の部分とは異なり、Page Reacts のエンドポイントは公開されています。ユーザーのブラウザから呼び出され、API キーは不要で、API クレジットも消費しません。各リアクションはリクエストを行ったユーザーに紐づくため、ユーザーは自分のリアクションのみを追加または削除できます。

エンドポイントは2つのセットがあります：

- `/page-reacts/v1/likes/:tenantId` - ユーザーごとにページあたり1つの「like」。いいねボタンに使用します。
- `/page-reacts/v2/:tenantId` - ページごとに複数のリアクションが可能で、短い `id`（例: `heart` や `laugh`）で識別します。

これらは `PublicApi` の一部として SDK でも利用可能で、例えば [JavaScript SDK](/guide-sdk-javascript.html) の `getV1PageLikes`、`createV1PageReact`、`deleteV1PageReact` があります。

### Showing Who Reacted

`includeUserInfo=public` を `GET` エンドポイントに渡すと、いいねまたはリアクションしたユーザーのユーザー名とアバターを取得でき、最新のものが先に返されます。プロフィールのアクティビティを非公開にしているユーザーや匿名ユーザーは除外されます。最大 100 件のリアクションが返されます。

SSO ユーザーは、SSO ペイロードで `isProfileActivityPrivate` を `false` に設定しない限り、プロフィールのアクティビティが非公開として作成されます。リストに表示させたいユーザーのためにこのフラグを設定してください。

SSO が必要なページでは、ページを閲覧できる SSO ユーザーだけがリアクションしたユーザーの一覧を取得できます。その他のユーザーは `403` エラーとコード `sso-required` または `page-forbidden` が返されます。

サーバー側からは、[`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) と [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) が API キーで同じ一覧を返し、`includeUserInfo=include-private` を指定すると非公開プロフィールのユーザーも含められます。

### Identifying the User

リアクションはリクエストを行ったユーザーに紐づきます：

- **SSO ユーザー:** `sso` クエリパラメータに、コメントウィジェットに渡すのと同じ SSO オブジェクトを URI エンコードした JSON を設定して送ります。詳細は [SSO](/guide-customizations-and-configuration.html#sso) を参照してください。
- **匿名ユーザー:** `sso` パラメータも FastComments のログインもない場合、サーバーはブラウザに FastComments セッションクッキーに保存された匿名 ID を割り当てます。リクエストは `credentials: 'include'` を付けてクッキーが保持されるようにしてください。サードパーティクッキーをブロックするブラウザでは匿名 ID が保持されないため、確実にユーザーを認識させる必要がある場合は SSO を使用してください。

### The urlId

`urlId` はページを識別します。コメントと同じ方式です。コメントウィジェットに渡すのと同じ `urlId` を使用すれば、いいねとコメントが同じページでカウントされます。URI エンコードを忘れずに行ってください。

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