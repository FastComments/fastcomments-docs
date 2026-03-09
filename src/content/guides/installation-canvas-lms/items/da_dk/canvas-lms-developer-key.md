#### Åbn Developer Keys i Canvas

Log ind på Canvas som administrator. Gå til **Admin** (i venstre sidepanel) > vælg din konto > **Developer Keys**.

#### Opret en LTI Developer Key

Klik på **+ Developer Key** og vælg **LTI Key**.

I konfigurationsformularen:

1. I feltet **Redirect URIs** (venstre side), indsæt **Launch URL** fra FastComments' opsætningsside.
2. Til højre, indstil **Method** til **Enter URL**.
3. Indsæt **Configuration URL**, som du kopierede fra FastComments, i feltet **JSON URL**.
4. Canvas indlæser automatisk LTI-konfigurationen.
5. Giv nøglen et navn (f.eks. "FastComments").
6. Klik på **Save**.

#### Aktiver Developer Key

Efter gemning vises den nye nøgle i Developer Keys-tabellen med dens **State** sat til **OFF**. Klik på kontakten for at sætte den til **ON**. Canvas kan bede dig om bekræftelse — klik på **Allow** for at aktivere nøglen.

#### Kopier Client ID

Developer Keys-tabellen viser et numerisk **Client ID** i Details-kolonnen (f.eks. `17000000000042`). Kopier dette nummer — du får brug for det i næste trin.