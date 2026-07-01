## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetV2PageReactsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReactsResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'getV2PageReacts Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetPageReacts(): Promise<void> {
    const tenantId: string = "acme-corp-tenant";
    const urlId: string = "article-2024-06-01";

    const reacts: GetV2PageReactsResponse = await getV2PageReacts(tenantId, urlId);

    // παράδειγμα προαιρετικής πρόσβασης ιδιοτήτων
    const customConfig: CustomConfigParameters | undefined = reacts.customConfig;
    console.log(reacts);
}

demoGetPageReacts();
[inline-code-end]

---