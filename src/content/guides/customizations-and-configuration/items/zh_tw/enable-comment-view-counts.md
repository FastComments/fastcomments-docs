[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 不會追蹤誰查看了每則留言，也不會提供相關統計資料。

不過，我們可以啟用此功能，系統會在每次使用者捲動到留言時開始追蹤。

當發生這種情況時，每則留言旁邊的眼睛圖示旁的計數會增加。該計數會即時更新，並根據使用者的地區設定進行縮寫。

我們可以透過將 **enableViewCounts** 標誌設為 true 來啟用此功能：

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

這可以在小工具（widget）自訂頁面上免程式碼地進行自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

我們會追蹤查看留言的使用者 id*，因此如果您再次查看該留言，計數不會增加。如果您再次查看該留言
在兩年後，計數將會增加。

- *注意：或匿名 session id，或使用者的 IP（以雜湊值形式）。