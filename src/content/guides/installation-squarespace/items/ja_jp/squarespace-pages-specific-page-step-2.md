次に、以下のコードスニペットをコピーします。スニペットの右上に表示されるコピーボタンを使用してください。

コード内で設定できる項目がいくつあります。4行目から7行目を参照してください。

[inline-code-attrs-start title = 'Squarespace 単一ページ用コード'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

以下のようになります:

<div class="screenshot white-bg">
    <div class="title">貼り付けて保存</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="貼り付けて保存" />
</div>

次に、右上の保存をクリックします。

`Preview in Safe Mode` オプションは機能しませんが、サイトにアクセスするとウィジェットが表示されます。

問題が発生した場合は、下部付近に `"tenantId": "demo"` と表示されていないことを確認してください。ログインしていれば、あなたのテナントIDが表示されているはずです。表示されていない場合は、サポートにお問い合わせください。