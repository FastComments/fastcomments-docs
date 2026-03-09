#### Open Developer Keys in Canvas

Prijavite se v Canvas kot skrbnik. Pojdite na **Admin** (v levi stranski vrstici) > izberite svoj račun > **Developer Keys**.

#### Create an LTI Developer Key

Kliknite **+ Developer Key** in izberite **LTI Key**.

V obrazcu za konfiguracijo:

1. V polje **Redirect URIs** (na levi strani) prilepite **Launch URL** s strani za nastavitev FastComments.
2. Na desni nastavite **Method** na **Enter URL**.
3. Prilepite **Configuration URL**, ki ste ga kopirali iz FastComments, v polje **JSON URL**.
4. Canvas bo samodejno naložil konfiguracijo LTI.
5. Ključ poimenujte (npr. "FastComments").
6. Kliknite **Save**.

#### Enable the Developer Key

Po shranjevanju se bo nov ključ prikazal v tabeli Developer Keys z **State** nastavljeno na **OFF**. Kliknite preklopnik, da ga nastavite na **ON**. Canvas vas morda pozove k potrdilu — kliknite **Allow**, da omogočite ključ.

#### Copy the Client ID

V tabeli Developer Keys je v stolpcu Details prikazan numeričen **Client ID** (npr. `17000000000042`). Kopirajte to številko - potrebovali jo boste v naslednjem koraku.