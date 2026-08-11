---
[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 只會在評論串中的使用者評論上顯示使用者徽章。

然而，我們可以透過在小工具自訂頁面啟用此功能，將使用者徽章顯示在評論表單上方、名稱旁邊：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='在小工具自訂頁面上，顯示頂部列的徽章勾選框，將徽章放在評論表單上方的名稱旁邊'; title='在頂部列顯示徽章選項' app-screenshot-end]

這將在頂部列區域中，將使用者的徽章與其名稱並排顯示，讓他們在撰寫評論時，其成就與身分更加顯眼。

請注意，此功能必須在小工具自訂 UI 中啟用才能生效。您也可以在程式碼設定中選擇性地將 **showBadgesInTopBar** 標誌設為 false，即使在伺服器層面已開啟，也能將其停用：

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
---