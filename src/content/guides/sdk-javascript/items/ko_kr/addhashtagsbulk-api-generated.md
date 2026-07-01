## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | No |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | No |  |

## 응답

반환: [`AddHashTagsBulkResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulkResponse.ts)

## 예시

[inline-code-attrs-start title = 'addHashTagsBulk 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---