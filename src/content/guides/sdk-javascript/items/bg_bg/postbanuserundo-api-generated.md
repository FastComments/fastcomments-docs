## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| banUserUndoParams | BanUserUndoParams | Да |  |
| sso | string | Не |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'postBanUserUndo Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9876";

const undoParams: BanUserUndoParams = {
  userId: "user_12345",
  // допълнителни задължителни полета могат да бъдат добавени тук
};

const resultWithoutSSO: APIEmptyResponse = await postBanUserUndo(tenantId, undoParams);

const ssoToken: string = "sso_abcdef123456";

const resultWithSSO: APIEmptyResponse = await postBanUserUndo(tenantId, undoParams, ssoToken);
[inline-code-end]