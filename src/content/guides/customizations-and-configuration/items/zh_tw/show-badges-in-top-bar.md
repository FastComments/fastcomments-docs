[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 只會在留言串中的使用者留言旁顯示使用者徽章。

不過，我們可以在 widget 自訂頁面啟用此功能，將使用者徽章顯示在留言表單上方名字旁：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

這會在頂部工具列區域於使用者名字旁顯示其徽章，讓他們在撰寫留言時的成就與身分更為顯眼。

請注意，必須在小工具自訂介面中啟用此功能才能生效。即使在伺服器層級已開啟，你也可以在程式碼設定中將 **showBadgesInTopBar** 標記設為 false，以選擇性地停用它：

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]