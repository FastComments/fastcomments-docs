### 开发者 - 强制关闭深色模式

可以通过在小部件配置中将 `hasDarkBackground` 传递为 `false` 来强制关闭深色模式。这适用于 VanillaJS、Angular、React、Vue 和 React Native 库。

每个库在 [GitHub](https://github.com/fastComments/) 上都有一个 `examples` 文件夹，其中包含如何使用深色模式的示例。

### 强制开启深色模式

我们可以通过将 `hasDarkBackground` 设置为 `true` 来强制深色模式始终开启。

我们也可以通过[这里](https://fastcomments.com/auth/my-account/customize-widget)的小部件自定义界面来完成此操作。

在 `Base Theme` 下，只需选择 `Force Dark Mode`。

### VanillaJS 小部件 - 更新深色模式

更新深色模式最简单的方法是遍历页面上小部件的所有实例并更新其配置：

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
