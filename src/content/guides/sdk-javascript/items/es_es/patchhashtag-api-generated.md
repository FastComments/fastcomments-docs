## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| tag | string | Sí |  |
| updateHashTagBody | UpdateHashTagBody | No |  |

## Respuesta

Devuelve: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateHashTagResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo patchHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const tag: string = "news";

  // Llamada sin cuerpo opcional
  const responseWithoutBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag);

  // Preparar cuerpo para la actualización
  const updateBody: UpdateHashTagBody = {
    name: "Latest News",
    description: "Tag for the most recent news articles"
  };

  const responseWithBody: UpdateHashTagResponse = await patchHashTag(tenantId, tag, updateBody);
})();
[inline-code-end]