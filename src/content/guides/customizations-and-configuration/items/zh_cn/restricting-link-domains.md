---
默认情况下，FastComments 允许链接到任何外部站点。

这可以限制为仅允许的站点或域名列表。尝试发布链接到未在定义列表中的站点或域名时，将向用户显示错误。

此验证仅适用于评论小部件和 API。导入不受影响。

此操作无需代码，在小部件自定义页面完成：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.restricted-link-domains-list'; selector = '.external-link-settings'; alt='外部链接设置，包含受限链接域名列表，可在此输入允许的站点'; title='限制外部链接域名' app-screenshot-end]
---