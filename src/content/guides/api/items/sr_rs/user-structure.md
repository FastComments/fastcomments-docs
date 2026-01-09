`User` је објекат који представља најчешћи именитељ свих корисника.

Имајте на уму да у FastComments имамо низ различитих случајева употребе за кориснике:

- Secure SSO
- Simple SSO
- Tenant Users (На пример: Administrators)
- Commenters

Овај API је за **Commenters** и кориснике креиране путем **Simple SSO**. У основи, сваки корисник креиран преко вашег сајта може се приступити преко овог API-ја. Tenant Users се такође могу преузети на овај начин, али ћете добити више информација интеракцијом са `/tenant-users/` API-јем.

За `Secure SSO` користите `/sso-users/` API.

Не можете ажурирати ове типове корисника. Они су креирали налог преко вашег сајта, тако да пружамо основни приступ само за читање, али не можете вршити измене. Ако желите да имате овакав ток - потребно је да подесите `Secure SSO`.

Структура за `User` објекат је следећа:

[inline-code-attrs-start title = 'Структура објекта User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** Ово је такође id који се користи као userId на comment објектима. **/
    id: string
    username: string
    /** Линк ка блогу коментатора, на пример. **/
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

---