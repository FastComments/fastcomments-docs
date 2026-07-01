## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Sim |  |

## Resposta

Retorna: [`UpdateQuestionResultResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateQuestionResultResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo updateQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdate() {
    const tenantId: string = "acme-corp-01";
    const id: string = "qr-20230915-001";

    const updateQuestionResultBody: UpdateQuestionResultBody = {
        // campos obrigatórios
        answer: "No",
        // campos opcionais
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