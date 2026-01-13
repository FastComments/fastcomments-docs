[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

預設情況下，FastComments 會以「Most Relevant」排序方向來排列留言。

「Most Relevant」排序會同時考量留言發表的時間與投票數來進行排序。

使用者可以在留言小工具的使用者介面中，將排序方向改為「Oldest」或「Newest First」。

不過，我們可以將預設值設定為三者中的任一個。例如，如果你想要預設顯示最舊的留言：

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

我們將 **defaultSortDirection** 的值設為 "OF" 以將排序方向設為 "OF"。

若要設定為「最新優先」排序方向，做法如下：

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

**defaultSortDirection** 的有效值為：

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

這也可以在不使用程式碼的情況下完成。在小工具自訂頁面中，請參閱「Default Sort Direction」章節。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

請注意，每個頁面在各種排序方向下的留言都是事先計算好的，因此所有排序方向的效能相同。