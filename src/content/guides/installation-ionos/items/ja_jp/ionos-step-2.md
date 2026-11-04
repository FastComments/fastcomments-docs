次に、FastCommentsウィジェットのコードをサイトに追加します。このコードは、タイトルが `FastComments Goes Here` であるすべてのフォームを検索し、FastCommentsに置き換えます。

それでは、サイトエディターの左下にある `Settings` に移動しましょう:

<div class="screenshot white-bg">
    <div class="title">設定を開く</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="設定を開く" />
</div>

`Custom Head Code` セクションを開きます:

<div class="screenshot white-bg">
    <div class="title">Custom Head Codeを開く</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Custom Head Codeを開く" />
</div>

Ionosの場合は、FastCommentsウィジェットコードの**特別なバージョン**が必要です。**他のチュートリアルのコードスニペットは動作しません。**

以下のコードをコピーします:

[inline-code-attrs-start title = 'Ionos FastCommentsスニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let loaded = false;
        let interval = null;

        function attemptLoad() {
            const nodes = document.querySelectorAll('h2');

            nodes.forEach(function (node) {
                if (node.innerText && node.innerText.trim() === 'FastComments Goes Here') {
                    // フル幅ではない要素を取得します
                    const target = node.parentNode.parentNode.parentNode.parentNode.parentNode;
                    target.innerHTML = '';
                    FastCommentsUI(target, {
                        tenantId: "demo"
                    });
                    interval && clearInterval(interval);
                    loaded = true;
                }
            });
        }

        attemptLoad();
        if (!loaded) {
            interval = setInterval(attemptLoad, 300);
        }
    })();
</script>
[inline-code-end]

...そして、以下のように貼り付けます:

<div class="screenshot white-bg">
    <div class="title">貼り付けて保存</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="貼り付けて保存" />
</div>