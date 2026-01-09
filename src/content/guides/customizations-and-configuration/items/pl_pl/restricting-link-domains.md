---
Domyślnie FastComments pozwala na umieszczanie linków do dowolnych zewnętrznych stron.

Można to ograniczyć do określonej listy stron lub domen. Próba opublikowania linku do strony lub domeny,
której nie ma na zdefiniowanej liście, spowoduje wyświetlenie użytkownikowi komunikatu o błędzie.

Ta walidacja dotyczy tylko widżetu komentarzy i API. Importy nie są objęte.

To odbywa się bez kodu, na stronie dostosowywania widżetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.restricted-link-domains-list'; selector = '.external-link-settings'; title='Restrict External Link Domains' app-screenshot-end]
---