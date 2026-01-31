以下のコードスニペットをコピーできます。スニペットの右上に表示されるコピー ボタンを使用してください。

コード内で設定できる項目がいくつかあります。4行目から7行目を参照してください。

[inline-code-attrs-start title = 'Squarespace シングルページ用コード'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // あなたのアカウントID
    }];
</script>
[inline-code-end]

このようになります：

<div class="screenshot white-bg">
    <div class="title">貼り付けて保存</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="貼り付けて保存" />
</div>

右上の保存をクリックしてください。

Note that the `Preview in Safe Mode` option will not work, but the widget will appear when you visit your site.

問題が発生している場合は、下部付近に `"tenantId": "demo"` と表示されていないことを確認してください。ログインしていれば、そこにあなたのテナントIDが表示されます。表示されない場合はサポートにお問い合わせください。