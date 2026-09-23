## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| page | number | Όχι |  |
| count | number | Όχι |  |
| textSearch | string | Όχι |  |
| byIPFromComment | string | Όχι |  |
| filters | string | Όχι |  |
| searchFilters | string | Όχι |  |
| sorts | string | Όχι |  |
| demo | boolean | Όχι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentsResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getApiComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const page: number = 2;
  const count: number = 20;
  const textSearch: string = "typescript";
  const byIPFromComment: string = "192.168.1.100";
  const filters: string = "spam,offensive";
  const searchFilters: string = "user:john";
  const sorts: string = "date_desc";
  const demo: boolean = true;
  const sso: string = "sso_token_abc";

  const response: ModerationAPIGetCommentsResponse = await getApiComments(
    tenantId,
    page,
    count,
    textSearch,
    byIPFromComment,
    filters,
    searchFilters,
    sorts,
    demo,
    sso
  );
}

fetchComments();
[inline-code-end]