### 開發者 - 強制關閉深色模式

可以通過在小工具配置中將 `hasDarkBackground` 傳遞為 `false` 來強制關閉深色模式。這適用於 VanillaJS、Angular、React、Vue 和 React Native 函式庫。

每個函式庫在 [GitHub](https://github.com/fastComments/) 上都有一個 `examples` 資料夾，其中包含如何使用深色模式的範例。

### 強制開啟深色模式

我們可以通過將 `hasDarkBackground` 設定為 `true` 來強制深色模式始終開啟。

我們也可以通過[這裡](https://fastcomments.com/auth/my-account/customize-widget)的小工具自訂介面來完成此操作。

在 `Base Theme` 下，只需選擇 `Force Dark Mode`。

### VanillaJS 小工具 - 更新深色模式

更新深色模式最簡單的方法是遍歷頁面上小工具的所有實例並更新其配置：

[inline-code-attrs-start title = 'VanillaJS - Dark Mode Toggle'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    let isDarkMode = somehowDetermineIfDarkModeEnabled();
    for (const instanceWrapped of window.fcUIInstances) {
        if (instanceWrapped.targetElement) {
            const config = instanceWrapped.config;
            config.hasDarkBackground = isDarkMode;
            instanceWrapped.instance.update(config)
        }
    }
[inline-code-end]
