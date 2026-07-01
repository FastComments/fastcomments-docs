## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Yes |  |

## 응답

Returns: [`UpdateQuestionResultResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateQuestionResultResponse.ts)

## 예제

[inline-code-attrs-start title = 'updateQuestionResult 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdate() {
    const tenantId: string = "acme-corp-01";
    const id: string = "qr-20230915-001";

    const updateQuestionResultBody: UpdateQuestionResultBody = {
        // 필수 필드
        answer: "No",
        // 선택 필드
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

---