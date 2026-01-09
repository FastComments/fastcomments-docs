---
クリックしたのは、先ほど追加した HTML 要素です。表示されるプロパティエディタの HTML フィールドに、次のコードを貼り付けてください:

[inline-code-attrs-start title = 'Bubble.io ライブコメント用コードスニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubbleはスニペットを非同期に変更する傾向があります
            const container = document.getElementById('fastcomments-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsUI) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsUI(container, {
                tenantId: 'demo',
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
            container.fastCommentsSetup = true;
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">FastCommentsコードを挿入</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="HTML要素にFastCommentsコードを挿入" />
</div>

Note: このコードには、Bubbleの動的な環境でFastCommentsが正しく読み込まれることを保証するための再試行メカニズムが含まれています。
他のコードスニペットは動作しません。

サインアップ後、'demo' を実際の FastComments テナント ID に置き換えることを忘れないでください。FastComments.com にログインしている場合は、すでに置き換えられているはずです。

---