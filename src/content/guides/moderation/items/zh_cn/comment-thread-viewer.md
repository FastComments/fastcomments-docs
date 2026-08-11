---
在审核和查看评论线程时，能够直接跳转到特定线程以获取上下文是很有需求的。

这意味着用户的操作流程从评论审核页面开始，然后必须从单个评论跳转到包含该评论的页面，等待页面加载，等待评论加载，最后滚动到该评论。

然而，FastComments 提供了一种更快捷的方式。在“审核评论”页面中，每条评论旁边的右下角都有一个“查看评论”按钮。

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=1&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.comments .comment-component'; alt='审核列表中的单个评论，右下角有“查看评论”按钮'; title='评论' app-screenshot-end]

如果该评论有回复，按钮文字会显示回复数量，但点击后执行的操作相同。

此按钮将带您进入 **评论线程查看器**。

评论线程查看器是 FastComments 托管的一个小型、加载快速的应用程序，它会渲染评论所在页面的评论线程，并滚动至该评论。

这使得审核员能够快速获取所需的上下文，而无需等待另一个页面加载。
---