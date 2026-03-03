[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

С Simple SSO можем да предоставим на коментарния уиджет информация за потребителя, така че той да не трябва да въвежда потребителското си име или имейл, за да коментира.

Можем да конфигурираме Simple SSO по следния начин:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

Потребителят ще бъде влязъл в системата и зад кулисите ще се създаде SSO потребител. Потребителят ще има `createdFromSimpleSSO` зададено на `true`, ако е извлечен от API-то.

Notes: 

- Имейлът е уникалният идентификатор за Simple SSO.
- Предоставянето на имейл при Simple SSO не е задължително, обаче по подразбиране коментарите им ще се показват като „Непотвърден“. <b>Ако не е предоставен имейл, потребителят не може да бъде напълно автентикиран.</b>
- **НОВО** От януари 2022 г.: Потребителските имена не е необходимо да са уникални в целия fastcomments.com
- Simple SSO може автоматично да създава и актуализира SSO потребители, ако е предоставен имейл и потребителят не е бил първоначално създаден чрез Secure SSO.
- Можете да зададете значки за потребителя чрез свойството `badgeConfig`. Масивът `badgeIds` съдържа идентификаторите на глобалните значки, които да се асоциират с потребителя. Масивът `pageBadgeIds` съдържа идентификаторите на значки, обвързани с текущата страница (`urlId`) — тези значки се показват само на страницата, на която са присвоени. Ако `override` е зададено на `true`, това ще замени вече показваните значки (глобалните и страницово-обвързаните се презаписват независимо); ако е `false`, ще се добави към съществуващите значки.