よく見られる症状と、その一般的な解決方法を以下に示します。

### 「This is a demo」メッセージ

これは、デモテナントを使用しているホームページからウィジェットコードをコピーした場合に表示されます。自分のテナントを使用するには、[こちら](https://fastcomments.com/auth/my-account/get-acct-code)からウィジェットコードをコピーしてください。

### 「FastComments cannot load on this domain」エラー

FastComments がアカウントに関連するリクエストを認証するためには、どのドメインがあなた所有かを把握する必要があります。このエラーを解決する方法については、[ドキュメントを参照してください](/guide-multiple-sites.html#add-domains-to-account)（正確なサブドメイン + ドメインをアカウントに追加するだけです）。

なお、これはトライアル期間終了後にのみ発生するはずです。トライアル期間中は、新しいドメインからのリクエストは自動的にアカウントに追加されます。

### カスタムインストールで移行済みコメントが表示されない

通常、これはインポートされたコメントが `Page ID` に紐付いており、URL を渡している場合（または値を渡していない場合、その場合はページ URL がデフォルトになります）に発生します。

[コメントをエクスポート](https://fastcomments.com/auth/my-account/manage-data/export)し、`URL ID` 列（現在は `B` 列）を確認することでデバッグできます。

`URL ID` 列に表示される値が、ウィジェット設定の `urlId` パラメータとして渡している値と同じであることを確認してください。

さらに詳しい説明については、[コメントがページや記事に紐付く仕組みのドキュメント](/guide-customizations-and-configuration.html#url-id)をお読みください。

それでも解決しない場合は、[お問い合わせください](https://fastcomments.com/auth/my-account/help)。

### コメントウィジェットが表示されない

コメントウィジェットが表示されない場合は、Chrome デベロッパーコンソールでエラーを確認してください。

ほとんどの設定ミスの場合、コメントウィジェットは読み込みが可能であれば、少なくともページ上にエラーを表示します。何も表示されない場合は、通常スクリプトエラーの兆候です。

### 期待どおりに設定が機能しない

[Chrome 拡張機能](https://chromewebstore.google.com/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj)を使用して、コメントウィジェットにどの設定が渡されているかを確認してください。それでも解決しない場合は、Chrome 拡張機能の表示内容のスクリーンショットを撮り、[お問い合わせください](https://fastcomments.com/auth/my-account/help)。

### 同じ URL でもハッシュバングが異なるとコメントが表示されない

デフォルトでは、FastComments はコメントが保存される「バケット」としてページ URL を使用します。URL に `#hashbangs` が含まれており、その `#hashbangs` がコメントスレッドを識別する識別子の一部であるべきでない場合は、ハッシュバングの値を単に無視することができます。例：

[inline-code-attrs-start title = 'ハッシュバングを無視する例'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

この変更を行った後、既存のコメントに対して移行を実行する必要があることに注意してください。[その場合は、お問い合わせください。](https://fastcomments.com/auth/my-account/help)

### URL クエリパラメータがウィジェットに影響する

デフォルトでは、FastComments はコメントが保存される「バケット」としてページ URL を使用します。URL に、コメントスレッドを識別する識別子の一部であるべきでないクエリパラメータが含まれている場合は、それらを単に無視することができます。例：

[inline-code-attrs-start title = 'クエリパラメータを無視する'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

この変更を行った後、既存のコメントに対して移行を実行する必要があることに注意してください。[その場合は、お問い合わせください。](https://fastcomments.com/auth/my-account/help)

### メールが受信できない

FastComments では、メール配信を可能な限り確実にするために多大な努力を払っています。しかし、一部のメールプロバイダーは確実に配信するのが難しいことで知られています。fastcomments.com からのメッセージがスパムフォルダにないか確認してください。

[お問い合わせいただければ](https://fastcomments.com/auth/my-account/help)、メールが表示されない理由についてより詳しい情報を提供できる通常は可能です。