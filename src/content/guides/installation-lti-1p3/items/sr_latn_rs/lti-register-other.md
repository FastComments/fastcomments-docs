#### Sakai

Sakai podržava LTI 1.3 Dinamičku registraciju na izdanjima sa LTI Advantage. Iz Administratorskog radnog prostora:

1. Prijavite se kao Sakai administrator i otvorite **Administratorski radni prostor**.
2. Odaberite **Eksterni alati** > **Instaliraj LTI 1.3 alat**.
3. Nalepite URL za registraciju FastComments-a i pošaljite.
4. Odobrite alat kada rukovanje bude završeno.

Alat će se potom pojaviti pod **Eksterni alati** i može biti dodat na sajtove od strane njihovih održavalaca.

#### Schoology

Schoology Enterprise instance podržavaju LTI 1.3, ali dostupnost Dinamičke registracije varira u zavisnosti od implementacije. Obratite se svom Schoology menadžeru naloga.

Ako Dinamička registracija nije dostupna na vašoj Schoology instanci, moraćete da konfigurišete integraciju ručno koristeći sledeće endpoint-e:

- **OIDC URL za prijavu**: `https://fastcomments.com/lti/v1p3/login`
- **URL ciljnog linka**: `https://fastcomments.com/lti/v1p3/launch`
- **URL javnog skupa ključeva (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **URL-ovi za preusmeravanje**: `https://fastcomments.com/lti/v1p3/launch`

Nakon što vam Schoology dodeli Client ID i Deployment ID, kontaktirajte podršku FastComments-a da registruje konfiguraciju na vašem tenantu.

#### Other LTI 1.3 Platforms

Bilo koji LMS koji sledi IMS LTI 1.3 Advantage specifikaciju trebalo bi da radi sa istim registracionim URL-om. Potražite podešavanje označeno kao "Dinamička registracija", "URL za registraciju alata", "endpoint za iniciranje registracije alata", ili slično.

Ako vaša platforma podržava samo ručno podešavanje LTI 1.3, koristite četiri endpoint-a navedena u odeljku Schoology iznad i obratite se podršci da biste finalizovali.