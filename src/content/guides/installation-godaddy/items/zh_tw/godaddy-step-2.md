---
既然您已新增自訂 HTML 區塊，我們就可以加入 FastComments 小工具的程式碼。

**請使用下列程式碼以適用於 Godaddy，請勿使用其他教學的程式碼。此程式碼為 Godaddy 專用。**

複製下列程式碼：

[inline-code-attrs-start title = 'Godaddy 評論程式碼片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

此程式碼片段專為與 Godaddy 相容而設計，並且僅會顯示在您的部落格文章頁面，而不會在首頁顯示。

現在將程式碼貼到在 `Step One` 提到的 `Custom Code` 區域。

<div class="screenshot white-bg">
    <div class="title">複製並貼上程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="複製並貼上程式碼" />
</div>

在右上角按下「完成」：

<div class="screenshot white-bg">
    <div class="title">按下完成</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="按下完成" />
</div>

第二步就完成了！

---