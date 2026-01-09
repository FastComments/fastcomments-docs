---
これからコードをコピーします。もしコードスニペットの6行目に `tenantId: "demo"` と書かれている場合は、FastCommentsのアカウントにログインして
このページをリロードし、コピーされたスニペットにあなたのアカウントIDが反映されるようにしてください。

[inline-code-attrs-start title = 'Systeme.io スニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo"
    });
</script>
[inline-code-end]

これをエディタに貼り付けて、保存をクリックしてください:

<div class="screenshot white-bg">
    <div class="title">FastComments コードを追加</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="FastComments コードを追加" />
</div>

... then save your site. That's it!

---