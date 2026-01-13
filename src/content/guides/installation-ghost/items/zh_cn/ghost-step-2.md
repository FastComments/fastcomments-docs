现在我们已下载 zip 文件，将其解压到一个文件夹。我已下载默认的 `casper.zip` 并在 Windows 上解压到 `Downloads\casper`。

接下来，请确保已安装 LTS 或更高版本的 NodeJS。您可以在此获取：https://nodejs.org/en/download/

NodeJS 安装完成后，请安装一个代码编辑器。

我们推荐（并使用）Webstorm，您可以在此获取试用 30 天（无需信用卡）：https://www.jetbrains.com/webstorm/

下一个最佳的免费选择可能是 Visual Studio Code： https://code.visualstudio.com/download

在编辑器中设置好并打开主题文件夹后，在 IDE 中打开终端并运行：

[inline-code-attrs-start title = '安装主题'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

成功的输出如下（您可以忽略警告）：

<div class="screenshot white-bg">
    <div class="title">成功的 npm install 输出</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="成功的 npm install 输出" />
</div>

这将为后续要运行的命令设置主题的依赖项。此外，导出依赖于已安装主题的依赖项，否则重新导入将无法正常工作。

---