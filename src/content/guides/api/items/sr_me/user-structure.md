`User` је објекат који представља најчешћи заједнички именитељ свих корисника.

Имајте на уму да у FastComments имамо неколико различитих сценарија коришћења за кориснике:

- Secure SSO
- Simple SSO
- Tenant Users (For example: Administrators)
- Commenters

Овај API је за **Commenters** и кориснике креиране преко **Simple SSO**. У основи, сваки корисник креиран преко вашег сајта може се приступити преко овог API-ја. Tenant Users се такође могу дохватити на овај начин, али ћете добити више информација интеракцијом са `/tenant-users/` API-јем.

За `Secure SSO` користите `/sso-users/` API.

Немате могућност да ажурирате ову врсту корисника. Они су креирали налог преко вашег сајта, па пружамо само основни приступ само за читање, али не можете правити измене. Ако желите да имате овакав ток - морате да подесите `Secure SSO`.

Структура за `User` објекат је следећа:

[inline-code-attrs-start title = 'Структура корисника'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** Ово је такође id који се користи као userId на comment објектима. **/
    id: string
    username: string
    /** Линк до блога коментатора, на пример. **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    avatarSrc?: string
    locale: FastCommentsLocale
    displayLabel?: string
    karma?: number
}
[inline-code-end]