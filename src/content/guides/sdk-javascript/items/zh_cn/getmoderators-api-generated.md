## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| skip | number | 否 |  |

## 响应

返回：[`GetModeratorsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorsResponse1.ts)

## 示例

[inline-code-attrs-start title = 'getModerators 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchModerators(): Promise<void> {
  const tenantId: string = "tenant-9876";
  const skip: number = 30; // 可选的分页偏移量
  const moderators: GetModeratorsResponse1 = await getModerators(tenantId, skip);
  // 示例（无分页）：
  // const allModerators: GetModeratorsResponse1 = await getModerators(tenantId);
}
[inline-code-end]

---