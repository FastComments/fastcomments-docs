#### Sakai

Sakai подржава LTI 1.3 динамичку регистрацију на издањима која укључују LTI Advantage. Из **Административног радног простора**:

1. Пријавите се као Sakai администратор и отворите **Административни радни простор**.
2. Изаберите **Спољни алати** > **Инсталирај LTI 1.3 алат**.
3. Налепите FastComments URL за регистрацију и пошаљите.
4. Одобрите алат када се руковање заврши.

Алат ће се затим појавити под **Спољни алати** и може бити додат на сајтове од стране њихових одржаваоца.

#### Schoology

Schoology Enterprise instance-и подржавају LTI 1.3, али доступност динамичке регистрације варира у зависности од имплементације. Проверите са вашим Schoology менаџером налога.

Ако динамичка регистрација није доступна на вашој Schoology инстанци, мораћете ручно да конфигуришете интеграцију користећи ове ендпоинте:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Након што вам Schoology достави Client ID и Deployment ID, контактирајте FastComments подршку да региструје конфигурацију на вашем tenant-у.

#### Other LTI 1.3 Platforms

Било који LMS који прати IMS LTI 1.3 Advantage спецификацију треба да ради са истим URL-ом за регистрацију. Потражите подешавање означено као "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint", или слично.

Ако ваша платформа подржава само ручно подешавање LTI 1.3, користите четири ендпоинта наведена у одељку Schoology изнад и контактирајте подршку да окончате процес.