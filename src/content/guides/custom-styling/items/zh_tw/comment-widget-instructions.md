## 如何自訂評論小工具樣式

您可以透過兩種方式自訂評論小工具的樣式：

### 選項 1：透過 `customCSS` 參數

在初始化小工具時，將自訂 CSS 以字串形式傳遞給 `customCSS` 參數：

```javascript
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
}];
```

### 選項 2：透過管理後台

1. 在您的管理後台前往 [小工具自訂頁面](https://fastcomments.com/auth/my-account/customize-widget)
2. 捲動到「進階」中的「自訂 CSS」區段
3. 輸入您的自訂 CSS
4. 點選「儲存」

您輸入的自訂 CSS 將套用到網站上所有的評論小工具。

## 提示

- 如有需要，使用 `!important` 以覆蓋預設樣式
- 針對特定選擇器設定，以避免影響網站的其他部分
- 在不同瀏覽器中測試您的 CSS 以確保相容性
- 小工具使用標準 CSS - 不需要特殊的前處理器

---