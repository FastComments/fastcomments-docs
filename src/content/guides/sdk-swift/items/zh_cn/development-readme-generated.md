### 重新生成客户端

要根据最新的 OpenAPI 规范重新生成 API 客户端：

1. 确保本地运行 FastComments 服务器，地址为 `http://localhost:3001`
2. 运行更新脚本：

```bash
./update.sh
```

这将会：
- 下载最新的 OpenAPI 规范
- 生成 Swift 客户端代码（API 文档位于 client/docs）
- 构建该包以验证一切正常