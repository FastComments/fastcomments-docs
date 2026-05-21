FastCommentsをShopify App Storeからインストールした場合、ショップのドメインはテナントの許可済みドメインに自動で追加されるため、ドメインエラーは発生しないはずです。このページは、手動でインストールした場合、またはストアフロントがアプリのインストール時にShopifyに登録されていなかったカスタムドメインで配信されている場合に該当します。

次のような認証エラーが表示されることがあります:

<div class="screenshot white-bg">
    <div class="title">ドメイン構成が見つかりません</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-webflow-step-5.png" alt="ドメイン構成が見つかりません" />
</div>

これは、ウィジェットが読み込まれているドメインがテナントで許可されているドメインとしてFastCommentsに認識されていないことを意味します。

解決するには、ドメインをFastCommentsアカウントに追加してください: [ドメインの構成](https://fastcomments.com/auth/my-account/configure-domains).