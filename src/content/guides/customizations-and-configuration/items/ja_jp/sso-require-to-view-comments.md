FastComments SSO (<a href="#sso">詳細はこちら</a>) により、ユーザーは別のプラットフォームにログインすることなくコメントを投稿できるようになります。

しかし、これだけではコメントスレッドは保護されません。デフォルトではコメントデータは公開されており、ページを閲覧できる誰でもコメントを閲覧できます。

設定を変更することで、管理者または有効なSSOユーザー以外はコメントを取得できないように制限できます。

#### No-Code セットアップ

SSO を設定している場合、<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">カスタマイズルール</a>を作成することで、コメントスレッドの閲覧や操作を防ぐことができます。

SSOで検索すると次のオプションが見つかります：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.require-sso'; selector = '.require-sso'; title='Require SSO To View Comments' app-screenshot-end]

それを有効にして、カスタマイズルールを保存してください。

#### 特定のドメインまたはページのみ保護する

特定のドメインまたはページのみを保護する場合は、カスタマイズルールをそのように設定するだけです。

カスタマイズUIの上部には、Domain と URL ID の2つの入力欄があります。

特定のドメインのみを保護するには、対象のドメインを "domain" フィールドに入力してください。

特定のページを保護するには、ページのURLを "URL ID" フィールドに入力してください。FastComments とカスタム統合している場合は、URLの代わりにここにIDの種類を入力することもできます。

#### セキュリティレベル

SSO を必須にする場合、Simple SSO と Secure SSO のどちらを要求するかを決める必要があります。Simple SSO を要求すると両方が許可されますが、Secure SSO を要求する場合は、APIキーでハッシュされた Secure SSO ペイロードでコンテンツを取得しない限り閲覧できなくなります。

「Require SSO To View Comments」を選択すると、セキュリティレベルのオプションが表示されます。

#### 閲覧以外の保護

このオプションを有効にすると、ユーザーがSSOでログインしていない限り、そのページやドメインへのコメント投稿ができないように保護されます。

#### 注意点

SSO 統合の前にコメントを作成したユーザーは、SSO 統合でログインしない限りそれらのコメントを閲覧できなくなります。