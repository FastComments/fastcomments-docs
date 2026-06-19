## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| updateModeratorBody | UpdateModeratorBody | Tak |  |

## Odpowiedź

Zwraca: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład updateModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-corp-tenant-92';
  const id: string = '9f3b2c1a-4d6e-11ec-81d3-0242ac130003';
  const updateModeratorBody: UpdateModeratorBody = {
    email: 'moderator.lead@acmecorp.com',
    displayName: 'Alex Rivera',
    roles: ['moderator', 'team_lead'],
    active: true,
    notify: true // opcjonalna flaga do powiadomienia moderatora o zmianach
  };
  const result: APIEmptyResponse = await updateModerator(tenantId, id, updateModeratorBody);
  console.log(result);
})();
[inline-code-end]

---