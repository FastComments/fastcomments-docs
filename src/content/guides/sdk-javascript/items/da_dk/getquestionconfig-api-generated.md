## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Respons

Returnerer: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfig200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getQuestionConfig Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-92';
const id: string = 'question-2026-07-42';
const response: GetQuestionConfig200Response = await getQuestionConfig(tenantId, id);

function summarize(cfg: GetQuestionConfig200Response, includeDetails?: boolean): string {
  return includeDetails ? 'Question config (detailed)' : 'Question config (summary)';
}

const summary: string = summarize(response);
[inline-code-end]

---