## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Oui |  |
| locale | string | Non |  |

## Réponse

Retourne : [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RenderEmailTemplateResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple renderEmailTemplate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";

const templateBody: RenderEmailTemplateBody = {
  templateId: "welcome-email",
  variables: {
    userName: "Jane Doe",
    signupDate: "2024-04-01"
  }
};

const locale: string = "en-US";

(async () => {
  const rendered: RenderEmailTemplateResponse = await renderEmailTemplate(
    tenantId,
    templateBody,
    locale
  );
  console.log(rendered);
})();
[inline-code-end]