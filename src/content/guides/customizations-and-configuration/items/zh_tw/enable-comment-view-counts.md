---
[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

預設情況下，FastComments 不會追蹤誰檢視了每則評論，也不會提供相關統計資料。

然而，我們可以啟用此功能，系統將會在每位使用者捲動至評論時開始追蹤。

發生此情況時，每則評論旁的眼睛圖示旁會顯示一個計數，該計數會即時更新，並依使用者的語系顯示縮寫形式。

我們可以透過將 **enableViewCounts** 旗標設為 true 來啟用此功能：

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

此設定可在 widget 自訂頁面上，無需撰寫程式碼即可自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='已勾選檢視次數核取方塊的 Widget 自訂頁面，讓每則評論顯示眼睛圖示與計數'; title='啟用評論檢視次數' app-screenshot-end]

我們會追蹤檢視評論的使用者 ID*，因此若您再次檢視同一則評論，計數不會再次遞增。若在兩年後再次檢視，計數將會再次遞增。

- *注意：或匿名會話 ID，或使用者的 IP（雜湊值）。
---