## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Да |  |
| id | string | Да |  |
| changeTicketStateBody | ChangeTicketStateBody | Да |  |

## Одговор

Враћа: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketState200Response.ts)

## Примјер

[inline-code-attrs-start title = 'Примјер changeTicketState'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-001";
const userId: string = "user_8742";
const id: string = "TCK-2026-00042";
const changeTicketStateBody: ChangeTicketStateBody = {
  state: "closed",
  comment: "Fixed in backend service; verified by QA",
  notifyFollowers: false
} as ChangeTicketStateBody;
const result: ChangeTicketState200Response = await changeTicketState(tenantId, userId, id, changeTicketStateBody);
[inline-code-end]

---