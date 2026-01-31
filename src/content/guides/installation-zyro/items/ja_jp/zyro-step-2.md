これでウィジェットのコードを追加しましょう。

下のコードをコピーしてください。アカウント情報でコードが事前入力されるように、[fastcomments.com](https://fastcomments.com) にサインインしていることを確認し、サインインしていない場合はこのページをリロードしてください。そうしないとデモ用のコードが表示されます。

それではコードをコピーしましょう：

[inline-code-attrs-start title = 'Zyro コメントコード'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

それではサイトビルダーに戻り、`Enter code` をクリックしてください：

<div class="screenshot white-bg">
    <div class="title">コードを入力</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="コードを入力" />
</div>

### 注意！

上記のコードを使用し、他のドキュメントのコードスニペットは使用しないでください。このスニペットは Zyro 向けに特別に作成されています。

次のように空白に見える状態になっているはずです。これは正常です。ウィジェットが表示されるエリアにマウスを移動させてください：

<div class="screenshot white-bg">
    <div class="title">コードウィジェットが追加されました</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="コードウィジェットが追加されました" />
</div>

ウィジェットを目的のサイズになるようドラッグすると、表示されるのが確認できます：

<div class="screenshot white-bg">
    <div class="title">サイズを変更</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="サイズを変更" />
</div>

...そしてプレビューして保存してください！