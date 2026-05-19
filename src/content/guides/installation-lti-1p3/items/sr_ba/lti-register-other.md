#### Sakai

Sakai подржава LTI 1.3 Dynamic Registration на верзијама са LTI Advantage. Из Administratorskog radnog простора:

1. Пријавите се као Sakai администратор и отворите **Administratorski radni prostor**.
2. Изаберите **Eksterni alati** > **Instaliraj LTI 1.3 alat**.
3. Залепите FastComments URL за регистрацију (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">преузмите га овде</a>) и пошаљите.
4. Одобрите алат када се успостави веза.

Алат ће се затим појавити под **Eksternim alatima** и одржаваоци сајтова га могу додати на своје сајтове.

#### Schoology

Schoology Enterprise инстанце подржавају LTI 1.3, али доступност Dinamičke registracije варира у зависности од имплементације. Провјерите са вашим Schoology менаџером налога.

Ако Dinamička registracija није доступна на вашој Schoology инстанци, мораћете конфигурисати интеграцију ручно користећи ове крајње тачке:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Након што вам Schoology додели Client ID и Deployment ID, контактирајте подршку FastComments да бисте регистровали конфигурацију на вашем tenant-у.

#### Ostale LTI 1.3 platforme

Било који LMS који прати IMS LTI 1.3 Advantage спецификацију требало би да ради са истим URL-ом за регистрацију (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">преузмите га овде</a>). Потражите поставку означену као "Dinamička registracija", "URL за регистрацију alata", "krajnja tačka za iniciranje registracije alata" или слично.

Ако ваша платформа подржава само ручно подешавање LTI 1.3, користите четири крајње тачке наведене у одељку за Schoology изнад и контактирајте подршку да завршите конфигурацију.