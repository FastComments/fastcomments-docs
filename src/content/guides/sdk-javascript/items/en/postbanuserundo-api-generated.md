## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| banUserUndoParams | BanUserUndoParams | Yes |  |
| sso | string | No |  |

## Response

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Example

[inline-code-attrs-start title = 'postBanUserUndo Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9876";

const undoParams: BanUserUndoParams = {
  userId: "user_12345",
  // additional required fields can be added here
};

const resultWithoutSSO: APIEmptyResponse = await postBanUserUndo(tenantId, undoParams);

const ssoToken: string = "sso_abcdef123456";

const resultWithSSO: APIEmptyResponse = await postBanUserUndo(tenantId, undoParams, ssoToken);
[inline-code-end]
