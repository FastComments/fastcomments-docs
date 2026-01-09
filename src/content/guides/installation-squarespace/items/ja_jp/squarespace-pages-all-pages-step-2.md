次に、以下のコードスニペットをコピーできます。スニペットの右上に表示されるコピー ボタンを使用してください。

コード内で設定できる項目がいくつかあります。4行目から7行目を参照してください。

[inline-code-attrs-start title = 'Squarespace すべてのページのコメントコード'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // あなたのアカウントID

        function tryLoad() {
            // 異なるレイアウトに対応して読み込もうとする
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

その後、コードエリアに貼り付けて保存をクリックします。`FOOTER` ブロックにコードが入っているはずです。次のように表示されます:

<div class="screenshot white-bg">
    <div class="title">貼り付けて保存</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="貼り付けて保存" />
</div>

問題が発生している場合は、下部付近に `"tenantId": "demo"` と表示されていないことを確認してください。ログインしている場合は、そこにあなたの tenant id が表示されるはずです。表示されていない場合は、サポートにお問い合わせください。