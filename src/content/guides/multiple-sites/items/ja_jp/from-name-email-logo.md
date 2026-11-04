FastCommentsでは、特にSecure SSOを使用していない場合、ユーザーにメールを送信する必要があることがあります。

その例として、初めてコメントする際のアカウントやアクティビティの確認が挙げられます。また、FastCommentsはコメントへの返信の通知も送信します。

FastCommentsがユーザーにメールを送信する際、デフォルトのFrom NameとEmailとして `FastComments Robot` と `noreply@fastcomments.com` を使用します。

また、これらのメールのフッターには当社のロゴを使用します。

FastComments FlexまたはProをお持ちの場合、これらはすべて「My Domainsページ」からドメインごとにカスタマイズできます:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; alt='From Name、From Email、ロゴアップロードフィールドを含むドメインごとのメール設定フォーム'; title='From Name、Email、ロゴのカスタマイズ' app-screenshot-end]

メールに表示されるロゴをカスタマイズする際は、アップロードするサイズがメールのフッターに表示したいサイズと同じであることを確認してください。

### `From Domain` をカスタマイズする場合

`From Domain` をカスタマイズする場合、メールプロバイダーやクライアントは、FastCommentsがあなたに代わってメールを送信する権限を持っていることを認識する必要があります。そうでない場合、`From Domain` を設定しても以下の手順に従わないと、メールがスパムに分類される可能性が高くなります。

#### 1. SPFの設定

FastCommentsがあなたのドメインとして安全にメールを送信できるようにするには、それを許可するSPFレコードを追加してください。

`mail.fastcomments.com` と `sib.fastcomments.com` があなたのドメインとしてメールを送信できるようにするSPFレコードが存在することを確認してください。

これを行う方法の詳細については、こちらをご覧ください: https://mailtrap.io/blog/multiple-spf-records/

#### 2. DKIMの設定

SPFに加えて、DKIMも設定することをお勧めします。DNS設定が完了したら、ドメイン設定ページで「Show Advanced」をクリックすると、ドメインごとのDKIM設定が表示されます。

また、[APIを呼び出して](/guide-api.html#domain-config-structure)DKIM設定を行うこともできます。

### 配信停止リンク

SSOを使用している場合、メールや通知で使用される配信停止機能は、[DomainConfigs API経由で](/guide-api.html#domain-config-structure)カスタマイズできます。

### メールリンクの難読化

サイトのドメイン評価により通知メールがスパムに分類されている場合、「コメントを表示」ボタンのリンクを直接ページに張る代わりに、`fastcomments.com` を経由するようにルーティングできます。メールボックスプロバイダーは、メール本文内のすべてのリンクをリンク先の評価に対してスコアリングするため、送信設定がどれほどクリーンであっても、ドメインがフラグされている場合、そのままのリンクはスパムスコアに寄与してしまいます。

これは、My Domainsページの「Show Advanced」にある「Email Link Obfuscation」セクションで有効にできます。この設定はドメインごとです。

有効にすると、メンション、返信、新規コメント、購読ページ、プロフィールコメント、ダイジェストの各メール内のリンクは、クリック時に元のページへリダイレクトする短いトークンに書き換えられます。リンク先はテナントに紐付けられます。リダイレクトは、設定済みのドメインのいずれかとホストが一致するURLにのみ転送され、トークンは30日後に自動的に失効します。

クリックスルー後の体験は変わりません。読者は引き続き、コメントが表示された状態であなたのページに到着します。