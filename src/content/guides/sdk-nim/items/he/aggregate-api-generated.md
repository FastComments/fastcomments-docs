מאגד מסמכים על‑ידי קיבוץ שלהם (אם מתקבל **groupBy**) והחלת מספר פעולות.  
פעולות שונות (למשל sum, countDistinct, avg, וכד') נתמכות.

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| aggregationRequest | AggregationRequest | לא |  |
| options | AggregateOptions | לא |  |

## תגובה

מחזיר: [`Option[AggregateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_aggregate_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה לאגרגציה'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (aggResp, httpResp) = client.aggregate(tenantId = "my-tenant-123", aggregationRequest = AggregationRequest(), options = AggregateOptions())
if aggResp.isSome:
  let response = aggResp.get()
  echo response
echo httpResp
[inline-code-end]