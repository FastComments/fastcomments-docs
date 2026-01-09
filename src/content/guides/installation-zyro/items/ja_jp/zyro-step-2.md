ではウィジェットのコードを追加しましょう。

下のコードをコピーしてください。コードがアカウント情報で事前入力されるように、[fastcomments.com](https://fastcomments.com) にサインインしていることを確認し、そうでない場合はこのページをリロードしてください。そうしないとデモコードが表示されます。

ではコードをコピーします：

[inline-code-attrs-start title = 'Zyro コメントコード'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

ではサイトビルダーに戻り、`Enter code` をクリックします：

<div class="screenshot white-bg">
    <div class="title">コードを入力</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="コードを入力" />
</div>

### 注意！

上記のコードを使用し、他のドキュメントのコードスニペットを使用しないことが重要です。このスニペットはZyro向けに特別に作成されています。

次のように何も表示されていない状態になっているはずです。これは予想された動作です。ウィジェットが表示されるはずの領域にマウスを移動してください：

<div class="screenshot white-bg">
    <div class="title">コードウィジェットが追加されました</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="コードウィジェットが追加されました" />
</div>

ウィジェットを目的のサイズになるまでドラッグすると、表示されます：

<div class="screenshot white-bg">
    <div class="title">サイズを変更</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="サイズを変更" />
</div>

...そしてプレビューして保存してください！