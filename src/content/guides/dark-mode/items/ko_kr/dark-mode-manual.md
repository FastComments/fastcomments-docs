### 개발자용 - 다크 모드 강제 해제

다크 모드를 강제로 끄려면 위젯 구성에서 `hasDarkBackground`를 `false`로 전달하면 됩니다. 이는 VanillaJS, Angular, React, Vue 및 React Native 라이브러리에서 작동합니다.

각 라이브러리에는 [GitHub](https://github.com/fastComments/)에 다크 모드 사용 방법에 대한 예제가 포함된 `examples` 폴더가 있습니다.

### 다크 모드 강제 활성화

`hasDarkBackground`를 `true`로 설정하여 다크 모드가 항상 켜져 있도록 강제할 수 있습니다.

[여기](https://fastcomments.com/auth/my-account/customize-widget)의 위젯 사용자 지정 UI를 통해서도 이 작업을 수행할 수 있습니다.

`Base Theme` 아래에서 `Force Dark Mode`를 선택하면 됩니다.

### VanillaJS 위젯 - 다크 모드 업데이트

다크 모드를 업데이트하는 가장 쉬운 방법은 페이지의 모든 위젯 인스턴스를 순회하고 구성을 업데이트하는 것입니다:

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
