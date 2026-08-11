[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

預設情況下，FastComments 會以向上和向下的箭頭呈現投票選項，允許使用者對評論進行讚或倒讚。

然而，您可以更改投票工具列的樣式。目前的選項包括預設的上下按鈕，或使用心形投票機制。

我們使用 **voteStyle** 旗標如下：

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

我們強烈建議您在不編寫程式碼的情況下完成此操作，因為它同時啟用了伺服器端驗證。請在小工具自訂頁面中查看「Vote Style」區段。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='小工具自訂頁面上的投票樣式設定，提供上下箭頭或心形投票'; title='變更投票樣式' app-screenshot-end]

投票也可以被停用，請參閱樣式選項上方的 `Disable Voting`。