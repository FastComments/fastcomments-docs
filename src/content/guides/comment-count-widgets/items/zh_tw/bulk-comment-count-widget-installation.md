批量評論計數小工具旨在高效地在同一頁面上顯示多個頁面的評論數量。該小工具不會為每個評論計數進行單獨的API調用，而是批量處理請求以獲得最佳效能。

## 基本安裝

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

## 運作原理

批量小工具的運作方式：

1. 掃描頁面中具有 `fast-comments-count` 類別的元素
2. 從每個元素讀取 `data-fast-comments-url-id` 屬性
3. 批量處理API請求以高效獲取多個評論計數
4. 使用適當的評論計數更新每個元素

## 配置選項

`FastCommentsCommentCountBulk` 函數接受以下配置選項：

- **tenantId**（必需）：您的FastComments租戶ID
- **apiHost**（可選）：如果您使用自託管實例，則為自訂API主機

## 實際範例

這是一個實用範例，展示如何在部落格文章列表中使用批量小工具：

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

## 效能考量

批量小工具透過以下方式自動優化效能：

- **請求批處理**：多個評論計數在單個API調用中獲取
- **請求大小限制**：如果URL列表變得太長（超過1,000個字元），請求會自動拆分
- **去重**：具有相同 `data-fast-comments-url-id` 的多個元素共享相同的計數

## 具有相同URL ID的多個元素

您可以在頁面上擁有多個具有相同 `data-fast-comments-url-id` 的元素。它們都將使用相同的計數進行更新：

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

批量小工具根據您的FastComments語言設定自動格式化評論計數。它為以下情況提供適當的文字：

- 零則評論
- 一則評論
- 多則評論

## 何時使用批量小工具與單個小工具

**在以下情況下使用批量小工具：**
- 您在同一頁面上有多個評論計數
- 您正在顯示帶有評論計數的文章列表
- 效能很重要（減少API調用）

**在以下情況下使用單個小工具：**
- 您只需要頁面上的一個評論計數
- 您需要即時更新（單個小工具支援即時更新）
- 您想要對單個小工具的行為有更多控制
