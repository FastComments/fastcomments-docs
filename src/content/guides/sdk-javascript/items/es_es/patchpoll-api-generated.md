Editar una encuesta en el lugar, manteniendo sus recuentos: cambiar la pregunta, volver a etiquetar una opción, cerrar o reabrirla, o cambiar quién puede ver a los votantes. Las opciones se identifican por id; para añadir, eliminar o reordenarlas, haga PUT de la lista completa.

## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| commentPollPatch | CommentPollPatch | Sí |  |

## Respuesta

Devuelve: [`SavePollResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SavePollResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo patchPoll'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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