#### 在 Canvas 開啟開發者金鑰

以管理員身分登入 Canvas。前往 **管理**（在左側邊欄）> 選取您的帳號 > **開發者金鑰**。

#### 建立 LTI 開發者金鑰

點選 **+ Developer Key** 並選擇 **LTI Key**。

在設定表單中：

1. 在 **Redirect URIs** 欄位（左側），貼上從 FastComments 設定頁面複製的 **Launch URL**。
2. 在右側，將 **Method** 設為 **Enter URL**。
3. 在 **JSON URL** 欄位貼上您從 FastComments 複製的 **Configuration URL**。
4. Canvas 會自動載入 LTI 設定。
5. 為金鑰命名（例如：「FastComments」）。
6. 點擊 **Save**。

#### 啟用開發者金鑰

儲存後，新的金鑰會出現在開發者金鑰表格中，其 **State** 顯示為 **OFF**。點擊切換按鈕將其設為 **ON**。Canvas 可能會提示您確認 — 點擊 **Allow** 以啟用金鑰。

#### 複製用戶端 ID

開發者金鑰表格在 Details 欄中顯示一個數字 **Client ID**（例如 `17000000000042`）。複製此數字 — 您在下一步會需要它。