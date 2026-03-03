[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO は SSO を実装するメカニズムとして HMAC-SHA256 暗号化を使用します。まず全体アーキテクチャを説明し、例と詳細な手順を示します。

類似の SSO メカニズムを持つ他のプロバイダからの移行や相違点に関するドキュメントもあります。

フローは以下のようになります：

<div class="screenshot white-bg">
    <div class="title">Secure SSO フロー</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

Secure SSO はフルスタックの開発を伴うため、Java/Spring、NodeJS/Express、およびバニラ PHP の完全な動作コード例が現在 <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">GitHub にあります</a>。

NodeJS の例では ExpressJS、Java の例では Spring を使用していますが、FastComments SSO を実装するためにこれらの実行環境でフレームワーク/ライブラリは必須ではありません - ネイティブの crypto パッケージで動作します。

FastComments SSO では新しい API エンドポイントを作成する必要はありません。シークレットキーでユーザー情報を暗号化し、そのペイロードをコメントウィジェットに渡すだけです。

#### Get Your API Secret Key

API シークレットは <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">このページ</a> から取得できます。マイアカウントに移動し、API/SSO タイルをクリックしてから "Get API Secret Key" をクリックしてもこのページにアクセスできます。

#### Comment Widget Parameters

コメントウィジェットのハイレベルな API ドキュメントは <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">こちら</a> にあります。

これらのパラメータが何を意味するか、詳細を見ていきます。

コメントウィジェットは設定オブジェクトを受け取ります - FastComments を使用している場合、customer id（tenantId と呼ばれます）を渡して既にこのオブジェクトを渡しているはずです。

SSO を有効にするには、新しい "sso" オブジェクトを渡します。そのオブジェクトには次のパラメータが必要です。値はサーバー側で生成してください。

- userDataJSONBase64: ユーザーのデータを JSON 形式で表したものを Base64 エンコードしたもの。
- verificationHash: UNIX_TIME_MILLIS + userDataJSONBase64 から作成した HMAC-SHA256 ハッシュ。
- timestamp: エポックタイムスタンプ、**ミリ秒**単位。未来の時刻であってはならず、過去2日以上前でもいけません。
- loginURL: コメントウィジェットがログイン表示に使える URL。
- logoutURL: コメントウィジェットがログアウト表示に使える URL。
- loginCallback: login URL の代わりに指定した場合、コメントウィジェットがログインボタンをクリックしたときに呼び出す関数。
- logoutCallback: logout URL の代わりに指定した場合、コメントウィジェットがログアウトボタンをクリックしたときに呼び出す関数。

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

ユーザーオブジェクトは次のスキーマを持ちます：
[inline-code-attrs-start title = 'The User Object'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** 必須。最大1,000文字。 **/
    id: string;
    /** 必須。最大1,000文字。注：一意である必要があります。 **/
    email: string;
    /** 必須。最大1,000文字。注：ユーザー名はメールアドレスにできません。一意である必要はありません。 **/
    username: string;
    /** 任意。URLは最大3,000文字。デフォルトはメールのGravatarから取得されます。Base64エンコードされた画像をサポートし、その場合は上限が50,000文字です。 **/ 
    avatar?: string;
    /** 任意。デフォルトはfalse。 **/
    optedInNotifications?: boolean;
    /** 任意。デフォルトはfalse。 **/
    optedInSubscriptionNotifications?: boolean;
    /** 任意。最大100文字。このラベルは名前の横に表示されます。該当する場合はデフォルトで管理者/モデレーターが表示されます。 **/
    displayLabel?: string;
    /** 任意。最大500文字。ユーザー名の代わりに表示されます。 **/
    displayName?: string;
    /** 任意。最大2,000文字。ユーザー名はこれにリンクします。 **/
    websiteUrl?: string;
    /** 任意。ユーザーごとに最大100グループ。グループIDは50文字を超えてはいけません。 **/
    groupIds?: string[];
    /** 任意。ユーザーを管理者として示します。 **/
    isAdmin?: boolean;
    /** 任意。ユーザーをモデレーターとして示します。 **/
    isModerator?: boolean;
    /** 任意。デフォルトはtrue。falseに設定するとユーザープロファイルの「activity」タブが有効になります。 **/
    isProfileActivityPrivate?: boolean;
    /** 任意。デフォルトはfalse。trueに設定するとプロフィールへのコメントを無効にします。 **/
    isProfileCommentsPrivate?: boolean;
    /** 任意。デフォルトはfalse。trueに設定するとこのユーザーへの直接メッセージを無効にします。 **/
    isProfileDMDisabled?: boolean;
    /** ユーザーバッジの任意設定。 **/
    badgeConfig?: {
        /** 割り当てるグローバルバッジIDの配列。バッジは最大30個まで。順序は保持されます。 **/
        badgeIds: string[];
        /** 現在のページ（urlId）に限定されたバッジIDの配列。割り当てられたページでのみ表示されます。 **/
        pageBadgeIds?: string[];
        /** true の場合、既存の表示バッジを置き換えます。グローバルとページスコープは独立して上書きされます。 **/
        override?: boolean;
        /** true の場合、テナント設定からバッジ表示プロパティを更新します。 **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderators and Administrators

For admins and moderators, pass the respective `isAdmin` or `isModerator` flags in the `SSOUser` object.

#### Notifications

通知を有効または無効にするには、`optedInNotifications` の値をそれぞれ `true` または `false` に設定します。ユーザーがこの値を含む SSO ペイロードでページを最初に読み込んだときに、通知設定が更新されます。

さらに、ユーザーが購読したページでのアクティビティに関する通知メール（アプリ内通知だけでなく）を受け取りたい場合は、`optedInSubscriptionNotifications` を `true` に設定してください。

#### VIP Users & Special Labels

オプションの "displayLabel" フィールドを使用して、ユーザー名の横に特別なラベルを表示できます。

#### Unauthenticated users

認証されていないユーザーを表すには、単に userDataJSONBase64、verificationHash、または timestamp を設定しないでください。loginURL を提供します。

これらのユーザーはコメントできず、代わりにログインメッセージ（設定によってメッセージ、リンク、またはボタン）が表示されます。

#### Direct Examples for Serializing and Hashing User Data

詳細な例は <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">こちら</a>（js）、<a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">こちら</a>（java）、および <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">こちら</a>（php）にあります。

統合は複雑で困難な場合があることを理解しています。担当者に連絡するか、<a href="https://fastcomments.com/auth/my-account/help" target="_blank">サポートページ</a>を利用してください。