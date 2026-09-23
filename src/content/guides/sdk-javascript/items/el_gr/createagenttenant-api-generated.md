Δημιουργεί έναν νέο λογαριασμό δοκιμής για έναν AI πράκτορα χωρίς ανθρώπινη εγγραφή. Δεν απαιτείται κλειδί API για την κλήση αυτού.

Η απάντηση περιέχει το tenant id, ένα API key που λειτουργεί αμέσως εναντίον του REST API και του MCP server, καθώς και ένα claim URL. Δώστε το claim URL στο άτομο για το οποίο εργάζεστε: το άνοιγμα του ενώ είστε συνδεδεμένοι στο FastComments συνδέει τον λογαριασμό με αυτόν. Οι μη απαιτημένοι λογαριασμοί και τα κλειδιά τους διαγράφονται 72 ώρες μετά τη δημιουργία. Μέχρι να απαιτηθεί, ο λογαριασμός έχει τα τυπικά όρια δοκιμής.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | Yes |  |

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Example

[inline-code-attrs-start title = 'Παράδειγμα createAgentTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // optional
  planId: 3 // optional
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]

---