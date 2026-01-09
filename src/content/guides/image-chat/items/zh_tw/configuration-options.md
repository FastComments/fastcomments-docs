### 概覽

FastComments Image Chat 擴充了標準的 FastComments 評論小工具，因此它繼承了基礎小工具的所有設定選項，同時新增了一些專屬於影像註解的設定。

### 必要設定

#### tenantId

需要提供您的 FastComments Tenant ID。您可以在您的 [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret) 找到它。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### 圖像聊天特定選項

#### urlId

預設情況下，Image Chat 會根據頁面 URL、影像來源和 X/Y 座標為每個對話產生唯一識別碼。您可以用自訂的 `urlId` 覆寫這個行為。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

這在您的 URL 結構可能會改變但想保留相同對話時很有用，或是當您想要在多個頁面之間共用註解時也很有用。

#### chatSquarePercentage

控制可點擊聊天標記的大小，為影像寬度的百分比。預設為 5%，表示每個標記為影像寬度的 5%。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // 較大、更顯眼的標記
});
```

較小的值會產生不那麼突兀的標記，較適合細節豐富的影像。較大的值則讓標記在繁忙的影像或行動裝置上更容易看見和點擊。

#### hasDarkBackground

當您的頁面為深色背景時啟用深色模式的樣式。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

當評論數變化時會觸發的回呼函式。這對更新 UI 元素（例如徽章或頁面標題）很有用。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### 繼承的設定選項

由於 Image Chat 擴充了標準評論小工具，您可以使用基礎 FastComments 小工具的任何設定選項。以下是一些常用的選項：

#### locale

設定小工具 UI 的語言。FastComments 支援數十種語言。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // 西班牙語
});
```

#### readonly

將所有對話設為唯讀。使用者可以檢視現有標記和討論，但無法建立新項目或回覆。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

使用單一登入整合您的驗證系統。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO 設定
    }
});
```

完整的驗證選項請參閱 SSO 文件。

#### maxReplyDepth

控制回覆可以有多少層深度。預設情況下，Image Chat 將此設為 0，表示所有評論為平鋪（無巢狀回覆）。如果您想要線程式對話，可以變更此數值。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 允許 3 層巢狀
});
```

### 內部設定

這些選項由 Image Chat 自動設定，不應被覆寫：

`productId` 會自動為 Image Chat 設為 `2`。會自動載入 `floating-chat` 擴充以提供聊天視窗功能。小工具會自動偵測行動裝置（螢幕寬度小於 768px）並相應調整 UI，使用全螢幕聊天視窗。

### 目標元素的彈性

傳遞給 `FastCommentsImageChat` 的第一個參數可以是直接的 `<img>` 元素，或是內含影像的容器元素：

```javascript
// 直接的圖像元素
FastCommentsImageChat(document.getElementById('my-image'), config);

// 包含圖像的容器
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

如果您傳入容器元素，小工具會自動尋找影像。

### 完整範例

下面是一個示範多個設定選項一起使用的範例：

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // 您的 SSO 設定
    },
    maxReplyDepth: 1
});
```

有關從基礎小工具繼承的所有可用設定選項的完整清單，請參閱主要的 FastComments 設定文件。