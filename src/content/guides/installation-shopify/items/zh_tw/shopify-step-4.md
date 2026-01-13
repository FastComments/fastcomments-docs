---
接下來我們要捲動到第 `100` 行:

<div class="screenshot white-bg">
    <div class="title">捲動到第 100 行</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="捲動到第 100 行" />
</div>

現在複製以下程式碼片段，此片段是**專為 Shopify 設計 - 不要使用其他教學的程式碼片段**:

[inline-code-attrs-start title = 'Shopify FastComments 程式碼片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

現在我們要把游標放在第 `101` 行 - 就在 `</div>` 之後 - 然後貼上。你應該會看到類似的畫面：

<div class="screenshot white-bg">
    <div class="title">加入 FastComments 程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="加入 FastComments 程式碼" />
</div>

現在我們可以儲存:

<div class="screenshot white-bg">
    <div class="title">儲存</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="儲存" />
</div>

---