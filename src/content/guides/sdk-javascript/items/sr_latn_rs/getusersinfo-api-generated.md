Skupni podaci o korisnicima za zakupca. Na osnovu userIds, vraća prikazne informacije iz User / SSOUser. Koristi se u vidžetu za komentare kako bi obogatio korisnike koji su se upravo pojavili putem događaja prisutnosti. Bez konteksta stranice: privatnost se primenjuje uniformno (privatni profili su maskirani).

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Odgovor

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Primer

[inline-code-attrs-start title = 'getUsersInfo Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]

---