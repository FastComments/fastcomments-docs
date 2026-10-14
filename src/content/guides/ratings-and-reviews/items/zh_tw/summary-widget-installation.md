以下是安裝摘要小工具的 Vanilla JS 程式碼。React 函式庫也提供此小工具。

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

此小工具會自動根據該頁面/站點的相應小工具設定，找出要在摘要中顯示的問題。

### 依評分篩選評論

當同一頁面的評論小工具也在頁面上，且已設定在評論中顯示每位評論者的答案時，分解表的列會變成可點擊的。點擊某一列，例如「5」，會將評論小工具篩選為給予該答案的評論。再次點擊該列，或在評論小工具中的「僅顯示」標籤上的 X，則會重新顯示所有評論。此功能不需要任何設定：摘要小工具會尋找具有相同 `tenantId` 和 `urlId` 的評論小工具。

此功能適用於星級、拇指、表情符號、量表以及單選問題。評論小工具在評論框上方顯示的分解表也可以以相同方式點擊。

如果您需要在我們其他未提供此功能的函式庫中使用此小工具，請提交支援工單以告知我們加入此功能。