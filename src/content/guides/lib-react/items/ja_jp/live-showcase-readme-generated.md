---
パブリック `demo` テナントに対してローカルで全てのウィジェットとフローを実行するには、リポジトリをクローンして以下を実行してください:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

例は `file:../..` を介してライブラリにリンクしているため、`dist/` を生成するためにルートで `npm run build` を一度実行する必要があります。

`examples/example-showcase/src/views/` 以下に各ウィジェット/フローのビューがあり、それらをそのまま自分の React アプリにコピーできます。
---