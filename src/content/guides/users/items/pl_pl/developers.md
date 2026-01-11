Dla programistów, których możesz nie chcieć uczynić `Administrators`, rozważ utworzenie użytkownika `Administrator` z następującymi uprawnieniami:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

Ten zestaw uprawnień da programiście wszystko, czego potrzebuje do skonfigurowania FastComments, a także zapewni widoczność w systemie niezbędną do upewnienia się, że działa poprawnie.

Uzasadnienie tych uprawnień jest następujące:

1. **Analytics Admin**: Może służyć do debugowania wykorzystania FastComments.
2. **Customizations Admin**: Będzie wymagane do zastosowania niestandardowego stylowania widżetu komentarzy.
3. **Data Management Admin**: Będzie wymagane do zarządzania importami i eksportami oraz do konfiguracji webhooków.
4. **Comment Moderation Admin**: Umożliwi przeglądanie danych komentarzy, przynajmniej podczas konfiguracji.
5. **API/SSO Admin**: Pozwoli im pobrać API keys bezpośrednio z naszej platformy. Uważamy, że jest to bezpieczniejsze niż gdyby `Administrator` kopiował je dla nich i wysyłał `API Secret` e-mailem, co może nie być zbyt bezpieczne.