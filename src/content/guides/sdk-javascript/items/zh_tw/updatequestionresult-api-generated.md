## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Yes |  |

## 回應

返回：[`UpdateQuestionResultResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateQuestionResultResponse.ts)

## 範例

[inline-code-attrs-start title = 'updateQuestionResult 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdate() {
    const tenantId: string = "acme-corp-01";
    const id: string = "qr-20230915-001";

    const updateQuestionResultBody: UpdateQuestionResultBody = {
        // 必填字段
        answer: "No",
        // 可選字段
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