「最近討論」小工具會顯示您網站上最近有留言活動的頁面。每個項目會顯示頁面標題、最後活動日期以及留言總數。它會自動偵測深色背景並相應調整樣式。

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

`FastCommentsRecentDiscussionsV2` 函式接受以下設定選項：

- **tenantId**（必填）：您的 FastComments 租戶 ID
- **count**（選填）：要顯示的頁面數量。預設為 `20`，最大為 `100`
- **hasDarkBackground**（選填）：強制使用深色模式樣式。若未設定，將自動從頁面背景偵測

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