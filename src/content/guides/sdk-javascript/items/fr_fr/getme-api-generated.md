Identifie les informations d’identification en cours d’utilisation : le locataire auquel elles appartiennent et, pour les jetons OAuth, l’utilisateur qui les a autorisées.  
Les intégrations utilisent cela pour tester une connexion et l’étiqueter.

## Parameters

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |

## Response

Retourne : [`GetMeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMeResponse.ts)

## Example

[inline-code-attrs-start title = 'Exemple getMe'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";
  const me: GetMeResponse = await getMe(tenantId);
  const authType: MeAuthType = me.auth.type;
  const scopes: OAuthScope[] = me.auth.scopes ?? [];
  const status: APIStatus = me.status;
})();
[inline-code-end]

---