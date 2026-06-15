## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Ні |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Ні |  |

## Відповідь

Повертає: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад addHashTagsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_corp_987";
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [
    { name: "product-update", description: "Announcements about new product releases", visible: true },
    { name: "customer-support", description: "Customer support related discussions", visible: false }
  ],
  createdBy: "moderator_jane"
};
const resultWithTenant: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
const resultWithoutTenant: AddHashTagsBulk200Response = await addHashTagsBulk(undefined, bulkCreateHashTagsBody);
[inline-code-end]

---