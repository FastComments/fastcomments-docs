次に、`100` 行までスクロールします:

<div class="screenshot white-bg">
    <div class="title">100行までスクロール</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="100行までスクロール" />
</div>

Now copy the following code snippet, which is designed **specifically for Shopify - do not use code snippets from other tutorials**:

[inline-code-attrs-start title = 'Shopify 用 FastComments スニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        urlId: window.location.pathname
    });
</script>
[inline-code-end]

次に、カーソルを`line 101` - `</div>` の直後 - に置いて貼り付けます。次のようになっているはずです:

<div class="screenshot white-bg">
    <div class="title">FastComments コードを追加</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="FastComments コードを追加" />
</div>

これで保存できます:

<div class="screenshot white-bg">
    <div class="title">保存</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="保存" />
</div>

---