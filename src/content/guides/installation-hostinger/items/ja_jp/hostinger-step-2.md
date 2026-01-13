Now let's add our widget code.

以下のコードをコピーしてください。[fastcomments.com](https://fastcomments.com) にサインインしていることを確認し、サインインしていない場合はこのページをリロードしてください。そうすることでコードがアカウント情報で事前に入力されます。サインインしていないとデモ用のコードが表示されます。

ではコードをコピーしましょう：

[inline-code-attrs-start title = 'Hostinger コメントコード'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

サイトビルダーに戻り、`Enter code` をクリックします：

<div class="screenshot white-bg">
    <div class="title">コードを入力</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Enter Code" />
</div>

### 注意！

上記のコードを使用し、他のドキュメントのコードスニペットを使用しないことが重要です。このスニペットはHostinger向けに特別に作成されています。

次のように空白のように見える表示になるはずです。これは正常です。ウィジェットが表示されるエリアにマウスを移動してください：

<div class="screenshot white-bg">
    <div class="title">コードウィジェットが追加されました</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Code Widget Added" />
</div>

ウィジェットをドラッグして希望のサイズに調整すると、表示されます：

<div class="screenshot white-bg">
    <div class="title">サイズを変更</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Resize It" />
</div>

...そして、プレビューして保存してください！