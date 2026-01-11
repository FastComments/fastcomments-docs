當使用者個人檔案在您的網站上下文中開啟（透過 comment widget），您套用到 FastComments widget 的任何自訂 CSS 樣式都會自動注入至個人檔案模式視窗。

### 運作方式

當使用者從您的 comment widget 點擊個人檔案連結時，會開啟具有類別 `.fast-comments-profile` 的個人檔案模態視窗。您小工具的自訂 CSS 會自動注入到個人檔案檢視中。如果您已經為 comment widget 設定樣式，這些樣式也會套用到個人檔案。

### CSS 類別

FastComments 個人檔案使用基於類別的 CSS 架構。它不使用 CSS 自訂屬性。

主要的個人檔案頁面以 `.user-profile` 作為根容器。標頭區段為 `.profile-header`，背景圖片使用 `.profile-header-background`。個人檔案內容位於 `.profile-content`。

大頭貼使用 `.profile-avatar` 與 `.profile-avatar-wrapper`。使用者名稱為 `.profile-name`，個人簡介文字為 `.profile-bio`。統計資料位於 `.profile-stats`，各項統計使用 `.stat`。

社群連結位於 `.profile-social-links`，個別連結為 `.social-link`。徽章使用 `.profile-badges` 與 `.badge`。徽章進度條使用 `.progress-outer` 與 `.progress-bar`。

分頁容器使用 `.profile-tabs`，個別分頁使用 `.tab`，選取中的分頁為 `.tab.active`。分頁內容使用 `.tab-body` 與 `.tab-body.active`。分頁上的通知數量使用 `.tab .count`。

通知項目使用 `.notification`，私訊對話使用 `.conversation`。線上狀態為 `.activity-indicator`，啟用狀態為 `.activity-indicator.online`。未讀計數使用 `.unread-count`。

個人檔案模態容器為 `.fast-comments-profile`，關閉按鈕為 `.fast-comments-profile-close`。

### 深色模式

深色模式透過在 `.user-profile` 上使用 `.dark` 類別修飾。

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### 範例

**標頭:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**徽章:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**分頁:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**模態視窗:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```