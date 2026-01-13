### 运行测试

```bash
# 设置环境变量
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# 运行测试
pytest tests/
```

### 重新生成客户端

要根据最新的 OpenAPI 规范重新生成 API 客户端：

```bash
./update.sh
```

这将：
1. 从正在运行的 FastComments 服务器下载最新的 OpenAPI 规范（或使用本地 openapi.yaml）
2. 生成 Python 客户端代码
3. 扁平化目录结构
4. 清理多余的配置文件