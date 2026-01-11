---
At definere, hvordan flere brugere interagerer, og teste det, er kompliceret. Her er den følgende specifikation, som vi følger for adgangskontrol,
som du kan bruge til at teste din implementering:

    Side med null gruppe-id'er, bruger med null gruppe-id'er - bør have adgang.
    Side med null gruppe-id'er, bruger med gruppe-id'er - bør have adgang.
    Side med gruppe-id'er, bruger med null gruppe-id'er - bør have adgang.
    Side med gruppe-id'er, bruger med tom liste - bør IKKE have adgang.
    Side med gruppe-id'er, bruger med gruppe-id'er - fællesmængde findes - bør have adgang.
    Side med gruppe-id'er, bruger med gruppe-id'er - ingen fællesmængde - bør IKKE have adgang.
    Side med tom liste af gruppe-id'er (ingen har adgang), bruger med null - bør IKKE have adgang.
    
    SSO-bruger A = Ingen gruppe-id'er defineret (null = fuld adgang).
    SSO-bruger B = Ingen gruppe-id'er defineret (null = fuld adgang).
    A kan @B.
    
    SSO-bruger A = Ingen gruppe-id'er defineret (null = fuld adgang).
    SSO-bruger B = Gruppe-id'er defineret.
    A kan @B.
    
    SSO-bruger A = Gruppe-id'er defineret.
    SSO-bruger B = Ingen gruppe-id'er defineret (null = fuld adgang).
    A kan @B.
    
    SSO-bruger A = Gruppe-id'er = [a].
    SSO-bruger B = Gruppe-id'er = [b].
    A kan IKKE @B.
    
    SSO-bruger A = Gruppe-id'er = [a].
    SSO-bruger B = Gruppe-id'er = [a, b].
    A kan @B.
---