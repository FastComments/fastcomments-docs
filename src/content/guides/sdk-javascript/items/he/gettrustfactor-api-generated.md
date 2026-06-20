## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| userId | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserTrustFactorResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTrustFactor'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = '550e8400-e29b-41d4-a716-446655440000';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.VXNlckRhdGE.signature';
const trustFactor: GetUserTrustFactorResponse = await getTrustFactor(userId, sso);
const trustFactorAnonymous: GetUserTrustFactorResponse = await getTrustFactor();
[inline-code-end]

---