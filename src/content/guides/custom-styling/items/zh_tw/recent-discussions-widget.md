「最近討論」小工具會顯示一個依最新留言活動排序的頁面清單。它包含標題、最後活動日期、帶圖示的留言數量,以及自動深色模式偵測。

## 基本安裝

[inline-code-attrs-start title = '最近討論小工具樣式設定'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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

- **tenantId**(必填):您的 FastComments 租戶 ID
- **count**(選填):要顯示的頁面數量。預設為 `20`,最大為 `100`
- **hasDarkBackground**(選填):強制使用深色模式樣式。若未設定,將自動從頁面背景偵測

## 小工具結構

小工具會以下列 HTML 結構呈現:

[inline-code-attrs-start title = '最近討論小工具 HTML 結構'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div class="fc-rd2">
    <div class="fc-rd2-heading">Recent Discussions</div>
    <div class="fc-rd2-list">
        <div class="fc-rd2-item">
            <div class="fc-rd2-detail">
                <a class="fc-rd2-title" href="...">Page Title</a>
                <span class="fc-rd2-activity">Last activity Mar 21, 2026</span>
            </div>
            <div class="fc-rd2-count">42</div>
        </div>
    </div>
</div>
[inline-code-end]

## 預設 CSS 參考

[inline-code-attrs-start title = '最近討論小工具預設 CSS'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rd2 {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
    text-align: left;
    line-height: 1.5;
    color: #1a1a1a;
    border: 1px solid #e0e0e0;
    border-radius: 12px;
    padding: 20px;
    background: #fff;
}
.fc-rd2-heading { font-size: 16px; font-weight: 700; margin-bottom: 14px; padding-bottom: 12px; border-bottom: 1px solid #eee; }
.fc-rd2-item { display: flex; align-items: center; gap: 12px; padding: 10px 0; border-bottom: 1px solid #f0f0f0; }
.fc-rd2-item:last-child { border-bottom: none; }
.fc-rd2-title { font-size: 13px; font-weight: 500; color: #1a1a1a; text-decoration: none; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.fc-rd2-activity { font-size: 11px; color: #999; }
.fc-rd2-count { font-size: 12px; font-weight: 600; color: #666; }
[inline-code-end]

## 自訂範例

### 移除容器邊框

[inline-code-attrs-start title = '移除容器邊框'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fc-rd2 {
    border: none !important;
    box-shadow: none !important;
}
[inline-code-end]

### 自訂連結顏色

[inline-code-attrs-start title = '自訂連結顏色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
a.fc-rd2-title:hover {
    color: #e63946 !important;
}
[inline-code-end]