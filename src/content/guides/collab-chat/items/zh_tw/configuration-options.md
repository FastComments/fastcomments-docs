### 概覽

FastComments Collab Chat 擴展了標準的 FastComments 留言小工具，因此它繼承了基礎小工具的所有配置選項，同時新增了幾個專屬於文字標註的選項。

### 必要配置

#### tenantId

必須提供您的 FastComments Tenant ID。您可以在 [FastComments 儀表板](https://fastcomments.com/auth/my-account/api-secret) 中找到它。

[inline-code-attrs-start title = "配置範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat 專屬選項

#### urlId

預設情況下，Collab Chat 會根據頁面 URL、元素的 DOM 路徑以及選取的文字範圍，為每個對話產生一個唯一識別碼。您可以使用自訂的 `urlId` 來覆蓋此行為。

[inline-code-attrs-start title = "配置範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

當您的 URL 結構可能會變更但您希望保留相同的對話，或者您想在多個頁面之間共享標註時，這會很有用。

#### topBarTarget

控制頂部列的顯示，該列會顯示使用者人數與討論數量。設為 `null` 可完全停用頂部列，或提供一個 DOM 元素以將其渲染在特定位置。

[inline-code-attrs-start title = "配置範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 停用頂部列
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// 在自訂位置渲染頂部列
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

當您的頁面具有深色背景時，啟用深色模式樣式。此偵測是自動進行的，但您可能希望覆蓋它。

[inline-code-attrs-start title = "配置範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

每當留言數量變更時觸發的回呼函式。這對於更新 UI 元素（例如徽章或頁面標題）很有用。

[inline-code-attrs-start title = "配置範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### 繼承的配置選項

由於 Collab Chat 擴展了標準留言小工具，您可以使用基礎 FastComments 小工具的任何配置選項。以下是一些常用的選項：

#### locale

設定小工具 UI 的語言。FastComments 支援數十種語言。

[inline-code-attrs-start title = "配置範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // 西班牙文
});
[inline-code-end]

#### readonly

將所有對話設為唯讀。使用者可以檢視現有的標註，但無法建立新的標註或回覆。

[inline-code-attrs-start title = "配置範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso 與 simpleSSO

使用單一登入（Single Sign-On）與您的驗證系統整合。

[inline-code-attrs-start title = "配置範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO 配置
    }
});
[inline-code-end]

有關驗證選項的完整詳細資訊，請參閱 SSO 文件。

#### maxReplyDepth

控制回覆可以巢狀到多少層。預設情況下，Collab Chat 將此設為 0，表示所有留言都是扁平的（沒有巢狀回覆）。如果您想要有討論串形式的對話，可以更改此設定。

[inline-code-attrs-start title = "配置範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 允許 3 層巢狀
});
[inline-code-end]

### 內部配置

這些選項由 Collab Chat 自動設定，不應被覆蓋：

`productId` 會自動設為 `3` 以用於 Collab Chat。`floating-chat` 擴充功能會自動載入以提供聊天視窗功能。小工具會自動偵測行動裝置（寬度低於 768px 的螢幕）並相應調整 UI。

### 完整範例

以下是一個同時展示多個配置選項的範例：

[inline-code-attrs-start title = "配置範例"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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
        // 您的 SSO 配置
    },
    maxReplyDepth: 1
});
[inline-code-end]

有關從基礎小工具繼承的所有可用配置選項的完整清單，請參閱 FastComments 主要配置文件。