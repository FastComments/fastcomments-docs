## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| banUserUndoParams | BanUserUndoParams | 예 |  |
| tenantId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`PostBanUserUndoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostBanUserUndoResponse.ts)

## 예시

[inline-code-attrs-start title = 'postBanUserUndo 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---