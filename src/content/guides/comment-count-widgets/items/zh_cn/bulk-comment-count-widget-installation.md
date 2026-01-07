批量评论计数小部件旨在高效地在同一页面上显示多个页面的评论数量。该小部件不会为每个评论计数进行单独的API调用，而是批量处理请求以获得最佳性能。

## 基本安装

[inline-code-attrs-start title = 'Bulk Comment Count Widget Installation'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<!-- Multiple elements with comment counts -->
<div class="fast-comments-count" data-fast-comments-url-id="page-1"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-2"></div>
<div class="fast-comments-count" data-fast-comments-url-id="page-3"></div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## 工作原理

批量小部件的工作方式：

1. 扫描页面中具有 `fast-comments-count` 类的元素
2. 从每个元素读取 `data-fast-comments-url-id` 属性
3. 批量处理API请求以高效获取多个评论计数
4. 使用适当的评论计数更新每个元素

## 配置选项

`FastCommentsCommentCountBulk` 函数接受以下配置选项：

- **tenantId**（必需）：您的FastComments租户ID
- **apiHost**（可选）：如果您使用自托管实例，则为自定义API主机

## 实际示例

这是一个实用示例，展示如何在博客文章列表中使用批量小部件：

[inline-code-attrs-start title = 'Blog Post Listing with Comment Counts'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .blog-post {
        border: 1px solid #ddd;
        margin: 10px 0;
        padding: 15px;
        border-radius: 5px;
    }
    .post-meta {
        color: #666;
        font-size: 14px;
        margin-top: 10px;
    }
    .comment-count {
        background: #e7f3ff;
        padding: 2px 8px;
        border-radius: 12px;
        font-size: 12px;
        display: inline-block;
    }
</style>

<div class="blog-post">
    <h3>How to Install FastComments</h3>
    <p>Learn how to add FastComments to your website in just a few minutes...</p>
    <div class="post-meta">
        Published: March 15, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="how-to-install-fastcomments"></span>
    </div>
</div>

<div class="blog-post">
    <h3>Advanced FastComments Configuration</h3>
    <p>Dive deep into the advanced configuration options for FastComments...</p>
    <div class="post-meta">
        Published: March 10, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="advanced-fastcomments-config"></span>
    </div>
</div>

<div class="blog-post">
    <h3>FastComments vs Other Solutions</h3>
    <p>See how FastComments compares to other commenting solutions...</p>
    <div class="post-meta">
        Published: March 5, 2024 |
        <span class="fast-comments-count comment-count" data-fast-comments-url-id="fastcomments-comparison"></span>
    </div>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## 性能考虑

批量小部件通过以下方式自动优化性能：

- **请求批处理**：多个评论计数在单个API调用中获取
- **请求大小限制**：如果URL列表变得太长（超过1,000个字符），请求会自动拆分
- **去重**：具有相同 `data-fast-comments-url-id` 的多个元素共享相同的计数

## 具有相同URL ID的多个元素

您可以在页面上拥有多个具有相同 `data-fast-comments-url-id` 的元素。它们都将使用相同的计数进行更新：

[inline-code-attrs-start title = 'Multiple Elements Same URL ID'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-widget-comment-count-bulk.min.js"></script>

<style>
    .count-example {
        margin: 10px 0;
        padding: 10px;
        background: #f9f9f9;
        border-radius: 5px;
    }
</style>

<div class="count-example">
    Header Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Sidebar Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<div class="count-example">
    Footer Count: <span class="fast-comments-count" data-fast-comments-url-id="shared-article"></span>
</div>

<script>
    window.FastCommentsBulkCountConfig = {
        tenantId: 'demo'
    };
</script>
[inline-code-end]

## 本地化

批量小部件根据您的FastComments语言设置自动格式化评论计数。它为以下情况提供适当的文本：

- 零条评论
- 一条评论
- 多条评论

## 何时使用批量小部件与单个小部件

**在以下情况下使用批量小部件：**
- 您在同一页面上有多个评论计数
- 您正在显示带有评论计数的帖子/文章列表
- 性能很重要（减少API调用）

**在以下情况下使用单个小部件：**
- 您只需要页面上的一个评论计数
- 您需要实时更新（单个小部件支持实时更新）
- 您想要对单个小部件的行为有更多控制
