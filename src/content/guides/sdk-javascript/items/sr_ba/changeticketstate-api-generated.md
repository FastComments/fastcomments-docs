## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Da |  |
| id | string | Da |  |
| changeTicketStateBody | ChangeTicketStateBody | Da |  |

## Odgovor

Vraća: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketState200Response.ts)

## Primjer

[inline-code-attrs-start title = 'changeTicketState Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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