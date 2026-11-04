#### Otvaranje Developer Keys u Canvasu

Prijavite se u Canvas kao administrator. Idite na **Admin** (u lijevom bočnom izborniku) > odaberite svoj račun > **Developer Keys**.

#### Stvaranje LTI Developer Keya

Kliknite **+ Developer Key** i odaberite **LTI Key**.

U obrascu za konfiguraciju:

1. U polje **Redirect URIs** (lijeva strana) zalijepite **Launch URL** s stranice za postavljanje FastComments.
2. S desne strane postavite **Method** na **Enter URL**.
3. Zalijepite **Configuration URL** koji ste kopirali iz FastComments u polje **JSON URL**.
4. Canvas će automatski učitati LTI konfiguraciju.
5. Dajte ključu naziv (npr. "FastComments").
6. Kliknite **Save**.

#### Omogućavanje Developer Keya

Nakon spremanja, novi ključ će se pojaviti u tablici Developer Keys sa **State** postavljenim na **OFF**. Kliknite prekidač da ga postavite na **ON**. Canvas vas može zatražiti potvrdu — kliknite **Allow** da omogućite ključ.

#### Kopiranje Client ID-a

Tablica Developer Keys prikazuje numerički **Client ID** u stupcu Details (npr. `17000000000042`). Kopirajte ovaj broj - trebat će vam u sljedećem koraku.