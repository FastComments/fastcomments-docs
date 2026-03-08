1. 前往 <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">your FastComments LTI Config</a>。
2. 輸入 **Configuration Name** 與您的 **Platform URL**（例如 `https://yourschool.instructure.com`）。選擇要啟用的 **Placements**（Assignment View 和/或 Editor Button — 預設兩者皆為開啟）。點選 **Create Configuration**。精靈會進入第 2 步並顯示您的 **Configuration URL**。
3. 在 Canvas 中，前往 **Admin > Developer Keys > + Developer Key > LTI Key**。將 **Method** 設為 "Enter URL" 並貼上 Configuration URL。儲存金鑰，然後將其 **State** 設為 **ON**，在提示時點選 **Allow**。
4. 從 Canvas 的 Developer Keys 表格中複製 **Client ID** 編號。回到 FastComments，將其貼到 **Client ID** 欄位並點選 **Save & Continue**。
5. 檢查設定摘要，然後點選 **Enable Integration** 使其上線。
6. 在 Canvas 安裝 External App（**Admin > Settings > Apps > + App > By Client ID**）。留言將會自動顯示在作業下方，且教師可以透過 Rich Content Editor 工具列按鈕，在 Pages、Quizzes 與 Announcements 中嵌入 FastComments。