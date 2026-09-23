תגובות אחרונות בדיוק בצורה שבה משלוחי webhook משתמשים, לבניית אינטגרציות (לדוגמה נתוני דוגמה של Zapier). כל אירוע מספק את אותו אובייקט תגובה, ולכן `event` צריך להיות תקף בלבד.

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| event | WebhookEventName | לא |  |
| limit | number | לא |  |

## תגובה

מחזיר: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getWebhookSamplePayloads'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";

const responseAll: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  "comment.created",
  10
);

const responseWithEvent: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  "comment.created"
);

const responseBasic: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId
);
[inline-code-end]

---