[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

在評論小工具頂部顯示的評論計數可以自訂。

這可以被任何字串取代，且 **[count]** 會被替換為使用者本地化的計數值。

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = '自訂評論計數文字'; code-example-end]

這可以在小工具自訂頁面上，無需程式碼即可自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='小工具自訂頁面上的評論計數文字欄位，會以即時總數取代 [count]'; title='自訂評論計數文字' app-screenshot-end]