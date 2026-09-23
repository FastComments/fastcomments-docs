人間のサインアップなしで AI エージェント用の新しいトライアルアカウントを作成します。この呼び出しには API キーは不要です。

レスポンスにはテナント ID、REST API および MCP サーバーで即座に使用できる API キー、そしてクレーム URL が含まれます。クレーム URL を担当する人に渡してください。FastComments にログインした状態でその URL を開くと、アカウントがその人に紐付けられます。クレームされていないアカウントとそのキーは作成から 72 時間後に削除されます。クレームされるまで、アカウントは標準のトライアル制限が適用されます。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | はい |  |

## 応答

返却: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## 例

[inline-code-attrs-start title = 'createAgentTenant の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // optional
  planId: 3 // optional
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]