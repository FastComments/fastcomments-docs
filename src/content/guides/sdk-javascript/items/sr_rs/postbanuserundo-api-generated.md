## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| banUserUndoParams | BanUserUndoParams | Да |  |
| tenantId | string | Не |  |
| sso | string | Не |  |

## Odgovor

Vraća: [`PostBanUserUndoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostBanUserUndoResponse.ts)

## Primer

[inline-code-attrs-start title = 'postBanUserUndo Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const undoParams: BanUserUndoParams = {
  userId: "user-8421",
  commentId: "comment-6702",
  reason: "Accidental ban"
};

const tenantId: string = "tenant-7f9e";
const ssoToken: string = "sso-4b2c1d";

async function runUndo() {
  const resultAll: PostBanUserUndoResponse = await postBanUserUndo(undoParams, tenantId, ssoToken);
  console.log(resultAll);

  const resultMinimal: PostBanUserUndoResponse = await postBanUserUndo(undoParams);
  console.log(resultMinimal);
}

runUndo();
[inline-code-end]