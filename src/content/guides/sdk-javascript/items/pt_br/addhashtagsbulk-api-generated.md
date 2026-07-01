## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Não |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Não |  |

## Resposta

Retorna: [`AddHashTagsBulkResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulkResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'addHashTagsBulk Exemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
    const tenantId: string | undefined = "tenant_9f8b7c6d";
    const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
        tags: [
            {
                name: "typescript",
                description: "Discussions about TypeScript",
                color: "#3178c6"
            },
            {
                name: "fastcomments",
                description: "Tags for FastComments integration",
                color: "#00aaff"
            }
        ]
    };
    const result: AddHashTagsBulkResponse = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
    console.log(result);
}();
[inline-code-end]