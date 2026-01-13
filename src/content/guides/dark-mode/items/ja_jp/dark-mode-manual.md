### 開発者向け - ダークモードを強制的にオフにする

ダークモードを強制的にオフにするには、ウィジェット設定で`hasDarkBackground`を`false`として渡します。これはVanillaJS、Angular、React、Vue、React Nativeライブラリで動作します。

各ライブラリには[GitHub](https://github.com/fastComments/)に`examples`フォルダがあり、ダークモードの使用方法の例が含まれています。

### ダークモードを強制的にオンにする

`hasDarkBackground`を`true`に設定することで、ダークモードを常にオンにすることができます。

これは[こちら](https://fastcomments.com/auth/my-account/customize-widget)のウィジェットカスタマイズUIからも行えます。

`Base Theme`の下で`Force Dark Mode`を選択するだけです。

### VanillaJSウィジェット - ダークモードの更新

ダークモードを更新する最も簡単な方法は、ページ上のウィジェットのすべてのインスタンスを通過し、その設定を更新することです：

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
