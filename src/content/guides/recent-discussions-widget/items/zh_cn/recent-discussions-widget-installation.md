最近讨论组件显示您网站上具有最新评论活动的页面。每个条目显示页面标题、最后活动日期和评论总数。它会自动检测深色背景并相应地调整样式。

## 基本安装

[inline-code-attrs-start title = '最近讨论组件安装'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## 配置选项

`FastCommentsRecentDiscussionsV2` 函数接受以下配置选项：

- **tenantId**（必填）：您的 FastComments 租户 ID
- **count**（可选）：显示的页面数量。默认为 `20`，最大为 `100`
- **hasDarkBackground**（可选）：强制使用深色模式样式。如果未设置，将自动从页面背景检测

## 高级示例

### 自定义数量

[inline-code-attrs-start title = '自定义数量的最近讨论'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        count: 5
    });
</script>
[inline-code-end]

### 强制深色模式

[inline-code-attrs-start title = '深色模式的最近讨论'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo',
        hasDarkBackground: true
    });
</script>
[inline-code-end]