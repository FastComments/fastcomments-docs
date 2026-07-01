## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createModeratorBody | CreateModeratorBody | Yes |  |

## Réponse

Renvoie : [`CreateModeratorResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'createModerator Exemple'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_9876";
  const moderatorPayload: CreateModeratorBody = {
    name: "Alice Johnson",
    email: "alice.johnson@example.com"
    // les champs optionnels comme la description sont omis
  };
  const result: CreateModeratorResponse1 = await createModerator(tenantId, moderatorPayload);
  console.log(result);
}

runExample();
[inline-code-end]

---