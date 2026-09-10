---
若要在本機查看所有 widget 與 flow 在公共 `demo` 租戶上執行，請克隆此儲存庫並執行以下指令：

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

範例透過 `file:../..` 連結至函式庫，因此需要在根目錄執行一次 `npm run build` 步驟以產生 `dist/`。

每個 widget/flow 在 `examples/example-showcase/src/views/` 目錄下都有自己的視圖，您可以直接將其複製到自己的 React 應用程式中。
---