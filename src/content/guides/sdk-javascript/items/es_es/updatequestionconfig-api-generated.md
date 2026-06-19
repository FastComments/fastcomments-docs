## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Sí |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-tenant-84f2';
  const id: string = '5d6a8b2f-1c4e-4a7b-9f3d-e2c123456789';
  const customOption: QuestionConfigCustomOptionsInner = { label: 'Helpful', value: 'helpful' };
  const updateQuestionConfigBody: UpdateQuestionConfigBody = {
    enabled: true,
    title: 'Is this information helpful?',
    // parámetro opcional demostrado:
    customOptions: [customOption]
  };
  const result: APIEmptyResponse = await updateQuestionConfig(tenantId, id, updateQuestionConfigBody);
  console.log(result);
})();
[inline-code-end]

---