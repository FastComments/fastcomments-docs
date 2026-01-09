## 如何自訂評論小工具樣式

您可以用兩種方式自訂評論小工具的樣式：

### 選項 1：透過 customCSS 參數

在初始化小工具時，將您的自訂 CSS 以字串傳入 `customCSS` 參數：

```javascript
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
});
```

### 選項 2：透過管理後台

1. 前往管理後台的 [小工具自訂頁面](https://fastcomments.com/auth/my-account/customize-widget)
2. 向下捲動到「進階」下的「自訂 CSS」區段
3. 輸入您的自訂 CSS
4. 點擊「儲存」

您輸入的自訂 CSS 將套用到您網站上的所有評論小工具。

## 提示

- 如有需要，使用 `!important` 來覆蓋預設樣式
- 針對特定選擇器設定樣式以避免影響網站的其他部分
- 在不同瀏覽器中測試您的 CSS 以確保相容性
- 此小工具使用標準 CSS - 不需使用任何特殊的預處理器