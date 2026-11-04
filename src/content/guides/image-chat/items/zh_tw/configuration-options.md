### 概覽

FastComments Image Chat 擴展了標準的 FastComments 留言小工具，因此它繼承了基礎小工具的所有配置選項，同時新增了幾個專屬於圖片標註的選項。

### 必要配置

#### tenantId

必須提供您的 FastComments Tenant ID。您可以在 [FastComments 儀表板](https://fastcomments.com/auth/my-account/api-secret) 中找到它。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat 專屬選項

#### urlId

預設情況下，Image Chat 會根據頁面 URL、圖片來源以及 X/Y 座標，為每個對話產生一個唯一識別碼。您可以使用自訂的 `urlId` 來覆蓋此行為。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

當您的 URL 結構可能會變動但您希望保留相同的對話時，或者當您想在多個頁面之間共享標註時，這會很有用。

#### chatSquarePercentage

以圖片寬度的百分比控制可點擊聊天標記的大小。預設值為 5%，表示每個標記佔圖片寬度的 5%。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // 更大、更醒目的標記
});
```

較小的值會產生較不干擾的標記，更適合細節豐富的圖片。較大的值則讓標記在內容繁雜的圖片上或對行動裝置使用者來說更容易看到和點擊。

#### hasDarkBackground

當您的頁面具有深色背景時，啟用深色模式樣式。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

每當留言數變更時觸發的回呼函式。這對於更新徽章或頁面標題等 UI 元素很有用。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### 繼承的配置選項

由於 Image Chat 擴展了標準留言小工具，您可以使用基礎 FastComments 小工具的任何配置選項。以下是一些常用的選項：

#### locale

設定小工具 UI 的語言。FastComments 支援數十種語言。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // 西班牙語
});
```

#### readonly

將所有對話設為唯讀。使用者可以檢視現有的標記與討論，但無法建立新的標記或回覆。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso 和 simpleSSO

使用單一登入（Single Sign-On）與您的驗證系統整合。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO 配置
    }
});
```

有關驗證選項的完整詳細資訊，請參閱 SSO 文件。

#### maxReplyDepth

控制回覆可以嵌套多少層。預設情況下，Image Chat 將此值設為 0，表示所有留言都是扁平的（沒有嵌套回覆）。如果您想要有討論串式的對話，可以更改此值。

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // 允許 3 層嵌套
});
```

### 內部配置

這些選項由 Image Chat 自動設定，不應被覆蓋：

`productId` 會自動設為 `2` 以供 Image Chat 使用。`floating-chat` 擴充功能會自動載入以提供聊天視窗功能。小工具會自動偵測行動裝置（寬度低於 768px 的螢幕），並相應地以全螢幕聊天視窗調整 UI。

### 目標元素的彈性

`FastCommentsImageChat` 的第一個參數可以直接是 `<img>` 元素，或是內含圖片的容器元素：

```javascript
// 直接傳入圖片元素
FastCommentsImageChat(document.getElementById('my-image'), config);

// 傳入內含圖片的容器
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

如果您傳入的是容器元素，小工具會自動找到其中的圖片。

### 完整範例

以下是一個同時展示多個配置選項的範例：

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
        // 您的 SSO 配置
    },
    maxReplyDepth: 1
});
```

有關從基礎小工具繼承的所有可用配置選項的完整清單，請參閱 FastComments 主要配置文件。