ウィジェットのコードを追加しましょう。

以下のコードをコピーしてください。コードがあなたのアカウント情報で事前入力されるように、[fastcomments.com](https://fastcomments.com) にサインインしていることを確認し、サインインしていない場合はこのページをリロードしてください。そうしないとデモ用のコードが表示されます。

それではコードをコピーしましょう:

[inline-code-attrs-start title = 'Hostinger コメントコード'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    }];
</script>
[inline-code-end]

サイトビルダーに戻り、`Enter code` をクリックしましょう:

<div class="screenshot white-bg">
    <div class="title">コードを入力</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="コードを入力" />
</div>

### 注意！

上記のコードを使用し、他のドキュメントにあるコードスニペットは使用しないことが重要です。このスニペットは Hostinger 向けに特別に作成されています。

次のように、空白に見える状態になっているはずです。これは想定どおりです。ウィジェットが表示されるエリアにマウスを移動してください：

<div class="screenshot white-bg">
    <div class="title">コードウィジェットが追加されました</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="コードウィジェットが追加されました" />
</div>

ウィジェットを希望のサイズになるようドラッグすると、表示されます：

<div class="screenshot white-bg">
    <div class="title">サイズを調整</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="サイズを調整" />
</div>

…そしてプレビューして保存してください！