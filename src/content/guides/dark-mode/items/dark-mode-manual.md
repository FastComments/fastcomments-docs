### For Developers - Forcing Dark Mode Off

Forcing dark mode off can be done by passing `hasDarkBackground` as `false` in the widget configuration. This works for the VanillaJS, Angular, React, Vue, and React Native libraries.

Each library has an `examples` folder on [GitHub](https://github.com/fastComments/) that contains examples on how to use dark mode.

### Forcing Dark Mode On

We can force dark mode to always be on by setting `hasDarkBackground` to `true`.

We can also do this via the Widget Customization UI [here](https://fastcomments.com/auth/my-account/customize-widget).

Under `Base Theme` simply select `Force Dark Mode`.

### VanillaJS Widget - Updating Dark Mode

The easiest way to update dark mode is to go through all instances of the widget on the page, and update their configuration:

[inline-code-attrs-start title = 'VanillaJS - Dark Mode Toggle'; type = 'javascript'; inline-code-attrs-end]
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
