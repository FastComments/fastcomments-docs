A `DomainConfig` オブジェクトはテナントのドメインに対する設定を表します。

`DomainConfig` オブジェクトの構造は次のとおりです:

[inline-code-attrs-start title = 'ドメイン設定の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** ドメイン（URLではありません）。例: "fastcomments.com" や "www.example.com"。サブドメインで制限したい場合はサブドメインを含めることができます。最大1000文字。 **/
    domain: string
    /** メール送信時に使用される送信者名 (From-Name) 。 **/
    emailFromName?: string
    /** メール送信時に使用される送信者メールアドレス (From-Email) 。この属性で使用するドメインとして mail.fastcomments.com が送信できるよう SPF を設定してください。 **/
    emailFromEmail?: string
    /** 読み取り専用。オブジェクトが作成された日時。 **/
    createdAt: string
    /** このドメインに関連するロゴ。メール内で使用されます。HTTPS を使用してください。 **/
    logoSrc?: string
    /** このドメインに関連する小さめのロゴ。HTTPS を使用してください。 **/
    logoSrc100px?: string
    /** SSO のみ。送信されるすべてのメールのフッターで使用される URL。"[userId]" 変数をサポートします。 **/
    footerUnsubscribeURL?: string
    /** SSO のみ。送信されるすべてのメールで使用されるヘッダー。例えば、配信率改善のために配信解除に関するヘッダーを設定するのに便利です。この Record の List-Unsubscribe エントリ（存在する場合）は "[userId]" 変数をサポートします。 **/
    emailHeaders?: Record<string, string>
    /** すべての配信解除リンクを無効にします。推奨されません。配信率が低下する可能性があります。 **/
    disableUnsubscribeLinks?: boolean
    /** DKIM の構成。 **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'DKIM 設定の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** DKIM レコード内のドメイン名。 **/
    domainName: string
    /** 使用する DKIM キーセレクタ。 **/
    keySelector: string
    /** PEM 形式の公開鍵。GET レスポンスで返されます。 **/
    publicKey: string
    /** @deprecated API レスポンスではもはや返されません。下位互換性のため書き込み時は受け入れられます。 **/
    privateKey?: string
}
[inline-code-end]

### 認証について

ドメイン構成は、どのサイトがあなたのアカウント用の FastComments ウィジェットをホストできるかを決定するために使用されます。これは基本的な認証の一形態であり、ドメイン構成を追加または削除すると、本番環境での FastComments の利用可能性に影響を与える可能性があります。

意図的にそのドメインを無効にするのでない限り、現在使用中のドメインに対して `Domain Config` の `domain` プロパティを削除または更新しないでください。

これは [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains) からドメインを削除するのと同じ動作です。

また、`My Domains` UI からドメインを削除すると、この UI 経由で追加された可能性のあるそのドメインに対応する設定も削除されることに注意してください。

### メールのカスタマイズについて

メールフッターの配信解除リンクや、多くのメールクライアントが提供するワンクリック配信解除機能は、それぞれ `footerUnsubscribeURL` と `emailHeaders` を定義することでこの API を通じて設定できます。

### DKIMについて

DKIM の DNS レコードを定義したら、定義された構造を使って DKIM 構成で DomainConfig を更新するだけです。