Einen Umfrage an einen Kommentar anhängen oder den vollständigen Zustand der bereits vorhandenen Umfrage festlegen.

Optionen werden anhand der id abgeglichen: Eine Option, die mit der id einer bestehenden Option gesendet wird, behält ihre Stimmen (und übernimmt das neue Label und die Position), eine Option ohne id wird hinzugefügt, und bestehende Optionen, die in der Liste fehlen, werden zusammen mit den auf sie abgegebenen Stimmen entfernt.

Wenn keine bestehenden Option-Ids auf einer Umfrage beibehalten werden, die bereits Stimmen hat, werden alle gelöscht – dafür ist replaceVotes=true erforderlich.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| commentPollPutInput | CommentPollPutInput | Ja |  |
| replaceVotes | boolean | Nein |  |

## Antwort

Gibt zurück: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'putPoll Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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