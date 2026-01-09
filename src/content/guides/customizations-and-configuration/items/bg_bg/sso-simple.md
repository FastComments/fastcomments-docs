[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

С Simple SSO можем да предоставим на уиджета за коментари информация за потребителя, така че той да не трябва да въвежда потребителското си име или имейл, за да коментира.

Можем да конфигурираме Simple SSO по следния начин:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]; title = 'Simple SSO'; code-example-end]

Потребителят ще бъде влязъл и зад кулисите ще бъде създаден SSO потребител. Потребителят ще има `createdFromSimpleSSO` зададено като `true`, ако е извлечен от API-то.

Notes: 

- Имейлът е уникалният идентификатор за Simple SSO.
- Предоставянето на имейл с Simple SSO не е задължително, но по подразбиране техните коментари ще се показват като "Непотвърдено". <b>Ако не е предоставен имейл, потребителят не може да бъде напълно удостоверен.</b>
- **НОВО** От януари 2022 г.: Потребителските имена не е необходимо да бъдат уникални в целия fastcomments.com
- Simple SSO може автоматично да създава и обновява SSO потребители, ако е предоставен имейл и потребителят не е бил първоначално създаден чрез Secure SSO.
- Можете да зададете значки за потребителя чрез свойството `badgeConfig`. Масивът `badgeIds` съдържа идентификаторите на значките, които да се асоциират с потребителя. Ако `override` е зададено на `true`, то ще замени всички съществуващи значки, показвани върху коментарите; ако е `false`, то ще добави към вече съществуващите значки.