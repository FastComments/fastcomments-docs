## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Non |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Non |  |

## Réponse

Renvoie: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple pour addHashTagsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Crée l'identifiant du locataire (paramètre optionnel)
const tenantId: string = "tenant_9f8c2b7a";

// Préparer les entrées de tags individuelles
const tag1: BulkCreateHashTagsBodyTagsInner = {
  name: "product-feedback",
  label: "Product Feedback",
  color: "#1f8a70",
  description: "User suggestions and enhancement requests",
  isActive: true
};

const tag2: BulkCreateHashTagsBodyTagsInner = {
  name: "bug-report",
  label: "Bug Report",
  color: "#d64545",
  description: "User-reported defects and issues",
  isActive: true
};

// Corps pour la création en masse (paramètre optionnel)
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// Appeler la fonction asynchrone globale et affecter le résultat typé
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]

---