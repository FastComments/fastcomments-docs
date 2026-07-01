## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| usernameStartsWith | string | Ні |  |
| mentionGroupIds | Array<string> | Ні |  |
| sso | string | Ні |  |
| searchSection | SearchUsersSearchSectionEnum | Ні |  |

## Відповідь

Повертає: [`SearchUsersResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsersResponse1.ts)

## Приклад

[inline-code-attrs-start title = 'searchUsers Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoSearch(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const urlId: string = "article-9876";
    const usernameStartsWith: string = "john";
    const mentionGroupIds: string[] = ["groupA", "groupB"];
    const sso: string = "sso_abc123";
    const searchSection: SearchUsersSearchSectionEnum = SearchUsersSearchSectionEnum.Users;

    const response: SearchUsersResponse1 = await searchUsers(
        tenantId,
        urlId,
        usernameStartsWith,
        mentionGroupIds,
        sso,
        searchSection
    );

    console.log(response);
}

demoSearch();
[inline-code-end]