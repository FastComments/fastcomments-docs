## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## 响应

返回: [`GetQuestionConfigsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigsResponse1.ts)

## 示例

[inline-code-attrs-start title = 'getQuestionConfigs 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchQuestionConfigs() {
    const tenantId: string = "tenant-9876";

    // 在未提供可选的 'skip' 参数的情况下调用
    const configsWithoutSkip: GetQuestionConfigsResponse1 = await getQuestionConfigs(tenantId);

    // 在提供可选的 'skip' 参数的情况下调用
    const skip: number = 10;
    const configsWithSkip: GetQuestionConfigsResponse1 = await getQuestionConfigs(tenantId, skip);

    console.log(configsWithoutSkip, configsWithSkip);
}
[inline-code-end]

---