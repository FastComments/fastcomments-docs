[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

评论小部件顶部显示的评论计数可以显示所有“顶级”评论，意思是那些
直接回复页面或文章本身的回复，或者它可以是**所有**嵌套评论的计数。

默认情况下，这是 `true` - 它是后者的计数 - 所有评论。在旧版本的评论小部件中，默认值是 `false`。

我们可以通过将 **countAll** 标志设置为 true 来更改行为，使其成为**所有**嵌套评论的计数。

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

如果我们希望计数仅反映顶级评论，我们将该标志设置为 false。

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

目前无法在不修改代码的情况下进行自定义。