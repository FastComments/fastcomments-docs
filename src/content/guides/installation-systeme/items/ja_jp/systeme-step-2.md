これからコードをコピーします。コードスニペットの6行目が `tenantId: "demo"` と表示されている場合は、FastCommentsのアカウントにログインしてください
そしてこのページを更新して、コピーしたコードスニペットにあなたのアカウントIDが反映されるようにしてください。

[inline-code-attrs-start title = 'Systeme.io スニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]

これをエディタに貼り付けて保存をクリックしてください:

<div class="screenshot white-bg">
    <div class="title">FastCommentsコードを追加</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="FastCommentsコードを追加" />
</div>

... その後サイトを保存してください。以上です！