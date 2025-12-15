よく見られる症状と一般的な解決策を以下に示します。

### 「This is a demo」メッセージ

これは、デモテナントを使用している私たちのホームページからウィジェットコードをコピーした場合に表示されます。あなたのテナントを使用するには、[こちら](https://fastcomments.com/auth/my-account/get-acct-code)からウィジェットコードをコピーしてください。

### 「FastComments cannot load on this domain」エラー

FastCommentsは、アカウントに関連するリクエストを認証するために、どのドメインがあなたのものかを知る必要があります。このエラーを解決する方法については、[ドキュメントをご覧ください](/guide-multiple-sites.html#add-domains-to-account)（単にアカウントに正確なサブドメイン+ドメインを追加するだけです）。

これはトライアル期間が終了した後にのみ発生するはずです。トライアル期間中、新しいドメインからのリクエストは自動的にアカウントに追加されます。

### カスタムインストールで移行したコメントが表示されない

通常、これはインポートされたコメントが`Page ID`に関連付けられており、URLを渡している場合（または値を渡さない場合、デフォルトでページURLが使用される）に発生します。

これをデバッグするには、[コメントをエクスポート](https://fastcomments.com/auth/my-account/manage-data/export)して`URL ID`列（現在は列`B`）を確認します。

`URL ID`列に表示される値が、ウィジェット設定に`urlId`パラメータとして渡している値と同じであることを確認してください。

詳細については、[コメントがページや記事にどのように関連付けられるかのドキュメント](/guide-customizations-and-configuration.html#url-id)をお読みください。

すべてがうまくいかない場合は、[お問い合わせください](https://fastcomments.com/auth/my-account/help)。

### コメントウィジェットが表示されない

コメントウィジェットが表示されない場合は、Chromeデベロッパーコンソールでエラーを確認してください。

ほとんどの設定ミスでは、コメントウィジェットは読み込みができる場合、少なくともページにエラーを表示します。何も表示されない場合は、通常スクリプトエラーの兆候です。

### 希望する設定が期待通りに動作しない

コメントウィジェットにどの設定が渡されているかを確認するには、[Chrome拡張機能](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US)をお試しください。すべてがうまくいかない場合は、Chrome拡張機能が表示する内容のスクリーンショットを撮って[お問い合わせください](https://fastcomments.com/auth/my-account/help)。

### 異なるハッシュバングを持つ同じURLでコメントが見つからない

デフォルトでは、FastCommentsはコメントが保存される「バケット」としてページURLを使用します。URLに`#hashbangs`が含まれており、これらの`#hashbangs`がコメントスレッドを識別する識別子の一部であるべきでない場合、ハッシュバング値を単に無視できます。例：

[inline-code-attrs-start title = 'ハッシュバングを無視する例'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

この変更を行った後、既存のコメントの移行が必要になります。[それについてはお問い合わせください。](https://fastcomments.com/auth/my-account/help)

### URLクエリパラメータがウィジェットに影響を与える

デフォルトでは、FastCommentsはコメントが保存される「バケット」としてページURLを使用します。URLにコメントスレッドを識別する識別子の一部であるべきでないクエリパラメータが含まれている場合、それらを単に無視できます。例：

[inline-code-attrs-start title = 'クエリパラメータを無視する'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

この変更を行った後、既存のコメントの移行が必要になります。[それについてはお問い合わせください。](https://fastcomments.com/auth/my-account/help)

### メールを受信できない

FastCommentsでは、メールの配信をできるだけ信頼性の高いものにするために多くの努力を払っています。しかし、一部のメールプロバイダーは信頼性の高い配信が非常に困難なことで知られています。fastcomments.comからのメッセージがないか、スパムフォルダーを確認してください。

[お問い合わせ](https://fastcomments.com/auth/my-account/help)いただければ、なぜ私たちからのメールが届かないのかについて、より多くの情報を提供できることがあります。
