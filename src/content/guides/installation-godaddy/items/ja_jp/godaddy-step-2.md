カスタムHTMLブロックを追加したので、FastCommentsウィジェットのコードを追加できます。

**以下のコードはGodaddy用です。他のチュートリアルのコードではなく、このコードはGodaddy専用です。**

以下のコードをコピーしてください：

[inline-code-attrs-start title = 'Godaddy コメントコードスニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

この特定のコードスニペットは Godaddy と互換性があるように設計されており、ホームページではなくブログ投稿にのみ表示されます。

次に、コードを `Step One` で述べた `Custom Code` エリアに貼り付けます。

<div class="screenshot white-bg">
    <div class="title">コードをコピーして貼り付け</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="コードをコピーして貼り付け" />
</div>

右上の「Done」をクリックしてください：

<div class="screenshot white-bg">
    <div class="title">「Done」をクリック</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="「Done」をクリック" />
</div>

これでステップ2は完了です！