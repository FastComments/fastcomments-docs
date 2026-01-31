現在我們來新增小工具程式碼。

複製下面的程式碼。請確定你已登入 [fastcomments.com](https://fastcomments.com) 並在未登入時重新載入此頁面，這樣程式碼會自動填入你的帳戶資訊，否則會顯示範例程式碼。

現在複製程式碼：

[inline-code-attrs-start title = 'Hostinger 評論程式碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    }];
</script>
[inline-code-end]

現在回到我們的網站編輯器並點選 `Enter code`：

<div class="screenshot white-bg">
    <div class="title">輸入程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="輸入程式碼" />
</div>

### 注意！

使用上方程式碼非常重要，不要使用其他文件中的程式碼片段，因為此程式碼片段是專為 Hostinger 精心製作的。

你現在應該會看到類似下方的畫面，看起來是空白的。這是預期的。將滑鼠移到小工具應出現的區域上：

<div class="screenshot white-bg">
    <div class="title">已新增程式碼小工具</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="已新增程式碼小工具" />
</div>

現在拖動小工具到想要的大小，你會看到它出現：

<div class="screenshot white-bg">
    <div class="title">調整大小</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="調整大小" />
</div>

...然後預覽並儲存！