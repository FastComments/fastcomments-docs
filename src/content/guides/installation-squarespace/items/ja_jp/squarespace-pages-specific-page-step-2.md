次に、以下のコードスニペットをコピーできます。スニペットの右上に表示されるコピー ボタンを使用してください。

コード内で設定できる項目がいくつかあります。4行目から7行目を参照してください。

[inline-code-attrs-start title = 'Squarespace 単一ページ用コード'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // あなたのアカウントID

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

このように表示されます:

<div class="screenshot white-bg">
    <div class="title">貼り付けて保存</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="貼り付けて保存" />
</div>

右上の「保存」をクリックしてください。

ただし、`Preview in Safe Mode` オプションは機能しませんが、サイトを訪問するとウィジェットは表示されます。

問題が発生している場合は、下部付近に `"tenantId": "demo"` と表示されていないことを確認してください。ログインしていれば、そこにはあなたのテナントIDが表示されるはずです。表示されない場合はサポートへお問い合わせください。