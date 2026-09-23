Adjunte una encuesta a un comentario, o establezca el estado completo de la encuesta que ya tiene.

Las opciones se emparejan por id: una opción enviada con el id de una opción existente conserva sus votos (y toma la nueva etiqueta y posición), una opción enviada sin id se agrega, y las opciones existentes que se omiten de la lista se eliminan junto con los votos emitidos en ellas.

Mantener sin ids de opciones existentes en una encuesta que tiene votos elimina todas ellas, por lo que se necesita `replaceVotes=true`.

## Parameters

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| commentPollPutInput | CommentPollPutInput | Sí |  |
| replaceVotes | boolean | No |  |

## Response

Returns: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Example

[inline-code-attrs-start title = 'Ejemplo putPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---