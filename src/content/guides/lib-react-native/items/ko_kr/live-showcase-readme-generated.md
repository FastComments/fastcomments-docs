공개 `demo` 테넌트에 대해 모든 위젯과 플로우를 로컬에서 실행해 보려면 저장소를 클론한 후 다음을 실행하세요:

```bash
yarn bootstrap
cd example
yarn ios       # 또는: yarn android, yarn web
```

쇼케이스 진입점은 `example/src/ShowcaseApp.tsx`입니다 — 모든 위젯, 테마, 플로우를 노출하는 단일 앱입니다.

`yarn web` 타겟은 `react-native-web` + `react-native-web-webview`(WebView를 iframe으로 렌더링)를 사용합니다. 브라우저에서 빠른 시각적 스모크 테스트에 유용하지만, `injectJavaScript` 및 `onShouldStartLoadWithRequest`와 같은 네이티브 전용 WebView API는 웹에서 완전히 동작하지 않습니다.