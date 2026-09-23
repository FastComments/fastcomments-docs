Bir yoruma anket ekleyin veya zaten mevcut olan anketin tam durumunu ayarlayın.

Seçenekler kimlik (id) ile eşleştirilir: mevcut bir seçeneğin kimliğiyle gönderilen bir seçenek oylarını korur (ve yeni etiketi ve konumu alır), kimliği olmadan gönderilen bir seçenek eklenir ve listeden çıkarılan mevcut seçenekler, üzerlerine verilen oylarla birlikte kaldırılır.

Oyları olan bir ankette mevcut seçenek kimlikleri tutulmazsa, hepsi silinir; bu nedenle `replaceVotes=true` gerekir.

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| commentPollPutInput | CommentPollPutInput | Yes |  |
| replaceVotes | boolean | No |  |

## Yanıt

Döndürür: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Örnek

[inline-code-attrs-start title = 'putPoll Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";
const commentId: string = "9876543210";

const optionA: CommentPollOptionInput = { text: "Dark mode" };
const optionB: CommentPollOptionInput = { text: "Light mode" };

const pollInput: CommentPollPutInput = {
  question: "Which UI theme do you prefer?",
  options: [optionA, optionB],
};

const replaceVotes: boolean = true;

const result: SavePollResponse = await putPoll(tenantId, commentId, pollInput, replaceVotes);
[inline-code-end]