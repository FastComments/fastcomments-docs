現在我們可以複製以下的程式碼片段。使用片段右上角出現的複製按鈕。

有幾個你可以在程式碼中設定的項目，請參見第 4 到第 7 行。

[inline-code-attrs-start title = 'Squarespace 單一頁面程式碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // 您的帳戶 ID
    }];
</script>
[inline-code-end]

看起來應該像這樣：

<div class="screenshot white-bg">
    <div class="title">貼上並儲存</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="貼上並儲存" />
</div>

現在在右上角點擊儲存。

請注意，`Preview in Safe Mode` 選項將無法使用，但當你造訪網站時小工具會出現。

如果你遇到問題，請確定靠近底部沒有顯示 `"tenantId": "demo"`。如果你已登入，應該會顯示你的租戶 ID。若沒有，請聯絡支援。