## 如何自定义评论小部件样式

您可以通过两种方式自定义评论小部件的样式：

### 选项 1：通过 `customCSS` 参数

在初始化小部件时，将您的自定义 CSS 以字符串形式传递给 `customCSS` 参数：

```javascript
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
});
```

### 选项 2：通过管理仪表板

1. 转到您的管理仪表板中的 [小部件自定义页面](https://fastcomments.com/auth/my-account/customize-widget)
2. 滚动到“高级”下的“自定义 CSS”部分
3. 输入您的自定义 CSS
4. 点击“保存”

您的自定义 CSS 将应用到您网站上的所有评论小部件。

## 提示

- 如有需要，使用 `!important` 覆盖默认样式
- 针对特定选择器以避免影响站点的其他部分
- 在不同浏览器中测试您的 CSS 以确保兼容性
- 该小部件使用标准 CSS - 无需特殊预处理器