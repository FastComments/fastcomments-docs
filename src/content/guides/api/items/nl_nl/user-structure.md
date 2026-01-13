`User` is een object dat het meest voorkomende gemeenschappelijke kenmerk van alle gebruikers vertegenwoordigt.

Houd er rekening mee dat we bij FastComments verschillende gebruikssituaties voor gebruikers hebben:

- Secure SSO
- Simple SSO
- Tenant Users (For example: Administrators)
- Commenters

Deze API is voor **Commenters** en gebruikers die via **Simple SSO** zijn aangemaakt. In feite kan elke gebruiker die via uw site is aangemaakt via deze API worden benaderd. Tenant Users kunnen ook op deze manier worden opgehaald, maar u krijgt meer informatie door te werken met de `/tenant-users/` API.

Gebruik voor `Secure SSO` de `/sso-users/` API.

U kunt deze soorten gebruikers niet bijwerken. Ze hebben hun account via uw site aangemaakt, dus we bieden beperkte alleen-lezen toegang, maar u kunt geen wijzigingen aanbrengen. Als u dit soort flow wilt hebben, moet u `Secure SSO` instellen.

De structuur van het `User`-object is als volgt:

[inline-code-attrs-start title = 'Structuur van User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** Dit is tevens de id die als userId wordt gebruikt op commentaarobjecten. **/
    id: string
    username: string
    /** Een link naar bijvoorbeeld de blog van de commenter. **/
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