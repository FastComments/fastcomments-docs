## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Да |  |

## Ответ

Возвращает: [`UpdateQuestionResultResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateQuestionResultResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример updateQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdate() {
    const tenantId: string = "acme-corp-01";
    const id: string = "qr-20230915-001";

    const updateQuestionResultBody: UpdateQuestionResultBody = {
        // обязательные поля
        answer: "No",
        // необязательные поля
        comment: "User clarified their response",
        // anotherOptionalField?: значение,
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