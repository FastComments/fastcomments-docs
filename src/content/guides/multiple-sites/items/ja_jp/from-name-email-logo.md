Sometimes FastComments has to email your users, especially if you are not using Secure SSO.

場合によっては、FastComments がユーザーにメールを送信する必要があります。特に Secure SSO を使用していない場合にそうなります。

Examples of this includes verifying their account or activity when commenting for the first time. FastComments
will also send them notifications for replies to their comments.

これには、初めてコメントする際のアカウントやアクティビティの確認といった例が含まれます。FastComments はまた、コメントへの返信に対する通知も送信します。

When FastComments emails your users, we will use a default From Name and Email of `FastComments Robot` and `noreply@fastcomments.com`.

FastComments がユーザーにメールを送信する場合、デフォルトの差出人名とメールアドレスとして `FastComments Robot` と `noreply@fastcomments.com` を使用します。

We'll also use our own logo in the footer of these emails.

これらのメールのフッターには当社のロゴも使用されます。

If you have FastComments Flex or Pro, this all can be customized on a per-domain basis via the "My Domains page":

FastComments Flex または Pro をご利用の場合、これらはすべてドメインごとに「My Domains」ページでカスタマイズできます：

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

When customizing the logo shown in emails, ensure that the size you are uploading is the same size that you want to show in the footer of the email.

メールに表示されるロゴをカスタマイズする際は、アップロードするサイズがメールのフッターに表示したいサイズと同じであることを確認してください。

### `From Domain` をカスタマイズする場合

If you customize the `From Domain`, Email providers and clients need to know that FastComments is authorized to send emails on your behalf. Otherwise,
defining the `From Domain` and not following the below steps likely will result in emails going to spam.

`From Domain` をカスタマイズする場合、メールプロバイダーやクライアントは FastComments があなたに代わってメールを送信する権限があることを知っている必要があります。そうでないと、`From Domain` を定義しても以下の手順に従わないとメールが迷惑メールに分類される可能性が高くなります。

#### 1. SPFの設定

To allow FastComments to securely send email as your domain, ensure you add an SPF record that allows us to do so.

FastComments があなたのドメインとして安全にメールを送信できるようにするため、当社が送信できるようにする SPF レコードを追加してください。

Ensure there are SPF records to allow `mail.fastcomments.com` and `sib.fastcomments.com` to send mail as your domain.

`mail.fastcomments.com` および `sib.fastcomments.com` があなたのドメインとしてメールを送信できるようにする SPF レコードがあることを確認してください。

Some more information on how to do this is here: https://mailtrap.io/blog/multiple-spf-records/

この設定方法の詳細は次をご覧ください: https://mailtrap.io/blog/multiple-spf-records/

#### 2. DKIMの設定

In addition to SPF, you should set up DKIM. Once your DNS configuration is ready, you can click "Show Advanced" in the domain configurations page
to show the DKIM settings per-domain.

SPF に加えて、DKIM も設定する必要があります。DNS の設定が完了したら、ドメイン構成ページで「詳細を表示」をクリックしてドメインごとの DKIM 設定を表示できます。

You can also [invoke the API](/guide-api.html#domain-config-structure) to set DKIM configuration.

DKIM 設定は、[API を呼び出す](/guide-api.html#domain-config-structure) ことで行うこともできます。

### Unsubscribe Links

SSOを使用している場合、メールや通知で使用される購読解除機能は[DomainConfigs API](/guide-api.html#domain-config-structure) を使用してカスタマイズできます。

### メールリンクの難読化

If your site's domain reputation is causing notification emails to land in spam, you can route the "view comment" buttons through `fastcomments.com` instead of linking directly to your page. Mailbox providers score every link in the email body against the destination's reputation, so when your domain is being flagged the bare links contribute to the spam score regardless of how clean your sending setup is.

サイトのドメイン評価が原因で通知メールが迷惑メールに振り分けられる場合、ページへ直接リンクする代わりに「コメントを見る」ボタンを `fastcomments.com` 経由でルーティングすることができます。メールプロバイダーはメール本文内のすべてのリンクをリンク先の評価に基づいて評価するため、ドメインがフラグされている場合は、送信設定がどれだけ適切でも生のリンクが迷惑メールスコアに影響を与えます。

Enable this under "Show Advanced" on the My Domains page, in the "Email Link Obfuscation" section. The setting is per-domain.

この設定は、My Domains ページの「詳細を表示」にある「メールリンクの難読化」セクションで有効にできます。設定はドメインごとに行われます。

When enabled, links in mention, reply, new-comment, subscribed-page, profile-comment, and digest emails are rewritten to short tokens that redirect to the original page on click. The destination is bound to your tenant: the redirect only forwards to URLs whose host matches one of your configured domains, and tokens auto-expire after 30 days.

有効にすると、mention、reply、new-comment、subscribed-page、profile-comment、および digest メール内のリンクは短いトークンに書き換えられ、クリック時に元のページへリダイレクトされます。リダイレクト先はあなたのテナントに紐づいており、リダイレクトは設定済みのドメインのホストと一致する URL のみを転送し、トークンは30日後に自動で失効します。

The clicked-through experience is unchanged. Readers still land on your page with the comment scrolled into view.

クリック後の体験は変更されません。読者はコメントがビュー内にスクロールされた状態であなたのページに到達します。