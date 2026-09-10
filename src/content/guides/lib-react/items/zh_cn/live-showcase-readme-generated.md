---
要在本地查看针对公共 `demo` 租户运行的每个小部件和流程，请克隆仓库并运行：

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

示例通过 `file:../..` 链接到库，因此需要在根目录运行一次 `npm run build` 步骤以生成 `dist/`。

每个小部件/流程在 `examples/example-showcase/src/views/` 下都有自己的视图，您可以直接复制到自己的 React 应用中。
---