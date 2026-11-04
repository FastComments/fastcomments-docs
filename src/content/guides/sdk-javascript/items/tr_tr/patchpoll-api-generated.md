Anketin sayımlarını koruyarak yerinde düzenleyin: soruyu değiştirin, bir seçeneği yeniden adlandırın, kapatın veya yeniden açın,
ya da seçmenleri kimlerin görebileceğini değiştirin. Seçenekler id ile adreslenir - eklemek, kaldırmak veya yeniden sıralamak için tam listeyi PUT ile gönderin.

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| commentPollPatch | CommentPollPatch | Evet |  |

## Yanıt

Döndürür: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Örnek

[inline-code-attrs-start title = 'patchPoll Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_9876543210";

  const pollPatch: CommentPollPatch = {
    question: "Which new feature would you like to see?",
    options: [
      { id: "opt-1", label: "Dark Mode" },
      { id: "opt-2", label: "Multi-language Support" }
    ]
  };

  const response: SavePollResponse = await patchPoll(tenantId, commentId, pollPatch);
  console.log(response);
})();
[inline-code-end]