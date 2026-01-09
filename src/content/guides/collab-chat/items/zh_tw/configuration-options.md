### 概觀

FastComments Collab Chat 擴充了標準的 FastComments 留言小工具，因此它繼承了基礎小工具的所有設定選項，同時新增了一些專屬於文字註解的選項。

### 必要的設定

#### tenantId

需要提供您的 FastComments Tenant ID。您可以在您的 [FastComments 控制台](https://fastcomments.com/auth/my-account/api-secret) 找到它。

[inline-code-attrs-start title = "設定範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat 專屬選項

#### urlId

預設情況下，Collab Chat 會基於頁面 URL、元素的 DOM 路徑以及所選文字範圍，自動產生每個對話的唯一識別符。您可以用自訂的 `urlId` 覆寫此行為。

[inline-code-attrs-start title = "設定範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

當您的 URL 結構可能會變動但仍想保留相同對話，或想要在多個頁面之間共用註解時，這會非常有用。

#### topBarTarget

控制顯示頂部工具列（顯示使用者數與討論數）。設為 `null` 可完全停用頂部工具列，或提供一個 DOM 元素以在特定位置渲染它。

[inline-code-attrs-start title = "設定範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 停用頂部工具列
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// 在自訂位置呈現頂部工具列
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

當您的頁面具有深色背景時，啟用深色模式樣式。此偵測為自動，但有時您可能想要覆寫它。

[inline-code-attrs-start title = "設定範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

當留言數量變動時會觸發的回呼函式。這對於更新像是徽章或頁面標題等 UI 元素非常有用。

[inline-code-attrs-start title = "設定範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### 繼承的設定選項

由於 Collab Chat 擴充了標準留言小工具，您可以使用任何來自基礎 FastComments 小工具的設定選項。以下是一些常用的選項：

#### locale

設定小工具 UI 的語言。FastComments 支援數十種語言。

[inline-code-attrs-start title = "設定範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // 西班牙語
});
[inline-code-end]

#### readonly

將所有對話設為唯讀。使用者可以檢視現有註解，但無法建立新註解或回覆。

[inline-code-attrs-start title = "設定範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

使用單一登入（Single Sign-On）整合您的驗證系統。

[inline-code-attrs-start title = "設定範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO 設定
    }
});
[inline-code-end]

完整的驗證選項請參閱 SSO 文件。

#### maxReplyDepth

控制回覆的最大層數。預設情況下，Collab Chat 將此設定為 0，表示所有留言為平面式（不允許巢狀回覆）。若想要使用討論串式對話，可更改此設定。

[inline-code-attrs-start title = "設定範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 允許 3 層巢狀
});
[inline-code-end]

### 內部設定

這些選項會由 Collab Chat 自動設定，且不應被覆寫：

The `productId` is automatically set to `3` for Collab Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly.

### 完整範例

以下示範多個設定項目一起使用的範例：

[inline-code-attrs-start title = "設定範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // 您的 SSO 設定
    },
    maxReplyDepth: 1
});
[inline-code-end]

欲查詢從基礎小工具繼承的所有可用設定選項之完整清單，請參閱主 FastComments 設定文件。