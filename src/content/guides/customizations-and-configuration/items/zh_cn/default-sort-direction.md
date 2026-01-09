[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

默认情况下，FastComments 将按“最相关”的排序方向对评论进行排序。

“最相关”排序会同时考虑评论的发布时间和投票数来进行排序。

然后用户可以在评论小部件界面中将排序方向更改为“最旧优先”或“最新优先”。

但是，我们可以将默认值更改为这三者中的任何一个。例如，如果您想先显示最旧的评论：

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

我们将 **defaultSortDirection** 的值设置为 "OF"，以将方向设置为 "OF"。

对于“最新优先”的排序方向，我们可以执行以下操作：

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

**defaultSortDirection** 的有效值为：

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

这也可以在不编写代码的情况下完成。在小部件自定义页面中，参见“默认排序方向”部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

请注意，每个页面中针对每种排序方向的评论都是预先计算的，因此所有排序方向的性能相同。