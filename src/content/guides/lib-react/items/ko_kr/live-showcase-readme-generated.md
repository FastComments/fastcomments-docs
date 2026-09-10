---
공용 `demo` 테넌트에서 로컬로 실행되는 모든 위젯과 플로우를 보려면, 저장소를 복제하고 다음을 실행하세요:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

예제들은 `file:../..`를 통해 라이브러리에 연결되므로, `dist/`를 생성하기 위해 루트에서 `npm run build` 단계를 한 번 실행해야 합니다.

`examples/example-showcase/src/views/` 아래에 각 위젯/플로우마다 자체 뷰가 있으며, 이를 그대로 복사하여 자신의 React 앱에 사용할 수 있습니다.
---