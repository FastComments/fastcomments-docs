在审核和查看评论线程时，希望能够直接跳转到某个线程以在审核过程中获取上下文。

这意味着用户的流程从 Comment Moderation 页面开始，然后必须从单个评论跳转到包含该评论的页面，等待页面加载，等待评论加载，然后滚动到该评论。

但是，FastComments 提供了一种更快的方式。在 Moderate Comments 页面，每条评论的右下角都有一个 "View Comment" 按钮。

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=1&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.comments .comment-component'; title='A Comment' app-screenshot-end]

如果该评论有回复，按钮文本会显示回复数量，但点击它的操作相同。

此按钮将带您进入 **评论线程查看器**。

评论线程查看器是由 FastComments 托管的一个小型、快速加载的应用程序，它会渲染该评论所在页面的评论线程，并滚动到该评论。

这使审核人员能够快速获取所需的上下文，而无需等待另一个页面加载。

---