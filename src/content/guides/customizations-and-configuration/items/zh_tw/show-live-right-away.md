[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

預設情況下，會即時顯示評論功能已啟用。這表示如果有任何評論被新增、刪除、編輯或釘選，變更會同時出現在所有正在檢視該評論串的使用者畫面上。

不過，預設情況下那些新評論會出現在一個動態顯示的按鈕之下，該按鈕上的文字類似 "顯示 2 則新評論"。

如果新評論是對該頁面的直接回覆，按鈕會顯示在評論串的頂部。如果它們是對某則特定評論的回覆，按鈕會顯示在該評論之下。

這樣做是為了避免頁面大小持續變動，避免使用者在嘗試抓取捲軸時產生挫折感。

但對於某些使用情境，例如即時競標或線上活動，這並不是理想的行為 - 你可能希望評論小工具更像一個「聊天」視窗，新的評論會「立即顯示」。

因此，啟用該功能的旗標名稱為：**showLiveRightAway**。

我們可以按如下方式開啟：

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

也可以在小工具自訂頁面上無需撰寫程式碼就進行自訂：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]

---