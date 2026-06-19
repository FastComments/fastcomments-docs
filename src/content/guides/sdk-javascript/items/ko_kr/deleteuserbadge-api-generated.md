## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptySuccessResponse.ts)

## 예제

[inline-code-attrs-start title = 'deleteUserBadge 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-42';
const badgeId: string = 'badge_9f8b2c1d';
const includeAudit: boolean | undefined = undefined; // 선택적 플래그 (deleteUserBadge에 필요하지 않음)
const result: APIEmptySuccessResponse = await deleteUserBadge(tenantId, badgeId);
[inline-code-end]

---