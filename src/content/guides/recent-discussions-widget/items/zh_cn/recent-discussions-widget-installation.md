最近讨论小部件显示您网站上具有最新评论活动的页面。每个条目显示页面标题、最后活动日期和总评论数。它会自动检测深色背景并相应调整其样式。

## 基本安装

[inline-code-attrs-start title = '最近讨论小部件安装'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

函数 `FastCommentsRecentDiscussionsV2` 接受以下配置选项：

- **tenantId** (required): Your FastComments tenant ID
- **count** (optional): Number of pages to show. Default is `20`, max `100`
- **hasDarkBackground** (optional): Force dark mode styling. Auto-detected from the page background if not set

## 高级示例

### 自定义数量

[inline-code-attrs-start title = '使用自定义数量的最近讨论'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

[inline-code-attrs-start title = '使用深色模式的最近讨论'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

---