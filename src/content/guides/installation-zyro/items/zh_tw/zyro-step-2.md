現在讓我們加入我們的小工具程式碼。

複製以下程式碼。請確定您已登入 [fastcomments.com](https://fastcomments.com) ，如果未登入請重新載入此頁面，這樣程式碼才會自動填入您的帳戶資訊，否則會顯示示範程式碼。

現在複製程式碼：

[inline-code-attrs-start title = 'Zyro 評論程式碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

現在回到我們的網站建置工具並點擊 `Enter code`：

<div class="screenshot white-bg">
    <div class="title">輸入程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="輸入程式碼" />
</div>

### 注意！

請務必使用上面的程式碼，而不要使用其他文件中的程式碼片段，因為此程式碼片段是專為 Zyro 特別設計的。

您現在應該會看到類似下方的畫面，看起來是空白的。這是預期中的情形。將滑鼠移到小工具應顯示的區域上：

<div class="screenshot white-bg">
    <div class="title">已新增程式碼小工具</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="已新增程式碼小工具" />
</div>

現在拖曳小工具以調整到所需大小，您會看到它出現：

<div class="screenshot white-bg">
    <div class="title">調整大小</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="調整大小" />
</div>

...然後預覽並儲存！