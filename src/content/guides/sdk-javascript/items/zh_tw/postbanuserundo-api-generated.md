## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| banUserUndoParams | BanUserUndoParams | Yes |  |
| sso | string | No |  |

## 回應

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'postBanUserUndo 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9876";

const undoParams: BanUserUndoParams = {
  userId: "user_12345",
  // 可以在此加入其他必填欄位
};

const resultWithoutSSO: APIEmptyResponse = await postBanUserUndo(tenantId, undoParams);

const ssoToken: string = "sso_abcdef123456";

const resultWithSSO: APIEmptyResponse = await postBanUserUndo(tenantId, undoParams, ssoToken);
[inline-code-end]