[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Ta ruta omogoča povabilo enega samega `Moderator`.

The following restrictions exist to send an invite email to a `Moderator`:
- `Moderator` mora že obstajati.
- `fromName` ne sme biti daljši od `100 characters`.

**Opombe:**
- Če uporabnik z navedenim e-poštnim naslovom že obstaja, bo povabljen, da moderira komentarje vašega najemnika.
- Če uporabnik z navedenim e-poštnim naslovom **ne obstaja** bo povezava za povabilo vodila skozi postopek ustvarjanja njihovega računa.
- Povabilo poteče po `30 days`.

Lahko ustvarimo `Moderator` za uporabnika, katerega poznamo le po e-poštnem naslovu:

[inline-code-attrs-start title = 'Primer cURL povabila moderatorja'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

To bo poslalo e-pošto, kot je `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Struktura zahteve za povabilo moderatorja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** E-pošta, poslana uporabniku, se bo prikazovala, kot da je poslana iz tega imena. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora na povabilo moderatorja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Vključeno ob napaki. **/
    reason?: string
}
[inline-code-end]