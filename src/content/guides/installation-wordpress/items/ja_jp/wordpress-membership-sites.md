FastCommentsは、SSO、つまりシングルサインオンを使用して会員専用サイトに対応します。あなたの会員はWordPressサイトにサインインしますが、  
コメントするためにFastCommentsのアカウントを作成したり、ソーシャルメディアでログインしたりする必要はありません。会員（管理者を含む）がログインしている  
WordPressサイトでは、コメントができるようになります。管理者やモデレーターは、WordPressのブログ記事から直接コメントをモデレート（管理）することもできます。

<sup>(オプション)</sup> 管理者を [User & Administrators](https://fastcomments.com/auth/my-account/users) に、モデレーターを [Comment Moderators](https://fastcomments.com/auth/my-account/moderate-comments/moderators) に追加しておくと、彼らの体験が向上し、モデレーターの統計トラッキングが有効になります。

SSOはプラグインのダッシュボードに移動して「SSO Settings」をクリックすることで有効にできます。

まずサイトの「Anyone can Register」機能を有効にする必要があります。

ユーザーがコメントスレッドを表示するたびに、すべてのユーザー情報はシームレスかつ安全に [HMAC](https://en.wikipedia.org/wiki/HMAC) を通じて FastComments に転送されます。

メンバーをFastCommentsのサーバーにコピーするための初期または継続的な同期を実行する必要はありません。これは、ユーザーがブログ記事を開いてコメントスレッドを表示したときに自動的に行われます。

## 名前とアバター

プラグインは、ユーザーが任意のコメントスレッドを表示した際に、FastComments内のそのユーザーが行ったすべてのコメントの表示名とアバターを自動的に更新します。アバターはGravatarまたはWordPress内の任意のアバター管理プラグインを通じてサポートされます。プラグインは `get_avatar_url` を呼び出します。