最近討論小工具會顯示你網站上有最新留言活動的頁面。每一項目會顯示頁面標題、最後活動日期和總留言數。它會自動偵測深色背景並相應地調整樣式。

## 基本安裝

[inline-code-attrs-start title = '最近討論小工具安裝'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/widget-recent-discussions-v2.min.js"></script>
<div id="fastcomments-widget-recent-discussions"></div>
<script>
    FastCommentsRecentDiscussionsV2(document.getElementById('fastcomments-widget-recent-discussions'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

## 設定選項

The `FastCommentsRecentDiscussionsV2` function accepts the following configuration options:

- **tenantId** (required): 您的 FastComments tenant ID
- **count** (optional): 要顯示的頁面數量。預設為 `20`，最大 `100`
- **hasDarkBackground** (optional): 強制深色模式樣式。如果未設定則從頁面背景自動偵測

## 進階範例

### 自訂數量

[inline-code-attrs-start title = '自訂數量的最近討論'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

### 強制深色模式

[inline-code-attrs-start title = '深色模式的最近討論'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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