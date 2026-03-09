#### Otvorite Developer Keys u Canvasu

Prijavite se u Canvas kao administrator. Idite na **Admin** (u levom bočnom meniju) > izaberite svoj nalog > **Developer Keys**.

#### Kreirajte LTI Developer Key

Kliknite **+ Developer Key** i izaberite **LTI Key**.

U obrascu za konfiguraciju:

1. U polje **Redirect URIs** (sa leve strane), nalepite **Launch URL** sa FastComments stranice za podešavanje.
2. Sa desne strane, podesite **Method** na **Enter URL**.
3. Nalepite **Configuration URL** koji ste kopirali iz FastComments u polje **JSON URL**.
4. Canvas će automatski učitati LTI konfiguraciju.
5. Dajte ključu ime (npr. "FastComments").
6. Kliknite **Save**.

#### Omogućite Developer Key

Posle čuvanja, novi ključ će se pojaviti u tabeli Developer Keys sa **State** podešenim na **OFF**. Kliknite prekidač da ga postavite na **ON**. Canvas može tražiti potvrdu — kliknite **Allow** da omogućite ključ.

#### Kopirajte Client ID

Tabela Developer Keys prikazuje numerički **Client ID** u koloni Details (npr. `17000000000042`). Kopirajte ovaj broj - biće vam potreban u narednom koraku.