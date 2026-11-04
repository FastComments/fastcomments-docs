[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO では、SSO を実装する仕組みとして HMAC-SHA256 暗号化を使用します。まず全体のアーキテクチャを説明し、その後に例と詳細な手順を示します。

また、類似の SSO 仕組みを持つ他のプロバイダーからの移行に関するドキュメントや、その違いについてのドキュメントもあります。

フローは以下のようになります:

<div class="screenshot white-bg">
    <div class="title">Secure SSO フロー</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO 図" />
</div>

Secure SSO はフルスタック開発を伴うため、Java/Spring、NodeJS/Express、素の PHP による完全に動作するコード例が現在 <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">GitHub</a> で公開されています。

NodeJS の例では ExpressJS、Java の例では Spring を使用していますが、FastComments SSO を実装するためにこれらのランタイムでフレームワークやライブラリは必要ありません。ネイティブの暗号化パッケージで動作します。

FastComments SSO では、新しい API エンドポイントを書く必要はありません。秘密鍵を使ってユーザー情報を暗号化し、そのペイロードをコメントウィジェットに渡すだけです。

#### API シークレットキーの取得

API シークレットは <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">このページ</a> から取得できます。このページには、マイアカウントに移動し、API/SSO タイルをクリックし、「Get API Secret Key」をクリックすることでもアクセスできます。

#### コメントウィジェットのパラメータ

コメントウィジェットの高度な API ドキュメントは <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">こちら</a> にあります。

これらのパラメータの意味について詳しく見ていきましょう。

コメントウィジェットは設定オブジェクトを受け取ります。FastComments を使用している場合、顧客 ID(tenantId と呼ばれます)を渡す際にすでにこれを渡しています。

SSO を有効にするには、新しい "sso" オブジェクトを渡します。このオブジェクトには以下のパラメータが必要です。値はサーバーサイドで生成する必要があります。

- userDataJSONBase64: JSON 形式のユーザーデータを Base64 エンコードしたもの。
- verificationHash: UNIX_TIME_MILLIS + userDataJSONBase64 から作成された HMAC-SHA256 ハッシュ。
- timestamp: エポックタイムスタンプ(**ミリ秒**)。未来の時刻、または 2 日以上過去の時刻であってはなりません。
- loginURL: コメントウィジェットがユーザーのログイン用に表示できる URL。
- logoutURL: コメントウィジェットがユーザーのログアウト用に表示できる URL。
- loginCallback: ログイン URL の代わりに指定できる、ログインボタンがクリックされたときにコメントウィジェットが呼び出す関数。
- logoutCallback: ログアウト URL の代わりに指定できる、ログアウトボタンがクリックされたときにコメントウィジェットが呼び出す関数。

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO クライアントコード'; isFunctional = false; code-example-end]

#### ユーザーオブジェクト

ユーザーオブジェクトには以下のスキーマが含まれます:
[inline-code-attrs-start title = 'ユーザーオブジェクト'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** 必須。最大 1k 文字。 **/
    id: string;
    /** 必須。最大 1k 文字。注意: 一意である必要があります。 **/
    email: string;
    /** 必須。最大 1k 文字。注意: ユーザー名にメールアドレスは使用できません。一意である必要はありません。 **/
    username: string;
    /** 任意。URL の場合は最大 3k 文字。デフォルトはメールに基づく gravatar の画像。Base64 エンコードされた画像もサポートし、その場合の上限は 50k 文字です。 **/ 
    avatar?: string;
    /** 任意。デフォルトは false。 **/
    optedInNotifications?: boolean;
    /** 任意。デフォルトは false。 **/
    optedInSubscriptionNotifications?: boolean;
    /** 任意。最大 100 文字。このラベルは名前の横に表示されます。該当する場合のデフォルトは Administrator/Moderator です。 **/
    displayLabel?: string;
    /** 任意。最大 500 文字。これはユーザー名の代わりに表示されます。 **/
    displayName?: string;
    /** 任意。最大 2k 文字。ユーザー名がこの URL にリンクされます。 **/
    websiteUrl?: string;
    /** 任意。ユーザーあたり最大 100 グループ。グループ ID は 50 文字を超えてはいけません。 **/
    groupIds?: string[];
    /** 任意。ユーザーを管理者として扱います。 **/
    isAdmin?: boolean;
    /** 任意。ユーザーをモデレーターとして扱います。 **/
    isModerator?: boolean;
    /** 任意、デフォルトは true。false に設定すると、ユーザーのプロフィールに「アクティビティ」タブが表示されます。 **/
    isProfileActivityPrivate?: boolean;
    /** 任意、デフォルトは false。true に設定すると、プロフィールへのコメントが無効になります。 **/
    isProfileCommentsPrivate?: boolean;
    /** 任意、デフォルトは false。true に設定すると、このユーザーへのダイレクトメッセージが無効になります。 **/
    isProfileDMDisabled?: boolean;
    /** 任意。ユーザーバッジの設定。 **/
    badgeConfig?: {
        /** 割り当てるグローバルバッジ ID の配列。最大 30 個のバッジ。順序は維持されます。 **/
        badgeIds: string[];
        /** 現在のページ(urlId)にスコープされたバッジ ID の配列。割り当てられたページでのみ表示されます。 **/
        pageBadgeIds?: string[];
        /** true の場合、既存の表示中のバッジを置き換えます。グローバルとページスコープはそれぞれ独立して上書きされます。 **/
        override?: boolean;
        /** true の場合、テナント設定からバッジの表示プロパティを更新します。 **/
        update?: boolean;
    };
}
[inline-code-end]

#### モデレーターと管理者

管理者およびモデレーターの場合は、`SSOUser` オブジェクトにそれぞれ `isAdmin` または `isModerator` フラグを渡してください。

#### 通知

通知を有効または無効にするには、`optedInNotifications` の値をそれぞれ `true` または `false` に設定します。この値が SSO ペイロードに含まれた状態でユーザーが初めてページを読み込むと、そのユーザーの通知設定が更新されます。

さらに、アプリ内通知だけでなく、ユーザーが購読したページでのアクティビティに関する通知メールを受け取らせたい場合は、`optedInSubscriptionNotifications` を `true` に設定してください。

#### VIP ユーザーと特別ラベル

オプションの "displayLabel" フィールドを使用すると、ユーザー名の横に特別なラベルを表示できます。

#### 未認証ユーザー

未認証ユーザーを表すには、userDataJSONBase64、verificationHash、timestamp を設定せず、loginURL を指定するだけです。

これらのユーザーはコメントできず、代わりにログインメッセージ(設定に応じてメッセージ、リンク、またはボタン)が表示されます。

#### ユーザーデータのシリアライズとハッシュ化の直接の例

詳細な例は <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/node-express/routes/index.js#L23" target="_blank">こちら</a>(js)、<a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java-springboot/src/main/java/com/winricklabs/ssodemo/DemoController.java#L68" target="_blank">こちら</a>(java)、<a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">こちら</a>(php)にあります。

どのような統合でも複雑で苦労するプロセスになり得ることを私たちは理解しています。担当者に遠慮なく連絡するか、<a href="https://fastcomments.com/auth/my-account/help" target="_blank">サポートページ</a> をご利用ください。