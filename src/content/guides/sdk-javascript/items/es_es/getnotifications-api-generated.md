## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| userId | string | No |  |
| urlId | string | No |  |
| fromCommentId | string | No |  |
| viewed | boolean | No |  |
| type | string | No |  |
| skip | number | No |  |

## Respuesta

Devuelve: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotifications200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_8f3b1a2c';
  const userId: string = 'user_42';
  const urlId: string = 'https://news.example.com/articles/2026/01/11/comment-thread';
  const fromCommentId: string = 'cmt_9a7b';
  const viewed: boolean = false;
  const type: string = 'mention';
  const skip: number = 0;
  const response: GetNotifications200Response = await getNotifications(tenantId, userId, urlId, fromCommentId, viewed, type, skip);
  console.log(response);
})();
[inline-code-end]

---