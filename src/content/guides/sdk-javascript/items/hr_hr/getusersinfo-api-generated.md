---
Skupni podaci o korisniku za najmodavca. Dajući userIds, vraća prikazne informacije iz User / SSOUser.  
Koristi se od strane widgeta za komentare kako bi obogatio korisnike koji su se upravo pojavili putem događaja prisutnosti.  
Nema konteksta stranice: privatnost se provodi jednolično (privatni profili su maskirani).

## Parameters

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| ids | string | Da |  |

## Odgovor

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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