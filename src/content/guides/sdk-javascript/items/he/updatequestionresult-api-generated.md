## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Yes |  |

## תגובה

מחזיר: [`UpdateQuestionResultResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateQuestionResultResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל‑updateQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdate() {
    const tenantId: string = "acme-corp-01";
    const id: string = "qr-20230915-001";

    const updateQuestionResultBody: UpdateQuestionResultBody = {
        // שדות נדרשים
        answer: "No",
        // שדות אופציונליים
        comment: "User clarified their response",
        // anotherOptionalField?: value,
    };

    const result: UpdateQuestionResultResponse = await updateQuestionResult(
        tenantId,
        id,
        updateQuestionResultBody
    );

    console.log(result);
}

runUpdate();
[inline-code-end]