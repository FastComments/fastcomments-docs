#### Ontwikkelaars-sleutels openen in Canvas

Meld u aan bij Canvas als beheerder. Ga naar **Admin** (in de linkerzijbalk) > selecteer uw account > **Developer Keys**.

#### Maak een LTI Developer Key

Klik op **+ Developer Key** en selecteer **LTI Key**.

In het configuratieformulier:

1. Plak in het veld **Redirect URIs** (linkerkant) de **Launch URL** van de FastComments-instellingspagina.
2. Stel aan de rechterkant **Method** in op **Enter URL**.
3. Plak de **Configuration URL** die u van FastComments gekopieerd hebt in het veld **JSON URL**.
4. Canvas laadt de LTI-configuratie automatisch.
5. Geef de key een naam (bijv. "FastComments").
6. Klik op **Save**.

#### Schakel de Developer Key in

Na het opslaan verschijnt de nieuwe key in de Developer Keys-tabel met de **State** ingesteld op **OFF**. Klik op de schakelaar om deze op **ON** te zetten. Canvas kan u vragen te bevestigen — klik op **Allow** om de key in te schakelen.

#### Kopieer de Client ID

In de Developer Keys-tabel staat in de kolom Details een numerieke **Client ID** (bijv. `17000000000042`). Kopieer dit nummer - u heeft het nodig in de volgende stap.