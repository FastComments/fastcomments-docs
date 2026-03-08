#### 在 Canvas 中開啟 Developer Keys

以管理員身分登入 Canvas。前往 **Admin**（在左側邊欄）> 選取您的帳戶 > **Developer Keys**。

#### 建立 LTI Developer Key

按一下 **+ Developer Key** 並選取 **LTI Key**。

在設定表單中：

1. 將 **Method** 設為 **Enter URL**。
2. 將您從 FastComments 複製的 **Configuration URL** 貼到 URL 欄位。
3. Canvas 會自動載入 LTI 設定。
4. 為該金鑰命名（例如 "FastComments"）。
5. 按一下 **Save**。

#### 啟用 Developer Key

儲存後，新的金鑰會出現在 Developer Keys 表格中，且其 **State** 設為 **OFF**。點擊切換按鈕將其設為 **ON**。Canvas 可能會提示您確認 — 按一下 **Allow** 以啟用該金鑰。

#### 複製 Client ID

Developer Keys 表格在 Details 欄中會顯示數字形式的 **Client ID**（例如 `17000000000042`）。複製此數字 — 您在下一步會需要它。