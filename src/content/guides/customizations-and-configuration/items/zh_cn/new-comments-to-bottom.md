[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

默认情况下，实时发布的新评论会出现在评论列表的顶部。

启用此选项后，实时发布的新评论将改为添加到列表底部。这会影响用户在查看评论线程时实时发布的评论的显示位置。

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

启用此设置后：
- 其他用户实时发布的新评论将出现在评论列表的底部
- 用户将在实时中看到新评论出现在现有评论之下
- 这只影响实时评论更新 - 不影响初始页面加载
- 当用户跟随讨论时，这有助于保持阅读流畅性

请注意，此设置仅影响实时到达的新评论的放置位置。它不会影响页面加载时的初始排序顺序。