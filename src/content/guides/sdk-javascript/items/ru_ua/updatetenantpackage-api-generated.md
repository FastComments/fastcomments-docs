## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Да |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример updateTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_sf_001";
  const id: string = "pkg-premium-v2";
  const updateTenantPackageBody: UpdateTenantPackageBody = {
    name: "San Francisco Premium",
    enabled: true,
    customConfig: { maxComments: 500 },
    tosConfig: { required: true } // необязательные поля показаны для примера; остальные опущены
  } as UpdateTenantPackageBody;
  const result: FlagCommentPublic200Response = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
  console.log(result);
})();
[inline-code-end]

---