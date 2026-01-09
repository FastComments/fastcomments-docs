A `DomainConfig` object represents configuration for a domain for a tenant.

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = 'ドメイン構成の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** ドメイン（URLではありません）。例えば "fastcomments.com" や "www.example.com"。サブドメインで制限したい場合はサブドメインを含めてもよい。最大1000文字。 **/
    domain: string
    /** メール送信時に使用される送信者名。 **/
    emailFromName?: string
    /** メール送信時に使用される送信元メールアドレス。SPF を設定して、mail.fastcomments.com がこの属性で指定されたドメインとしてメールを送信できるようにしてください。 **/
    emailFromEmail?: string
    /** READONLY。オブジェクトが作成された日時。 **/
    createdAt: string
    /** このドメインに関連するロゴ。メールで使用されます。HTTPS を使用してください。 **/
    logoSrc?: string
    /** このドメインに関連する小さいロゴ。HTTPS を使用してください。 **/
    logoSrc100px?: string
    /** SSO 専用。送信されるすべてのメールのフッターで使用される URL。"[userId]" 変数をサポートします。 **/
    footerUnsubscribeURL?: string
    /** SSO 専用。送信されるすべてのメールで使用されるヘッダー。例として、配信改善のために購読解除関連のヘッダーを設定するのに便利です。この Record の List-Unsubscribe エントリが存在する場合、それは "[userId]" 変数をサポートします。 **/
    emailHeaders?: Record<string, string>
    /** すべての購読解除リンクを無効にします。推奨されません。配信率に悪影響を与える可能性があります。 **/
    disableUnsubscribeLinks?: boolean
    /** DKIM の設定。 **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'DKIM構成の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** DKIM レコードに記載するドメイン名。 **/
    domainName: string
    /** 使用する DKIM キーのセレクタ。 **/
    keySelector: string
    /** 秘密鍵。-----BEGIN PRIVATE KEY----- で始まり -----END PRIVATE KEY----- で終わること。 **/
    privateKey: string
}
[inline-code-end]

### 認証について

ドメイン構成は、どのサイトがあなたのアカウントのFastCommentsウィジェットをホストできるかを決定するために使用されます。これは基本的な形態
の認証であり、ドメイン構成の追加または削除はあなたのFastCommentsインストールの可用性に影響を与える可能性があります
本番環境で。

Don't remove or update the `domain` property of a `Domain Config` for a domain that is currently in use unless disabling that domain is intended.

This has the same behavior as removing a domain from [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Also note that removing a domain from the `My Domains` UI will remove any corresponding configuration for that domain that may have been added via this UI.

### メールのカスタマイズについて

The unsubscribe link in the email footer, and the one-click-unsubscribe feature offered by many email clients, can be configured via this API by defining `footerUnsubscribeURL` and `emailHeaders`, respectively.

### DKIMについて

After defining your DKIM DNS records, simply update the DomainConfig with your DKIM configuration using the defined structure. 

---