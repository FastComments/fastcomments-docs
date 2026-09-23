---
Edytuj ankietę w miejscu, zachowując jej wyniki: zmień pytanie, zmień etykietę opcji, zamknij lub otwórz ją ponownie,
lub zmień, kto może zobaczyć wyborców. Opcje są adresowane po identyfikatorze – aby dodać, usunąć lub zmienić ich kolejność, użyj PUT z pełną listą.

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| commentPollPatch | CommentPollPatch | Tak |  |

## Odpowiedź

Zwraca: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Przykład

[inline-code-attrs-start title = 'patchPoll Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---