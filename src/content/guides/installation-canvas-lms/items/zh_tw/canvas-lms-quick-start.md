1. 登入 FastComments 並前往 <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">我的帳戶 > Canvas LTI 設定</a>。
2. 輸入 **設定名稱** 與您的 **平台 URL**（例如 `https://yourschool.instructure.com`）。選擇要啟用的 **放置位置**（作業檢視和/或編輯器按鈕 — 兩者預設皆已啟用）。點選 **建立設定**。精靈會進入步驟 2 並顯示您的 **設定 URL**。
3. 在 Canvas 中，前往 **管理 > 開發人員金鑰 > + 開發人員金鑰 > LTI 金鑰**。將 **方法** 設為「輸入 URL」，並貼上設定 URL。儲存該金鑰，然後將其 **狀態** 設為 **開啟**，並在提示時點選 **允許**。
4. 從 Canvas 的開發人員金鑰清單中複製 **Client ID** 編號。回到 FastComments，將其貼到 **Client ID** 欄位，然後點選 **儲存並繼續**。
5. 檢查設定摘要，然後點選 **啟用整合** 以正式啟用。
6. 在 Canvas 安裝外部應用程式（**管理 > 設定 > 應用程式 > + 應用程式 > 以 Client ID**）。留言會自動出現在作業下方，且教師可以透過富內容編輯器工具列按鈕，在頁面、測驗和公告中嵌入 FastComments。