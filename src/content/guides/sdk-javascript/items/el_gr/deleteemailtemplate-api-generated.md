## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'deleteEmailTemplate Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface APIStatus {
  code: number;
  message: string;
}

interface APIEmptyResponse {
  status?: APIStatus;
}

(async () => {
  const tenantId: string = "tenant_12345";
  const templateId: string = "template_987";

  const result: APIEmptyResponse = await deleteEmailTemplate(tenantId, templateId);
})();
[inline-code-end]

---