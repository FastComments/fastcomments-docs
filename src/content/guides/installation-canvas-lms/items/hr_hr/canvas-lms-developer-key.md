#### Open Developer Keys in Canvas

Prijavite se u Canvas kao administrator. Idite na **Admin** (u lijevom bočnom izborniku) > odaberite svoj račun > **Developer Keys**.

#### Create an LTI Developer Key

Kliknite **+ Developer Key** i odaberite **LTI Key**.

U obrascu za konfiguraciju:

1. U polje **Redirect URIs** (s lijeve strane) zalijepite **Launch URL** sa stranice za postavljanje FastComments.
2. S desne strane postavite **Method** na **Enter URL**.
3. Zalijepite **Configuration URL** koji ste kopirali iz FastComments u polje **JSON URL**.
4. Canvas će automatski učitati LTI konfiguraciju.
5. Dajte ključu ime (npr. "FastComments").
6. Kliknite **Save**.

#### Enable the Developer Key

Nakon spremanja, novi će ključ biti prikazan u tablici Developer Keys sa svojim **State** postavljenim na **OFF**. Kliknite prekidač da ga postavite na **ON**. Canvas vas može zatražiti potvrdu — kliknite **Allow** da omogućite ključ.

#### Copy the Client ID

Tablica Developer Keys prikazuje numerički **Client ID** u stupcu Details (npr. `17000000000042`). Kopirajte taj broj - trebat će vam u sljedećem koraku.