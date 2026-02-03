FastComments vam omogućuje zahtijevati od korisnika koji komentiraju prvi put da prihvate vaše Uvjete korištenja prije slanja komentara.

When enabled:
- **Anonymous users** će vidjeti potvrdni okvir za Uvjete korištenja svaki put kada komentiraju
- **Authenticated users** će vidjeti potvrdni okvir samo kod svog prvog komentara, ili kada ažurirate Uvjete korištenja

### Omogućavanje Uvjeta korištenja

Navigate to the widget customization page and enable the "Require Terms of Service acceptance" checkbox:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Prilagodba teksta Uvjeta korištenja

By default, the checkbox displays "I agree to the Terms of Service and Privacy Policy" with links to both documents. You can customize this text per locale if needed:

1. Odaberite "Customize text per locale"
2. Odaberite lokalizaciju iz padajućeg izbornika i unesite vlastiti tekst

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Ažuriranje vaših Uvjeta korištenja

When you update your Terms of Service, set the "Last Updated" date. Users who accepted the TOS before this date will be required to accept again:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### Kako funkcionira

- Vremenska oznaka prihvaćanja Uvjeta korištenja pohranjuje se po korisniku i po komentaru
- Kada korisnik prihvati Uvjete korištenja, datum se bilježi u njihovom korisničkom profilu (po zakupcu)
- Ako postavite datum "Last Updated" koji je nakon datuma prihvaćanja korisnika, morat će ponovno prihvatiti
- Za anonimne korisnike koje nije moguće pratiti, potvrdni okvir pojavljuje se pri svakom slanju komentara