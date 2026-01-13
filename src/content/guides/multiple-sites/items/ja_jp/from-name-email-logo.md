時々、FastComments はユーザーにメールを送信する必要があります。特に Secure SSO を使用していない場合はそうです。

これには、アカウントの確認や初めてコメントする際のアクティビティの確認などが含まれます。FastComments はまた、コメントへの返信に関する通知をユーザーに送信します。

FastComments がユーザーにメールを送信する際、送信元の名前とメールアドレスのデフォルトは `FastComments Robot` と `noreply@fastcomments.com` になります。

これらのメールのフッターには当社のロゴも使用されます。

FastComments Flex または Pro をお持ちの場合、これらはすべて「My Domains page」からドメインごとにカスタマイズできます:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

メールに表示されるロゴをカスタマイズする場合、アップロードするサイズがメールのフッターに表示したいサイズと同じであることを確認してください。

### `From Domain` をカスタマイズする場合

`From Domain` をカスタマイズする場合、Email プロバイダとクライアントは FastComments があなたに代わってメールを送信することを許可されていると認識する必要があります。そうしないと、
`From Domain` を定義しても以下の手順に従わないことで、メールがスパム扱いになる可能性が高くなります。

#### 1. SPF の設定

FastComments があなたのドメインとして安全にメールを送信できるようにするには、当社が送信を許可されるよう SPF レコードを追加してください。

`mail.fastcomments.com` と `sib.fastcomments.com` があなたのドメインとしてメールを送信できるようにする SPF レコードがあることを確認してください。

これを行う方法に関する追加情報はこちらです: https://mailtrap.io/blog/multiple-spf-records/

#### 2. DKIM の設定

SPF に加えて、DKIM を設定するべきです。DNS 構成が整ったら、ドメイン構成ページで「Show Advanced」をクリックして、ドメインごとの DKIM 設定を表示できます。

また、[invoke the API](/guide-api.html#domain-config-structure) して DKIM 構成を設定することもできます。

### 退会リンク

SSO を使用している場合、メールおよび通知で使用される退会機能は [DomainConfigs API](/guide-api.html#domain-config-structure) 経由でカスタマイズできます。