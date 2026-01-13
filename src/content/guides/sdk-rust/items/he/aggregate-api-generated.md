מבצע אגרגציה על מסמכים על ידי קיבוצם (אם נמסר groupBy) ויישום מספר פעולות.
נתמכות פעולות שונות (למשל sum, countDistinct, avg וכו.).

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| aggregation_request | models::AggregationRequest | כן |  |
| parent_tenant_id | String | לא |  |
| include_stats | bool | לא |  |

## תגובה

מחזיר: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---