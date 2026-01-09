---
下面是用於安裝摘要小工具的純 JavaScript (Vanilla JS) 程式碼。React 函式庫也有這個小工具。

[inline-code-attrs-start title = '摘要小工具安裝'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

該小工具會根據該頁面/網站的對應小工具設定，自動找到要在摘要中顯示的問題。

如果您需要在我們其他未包含該小工具的函式庫中使用它，請提交支援工單，以便我們知道要將其加入。

---