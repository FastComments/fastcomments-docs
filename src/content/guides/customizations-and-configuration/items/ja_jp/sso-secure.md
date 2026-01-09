[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO は、SSO を実装するメカニズムとして HMAC-SHA256 暗号化を使用します。まず全体的なアーキテクチャを説明し、例と詳細な手順を示します。

また、類似の SSO メカニズムを持つ他のプロバイダーからの移行に関するドキュメントや、相違点についての説明もあります。

フローは次のとおりです:

<div class="screenshot white-bg">
    <div class="title">Secure SSO Flow</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

Secure SSO はフルスタック開発を伴うため、Java/Spring、NodeJS/Express、バニラ PHP による完全な動作コード例が現在 <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">GitHub にあります</a>。

NodeJS の例では ExpressJS、Java の例では Spring を使用していますが、これらのランタイムで FastComments SSO を実装するためにフレームワーク／ライブラリは必須ではありません — ネイティブの crypto パッケージで動作します。

FastComments SSO を使用する際に新しい API エンドポイントを作成する必要はありません。ユーザーの情報をシークレットキーで暗号化して、ペイロードをコメントウィジェットに渡すだけです。

#### Get Your API Secret Key

API Secret は <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">このページ</a> から取得できます。このページは My Account に移動し、API/SSO タイルをクリックしてから「Get API Secret Key」をクリックしても見つかります。

#### Comment Widget Parameters

コメントウィジェットのハイレベルな API ドキュメントは <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">こちら</a> にあります。

これらのパラメータが何を意味するか、もう少し詳しく説明します。

コメントウィジェットは設定オブジェクトを受け取ります — FastComments を使用してテナント ID（tenantId と呼ばれる）を渡している場合はすでにこれを渡しています。

SSO を有効にするには、新しい "sso" オブジェクトを渡します。これは次のパラメータを持つ必要があります。これらの値はサーバー側で生成してください。

- userDataJSONBase64: ユーザーのデータを JSON 形式にしたものを Base64 エンコードしたもの。
- verificationHash: UNIX_TIME_MILLIS + userDataJSONBase64 から生成された HMAC-SHA256 ハッシュ。
- timestamp: エポックタイムスタンプ（**ミリ秒**）。未来の時刻であってはならず、かつ過去 2 日以上前であってはなりません。
- loginURL: コメントウィジェットがログイン用に表示できる URL。
- logoutURL: コメントウィジェットがログアウト用に表示できる URL。
- loginCallback: login URL の代わりに提供された場合、コメントウィジェットがログインボタンをクリックしたときに呼び出す関数。
- logoutCallback: logout URL の代わりに提供された場合、コメントウィジェットがログアウトボタンをクリックしたときに呼び出す関数。

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

The User Object
[inline-code-attrs-start title = 'ユーザーオブジェクト'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Required. 1k Characters Max. **/
    id: string;
    /** Required. 1k Characters Max. Note: Must be unique. **/
    email: string;
    /** Required. 1k Characters Max. Note: The username cannot be an email. Does not have to be unique. **/
    username: string;
    /** Optional. 3k Characters Max for URLs. Default is from gravatar based on email. Supports 64 encoded images, in which case the limit is 50k characters. **/ 
    avatar?: string;
    /** Optional. Default false. **/
    optedInNotifications?: boolean;
    /** Optional. Default false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Optional. 100 Characters Max. This label will be shown next to their name. Default is Administrator/Moderator when applicable. **/
    displayLabel?: string;
    /** Optional. 500 Characters Max. This will be shown instead of the username. **/
    displayName?: string;
    /** Optional. 2k Characters Max. The user's name will link to this. **/
    websiteUrl?: string;
    /** Optional. Up to 100 groups per user. A group id may not be longer than 50 characters. **/
    groupIds?: string[];
    /** Optional. Denotes the user as an administrator. **/
    isAdmin?: boolean;
    /** Optional. Denotes the user as a moderator. **/
    isModerator?: boolean;
    /** Optional, default true. Set to false to enable the "activity" tab in the user's profile. **/
    isProfileActivityPrivate?: boolean;
    /** Optional, default false. Set to true to disable profile comments. **/
    isProfileCommentsPrivate?: boolean;
    /** Optional, default false. Set to true to disable direct messaging this user. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderators and Administrators

管理者およびモデレーターには、`SSOUser` オブジェクト内でそれぞれ `isAdmin` または `isModerator` フラグを渡してください。

#### Notifications

通知を有効または無効にするには、`optedInNotifications` の値をそれぞれ `true` または `false` に設定してください。ユーザーがこの値を含む SSO ペイロードでページを最初に読み込んだときに、通知設定が更新されます。

さらに、ユーザーがサブスクライブしたページでのアクティビティに関する通知メール（アプリ内通知だけでなく）を受け取りたい場合は、`optedInSubscriptionNotifications` を `true` に設定してください。

#### VIP Users & Special Labels

オプションの "displayLabel" フィールドを使用すると、ユーザー名の横に特別なラベルを表示できます。

#### Unauthenticated users

認証されていないユーザーを表すには、単に userDataJSONBase64、verificationHash、timestamp を設定しないでください。loginURL を提供してください。

これらのユーザーはコメントすることができず、代わりにログインメッセージ（設定に応じてメッセージ、リンク、またはボタン）が表示されます。

#### Direct Examples for Serializing and Hashing User Data

シリアライズとハッシュ化の具体例については、詳細を <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">こちら</a>（js）、<a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">こちら</a>（java）、および <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">こちら</a>（php）で確認できます。

統合は複雑で手間のかかるプロセスになり得ることを理解しています。担当者に連絡するか、<a href="https://fastcomments.com/auth/my-account/help" target="_blank">サポートページ</a> をご利用ください。

---