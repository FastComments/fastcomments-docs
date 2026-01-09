現在讓我們加入小工具程式碼。

複製下面的程式碼。你會想確保已登入 [fastcomments.com](https://fastcomments.com) 
並在未登入時重新載入此頁面，這樣程式碼才會自動填入你的帳戶資訊，否則它會顯示示範程式碼。

現在複製程式碼：

[inline-code-attrs-start title = 'Hostinger 評論程式碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

現在回到我們的網站編輯器並點擊 `Enter code`:

<div class="screenshot white-bg">
    <div class="title">輸入程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="輸入程式碼" />
</div>

### 注意！

重要的是你必須使用上述程式碼，而不是其他文件中的程式碼片段，因為此片段是專為 Hostinger 所製作。

你現在應該會看到類似下方的畫面，顯示為空白。這是預期的行為。將滑鼠移到小工具應該顯示的位置：

<div class="screenshot white-bg">
    <div class="title">已新增程式碼小工具</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="已新增程式碼小工具" />
</div>

現在拖曳小工具至所需大小，你會看到它出現：

<div class="screenshot white-bg">
    <div class="title">調整大小</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="調整大小" />
</div>

...接著預覽並儲存！