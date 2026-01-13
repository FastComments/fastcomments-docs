既然我們已經下載了 zip 檔案，請將它解壓到一個資料夾。我已下載預設的 `casper.zip`，並在 Windows 上解壓到 `Downloads\casper`。

接著，請確認已安裝 LTS 或更新版本的 NodeJS。你可以在這裡下載：https://nodejs.org/en/download/

安裝好 NodeJS 後，請安裝一個程式碼編輯器。

我們推薦（並使用）WebStorm，你可以在這裡取得 30 天試用（不需信用卡）：https://www.jetbrains.com/webstorm/

次佳的免費選擇可能是 Visual Studio Code： https://code.visualstudio.com/download

在編輯器設定好並打開主題資料夾後，在 IDE 中開啟終端機並執行：

[inline-code-attrs-start title = '安裝主題'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

成功的輸出會像這樣（你可以忽略警告）：

<div class="screenshot white-bg">
    <div class="title">成功的 npm install 輸出</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="成功的 npm install 輸出" />
</div>

這會為之後要執行的指令設定主題的相依套件。此外，匯出功能仰賴主題相依套件已安裝，否則重新匯入將無法正常運作。