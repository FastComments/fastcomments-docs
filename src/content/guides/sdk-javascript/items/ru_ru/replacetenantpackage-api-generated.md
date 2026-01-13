## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Да |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример replaceTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9b72f2';
const packageId: string = 'pkg-prod-v2';
const replaceTenantPackageBody: ReplaceTenantPackageBody = {
  name: 'Premium Moderation Bundle',
  enabled: true,
  maxModerators: 4,
  // необязательные поля, такие как "notes" или "trialExpiry", намеренно опущены здесь
} as ReplaceTenantPackageBody;
const result: FlagCommentPublic200Response = await replaceTenantPackage(
  tenantId,
  packageId,
  replaceTenantPackageBody
);
[inline-code-end]

---