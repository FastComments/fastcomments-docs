### テストの実行

```bash
# 環境変数を設定
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# テストを実行
pytest tests/
```

### クライアントの再生成

最新の OpenAPI 仕様から API クライアントを再生成するには:

```bash
./update.sh
```

これにより:
1. 稼働中の FastComments サーバーから最新の OpenAPI 仕様をダウンロードします（またはローカルの openapi.yaml を使用）
2. Python クライアントコードを生成します
3. ディレクトリ構造を平坦化します
4. 冗長な設定ファイルをクリーンアップします