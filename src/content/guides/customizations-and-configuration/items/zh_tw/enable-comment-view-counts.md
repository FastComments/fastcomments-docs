[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 不會追蹤誰檢視了每則評論，也不會提供任何相關統計。

然而，我們可以啟用此功能，系統將會在每位使用者捲動至評論時開始追蹤。

發生此情況時，每則評論旁的眼睛圖示旁的計數會遞增。計數會即時更新，並依使用者的語系進行縮寫。

我們可以透過將 **enableViewCounts** 旗標設為 true 來啟用此功能：

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

此設定可在小工具自訂頁面上，無需撰寫程式碼即可自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='小工具自訂頁面，已勾選檢視次數核取方塊，使每則評論顯示眼睛圖示與計數'; title='啟用評論檢視次數' app-screenshot-end]

我們會追蹤檢視評論的使用者 ID*，保留一週。若在同一週內再次檢視該評論，計數不會遞增。若在一週過後再次檢視，計數將再次遞增。

- *注意：或是匿名會話 ID，或是使用者的 IP（以雜湊值形式）。