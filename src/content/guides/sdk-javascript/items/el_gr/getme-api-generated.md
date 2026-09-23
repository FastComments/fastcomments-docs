---
Αναγνωρίζει το διαπιστευτήριο σε χρήση: το μισθωτή στο οποίο ανήκει και, για διακριτικά OAuth, τον χρήστη που το εξουσιοδότησε.  
Οι ενσωματώσεις το χρησιμοποιούν για να δοκιμάσουν μια σύνδεση και να την ετικετοποιήσουν.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetMeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMeResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getMe'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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