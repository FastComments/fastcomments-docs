[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

預設情況下，FastComments 會以向上和向下的箭頭顯示投票選項，讓使用者能對留言進行向上或向下投票。

不過，可以變更投票工具列的樣式。當前選項包括預設的向上/向下按鈕，或使用愛心（Heart）樣式的投票機制。

我們使用 **voteStyle** 標誌如下：

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

我們強烈建議您在不撰寫程式碼的情況下進行此設定，因為這同時啟用伺服器端驗證。在小工具自訂頁面，請參閱「投票樣式」區段。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

也可以停用投票，請參閱上述樣式選項上方的 `Disable Voting`。

---