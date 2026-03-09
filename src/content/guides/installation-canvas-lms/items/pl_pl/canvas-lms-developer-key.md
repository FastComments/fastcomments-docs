#### Open Developer Keys in Canvas

Zaloguj się do Canvas jako administrator. Przejdź do **Admin** (na lewym pasku bocznym) > wybierz swoje konto > **Developer Keys**.

#### Create an LTI Developer Key

Kliknij **+ Developer Key** i wybierz **LTI Key**.

W formularzu konfiguracji:

1. W polu **Redirect URIs** (lewa strona) wklej **Launch URL** ze strony konfiguracji FastComments.
2. Po prawej ustaw **Method** na **Enter URL**.
3. Wklej **Configuration URL**, który skopiowałeś z FastComments, do pola **JSON URL**.
4. Canvas automatycznie załaduje konfigurację LTI.
5. Nadaj kluczowi nazwę (np. "FastComments").
6. Kliknij **Save**.

#### Enable the Developer Key

Po zapisaniu nowy klucz pojawi się w tabeli Developer Keys z jego **State** ustawionym na **OFF**. Kliknij przełącznik, aby ustawić go na **ON**. Canvas może poprosić o potwierdzenie — kliknij **Allow**, aby włączyć klucz.

#### Copy the Client ID

W tabeli Developer Keys w kolumnie Details widoczny jest numeryczny **Client ID** (np. `17000000000042`). Skopiuj tę liczbę — będziesz jej potrzebować w następnym kroku.