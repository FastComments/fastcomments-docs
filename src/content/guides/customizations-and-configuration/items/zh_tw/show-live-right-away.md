[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

預設情況下，即時評論功能是啟用的。這表示如果有任何評論被新增、刪除、編輯或置頂，變更會即時顯示給所有正在觀看該評論串的使用者。

然而，預設情況下這些新評論會出現在一個動態顯示的按鈕下，按鈕文字類似於「顯示 2 則新評論」。

如果新評論是直接回覆到頁面，按鈕會顯示在評論串的頂部；如果它們是回覆到特定評論，按鈕則會顯示在該評論之下。

這樣的設計是為了避免頁面大小不斷變化，從而在使用者嘗試抓取捲軸時造成挫折感。

對於某些使用情境，例如即時競標或線上活動，這並非理想的行為——您可能希望評論小工具更像「聊天」框，讓新評論「立即顯示」。

因此，啟用此功能的旗標名稱為：**showLiveRightAway**。

我們可以這樣開啟它：

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = '立即顯示即時評論'; code-example-end]

此設定也可以在小工具自訂頁面上，無需撰寫程式碼即可調整：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='折疊即時評論設定已切換，使新評論立即顯示，而不是在按鈕後面'; title='立即顯示即時評論' app-screenshot-end]