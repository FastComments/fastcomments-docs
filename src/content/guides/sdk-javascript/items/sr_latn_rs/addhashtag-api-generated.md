## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Ne |  |
| createHashTagBody | CreateHashTagBody | Ne |  |

## Odgovor

Vraća: [`AddHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagResponse.ts)

## Primer

[inline-code-attrs-start title = 'addHashTag Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9876";

const newHashTag: CreateHashTagBody = {
  tag: "typescript",
  description: "Discussions about TypeScript"
};

const responseWithTenant: AddHashTagResponse = await addHashTag(tenantId, newHashTag);

const responseWithoutTenant: AddHashTagResponse = await addHashTag(undefined, newHashTag);
[inline-code-end]