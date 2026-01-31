ここではよく見られるいくつかの症状と一般的な解決策を示します。 

### "This is a demo" Message

これは、ホームページからウィジェットコードをコピーした場合に表示されます。ホームページではデモ用の
テナントが使用されています。ご自身のテナントを使用するには、[こちら](https://fastcomments.com/auth/my-account/get-acct-code)からウィジェットコードをコピーしてください。

### "FastComments cannot load on this domain" Error

FastComments は、アカウントに関連するリクエストを認証するために、どのドメインがあなたの所有であるかを知る必要があります。
このエラーを解決する方法については[ドキュメントを確認してください](/guide-multiple-sites.html#add-domains-to-account)（アカウントに正確なサブドメイン＋ドメインを追加するだけです）。

このエラーは試用期間が終了した後にのみ発生する点に注意してください。試用期間中は、新しいドメインからのリクエストは
自動的にあなたのアカウントに追加されます。

### Migrated Comments Not Showing for Custom Installations

通常、インポートされたコメントが `Page ID` に紐付けられており、ウィジェットに URL を渡している（または値を渡していない場合はページの URL がデフォルトで使用されます）ときにこの問題が発生します。

これをデバッグするには、[コメントをエクスポートする](https://fastcomments.com/auth/my-account/manage-data/export) と `URL ID` 列（現在は列 `B`）を確認してください。

`URL ID` 列で表示される値が、ウィジェット設定で `urlId` パラメータとして渡している値と同じであることを確認してください。

詳細については、[コメントがページや記事にどのように紐付けられるかのドキュメント](/guide-customizations-and-configuration.html#url-id) をお読みください。

それでも解決しない場合は、[お問い合わせください](https://fastcomments.com/auth/my-account/help)。

### Comment Widget Not Showing

コメントウィジェットが表示されない場合は、Chrome の開発者コンソールでエラーを確認してください。

ほとんどの設定ミスでは、ウィジェットが読み込める場合はページ上に少なくともエラーを表示します。何も表示されない場合は通常スクリプトエラーの兆候です。

### Desired Configuration Not Working as Expected

ウィジェットに渡されている設定を確認するには、[Chrome 拡張機能](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) をお試しください。すべて失敗した場合は、拡張機能の表示をスクリーンショットに撮って [お問い合わせください](https://fastcomments.com/auth/my-account/help)。

### Comments Missing on Same URL With Different Hash Bang

デフォルトでは、FastComments はコメントが保存される「バケット」としてページの URL を使用します。URL に `#hashbangs` が含まれており、これらの `#hashbangs`
がコメントスレッドを識別する識別子の一部であってはならない場合、ハッシュバングの値を単純に無視することができます。例えば:

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

この変更を行った後、既存のコメントに対してはマイグレーションを実行する必要がある点に注意してください。[その件についてはお問い合わせください。](https://fastcomments.com/auth/my-account/help)

### URL Query Parameters Affecting Widget

デフォルトでは、FastComments はコメントが保存される「バケット」としてページの URL を使用します。URL にスレッドを識別する識別子の一部に含めたくないクエリパラメータが含まれている場合、それらを単純に無視することができます。例えば:

[inline-code-attrs-start title = 'クエリパラメータを無視する例'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

この変更を行った後、既存のコメントに対してはマイグレーションを実行する必要がある点に注意してください。[その件についてはお問い合わせください。](https://fastcomments.com/auth/my-account/help)

### Not Receiving Emails

FastComments では、メール配信の信頼性をできるだけ高めるために多くの努力を払っています。しかし、一部のメールプロバイダは信頼性の高い配信が難しいことで知られています。fastcomments.com からのメッセージが迷惑メールフォルダに入っていないか確認してください。

もし [お問い合わせいただければ](https://fastcomments.com/auth/my-account/help)、なぜメールが届かないかについてさらに詳しい情報を提供できることが多いです。