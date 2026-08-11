---
[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

默认情况下，FastComments 会按“最相关”排序方向对评论进行排序。

“最相关”排序会考虑评论留下的时间以及投票数量来进行排序。

用户随后可以在评论小部件 UI 中将排序方向更改为“最旧”或“最新”。

但是，我们可以将默认值更改为这三种之一。例如，如果您想先显示最旧的评论：

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = '将默认排序更改为最旧优先'; code-example-end]

我们将 **defaultSortDirection** 的值设置为 “OF”，以将方向设为 “OF”。

对于“最新优先”排序方向，我们可以这样做：

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = '将默认排序更改为最新优先'; code-example-end]

**defaultSortDirection** 的有效值包括：

- MR: “最近”
- NF: “最新优先”
- OF: “最旧优先”

这也可以在不编写代码的情况下完成。在小部件自定义页面中，查看“默认排序方向”部分。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='提供“最相关”“最新优先”和“最旧优先”的默认排序方向选择器'; title='更改默认排序方向' app-screenshot-end]

请注意，每个页面针对每种排序方向的评论都是预先计算好的，因此所有排序方向的性能相同。

---